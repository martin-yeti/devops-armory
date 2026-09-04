use actix_web::{error::Error};

use diesel_migrations::{
    embed_migrations,
    EmbeddedMigrations,
    MigrationHarness
};

use serwus::{
    db_pool::multi::{
        MultiPool,
        MultiPoolBuilder
    },
    server::stats::StatsPresenter,
};

use futures::future::{
    ok as fut_ok,
    Future
};

use std::pin::Pin;
use serde::Serialize;
use diesel::prelude::*;
use diesel::r2d2::{
    self,
    ConnectionManager
};
use std::env;

/// DBPool managed by r2d2
pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Connect to DB
/// Variable DATABASE_URL must be set
pub fn establish_connection() -> PgConnection {

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))

    }

#[derive(Clone)]
pub struct AppData {
    pub db_pool: MultiPool,
}

#[derive(Serialize)]
pub struct MainStats {}

impl StatsPresenter<MainStats> for AppData {
    fn is_ready(&self) -> Pin<Box<dyn Future<Output = Result<bool, Error>>>> {
        Box::pin(fut_ok(true))
    }

    fn get_stats(&self) -> Pin<Box<dyn Future<Output = Result<MainStats, Error>>>> {
        Box::pin(fut_ok(MainStats {}))
    }
}

/// Migrations for PostgreSQL DB
/// Located in src/monitoring/gke/collector_db directory
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/monitoring/gke/collector_db/migrations");

pub fn prepare_app_data() -> AppData {

    let db_pool = MultiPoolBuilder::default().connect().unwrap();

    // Run db migrations
    let mut conn = db_pool
        .write()
        .expect("Can't access database for migrations");

    // This panic is only visible if the caller awaits/logs the task it runs
    // in (e.g. the JoinHandle from `tokio::spawn(setup_server(...))`) —
    // logging here first ensures it shows up even if that's dropped.
    if let Err(err) = conn.run_pending_migrations(MIGRATIONS) {
        log::error!("Can't run pod_metrics migrations: {err}");
        panic!("Can't run pod_metrics migrations: {err}");
    }

    AppData { db_pool }

}
