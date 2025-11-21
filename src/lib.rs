//! DevOps Armory
//!
//! Library created to improve DevOps work.
//!
//! ## Example
//! 
//! ```rust
//!     #[actix_web::main]
//!     async fn test_function() -> Result<()> {
//!     
//!         let ssh_user = "user".to_string();
//!         let ssh_key_location = "path_to_your_private_ssh_key".to_string();
//!         let toml_file = "path_to_your_config.toml".to_string();
//!         let toml_data = toml_parser(toml_file);
//!         let f = toml_data.unwrap_or_default();
//!         let g = &f.rustible[0];
//!         let slack_vm_address_list = &g.vm[0].slackware.ip_address_list;
//!         let slack_vm_commands = &g.vm[0].slackware.commands;
//!         exec_command_on_remote(ssh_user, ssh_key_location, slack_vm_address_list.to_vec(), slack_vm_commands.to_vec()).await?;
//!         Ok(())
//!     
//!     }
//! ```

pub mod toml_parser;

pub mod rustible;

pub fn test_vm_ubuntu(ip: String, command: String) -> (String, String) {
    (ip.to_string(), command.to_string())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_vm_ubuntu_config() {
        let test_data_vm_ubuntu = test_vm_ubuntu("1.1.1.1".to_string(), "command_ubuntu1".to_string());
        assert_eq!(test_data_vm_ubuntu, ("1.1.1.1".to_string(), "command_ubuntu1".to_string()));
    }

}
