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

/// Function to get mem inactive file in CGROUP2
/// Token, gke cluster endpoint and namespace need to be provided
pub async fn mem_inactive_file_cgroup2(
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

        loop {
            let response = connection2.next().await;
            match response {
                Some(Ok(Frame::Binary(b))) => {
                    let s = String::from_utf8_lossy(&b);
                    let v = s.trim_matches(|c: char| !c.is_alphanumeric());
                    if v.is_empty() {
                        continue;
                    }

                    let mem: String = v.parse().expect("Incorrect type");

                    let z = mem.split("\n").collect::<Vec<&str>>()[0].trim();

                    let zz = z.split(" ").collect::<Vec<&str>>()[0].trim();
                    let mem_inactive_file: f64 = zz.parse().expect("Parse number error");

                    return Ok(mem_inactive_file);
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

/// Function to get mem inactive file in CGROUP1
/// Token, gke cluster endpoint and namespace need to be provided
pub async fn mem_inactive_file_cgroup1(
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
                .ws(format!("https://{gke_cluster_endpoint}:443/api/v1/namespaces/{gke_cluster_namespace}/pods/{gke_pod_name}/exec?command=cat&command=/sys/fs/cgroup/memory/memory.stat&stdin=true&stdout=true&stderr=true"))
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

                    let mem: String = v.parse().expect("Incorrect type");

                    let z = mem.split("\n").collect::<Vec<&str>>()[37].trim();

                    let zz = z.split(" ").collect::<Vec<&str>>()[1].trim();
                    let mem_inactive_file: f64 = zz.parse().expect("Parse number error");

                    return Ok(mem_inactive_file);
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
