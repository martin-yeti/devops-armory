# devops-armory
Rust toolset to improve DevOps work
It is based on russh, toml and actix-web library.

## How to get started ?
As of end of November 2025, you can use Rust stable channel - 1.91.
Install Rust, create project, then add to Cargo.toml in your Rust Project below line:
```
[dependencies]
actix-web = "4.12.1"
devops-armory = "0.2.2"
```

After that, read below example and that should get you going.

## How does it work ?
It uses toml file as the source data. It is parsed via toml_parser - in case toml file is malformed it will throw error where exactly problem occurs.
You will have to provide function with location of TOML config file, SSH username and SSH private key path like below:

```
use devops_armory::{
    rustible::vm::vm_remote::vm_remote::exec_command_on_remote, 
    toml_parser::parser::toml_parser
};

#[actix_web::main]
async fn test_function() -> Result<(), std::io::Error> {

    let ssh_user = "user".to_string();
    let ssh_key_location = "path_to_your_private_ssh_key".to_string();
    let toml_file = "path_to_your_config.toml".to_string();
    let toml_data = toml_parser(toml_file);
    let f = toml_data.unwrap_or_default();
    let g = &f.rustible[0];
    let slack_vm_address_list = &g.vm[0].slackware.ip_address_list;
    let slack_vm_commands = &g.vm[0].slackware.commands;

    exec_command_on_remote(ssh_user, ssh_key_location, slack_vm_address_list.to_vec(), slack_vm_commands.to_vec()).await?;
    
    Ok(())

}
```

In the above example we are setting user, ssh private key path, and toml config file, then we're reading data from toml file and then execute commands on remote.

Caveat 1 - Commands are arrays, so if you want to execute single command on single host just put proper index in it (arrays in Rust begin with 0)
Caveat 2 - IP addresses are also arrays, so again - if you want to use single address just put proper index in it (see above)


Below you can find interactive example - use below with command line arguments like:

cargo run -- --host 1.1.1.1, 2.2.2.2 "user" "path_to_your_key" "ls, pwd"
```
use devops_armory::rustible::vm::vm_remote::vm_remote::exec_command_on_remote_cli;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {

    let x = exec_command_on_remote_cli().await;

    match x {
        Ok(s) => s,
        Err(e) => println!("{}", e)
    }
    Ok(())

}
```

In toml directory there are 2 example files:
1. example toml config - as a basis for further use (keep in mind, that all fields in the file are required - right now it's statically typed, not yet Option-alized)
2. example json file - JSON representation of toml file, so toml file will be more readable (in case someone is not familiar with TOML)

### Current features:
1 - Rustible - tool to execute command on remote server. These are are executed in separate session - in case you need faciliate work you can use "&&" between commands in toml file.

### Bug reports/Feature requests: 
Please use below formatting for creating issues/bug report/feature requests:
```
[Rustible] Name of your issue/bug/feature
```

