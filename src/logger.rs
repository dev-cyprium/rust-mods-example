use chrono::Local;
use colored::*;

pub fn ts() -> String {
    let current_ts = Local::now();
    let formated_ts = current_ts.format("%Y-%m-%d %H:%M:%S").to_string();
    formated_ts
}

pub fn log(message: &str) {
    let now = ts();
    println!("[{}] {}", now.red(), message);
}
