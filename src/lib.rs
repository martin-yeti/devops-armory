//! DevOps Armory
//!
//! Library created to improve DevOps work.
//!
//! ## Example Standalone
//! 
//! ```rust
//!     use devops_armory::{
//!         rustible::vm::vm_remote::vm_remote::exec_command_on_remote, 
//!         toml_parser::parser::toml_parser
//!         };
//!     use actix_web;
//! 
//!     #[actix_web::main]
//!     async fn test_function() -> Result<(), std::io::Error> {
//!     
//!         let ssh_user = "user".to_string();
//!         let ssh_key_location = "path_to_your_private_ssh_key".to_string();
//!         let toml_file = "path_to_your_config.toml".to_string();
//!         let toml_data = toml_parser(toml_file);
//!         let f = toml_data.unwrap_or_default();
//!         let g = &f.rustible[0];
//!         let slack_vm_address_list = &g.vm[0].slackware.ip_address_list;
//!         let slack_vm_commands = &g.vm[0].slackware.commands;
//!         exec_command_on_remote(ssh_user, ssh_key_location, slack_vm_address_list.to_vec(), slack_vm_commands.to_vec()).await;
//!         Ok(())
//!     
//!     }
//! ```
//! 
//! ## Example Interactive - Clap Cli
//! 
//! ```rust
//!     use devops_armory::rustible::vm::vm_remote::vm_remote::exec_command_on_remote_cli;
//!     use actix_web;
//!     
//!     #[actix_web::main]
//!     async fn main() -> Result<(), std::io::Error> {
//!     
//!         let x = exec_command_on_remote_cli().await;
//!     
//!         match x {
//!             Ok(s) => s,
//!             Err(e) => println!("{}", e)
//!         }
//!         Ok(())
//!     
//!     }
//! ```

pub mod toml_parser;

pub mod rustible;

#[cfg(test)]
mod tests {

    use crate::rustible::vm::vm_remote::vm_remote::exec_command_on_remote;
    use tokio;

    #[tokio::test]
    async fn check_rustible_standalone() {

        //Test to pass need actual data to process
        //Change user, ssh_key_path, ip_list to real values
        let user = "user".to_string();
        let ssh_key_path = "path_to_your_key".to_string();
        let ip_list = vec!["SERVER_IP_to_connect".to_string()];
        let commands = vec!["ls".to_string(), "pwd".to_string()];

        let test_standalone_result = exec_command_on_remote(user, ssh_key_path, ip_list, commands).await;
        assert!(test_standalone_result.is_ok());
    }

}
