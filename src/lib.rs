use std::os::raw::c_char;
use std::{fs, slice};

use chrono::{SecondsFormat, TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum LogLevel {
    #[serde(rename = "debug")]
    DEBUG,
    #[serde(rename = "info")]
    INFO,
    #[serde(rename = "warn")]
    WARN,
    #[serde(rename = "error")]
    ERROR,
}
#[derive(Debug, Serialize, Deserialize)]
struct Config {
    pub log_level: LogLevel,
    pub name: String,
}
const FLB_FILTER_WASM_CONFIG: &'static str = "flb_filter_wasm_config.yaml";

#[unsafe(no_mangle)]
fn _start() {}

#[unsafe(no_mangle)]
pub extern "C" fn rust_filter(
    _tag: *const c_char,
    _tag_len: u32,
    time_sec: u32,
    time_nsec: u32,
    record: *const c_char,
    record_len: u32,
) -> *const u8 {
    let dt = Utc
        .timestamp_opt(time_sec as i64, time_nsec)
        .unwrap()
        .to_rfc3339_opts(SecondsFormat::Millis, true);

    let test_msg = "aabb";
    let config = parse_config(FLB_FILTER_WASM_CONFIG);

    logger(
        &dt,
        config.log_level,
        &config.name,
        format!("{}", test_msg).as_str(),
    );

    let slice_record = unsafe { slice::from_raw_parts(record as *const u8, record_len as usize) };
    let vrecord = unsafe { str::from_utf8_unchecked(slice_record) };
    vrecord.as_ptr()
}

fn logger(dt: &str, level: LogLevel, title: &str, msg: &str) {
    println!("[{}] [ {:?} ] [ {} ], {}", dt, level, title, msg);
}

fn parse_config(file_name: &str) -> Config {
    let config_string = fs::read_to_string(file_name).expect("Can not open config file");
    serde_yaml::from_str(&config_string).expect("error parsing config")
}
