use std::process::exit;

/**
 * Built in functions
 */

pub fn run_built_int(name: &str, value: f64) {
    match name  {
        "exit" => exit(value as i32),

        _ => {}
    }
}