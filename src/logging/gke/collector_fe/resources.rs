use super::models::{Log};
use vertigo::{get_driver, Resource, Value};
use std::*;
use super::models::JsonError;

pub fn log_list(gcp_idd: &str, resource: Value<Resource<Vec<Log>>>) {
    
    //let gcp_ids: &str = &gcp_idd.unwrap_or_default();

    resource.set(Resource::Loading);
    let url = format!("http://localhost:8888/logs/{gcp_idd}?offset=0");

    get_driver().spawn(async move {
        let response = get_driver().request_get(url).call().await;

        match response.status() {
            Some(200) => {
                let res: Result<Vec<Log>, _> = response.into_data();

                match res {
                    Ok(config_view) => {
                        resource.set(Resource::Ready(config_view));
                    }
                    Err(err) => resource.set(Resource::Error(err)),
                };
            }
            _ => JsonError::handle(response, resource),
        };
    });
}
