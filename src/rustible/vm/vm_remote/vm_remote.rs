use crate::rustible::vm::{
    client::client::Session,
    models::Cli};
use clap::Parser;
use anyhow::{Ok, Result};

/// Function to interact with remote server
pub async fn exec_command_on_remote(user: String, ssh_key_path: String, ip_list: Vec<String>, commands: Vec<String>) -> Result<()> {

    let username = user.to_string();
    let private_key = ssh_key_path.to_string();

    let ip_s = ip_list.clone(); 
    let command_s = commands.clone();

    for i in ip_s.iter() {
        for c in command_s.iter() {
            println!("{:#?}", i);
            let mut ssh = Session::connect(
                private_key.clone(),
                username.clone(),
                (i.to_string(), 22),
                ).await?;
            let code = ssh
                .call(
                    c
                ).await?;
            println!("Exitcode: {:?}", code);
        }
    }

    Ok(())

}

/// Function to interact with remote server via cli
/// Host and Commands are Vec<String>
/// Additional arguments are separated by ","
pub async fn exec_command_on_remote_cli() -> Result<()> {

    let cli = Cli::parse();

    let username = cli.username;
    let private_key = cli.private_key;

    let ip_s = cli.host; 
    let command_s = cli.command;

    for i in ip_s.iter() {
        for c in command_s.iter() {
            println!("{:#?}", i);
            let mut ssh = Session::connect(
                private_key.clone(),
                username.clone(),
                (i.to_string(), 22),
                ).await?;
            let code = ssh
                .call(
                    c
                ).await?;
            println!("Exitcode: {:?}", code);
        }
    }

    Ok(())

}
