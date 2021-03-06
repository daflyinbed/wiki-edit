use crate::Config;
use serde::{Deserialize, Serialize};
#[derive(Serialize)]
struct LoginReq<'a> {
    format: &'static str,
    action: &'static str,
    lgname: &'a str,
    lgpassword: &'a str,
    lgtoken: &'a str,
}
impl<'a> LoginReq<'a> {
    fn new(name: &'a str, password: &'a str, token: &'a str) -> LoginReq<'a> {
        LoginReq {
            format: "json",
            action: "login",
            lgname: name,
            lgpassword: password,
            lgtoken: token,
        }
    }
}

#[derive(Deserialize)]
struct InnerLoginResp {
    result: String,
    lgusername: String,
}
#[derive(Deserialize)]
struct LoginResp {
    login: InnerLoginResp,
}

pub(crate) fn login(token: &str, name: &str, password: &str) {
    let config = Config::get();
    let resp = config
        .client
        .post(&config.end_point)
        .form(&LoginReq::new(&name, &password, token))
        .send()
        .unwrap();
    let login = resp.json::<LoginResp>().unwrap().login;
    if login.result != "Success" {
        panic!("login failed");
    }
    println!("hello {}", login.lgusername);
}
