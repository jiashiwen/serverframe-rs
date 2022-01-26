use crate::cmd::{new_config_cmd, new_start_cmd, new_stop_cmd};
use crate::commons::CommandCompleter;
use crate::commons::SubCmd;
use crate::configure::{generate_default_config, set_config_file_path};
use crate::configure::{get_config, get_config_file_path, get_current_config_yml, set_config};
use crate::resources::{get_tikv_handler, set_tikv};
use crate::{httpserver, interact};

use clap::{App, AppSettings, Arg, ArgMatches};
use fork::{daemon, Fork};
use lazy_static::lazy_static;
use std::borrow::Borrow;
use std::net::SocketAddr;
use std::process::Command;
use std::str::FromStr;
use std::{env, fs};
use sysinfo::{Pid, ProcessExt, RefreshKind, System, SystemExt};

lazy_static! {
    static ref CLIAPP: clap::App<'static> = App::new("serverframe-rs")
        .version("1.0")
        .author("Shiwen Jia. <jiashiwen@gmail.com>")
        .about("RustBoot")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true)
        )
        .arg(
            Arg::new("interact")
                .short('i')
                .long("interact")
                .help("run as interact mod")
        )
        .subcommand(
            new_start_cmd().arg(
                Arg::new("daemon")
                    .short('d')
                    .long("daemon")
                    .help("run as daemon")
            )
        )
        .subcommand(new_stop_cmd())
        .subcommand(new_config_cmd());
    static ref SUBCMDS: Vec<SubCmd> = subcommands();
}

pub fn run_app() {
    let matches = CLIAPP.clone().get_matches();
    set_config("");
    if let Ok(cfg) = get_config() {
        println!("{:?}", cfg);
        // let mut pd: Vec<&str> = vec![];
        // for str in cfg.tikv.pdaddrs {
        //     pd.push(String::from(str).as_str());
        // }
        // cfg.tikv
        //     .pdaddrs
        //     .iter()
        //     .map(|iterm| pd.push(String::from(iterm).as_str()));
        let pd: Vec<&str> = cfg.tikv.pdaddrs.iter().map(|s| &**s).collect();
        println!("{:?}", pd);
        // init_tikv();
        set_tikv(pd);
        get_tikv_handler();
    };

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

fn cmd_match(matches: &ArgMatches) {
    if let Some(c) = matches.value_of("config") {
        set_config_file_path(c.to_string());
        set_config(&get_config_file_path());
    }

    if matches.is_present("interact") {
        interact::run();
        return;
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

                let child = cmd.spawn().expect("Child process failed to start.");
                fs::write("pid", child.id().to_string()).expect("Write pid file error!");
                println!("process id is:{}", std::process::id());
                println!("child id is:{}", child.id());
            }
            println!("{}", "daemon mod");
            std::process::exit(0);
        }

        let rt = tokio::runtime::Runtime::new().unwrap();
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        let async_req = async {
            let config = get_config().unwrap();
            let mut addr = config.http.addr;
            let port = config.http.port;
            addr.push_str(":");
            addr.push_str(&*port.to_string());

            let mut http_server = httpserver::HttpServer::default();
            let bind: SocketAddr = addr.parse().expect("unreachable panic");
            http_server.addr = bind;

            let http_handler = http_server.run(rx).await;
            let _http = tokio::join!(http_handler);
        };
        rt.block_on(async_req);
    }

    if let Some(ref _matches) = matches.subcommand_matches("stop") {
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
                    let yml = get_current_config_yml();
                    match yml {
                        Ok(str) => {
                            println!("{}", str);
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                        }
                    }
                }
                Some("info") => {
                    println!("config show info");
                }
                _ => {}
            }
        }

        if let Some(gen_config) = config.subcommand_matches("gendefault") {
            let mut file = String::from("");
            if let Some(path) = gen_config.value_of("filepath") {
                file.push_str(path);
            } else {
                file.push_str("config_default.yml")
            }
            if let Err(e) = generate_default_config(file.as_str()) {
                log::error!("{}", e);
                return;
            };
            println!("{} created!", file);
        }
    }
}
