use std::rc::Rc;
use vertigo::{Resource, Value, router::Router, transaction};

use super::resources::log_list;
use super::models::Log;
use super::route::Route;

#[derive(Clone, PartialEq)]
pub struct ApiData {
    pub log_view: Value<Resource<Vec<Log>>>,
    pub filter_state: Rc<LogFilters>,
}

impl ApiData {
    pub fn new(gcp_idd: &str) -> Self {
        let data = Self {
            log_view: Value::new(Resource::Loading),
            filter_state: Rc::new(LogFilters::new()),
        };

        log_list(&gcp_idd, data.log_view.clone());

        data
    }

}

#[derive(Clone)]
pub struct AppState {
    pub google_project_id: Rc<String>,
    pub router: Router<Route>,
    pub api_data: Value<Option<ApiData>>,
}

impl Default for AppState {
    fn default() -> Self {

        let state = Self {
            router: Router::new_history_router(),
            api_data: Value::new(None),
            google_project_id: Rc::new("".to_string()),
        };


        state.create_api_data();


        state
    }
}

impl AppState {

    pub fn create_api_data(&self) {
        transaction(|_ctx| {
                self.api_data
                    .set(Some(ApiData::new(&self.google_project_id ))); // /&self.google_project_id )))
        })
    }
}

#[derive(Clone, PartialEq)]
pub struct LogFilters {
    pub gcp_idd: Value<String>,
    //pub project_idd: Value<String>,
    //pub api_data_logs: Value<Resource<Vec<Log>>>,
}

impl LogFilters {
    pub fn new() -> LogFilters {
        LogFilters {
            gcp_idd: Value::default()

        }
    }

}

