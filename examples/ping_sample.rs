use std::process::Command;
use curl::easy::Easy;
use curl::Error;
use std::io::{stdout, Write};
use regex::{Match, Regex};


fn main() {
    // println!("{}", check_ping("www.baidu.com"));
    let re = Regex::new(r"(?x)
            ^(?P<login>[^@\s]+)@
            ([[:word:]]+\.)*
            [[:word:]]+$
            ").unwrap();
    let res = re.captures("I❤email@example.com").and_then(|cap| {
        cap.name("login").map(|login| login.as_str())
    });
    println!("{:?}", res);


//     let re1 = Regex::new(r"^PING\\b # match ping
// [^(]*\\(([^)]*)\\) # capture IP
// \\s([^.]*)\\. # capture the bytes of data
// .*?^(\\d+\\sbytes) # capture bytes
// .*?icmp_seq=(\\d+) # capture icmp_seq
// .*?ttl=(\\d+) # capture ttl
// .*?time=(.*?ms) # capture time
// .*?(\\d+)\\spackets\\stransmitted
// .*?(\\d+)\\sreceived
// .*?(\\d+%)\\spacket\\sloss
// .*?time\\s(\\d+ms)
// .*?=\\s([^\\/]*)\\/([^\\/]*)\\/([^\\/]*)\\/(.*?)\\sms").unwrap();

    let re1 = Regex::new(r"time=(.*) ms").unwrap();

    let res1 = re1.captures("64 bytes from 127.0.0.1: icmp_seq=0 ttl=64 time=3.643 ms")
        .and_then(|cap| {
            let time = cap.get(0);
            time.map(|t| t.as_str())
        });

    println!("{:?}", res1);
}


fn ping_get_time() {}

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
