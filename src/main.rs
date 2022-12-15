use clap::{arg, command};
use env_logger::Builder;
use log::LevelFilter;
use rcon_bruteforcer::start_process;

#[tokio::main]
async fn main() {
    Builder::new().filter_level(LevelFilter::Info).init();

    let matches = command!()
        .version("0.2.0")
        .author("CCBlueX & MoskalykA")
        .about("Allows you to bruteforce RCON server")
        .arg(
            arg!(
                --host <String> "Set the target host"
            )
            .required(true),
        )
        .arg(
            arg!(
                --charset <String> "Specify the password charset"
            )
            .default_value("ABCDEFGHIJKLMNOPQRSTUVWXAZabcdefghijklmnopqrstuvwxyz123456789"),
        )
        .arg(
            arg!(
                --concurrents <String> "How many concurrent connections"
            )
            .default_value("10"),
        )
        .get_matches();

    let host = matches.get_one::<String>("host").unwrap();
    let concurrents = matches
        .get_one::<String>("concurrents")
        .unwrap()
        .parse()
        .unwrap();

    let charset = matches.get_one::<String>("charset").unwrap();
    start_process(host, concurrents, charset).await
}
