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
/// Located in . directory
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/logging/gke/collector_db/migrations");

pub fn prepare_app_data() -> AppData {
    
    let db_pool = MultiPoolBuilder::default().connect().unwrap();

    // Run db migrations
    let mut conn = db_pool
        .write()
        .expect("Can't access database for migrations");

    conn.run_pending_migrations(MIGRATIONS)
        .expect("Can't run migrations");

    AppData { db_pool }

}

