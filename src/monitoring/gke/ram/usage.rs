use awc::{
    Client, Connector,
    ws::{
        self, 
        Frame
    },
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

pub async fn mem_usage_in_bytes_cgroup2(
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
                .ws(format!("https://{gke_cluster_endpoint}:443/api/v1/namespaces/{gke_cluster_namespace}/pods/{gke_pod_name}/exec?command=cat&command=/sys/fs/cgroup/memory.current&stdin=true&stdout=true&stderr=true"))
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

                    match v.parse::<f64>() {
                        Ok(mem) => return Ok(mem),
                        Err(_) => {
                            invalid_responses += 1;
                            warn!("Unexpected exec response reading memory.current, ignoring: {:?}", v);
                            if invalid_responses >= MAX_INVALID_RESPONSES {
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::InvalidData,
                                    format!("Too many invalid exec responses reading memory.current, last was: {:?}", v),
                                ));
                            }
                        }
                    }
                }
                Some(Ok(Frame::Close(_c))) => {
                    println!("Connection closed");
                    //continue;
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

pub async fn mem_usage_in_bytes_cgroup1(
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
                .ws(format!("https://{gke_cluster_endpoint}:443/api/v1/namespaces/{gke_cluster_namespace}/pods/{gke_pod_name}/exec?command=cat&command=/sys/fs/cgroup/memory/memory.usage_in_bytes&stdin=true&stdout=true&stderr=true"))
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
            //let mut combined_stream = select_all(&streams);
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
                        Ok(mem) => return Ok(mem),
                        Err(_) => {
                            invalid_responses += 1;
                            warn!("Unexpected exec response reading memory.usage_in_bytes, ignoring: {:?}", v);
                            if invalid_responses >= MAX_INVALID_RESPONSES {
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::InvalidData,
                                    format!("Too many invalid exec responses reading memory.usage_in_bytes, last was: {:?}", v),
                                ));
                            }
                        }
                    }
                }
                Some(Ok(Frame::Close(_c))) => {
                    println!("Connection closed");
                    //continue;
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
