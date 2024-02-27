use std::time::Duration;

use ipnetwork::Ipv4Network;
use reqwest::{Client, ClientBuilder, Error, Url};
use tokio::{task::JoinError, time::sleep};
use tracing::Level;

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
    for network in ipv4_nets.iter() {
        for addr in network.iter() {
            // tracing::info!("{} {}", network, addr);
            let url = format!("http://{addr}:{}", target_port);
            // self.client.get(url);
            tasks.spawn(client.get(url).send());
            if tasks.len() == 20000 {
                tracing::info!("CHUNCK END");
                while let Some(res) = tasks.join_next().await {
                    match res {
                        Err(_err) => {
                            tracing::debug!("{:?}", _err)
                        }
                        Ok(_r) => match _r {
                            Err(e) => tracing::trace!("Ping Error {:?}", e.status()),
                            Ok(resp) => tracing::trace!("Ping Succes {:?}", resp.status()),
                        },
                    }
                }
            }
        }
    }
    tracing::info!("ALL TASKS QUEUED: {}", tasks.len());
    while let Some(res) = tasks.join_next().await {
        match res {
            Err(_err) => {
                tracing::debug!("{:?}", _err)
            }
            Ok(_r) => match _r {
                Err(e) => tracing::trace!("Ping Error {:?}", e.status()),
                Ok(resp) => tracing::trace!("Ping Succes {:?}", resp.status()),
            },
        }
    }
    tracing::debug!("TASKS LEFT: {}", tasks.len());
}
