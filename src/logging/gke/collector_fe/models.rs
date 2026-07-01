use vertigo::{AutoJsJson, RequestResponse, Resource, Value};


#[derive(AutoJsJson, Clone, PartialEq)]
pub struct Log {
    pub id: i32,
    pub google_project_id: String,
    pub project_id: String,
    pub region: String,
    pub host: String,
    pub message: String,
    pub time: String,
}

#[derive(AutoJsJson, Clone, PartialEq)]
pub struct Paginate {
    pub total: u64,
    pub totalPages: u64,
    pub nextPage: u64,
    pub data: Vec<Log>,
}

#[derive(Debug, AutoJsJson)]
pub struct JsonError {
    pub status: i32,
    pub r#type: String,
    pub message: String,
    pub debug: String,
    pub reason: String,
}

impl JsonError {
    pub fn handle<T: Clone + PartialEq + 'static>(response: RequestResponse, resource: Value<Resource<T>>) {
        match response.status() {
            Some(status) => {
                let res: Result<JsonError, _> = response.into_data();
                match res {
                    Ok(json_error) => {
                        vertigo::log::error!("Api response error: {:?}", json_error);
                        resource.set(Resource::Error(json_error.message));
                    }
                    Err(err) => {
                        vertigo::log::error!("Api error error: {} {:?}", status, err);
                        resource.set(Resource::Error(err));
                    }
                }
            }
            None => {
                vertigo::log::error!("Connection problem");
                resource.set(Resource::Error("Connection problem".to_string()));
            }
        }
    }

    pub fn handle_opt<T: Clone + PartialEq + 'static>(
        response: RequestResponse,
        resource: Value<Option<Resource<T>>>,
    ) {
        match response.status() {
            Some(status) => {
                let res: Result<JsonError, _> = response.into_data();
                match res {
                    Ok(json_error) => {
                        vertigo::log::error!("Api response error: {:?}", json_error);
                        resource.set(Some(Resource::Error(json_error.message)));
                    }
                    Err(err) => {
                        vertigo::log::error!("Api error error: {} {:?}", status, err);
                        resource.set(Some(Resource::Error(err)));
                    }
                }
            }
            None => {
                vertigo::log::error!("Connection problem");
                resource.set(Some(Resource::Error("Connection problem".to_string())));
            }
        }
    }
}
