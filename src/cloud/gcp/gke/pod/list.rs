use std::time::Duration;

use awc::{
    Client, Connector,
};

use openssl::ssl::{
    SslConnector, 
    SslMethod, 
    SslVerifyMode
};

use super::models::PodList;

/// Get the pod list from namespace
/// Token, gke endpoint and namespace need to be provided
pub async fn pod_list(
    token: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String
) -> Result<(), std::io::Error> {

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);
    let myconnector = builder.build();
    let client = Client::builder()
        .connector(Connector::new().openssl(myconnector))
        .finish();

    let pod_list = client
        .get(format!("https://{gke_cluster_endpoint}:443/api/v1/namespaces/{gke_cluster_namespace}/pods"))
        .bearer_auth(format!("{token}"))
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .expect("Failed to get pods in current namespace")
        //.body()
        .json::<PodList>()
        .await;


    let pod_list_data = pod_list.unwrap_or_default().items;

    for x in pod_list_data.clone().into_iter() {
        println!("{:?}", x.metadata.name)
    }
    //println!("{:#?}", pod_list_data);

    Ok(())
    
}
