use super::models::Log;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use paperclip::actix::Apiv2Schema;
use super::schema::logs;


#[derive(Apiv2Schema, Deserialize, Serialize)]
pub struct ListLogsQuery {
    pub offset: i64,
}

pub fn get_gcp_logs_by_project_id (

    google_project_id: &str,
    offset: i64,
    connection: &mut PgConnection,

    ) -> QueryResult<Vec<Log>> {
        logs::table
            .filter(logs::google_project_id.eq(google_project_id))
            //.paginate(params.page)
            //.per_page(params.per_page)
            .order_by(logs::id.asc())
            .limit(10000)
            .offset(offset)
            //.paginate(params.page)
            //.per_page(params.per_page)
            .load(connection)

}

