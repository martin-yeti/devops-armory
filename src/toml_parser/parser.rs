use std::fs;
use std::process::exit;

use toml;

use super::models::Root;

/// Parse TOML based on Root struct. 
/// File Path location set in parameters
pub fn toml_parser(file_path: String) -> Result <Root, std::io::Error> {

    let filename = file_path.to_string();

    // Read the contents of the file using a `match` block 
    // to return the `data: Ok(c)` as a `String` 
    // or handle any `errors: Err(_)`.
    let contents = match fs::read_to_string(&filename) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(e) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read file: {}: {}", filename, e);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    //println!("{}", contents);

    // Use a match block to return the 
    // file contents as a Root struct: Ok(d)
    // or handle any errors: Err(e).
    let data: Root = match toml::from_str(&contents) {
        // If successful, return data as Root struct.
        // d is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(e) => {
            // Write error to stderr.
            eprintln!("Unable to load data from {}", e);
            // Exit the program with exit code 1.
            exit(1);
        }
    };

    Ok(data)

}

