/// Below example demonstrates 
/// how to execute command(s) on remote hosts via CLI
/// CLI command: cargo run -- --host 1.1.1.1, 2.2.2.2 "user" "path_to_your_key" "ls, pwd"

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

