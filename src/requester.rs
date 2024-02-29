use std::{collections::HashMap, time::Duration};

use tokio::{io::AsyncWriteExt, net::TcpStream, time::sleep};

use crate::config::Address;

pub async fn looped_ping(
    timeout: Duration,
    ping_period: Duration,
    ipv4_nets: Vec<Address>,
    target_port: u32,
) {
    loop {
        tracing::info!("Begin pinging cycle");
        send(&ipv4_nets, target_port, timeout).await;
        tracing::info!("End pinging cycle");
        sleep(ping_period).await;
    }
}

pub async fn send(ipv4_nets: &Vec<Address>, target_port: u32, _timeout: Duration) {
    let mut tasks = tokio::task::JoinSet::new();
    let mut task_adress: HashMap<String, String> = HashMap::new();
    for network in ipv4_nets.iter() {
        for addr in network.addr.iter().filter(|i| {
            let last = i.octets()[3];
            last >= network.offset && last <= network.limit
        }) {
            let url = format!("{addr}:{target_port}");
            let r =
                tasks
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
