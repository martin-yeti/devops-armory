use paperclip::actix::{api_v2_errors, api_v2_operation, web};
use serwus::{
    db_pool::{async_read_query, Canceled}, server::json_error::{ErrorBuilder, ResponseFromBuilder}
};

use super::{
    {
        connection::AppData, 
        models::Log,    
        queries::
        {
            self, 
            ListLogsQuery
        },
    }
};


#[api_v2_operation(tags(Config))]
pub async fn filter_logs(
    google_project_id: web::Path<String>,
    query: web::Query<ListLogsQuery>,
    data: web::Data<AppData>,
) -> Result<web::Json<Vec<Log>>, ConfigViewError> {
    let google_project_id = google_project_id.into_inner();
    let params = query.into_inner();
    let items = {
        async_read_query::<_, _, ConfigViewError>(data.db_pool.clone(), move |connection| {
            Ok(queries::get_gcp_logs_by_project_id(
                google_project_id,
                &params.project_id,
                &params.region,
                &params.host,
                &params.message,
                params.date_from,
                params.date_to,
                params.offset,
                connection,
            )?)
        })
        .await?
    };

    Ok(web::Json(items))
}

#[api_v2_errors(
    400,
    description = "Early bad request",
    404,
    description = "No such operator",
    406,
    description = "No such environment",
    500,
    description = "Database error or internal error"
)]
#[derive(Debug, Canceled, ResponseFromBuilder, thiserror::Error)]
pub enum ConfigViewError {
    #[error("Task canceled")]
    Canceled,
    #[error("Database inaccessible: {0}")]
    DbConn(#[from] r2d2::Error),
    #[error("Database error: {0}")]
    DbGetConfigurationFailed(#[from] diesel::result::Error),
    #[error("Env not found or invalid access code")]
    EnvAccess,
    #[error("Config not found")]
    ConfigNotFound,
}

impl From<&ConfigViewError> for ErrorBuilder {
    fn from(value: &ConfigViewError) -> Self {
        match value {
            ConfigViewError::Canceled => ErrorBuilder::internal(value.to_string()),
            ConfigViewError::DbConn(_) => ErrorBuilder::database(value.to_string()),
            ConfigViewError::DbGetConfigurationFailed(_) => {
                ErrorBuilder::database(value.to_string())
            }
            ConfigViewError::EnvAccess => ErrorBuilder::validation_fail(value.to_string()),
            ConfigViewError::ConfigNotFound => ErrorBuilder::validation_fail(value.to_string()),
        }
        .debug(value)
    }
}

