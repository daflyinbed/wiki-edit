use clap::{App, Arg, SubCommand};
use reqwest::blocking::Client;

use std::{env, fs::File, io::Read};
mod types;
use types::*;

fn get_login_token(url: &str, client: &Client) -> String {
    let resp = client.post(url).form(&TokenReq::login()).send().unwrap();
    resp.json::<QueryLoginTokenResp>()
        .unwrap()
        .query
        .tokens
        .logintoken
}
fn get_csrf_token(url: &str, client: &Client) -> String {
    let resp = client.post(url).form(&TokenReq::csrf()).send().unwrap();
    resp.json::<QueryCsrfTokenResp>()
        .unwrap()
        .query
        .tokens
        .csrftoken
}
fn login(url: &str, token: &str, client: &Client) {
    let name = env::var("NAME").unwrap();
    let password = env::var("PASSWORD").unwrap();
    let resp = client
        .post(url)
        .form(&LoginReq::new(&name, &password, token))
        .send()
        .unwrap();
    let login = resp.json::<LoginResp>().unwrap().login;
    if login.result != "Success" {
        panic!("login failed");
    }
    println!("hello {}", login.lgusername);
}
fn edit(url: &str, client: &Client, title: &str, text: &str) {
    let csrf_token = get_csrf_token(url, client);
    let resp = client
        .post(url)
        .form(&EditReq::new(title, text, &csrf_token))
        .send()
        .unwrap();
    if resp.json::<EditResp>().unwrap().edit.result != "Success" {
        panic!("edit failed")
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
    let api = env::var("API_URL").unwrap();
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
        let client = Client::builder()
            .user_agent("rust_wiki_edit")
            .cookie_store(true)
            .build()
            .unwrap();
        let login_token = get_login_token(&api, &client);
        login(&api, &login_token, &client);
        edit(&api, &client, title, text);
    }
}
