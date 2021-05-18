use serde::{Deserialize, Serialize};
#[derive(Serialize)]
struct EditReq<'a> {
    format: &'static str,
    action: &'static str,
    token: &'a str,
    title: &'a str,
    text: &'a str,
    bot: i32,
}
impl<'a> EditReq<'a> {
    fn new(title: &'a str, text: &'a str, token: &'a str) -> EditReq<'a> {
        EditReq {
            format: "json",
            action: "edit",
            title,
            text,
            token,
            bot: 1,
        }
    }
}

#[derive(Deserialize)]
struct InnerEditResp {
    result: String,
}
#[derive(Deserialize)]
struct EditResp {
    edit: InnerEditResp,
}
use crate::Config;
pub(crate) fn edit(csrf_token: &str, title: &str, text: &str) {
    let config = Config::get();
    let resp = config
        .client
        .post(&config.end_point)
        .form(&EditReq::new(title, text, &csrf_token))
        .send()
        .unwrap();
    if resp.json::<EditResp>().unwrap().edit.result != "Success" {
        panic!("edit failed")
    }
}
