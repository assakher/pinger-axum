use std::{collections::HashMap, net::Ipv4Addr, time::Duration};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::sleep,
};

use crate::{config::Address, models::CgStatusResponse};

fn intersperse_networks(ipv4_nets: Vec<Address>) -> Vec<Ipv4Addr> {
    let mut interspersed = vec![];
    let mut empty = 0;
    let mut idx = 0;
    let a: Vec<Vec<Ipv4Addr>> = ipv4_nets
        .iter()
        .map(|i| {
            i.addr
                .iter()
                .filter(|j| {
                    let last = j.octets()[3];
                    last >= i.offset && last <= i.limit
                })
                .collect()
        })
        .collect();
    let subnets_len = a.len();
    while empty < subnets_len {
        for v in &a {
            match v.get(idx) {
                Some(addr) => interspersed.push(addr.clone()),
                None => empty += 1,
            }
        }
        idx += 1
    }
    interspersed
}

fn handle() {}

// TODO: rename
async fn process_tcp(mut stream: TcpStream) -> Result<Vec<u8>, anyhow::Error> {
    // stream.set_nonblocking(true)?;
    let _res = stream.write("{\"command\":\"summary\"}".as_bytes()).await?;
    let mut res: Vec<u8> = vec![];
    let mut buf = [0; 4096];
    while let Ok(_chunck) = stream.read(&mut buf[..]).await {
        res.extend(buf.iter());
        buf = [0; 4096];
    }
    stream.shutdown().await.unwrap_or(());
    Ok(res)
}

// TODO: nedds better name
async fn deserialize(data: Vec<u8>) -> Result<CgStatusResponse, anyhow::Error> {
    // TODO: replace with take_while
    // TODO: remove double sanitizing
    let convert = String::from_utf8(data.into_iter().filter(|c| c > &0).collect())?;
    println!("STRINGIFIED: {convert}");

    let response: CgStatusResponse =
        serde_json::from_str(convert.trim_end_matches(char::is_whitespace))?;
    Ok(response)
}

// async fn send_metrics(task_addr: String, resp: Response) {
//     metrics::gauge!("pingError", "ipAddr" => task_addr, "reason" => err.kind().to_string());
// }

// TODO: reorganize function order
// put outer functions above
pub async fn looped_ping(
    timeout: Duration,
    ping_period: Duration,
    ipv4_nets: Vec<Address>,
    target_port: u32,
) {
    let interspersed = intersperse_networks(ipv4_nets);
    loop {
        tracing::info!("Begin pinging cycle");
        send(&interspersed, target_port, timeout).await;
        tracing::info!("End pinging cycle");
        sleep(ping_period).await;
    }
}

// TODO: refactor different logic for buffer and outer loops
//  maybe replace queue limit with something more sophisticated
pub async fn send(ipv4_nets: &Vec<Ipv4Addr>, target_port: u32, _timeout: Duration) {
    let mut tasks = tokio::task::JoinSet::new();
    let mut task_adress: HashMap<String, String> = HashMap::new();

    for network in ipv4_nets {
        let addr = network.to_string();
        let url = format!("{addr}:{target_port}");
        tracing::info!("CONN TO: {url}");
        let r = tasks
            .build_task()
            .name(format!("{addr}").as_str())
            .spawn(tokio::time::timeout(
                Duration::from_secs(10),
                TcpStream::connect(url),
            ));
        match r {
            Ok(res) => {
                task_adress.insert(res.id().to_string(), addr.to_string());
            }
            Err(_) => (),
        }
        // FIXME: local testing, return 20000
        if tasks.len() == 1 {
            while let Some(res) = tasks.join_next_with_id().await {
                let unwrap_join = match res {
                    Err(_err) => {
                        tracing::error!("JOIN ERROR: {:?}", _err);
                        None
                    }
                    Ok((task_id, _r)) => Some((task_id, _r)),
                };
                if unwrap_join.is_some() {
                    let (task_id, task) = unwrap_join.unwrap();
                    let task_addr = task_adress[&task_id.to_string()].clone();
                    match task {
                        Err(e) => {
                            tracing::error!("ELAPSED: {:?}", e);
                            metrics::gauge!("pingError", "ipAddr" => task_addr, "reason" => "connectionTimeout").set(1.0);
                        }
                        Ok(resp) => match resp {
                            Ok(mut stream) => {
                                let r = process_tcp(stream).await;
                                if r.is_ok() {
                                    // TODO: put resp data into prometheus
                                    let resp = deserialize(r.unwrap()).await;
                                    // TODO: probably needs macros to avoid boilerplate and use introscpection
                                    match resp {
                                        Ok(res) => tracing::info!("RESPOPNSE: {:?}", res),
                                        Err(err) => {
                                            tracing::error!("RESPONSE READ ERROR: {:?}", err)
                                        }
                                    }
                                }
                            }
                            Err(err) => {
                                tracing::error!("ERR: {:?}", err);
                                metrics::gauge!("pingError", "ipAddr" => task_addr, "reason" => err.kind().to_string());
                            }
                        },
                    }
                }
            }
        }
    }

    while let Some(res) = tasks.join_next_with_id().await {
        let unwrap_join = match res {
            Err(_err) => {
                tracing::error!("JOIN ERROR: {:?}", _err);
                None
            }
            Ok((task_id, _r)) => Some((task_id, _r)),
        };
        if unwrap_join.is_some() {
            let (task_id, task) = unwrap_join.unwrap();
            let task_addr = task_adress[&task_id.to_string()].clone();
            match task {
                Err(e) => {
                    tracing::error!("ELAPSED: {:?}", e);
                    metrics::gauge!("pingError", "ipAddr" => task_addr, "reason" => "connectionTimeout").set(1.0);
                }
                Ok(resp) => match resp {
                    Ok(mut stream) => {
                        stream.shutdown().await.unwrap_or(());
                    }
                    Err(err) => {
                        tracing::error!("ERR: {:?}", err);
                        metrics::gauge!("pingError", "ipAddr" => task_addr, "reason" => err.kind().to_string());
                    }
                },
            }
        }
    }
}
