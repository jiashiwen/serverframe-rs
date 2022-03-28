use std::fmt;
use std::process::Command;
use curl::easy::Easy;
use curl::Error;
use std::io::{stdout, Write};
use std::str::FromStr;
use regex::{Match, Regex};
use anyhow::Result;


fn main() {


    // let re1 = Regex::new(r"time=(.*) ms").unwrap();
    //
    // let res1 = re1.captures("64 bytes from 127.0.0.1: icmp_seq=0 ttl=64 time=3.643 ms")
    //     .and_then(|cap| {
    //         let time = cap.get(0);
    //         time.map(|t| t.as_str())
    //     });


    let txt = r"PING 127.0.0.1 (127.0.0.1): 56 data bytes
64 bytes from 127.0.0.1: icmp_seq=0 ttl=64 time=0.146 ms
64 bytes from 127.0.0.1: icmp_seq=1 ttl=64 time=0.123 ms
64 bytes from 127.0.0.1: icmp_seq=2 ttl=64 time=0.130 ms

--- 127.0.0.1 ping statistics ---
3 packets transmitted, 3 packets received, 0.0% packet loss
round-trip min/avg/max/stddev = 0.123/0.133/0.146/0.010 msPING 127.0.0.1 (127.0.0.1): 56 data bytes
64 bytes from 127.0.0.1: icmp_seq=0 ttl=64 time=0.146 ms
64 bytes from 127.0.0.1: icmp_seq=1 ttl=64 time=0.123 ms
64 bytes from 127.0.0.1: icmp_seq=2 ttl=64 time=0.130 ms

--- 127.0.0.1 ping statistics ---
3 packets transmitted, 3 packets received, 0.0% packet loss
round-trip min/avg/max/stddev = 0.123/0.133/0.146/0.010 ms";

    let res = cmd_ping("127.0.0.1");
    match res {
        Ok(ref str) => {
            let time = ping_get_time(str.as_str());
            if let Some(t) = time {
                let first_col = t.split(' ').collect::<Vec<&str>>()[0];
                let number = first_col.split('=').collect::<Vec<&str>>()[1];
                let f = number.parse::<f32>().unwrap();
                let digits = format!("{:.2}", f);
                println!("{:?}", digits);
            }
        }
        Err(ref e) => {
            eprintln!("{:?}", e);
        }
    }
    println!("{:?}", res);
}


fn ping_get_time(text: &str) -> Option<&str> {
    let regex = Regex::new(r"time=(.*) ms");
    return match regex {
        Ok(re) => {
            let res = re.captures(text)
                .and_then(|cap| {
                    let time = cap.get(0);
                    time.map(|t| t.as_str())
                });
            res
        }
        Err(_) => { None }
    };
}

fn cmd_ping(addr: &str) -> Result<String> {
    let cmd = format!("ping {} -c 3 -t 10", addr);
    let ping_cmd = Command::new("sh").arg("-c")
        .arg(cmd.as_str()).output()?;

    let str = std::str::from_utf8(&*ping_cmd.stdout).map_err(|e| {
        anyhow::Error::new(e)
    })?;
    Ok(str.to_string())
}

fn check_ping(addr: &str) -> bool {
    let cmd = format!("ping {} -c 3 -t 10", addr);
    let ping_cmd = Command::new("sh").arg("-c")
        .arg(cmd.as_str()).output().unwrap();

    let str = std::str::from_utf8(&*ping_cmd.stdout).unwrap();

    for l in str.lines() {
        if l.contains(',') {
            let res = l.split(',').collect::<Vec<&str>>();
            let recive_line = res[1].trim();
            let col_one = recive_line.split(' ').collect::<Vec<&str>>()[0];
            if col_one.parse::<usize>().is_ok() {
                if col_one.eq("0") {
                    return false;
                }
            }
        }
    }
    true
}

fn curl_response_status_code(uri: &str) -> Result<u32, Error> {
    let mut easy = Easy::new();
    easy.url(uri)?;
    easy.perform()?;
    easy.response_code()
}
