// Import the required dependencies.
use serde::Deserialize;
use std::fs;
use std::process::exit;
use toml::from_str;

// Top level struct to hold the TOML data.
#[derive(Deserialize)]
struct Data {
    websocket: Config,
}

// Config struct holds to data from the `[config]` section.
#[derive(Deserialize)]
pub struct Config {
    pub ip: String,
    pub websocket_port: u16,
}

pub fn load_config() -> Config {
    // Variable that holds the filename as a `&str`.
    let filename = "config.toml";

    // Read the contents of the file using a `match` block
    // to return the `data: Ok(c)` as a `String`
    // or handle any `errors: Err(_)`.
    let contents = match fs::read_to_string(filename) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read file `{}`", filename);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    // Use a `match` block to return the
    // file `contents` as a `Data struct: Ok(d)`
    // or handle any `errors: Err(_)`.
    let data: Data = match from_str(&contents) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Unable to load data from `{}`", filename);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    return data.websocket;
}
