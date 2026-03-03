use std::path::Path;

use ini::Ini;

/// INI file creator based on provided parameters
/// Can be used in SystemD OS, like Ubuntu or Debian
pub fn ini_parser(
    section_unit: &str,
    description: &str,
    after: &str,
    section_service: &str,
    environment: &str,
    r#type: &str,
    user: &str,
    working_directory: &str,
    exec_start: &str,
    restart: &str,
    restart_delay: &str,
    std_out: &str,
    std_err: &str,
    section_install: &str,
    wanted_by: &str,
    filename: &Path
) {
    // Init INI formatter
    let mut conf = Ini::new();

    conf.with_section(Some(section_unit))
        .set("Description", description)
        .set("After", after);
    conf.with_section(Some(section_service))
        .set("Environment", environment)
        .set("Type", r#type)
        .set("User", user)
        .set("WorkingDirectory", working_directory)
        .set("ExecStart", exec_start)
        .set("Restart", restart)
        .set("RestartSec", restart_delay)
        .set("StandardOutput", std_out)
        .set("StandardError", std_err);
    conf.with_section(Some(section_install))
        .set("WantedBy", wanted_by);
    conf.write_to_file(filename).unwrap();

}
