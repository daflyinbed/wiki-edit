use serde::{Deserialize, Serialize};
#[derive(Serialize)]
pub struct TokenReq {
    action: &'static str,
    meta: &'static str,
    #[serde(rename = "type")]
    type_: &'static str,
    format: &'static str,
}
impl TokenReq {
    pub fn csrf() -> TokenReq {
        TokenReq {
            action: "query",
            meta: "tokens",
            type_: "csrf",
            format: "json",
        }
    }
    pub fn login() -> TokenReq {
        TokenReq {
            action: "query",
            meta: "tokens",
            type_: "login",
            format: "json",
        }
    }
}
#[derive(Serialize)]
pub struct LoginReq<'a> {
    format: &'static str,
    action: &'static str,
    lgname: &'a str,
    lgpassword: &'a str,
    lgtoken: &'a str,
}
impl<'a> LoginReq<'a> {
    pub fn new(name: &'a str, password: &'a str, token: &'a str) -> LoginReq<'a> {
        LoginReq {
            format: "json",
            action: "login",
            lgname: name,
            lgpassword: password,
            lgtoken: token,
        }
    }
}
#[derive(Serialize)]
pub struct EditReq<'a> {
    format: &'static str,
    action: &'static str,
    token: &'a str,
    title: &'a str,
    text: &'a str,
    bot: i32,
}
impl<'a> EditReq<'a> {
    pub fn new(title: &'a str, text: &'a str, token: &'a str) -> EditReq<'a> {
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
pub struct LoginTokenResp {
    pub logintoken: String,
}
#[derive(Deserialize)]
pub struct CsrfTokenResp {
    pub csrftoken: String,
}

#[derive(Deserialize)]
pub struct TokenResp<T> {
    pub tokens: T,
}
#[derive(Deserialize)]
pub struct QueryLoginTokenResp {
    pub query: TokenResp<LoginTokenResp>,
}
#[derive(Deserialize)]
pub struct QueryCsrfTokenResp {
    pub query: TokenResp<CsrfTokenResp>,
}

#[derive(Deserialize)]
pub struct InnerLoginResp {
    pub result: String,
    pub lgusername: String,
}
#[derive(Deserialize)]
pub struct LoginResp {
    pub login: InnerLoginResp,
}

#[derive(Deserialize)]
pub struct InnerEditResp {
    pub result: String,
}
#[derive(Deserialize)]
pub struct EditResp {
    pub edit: InnerEditResp,
}
