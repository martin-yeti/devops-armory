// This example shows how to:
// Setup, create PostgreSQL instance, db, user in GCP

use devops_armory::cloud::gcp::{
    auth::auth::gcp_get_credentials_token_iam,
    sql::{
        create::create_sql_instance, 
        database::{create::create_sql_db, models::SqlDb}, 
        get::get_sql_instance_info, 
        models::{
            SqlInstance, 
            SqlIpConfig, 
            SqlSettings, 
            SqlUserLabels
        }, 
        user::{create::create_sql_user, models::SqlUser}
    }
};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {

    // Token, project, sql instance name
    let token = gcp_get_credentials_token_iam().await.unwrap();
    let project = "some_project_id".to_string();
    let sql_instance_name = "some_pgsql_instance_name".to_string();

    // Get PostgreSQL instance info
    get_sql_instance_info(
        token.clone(), 
        project.clone(), 
        sql_instance_name
    ).await.unwrap();

    // Provide details regarding PostgreSQL instance
    let sql_instance = SqlInstance { 
        name: "Some instance name".to_string(), 
        databaseVersion: "PGSQL version".to_string(), 
        settings: SqlSettings { 
            tier: "vm_type".to_string(), 
            userLabels: SqlUserLabels { 
                created_by: "custom label".to_string() 
            }, 
            ipConfiguration: SqlIpConfig { 
                privateNetwork: "network name".to_string(), 
                sslMode: "SSL mode".to_string(), 
                ipv4Enabled: false, 
                requireSsl: false 
            }, 
            dataDiskType: "PD_SSD_or_PD_HDD".to_string() 
        }, 
        region: "some region".to_string(), 
        project: "some_project_id".to_string(), 
        rootPassword: "default_root_pass".to_string() 
    };

    // Create PostgreSQL instance with above data
    create_sql_instance(
        token.clone(), 
        project.clone(), 
        sql_instance
    ).await.unwrap();

    // Provide details to create PgSQL DB
    let sql_instance_name = "some sql instance".to_string();
    let sql_database = SqlDb { 
        charset: "some charset".to_string(), 
        collation: "some collation".to_string(), 
        instance: "name sql instance".to_string(), 
        project: "some_project_id".to_string(), 
        name: "db_name".to_string() 
    };

    // Create PgSQL DB
    create_sql_db(
        token.clone(), 
        project.clone(), 
        sql_instance_name.clone(), 
        sql_database
    ).await.unwrap();

    // Provide details for PgSQL user
    let sql_user = SqlUser { 
        password: "user pass".to_string(), 
        name: "user name".to_string(), 
        instance: "PostgreSQL instance name".to_string(), 
        project: "project name".to_string() 
    };

    // Create PostgreSQL user with above data
    create_sql_user(
        token, 
        project, 
        sql_instance_name, 
        sql_user
    ).await.unwrap();

    Ok(())

}
