use crate::cmd::requestsample::new_requestsample_cmd;
use crate::cmd::{new_config_cmd, new_multi_cmd, new_start_cmd, new_stop_cmd};
use crate::commons::CommandCompleter;
use crate::commons::SubCmd;

use crate::configure::set_config_file_path;
use crate::configure::{self, get_config, get_config_file_path};
use crate::request::{req, ReqResult, Request, RequestTaskListAll};
use crate::{configure::set_config, httpserver, interact};
use clap::{App, AppSettings, Arg, ArgMatches};
use lazy_static::lazy_static;
use log::info;

use std::borrow::Borrow;
use std::{env, fs, thread};

use crate::cmd::loopcmd::new_loop_cmd;
use chrono::prelude::Local;
use fork::{daemon, Fork};
use std::fs::File;
use std::io::Read;
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::{Pid, ProcessExt, RefreshKind, System, SystemExt};

lazy_static! {
    static ref CLIAPP: clap::App<'static> = App::new("serverframe-rs")
        .version("1.0")
        .author("Shiwen Jia. <jiashiwen@gmail.com>")
        .about("command line sample")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .about("Sets a custom config file")
                .takes_value(true)
        )
        .arg(
            Arg::new("interact")
                .short('i')
                .long("interact")
                .about("run as interact mod")
           // .conflicts_with("daemon")
        )
        .arg(
            Arg::new("v")
                .short('v')
                .multiple_occurrences(true)
                .takes_value(true)
                .about("Sets the level of verbosity")
        )
        .subcommand(new_requestsample_cmd())
        .subcommand(
            new_start_cmd().arg(
                Arg::new("daemon")
                    .short('d')
                    .long("daemon")
                    .about("run as daemon")
            )
        )
        .subcommand(new_stop_cmd())
        .subcommand(new_config_cmd())
        .subcommand(new_multi_cmd())
        .subcommand(new_loop_cmd());
    static ref SUBCMDS: Vec<SubCmd> = subcommands();
}

pub fn run_app() {
    // fs::write("pid", std::process::id().to_string());
    let matches = CLIAPP.clone().get_matches();
    if let Some(c) = matches.value_of("config") {
        println!("config path is:{}", c);
        set_config_file_path(c.to_string());
    }
    set_config(&get_config_file_path());
    cmd_match(&matches);
}

pub fn run_from(args: Vec<String>) {
    match App::try_get_matches_from(CLIAPP.to_owned(), args.clone()) {
        Ok(matches) => {
            cmd_match(&matches);
        }
        Err(err) => {
            err.print().expect("Error writing Error");
        }
    };
}

// 获取全部子命令，用于构建commandcompleter
pub fn all_subcommand(app: &App, beginlevel: usize, input: &mut Vec<SubCmd>) {
    let nextlevel = beginlevel + 1;
    let mut subcmds = vec![];
    for iterm in app.get_subcommands() {
        subcmds.push(iterm.get_name().to_string());
        if iterm.has_subcommands() {
            all_subcommand(iterm, nextlevel, input);
        } else {
            if beginlevel == 0 {
                all_subcommand(iterm, nextlevel, input);
            }
        }
    }
    let subcommand = SubCmd {
        level: beginlevel,
        command_name: app.get_name().to_string(),
        subcommands: subcmds,
    };
    input.push(subcommand);
}

pub fn get_command_completer() -> CommandCompleter {
    CommandCompleter::new(SUBCMDS.to_vec())
}

fn subcommands() -> Vec<SubCmd> {
    let mut subcmds = vec![];
    all_subcommand(CLIAPP.clone().borrow(), 0, &mut subcmds);
    subcmds
}

pub fn process_exists(pid: &i32) -> bool {
    let mut sys = System::new_all();
    sys.refresh_all();
    for (syspid, _) in sys.processes() {
        if syspid == pid {
            return true;
        }
    }
    return false;
}

fn cmd_match(matches: &ArgMatches) {
    let config = get_config().unwrap();
    let server = &config["server"];
    let req = Request::new(server.clone());

    if matches.is_present("interact") {
        interact::run();
        return;
    }

    if let Some(ref matches) = matches.subcommand_matches("loop") {
        let term = Arc::new(AtomicBool::new(false));
        let sigint_2 = Arc::new(AtomicBool::new(false));
        signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term)).unwrap();
        signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&sigint_2)).unwrap();
        loop {
            if sigint_2.load(Ordering::Relaxed) {
                println!("{}", "singint signal recived");
                break;
            }
            thread::sleep(Duration::from_millis(1000));
            if term.load(Ordering::Relaxed) {
                println!("{:?}", term);
                break;
            }
            let dt = Local::now();
            fs::write("timestamp", dt.timestamp_millis().to_string());
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("requestsample") {
        if let Some(_) = matches.subcommand_matches("baidu") {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let async_req = async {
                let result = req::get_baidu().await;
                println!("{:?}", result);
            };
            rt.block_on(async_req);
        };
    }

    if let Some(ref matches) = matches.subcommand_matches("start") {
        if matches.is_present("daemon") {
            let args: Vec<String> = env::args().collect();
            if let Ok(Fork::Child) = daemon(true, true) {
                // 启动子进程
                let mut cmd = Command::new(&args[0]);

                for idx in 1..args.len() {
                    let arg = args.get(idx).expect("get cmd arg error!");
                    // 去除后台启动参数,避免重复启动
                    if arg.eq("-d") || arg.eq("-daemon") {
                        continue;
                    }
                    cmd.arg(arg);
                }

                let mut child = cmd.spawn().expect("Child process failed to start.");
                fs::write("pid", child.id().to_string());
                println!("process id is:{}", std::process::id());
                println!("child id is:{}", child.id());
            }
            println!("{}", "daemon mod");
            std::process::exit(0);
        }
        println!("server start!");

        let rt = tokio::runtime::Runtime::new().unwrap();
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        let async_req = async {
            // let (tx, rx) = tokio::sync::oneshot::channel::<()>();
            let mut http_server = httpserver::HttpServer::default();
            let handler = http_server.run(rx).await;
            tokio::join!(handler);
        };
        rt.block_on(async_req);
    }

    if let Some(ref matches) = matches.subcommand_matches("stop") {
        println!("server stopping...");
        let sys = System::new_with_specifics(RefreshKind::with_processes(Default::default()));

        let pidstr = String::from_utf8(fs::read("pid").unwrap()).unwrap();
        let pid = Pid::from_str(pidstr.as_str()).unwrap();

        if let Some(p) = sys.process(pid) {
            println!("terminal process: {:?}", p.pid());
        } else {
            println!("Server not run!");
            return;
        };
        Command::new("kill")
            .args(["-15", pidstr.as_str()])
            .output()
            .expect("failed to execute process");
    }

    if let Some(config) = matches.subcommand_matches("config") {
        if let Some(show) = config.subcommand_matches("show") {
            match show.subcommand_name() {
                Some("all") => {
                    println!("config show all");
                    info!("log show all");
                    configure::get_config_file_path();
                    println!("{:?}", configure::get_config());
                }
                Some("info") => {
                    println!("config show info");
                }
                _ => {}
            }
        }
    }
}
