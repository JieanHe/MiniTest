use clap::{App, Arg};
use log::{debug, error, info};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
struct Config {
    tests: Vec<Test>,
}

#[derive(Debug, Deserialize)]
struct Test {
    name: String,
    cmds: Vec<Cmd>,
}

#[derive(Debug, Deserialize)]
struct Cmd {
    cmd: u32,
    expect_res: i32,
    repeat_time: u32,
    thread_num: u32,
    args: HashMap<String, String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 命令行参数
    let matches = App::new("TTest")
        .version("1.0")
        .author("He Jiean")
        .about("A test tool for executing and verifying commands based on a configuration file")
        .arg(
            Arg::with_name("config")
                .short('f')
                .long("file")
                .value_name("test case file")
                .help("Sets as the test case config file path")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("log")
                .short('l')
                .long("log")
                .value_name("log level")
                .help("Sets as 'info', ' debug', 'error' to control the log level")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let config_path = matches
        .value_of("config")
        .expect("you must specify one test case file 'xxx.toml' by -f!");

    let log_level = matches.value_of("log").unwrap_or("info");

    // 日志级别
    std::env::set_var("RUST_LOG", log_level);
    env_logger::init();

    // 输入检查
    let config_content = fs::read_to_string(config_path)?;
    if config_content.is_empty() {
        panic!("input file {} is empty, do nothing!", config_path);
    }

    let config: Config = toml::from_str(&config_content)?;
    if config.tests.is_empty() {
        panic!("no test cases be find, do nothing!");
    }

    // 执行用例
    for test in config.tests {
        info!("Starting run test case: {}", test.name);
        for cmd in test.cmds {
            debug!(
                "Executing cmd: {} Expect result: {}, Repeat time: {}, Thread num: {}",
                cmd.cmd, cmd.expect_res, cmd.repeat_time, cmd.thread_num
            );
            for (key, value) in &cmd.args {
                debug!("Arg {}: {}", key, value);
            }
            error!("Run test case {}.cmd{} failed!", test.name, cmd.cmd);
        }
    }

    Ok(())
}
