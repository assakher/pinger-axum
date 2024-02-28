use std::time::Duration;

use ipnetwork::Ipv4Network;
use reqwest::{Client, ClientBuilder};
use tokio::time::sleep;

pub async fn looped_ping(timeout: Duration, ipv4_nets: Vec<Ipv4Network>, target_port: u32) {
    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(10))
        .tcp_keepalive(None)
        .pool_max_idle_per_host(1)
        .no_proxy()
        .build()
        .expect("Should not have TLS");
    loop {
        tracing::info!("Begin pinging cycle");
        send(&client, &ipv4_nets, target_port).await;
        tracing::info!("End pinging cycle");
        sleep(timeout).await;
    }
}
pub async fn send(client: &Client, ipv4_nets: &Vec<Ipv4Network>, target_port: u32) {
    let mut tasks = tokio::task::JoinSet::new();
    let mut internal_err = 0;
    let mut success = 0;
    let mut connect_err = 0;
    let mut bad_response = 0;
    for network in ipv4_nets.iter() {
        for addr in network.iter() {
            let url = format!("http://{addr}:{}", target_port);
            tasks.spawn(client.get(url).send());
            if tasks.len() == 20000 {
                tracing::info!("CHUNCK END");
                while let Some(res) = tasks.join_next().await {
        match res {
            Err(_err) => {
                tracing::error!("{:?}", _err);
                internal_err += 1;
            }
            Ok(_r) => match _r {
                Err(e) => {
                    if e.is_connect() {
                        connect_err += 1
                    }
                    else if e.status().is_none() {
                        connect_err += 1
                    } else {
                        bad_response += 1
                    };
                    // tracing::trace!("Ping Error {:?}", e.status());
                }
                Ok(resp) => {
                    tracing::trace!("Ping Succes {:?}", resp.status());
                    success += 1;
                }
            },
        };
    }
            }
        }
    }
    tracing::info!("ALL TASKS QUEUED: {}", tasks.len());
    while let Some(res) = tasks.join_next().await {
        match res {
            Err(_err) => {
                tracing::error!("{:?}", _err);
                internal_err += 1;
            }
            Ok(_r) => match _r {
                Err(e) => {
                    if e.is_connect() {
                        connect_err += 1
                    } else {
                        bad_response += 1
                    };
                    // tracing::trace!("Ping Error {:?}", e.status());
                }
                Ok(resp) => {
                    tracing::trace!("Ping Succes {:?}", resp.status());
                    success += 1;
                }
            },
        };
    }
    metrics::gauge!("reached").set(success as f64);
    metrics::gauge!("failed_to_reach", "reason" => "internal_error").set(internal_err as f64);
    metrics::gauge!("failed_to_reach", "reason" => "bad_response").set(bad_response as f64);
    metrics::gauge!("failed_to_reach", "reason" => "connection_failed").set(connect_err as f64);
}
