use std::process::exit;

pub fn is_built_in(name: &str) -> bool {
    match name {
        "exit" => true,

        _ => false
    }
}

pub fn run_built_in(name: &str, value: f64) {
    match name  {
        "exit" => exit(value as i32),

        _ => {}
    }
}