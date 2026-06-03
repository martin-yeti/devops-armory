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

        loop {
            let response = connection2.next().await;
            match response {
                Some(Ok(Frame::Binary(b))) => {
                    let s = String::from_utf8_lossy(&b);
                    let v = s.trim_matches(|c: char| !c.is_alphanumeric());
                    if v.is_empty() {
                        continue;
                    }

                    let cpu_stg = v.split(" ").collect::<Vec<&str>>()[1].trim();

                    let cpu: f64 = cpu_stg.parse().expect("not correct type");

                    sleep(time::Duration::from_millis(1000));

                    return Ok(cpu);
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

                    let cpu: f64 = v.parse().expect("not correct type");

                    sleep(time::Duration::from_millis(1000));

                    return Ok(cpu);
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
