extern crate clap;
use clap::{Arg, App};

use rcon_bruteforcer::start_process;

#[tokio::main]
async fn main() {
    let matches = App::new("RCON Bruteforcer")
        .version("0.1.1")
        .author("CCBlueX")
        .about("Allows you to bruteforce RCON server.")
        .arg(Arg::with_name("host")
            .short("h")
            .long("host")
            .value_name("HOST")
            .help("Set the target host (127.0.0.1:25575)")
            .index(1)
            .required(true))
        .arg(Arg::with_name("charset")
            .short("s")
            .long("charset")
            .value_name("CHARSET")
            .help("Specify the password charset")
            .default_value("ABCDEFGHIJKLMNOPQRSTUVWXAZabcdefghijklmnopqrstuvwxyz123456789"))
        .arg(Arg::with_name("concurrents")
            .short("c")
            .long("concurrents")
            .value_name("CONCURRENTS")
            .help("How many concurrent connections")
            .default_value("10"))
        .get_matches();

    let server = matches.value_of("host").unwrap();
    let concurrents = matches.value_of("concurrents").unwrap().parse::<usize>().unwrap();
    let string_charset = matches.value_of("charset").unwrap();

    start_process(server, concurrents, string_charset).await;
}
