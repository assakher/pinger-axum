use std::{collections::HashMap, io::Error, net::Ipv4Addr, time::Duration};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::{error::Elapsed, sleep},
};

use crate::{
    config::Address,
    models::{CgPoolsResponse, CgStatusResponse, CgVersionResponse, MetricsInfo},
};

pub async fn looped_ping(
    timeout: Duration,
    ping_period: Duration,
    ipv4_nets: Vec<Address>,
    target_port: u32,
    max_conn: usize,
) {
    let interspersed = intersperse_networks(ipv4_nets);
    loop {
        tracing::info!("Begin pinging cycle");
        cycle_connections(&interspersed, target_port, timeout, max_conn).await;
        tracing::info!("End pinging cycle");
        sleep(ping_period).await;
    }
}

fn intersperse_networks(ipv4_nets: Vec<Address>) -> Vec<Ipv4Addr> {
    let mut interspersed = vec![];
    let mut empty = 0;
    let mut idx = 0;
    let mut a: Vec<Vec<Ipv4Addr>> = ipv4_nets
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
        let mut is_empty = -1;
        for (i, v) in a.iter().enumerate() {
            match v.get(idx) {
                Some(addr) => interspersed.push(addr.clone()),
                None => {
                    empty += 1;
                    is_empty = i as i32;
                }
            }
        }
        idx += 1;
        if is_empty > -1 {
            a.remove(is_empty.abs() as usize);
        }
    }
    interspersed
}

pub async fn cycle_connections(
    ipv4_nets: &Vec<Ipv4Addr>,
    target_port: u32,
    timeout: Duration,
    max_conn: usize,
) {
    let mut tasks = tokio::task::JoinSet::new();
    let mut task_adress: HashMap<String, String> = HashMap::new();

    for network in ipv4_nets {
        let addr = network.to_string();
        let url = format!("{addr}:{target_port}");
        tracing::info!("CONN TO: {url}");
        let r = tasks
            .build_task()
            .name(format!("{addr}").as_str())
            .spawn(tokio::time::timeout(timeout, TcpStream::connect(url)));
        match r {
            Ok(res) => {
                task_adress.insert(res.id().to_string(), addr.to_string());
            }
            Err(e) => tracing::error!("FAILED TO CONNECT TO {addr}; {e:?}"),
        }
        if tasks.len() == max_conn {
            ping(&mut tasks, &task_adress).await;
        }

        ping(&mut tasks, &task_adress).await;
    }
}

async fn ping(
    tasks: &mut tokio::task::JoinSet<Result<Result<TcpStream, Error>, Elapsed>>,
    task_adress: &HashMap<String, String>,
) {
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
                    Ok(stream) => {
                        let results = send_ping_rpc(stream).await;
                        let mut metrics = MetricsInfo::new();
                        if results[2].is_ok() {
                            let version: Result<CgVersionResponse, anyhow::Error> =
                                deserialize_response(results[2].as_ref().unwrap()).await;
                            match version {
                                Ok(version) => metrics.fill_version(version),
                                Err(e) => {
                                    tracing::error!("RESPONSE VERSION READ ERROR: {:?}", e);
                                }
                            }
                        }
                        if results[0].is_ok() {
                            let status: Result<CgStatusResponse, anyhow::Error> =
                                deserialize_response(results[0].as_ref().unwrap()).await;
                            match status {
                                Ok(status) => metrics.fiil_status(status),
                                Err(e) => {
                                    tracing::error!("RESPONSE STATUS READ ERROR: {:?}", e);
                                }
                            }
                        }

                        if results[1].is_ok() {
                            let pools: Result<CgPoolsResponse, anyhow::Error> =
                                deserialize_response(results[1].as_ref().unwrap()).await;
                            match pools {
                                Ok(pools) => metrics.fill_pools(pools),
                                Err(e) => {
                                    tracing::error!("RESPONSE POOLS READ ERROR: {:?}", e);
                                }
                            }
                        }
                        save_metrics(task_addr, metrics);
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

async fn send_ping_rpc(mut stream: TcpStream) -> Vec<Result<Vec<u8>, Error>> {
    tracing::debug!("Sending ping request");
    let commands = vec![
        "{\"command\":\"summary\"}".as_bytes(),
        "{\"command\":\"pools\"}".as_bytes(),
        "{\"command\":\"version\"}".as_bytes(),
    ];
    let mut results: Vec<Result<Vec<u8>, Error>> = Vec::with_capacity(3);
    for c in commands {
        let mut buf = [0; 1024];
        let _res = stream.write(c).await;
        let r = match _res {
            Ok(_) => {
                let mut res: Vec<u8> = vec![];
                while let Ok(_chunck) = stream.read(&mut buf[..]).await {
                    tracing::debug!("Reading response, total length: {}", res.len());
                    res.extend(buf.iter());
                    if res.ends_with(&[0]) {
                        break;
                    }
                    buf.fill(0);
                }
                tracing::debug!("PING SUCCESS {_res:?}");
                Ok(res)
            }
            Err(err) => {
                tracing::debug!("PING ERROR: {:?}", err);
                Err(err)
            }
        };
        buf.fill(0);
        results.push(r)
    }
    stream.shutdown().await.unwrap_or(());
    results
}

async fn deserialize_response<T: serde::de::DeserializeOwned>(
    data: &Vec<u8>,
) -> Result<T, anyhow::Error> {
    let convert = String::from_utf8(data.iter().take_while(|c| **c > 0).map(|c| *c).collect())?;
    let response: T = serde_json::from_str(convert.as_str())?;
    Ok(response)
}

fn save_metrics(task_addr: String, metrics: MetricsInfo) {
    let labels = [
        ("ipAddr", format!("{}", task_addr)),
        ("model", format!("{}", metrics.model)),
        ("loader", format!("{}", metrics.loader)),
        ("dna", format!("{}", metrics.dna)),
        ("mac", format!("{}", metrics.mac)),
        (
            "pool",
            format!(
                "{}",
                metrics
                    .pools
                    .get(0)
                    .and_then(|i| Some(i.pool))
                    .or(Some(0))
                    .unwrap()
            ),
        ),
        (
            "url",
            format!(
                "{}",
                metrics
                    .pools
                    .get(0)
                    .and_then(|i| Some(i.url.as_str()))
                    .or(Some(""))
                    .unwrap()
            ),
        ),
        (
            "status",
            format!(
                "{}",
                metrics
                    .pools
                    .get(0)
                    .and_then(|i| Some(i.status.as_str()))
                    .or(Some(""))
                    .unwrap()
            ),
        ),
        (
            "user",
            format!(
                "{}",
                metrics
                    .pools
                    .get(0)
                    .and_then(|i| Some(i.user.as_str()))
                    .or(Some(""))
                    .unwrap()
            ),
        ),
    ];
    metrics::gauge!("mhs_av", &labels).set(metrics.mhs_av);
    metrics::gauge!("mhs30s", &labels).set(metrics.mhs30s);
    metrics::gauge!("mhs1m", &labels).set(metrics.mhs1m);
    metrics::gauge!("mhs5m", &labels).set(metrics.mhs5m);
    metrics::gauge!("mhs15m", &labels).set(metrics.mhs15m);
    metrics::gauge!("rejected", &labels).set(metrics.rejected as f64);
}
