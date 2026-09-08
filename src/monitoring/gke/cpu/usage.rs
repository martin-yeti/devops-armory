use std::{
    thread::sleep, 
    time
};

use awc::{
    Client, Connector,
    ws::{self, Frame},
};

use futures::StreamExt;
use futures_util::SinkExt as _;

use openssl::ssl::{
    SslConnector,
    SslMethod,
    SslVerifyMode
};

use log::warn;

/// A pod mid-restart/eviction can have its exec session return stderr text
/// (e.g. "No such file or directory") instead of the expected numeric line.
/// Cap how many such responses we tolerate on one connection before giving
/// up, rather than looping forever waiting for data that will never arrive.
const MAX_INVALID_RESPONSES: u32 = 5;

/// Function returning CPU usage by pod in CFGROUP_2
/// Utilize websocket connection
pub async fn cpu_usage_cfgroup2(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_pod_name: String
) -> Result<f64, std::io::Error> {

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);
    let myconnector = builder.build();
    let client = Client::builder()
        .connector(Connector::new().openssl(myconnector))
        .finish();
    loop {
        let Ok((_resp, mut connection2)) = client
                    .ws(format!("https://{gke_cluster_endpoint}:443/api/v1/namespaces/{gke_cluster_namespace}/pods/{gke_pod_name}/exec?command=grep&command=user&command=/sys/fs/cgroup/cpu.stat&stdin=true&stdout=true&stderr=true"))
                    .bearer_auth(format!("{token}"))
                    .connect()
                    .await
                    else {
                        continue;
                    };

        connection2
            .send(ws::Message::Text("SGVsbG8sIHdvcmxkIQ==".into()))
            .await
            .unwrap();

        let mut invalid_responses = 0;

        loop {
            let response = connection2.next().await;
            match response {
                Some(Ok(Frame::Binary(b))) => {
                    let s = String::from_utf8_lossy(&b);
                    let v = s.trim_matches(|c: char| !c.is_alphanumeric());
                    if v.is_empty() {
                        continue;
                    }

                    let cpu_stg = v.split(" ").collect::<Vec<&str>>().get(1).map(|field| field.trim());

                    match cpu_stg.and_then(|field| field.parse::<f64>().ok()) {
                        Some(cpu) => {
                            sleep(time::Duration::from_millis(1000));
                            return Ok(cpu);
                        }
                        None => {
                            invalid_responses += 1;
                            warn!("Unexpected exec response reading cpu.stat, ignoring: {:?}", v);
                            if invalid_responses >= MAX_INVALID_RESPONSES {
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::InvalidData,
                                    format!("Too many invalid exec responses reading cpu.stat, last was: {:?}", v),
                                ));
                            }
                        }
                    }
                }
                Some(Ok(Frame::Close(_c))) => {
                    //println!("Connection closed");
                }
                Some(Ok(_)) => todo!(),
                Some(Err(e)) => {
                    println!("Some error occured: {e}");
                    //continue;
                }
                None => {
                    break;
                }
            }
        }
    }
}

/// Function returning CPU usage by pod in CFGROUP_1
/// Utilize websocket connection
pub async fn cpu_usage_cfgroup1(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_pod_name: String
) -> Result<f64, std::io::Error> {

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);
    let myconnector = builder.build();
    let client = Client::builder()
        .connector(Connector::new().openssl(myconnector))
        .finish();
    loop {
        let Ok((_resp, mut connection2)) = client
                    .ws(format!("https://{gke_cluster_endpoint}:443/api/v1/namespaces/{gke_cluster_namespace}/pods/{gke_pod_name}/exec?command=cat&command=/sys/fs/cgroup/cpu/cpuacct.usage&stdin=true&stdout=true&stderr=true"))
                    .bearer_auth(format!("{token}"))
                    .connect()
                    .await
                    else {
                        continue;
                    };

        connection2
            .send(ws::Message::Text("SGVsbG8sIHdvcmxkIQ==".into()))
            .await
            .unwrap();

        let mut invalid_responses = 0;

        loop {
            let response = connection2.next().await;
            match response {
                Some(Ok(Frame::Binary(b))) => {
                    //println!("{:?}", b);
                    let s = String::from_utf8_lossy(&b);
                    let v = s.trim_matches(|c: char| !c.is_alphanumeric());
                    if v.is_empty() {
                        continue;
                    }

                    match v.parse::<f64>() {
                        Ok(cpu) => {
                            sleep(time::Duration::from_millis(1000));
                            return Ok(cpu);
                        }
                        Err(_) => {
                            invalid_responses += 1;
                            warn!("Unexpected exec response reading cpuacct.usage, ignoring: {:?}", v);
                            if invalid_responses >= MAX_INVALID_RESPONSES {
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::InvalidData,
                                    format!("Too many invalid exec responses reading cpuacct.usage, last was: {:?}", v),
                                ));
                            }
                        }
                    }
                }
                Some(Ok(Frame::Close(_c))) => {
                    //println!("Connection closed");
                }
                Some(Ok(_)) => todo!(),
                Some(Err(e)) => {
                    println!("Some error occured {e}");
                    //continue;
                }
                None => {
                    break;
                }
            }
        }
    }
}
