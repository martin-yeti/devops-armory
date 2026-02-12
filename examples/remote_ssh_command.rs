/// Example using exec command on remote 
/// Custom function and call of that function in main function
/// Below example demonstrates how to use parsing TOML file
/// and execute on remote hosts command(s)

use devops_armory::{
    rustible::vm::vm_remote::vm_remote::exec_command_on_remote, 
    toml_parser::parser::toml_parser
};

async fn remote_ssh_command() -> Result<(), std::io::Error> {

    // Set user
    let ssh_user = "user".to_string();
    
    // Select path to your private SSH key
    let ssh_key_location = "path_to_your_private_ssh_key".to_string();
    
    // Select path to your TOML file
    let toml_file = "path_to_your_config.toml".to_string();
    
    // Parse TOML file and check for errors
    let toml_data = toml_parser(toml_file);
    let f = toml_data.unwrap_or_default();
    
    // Select 1st field from rustible section
    let g = &f.rustible[0];

    // Select IP from Rustible -> VM -> Slackware
    let slack_vm_address_list = &g.vm[0].slackware.ip_address_list;
    
    // Select Commands from Rustible -> VM -> Slackware
    let slack_vm_commands = &g.vm[0].slackware.commands;

    // Execute above or throw error
    exec_command_on_remote(
        ssh_user, 
        ssh_key_location, 
        slack_vm_address_list.to_vec(), 
        slack_vm_commands.to_vec()
    ).await.unwrap();
    
    Ok(())

}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {

    remote_ssh_command().await.unwrap();
    Ok(())
    
}
