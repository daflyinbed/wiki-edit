use clap::{App, Arg, SubCommand};
use reqwest::blocking::Client;

use std::{env, fs::File, io::Read};
mod api;
use api::{edit::edit, login::login, query::meta::*};
use once_cell::sync::OnceCell;
struct Config {
    end_point: String,
    client: Client,
}
static CONFIG: OnceCell<Config> = OnceCell::new();
impl Config {
    pub fn get() -> &'static Config {
        CONFIG.get().unwrap()
    }
    fn init(end_point: String) {
        CONFIG.set(Config {
            client: Client::builder()
                .user_agent("rust_wiki_edit")
                .cookie_store(true)
                .build()
                .unwrap(),
            end_point,
        });
    }
}

fn main() {
    dotenv::dotenv().ok().unwrap();
    let matches = App::new("wiki edit")
        .version("0.1")
        .author("xwbx <1677759063@qq.com>")
        .about("a cmd wiki editor")
        .subcommand(
            SubCommand::with_name("edit")
                .about("edit page")
                .arg(
                    Arg::with_name("title")
                        .long("title")
                        .help("page title")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("text")
                        .long("text")
                        .help("page content")
                        .required_unless("file")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("file")
                        .long("file")
                        .help("page content")
                        .takes_value(true)
                        .required_unless("text")
                        .conflicts_with("text"),
                ),
        )
        .get_matches();
    Config::init(env::var("API_URL").unwrap());
    let name = env::var("NAME").unwrap();
    let password = env::var("PASSWORD").unwrap();
    if let Some(matches) = matches.subcommand_matches("edit") {
        let title = matches.value_of("title").unwrap();
        let buf = if let Some(path) = matches.value_of("file") {
            let mut file = File::open(path).unwrap();
            let mut buffer = String::new();
            file.read_to_string(&mut buffer).unwrap();
            Some(buffer)
        } else {
            None
        };
        let text = buf
            .as_deref()
            .unwrap_or_else(|| matches.value_of("text").unwrap());

        login(&get_login_token(), &name, &password);
        edit(&get_csrf_token(), title, text);
    }
}
