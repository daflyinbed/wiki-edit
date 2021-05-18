pub(crate) mod meta {

    pub fn get_csrf_token() -> String {
        tokens::csrf::get()
    }
    pub fn get_login_token() -> String {
        tokens::login::get()
    }
    mod tokens {
        use serde::{Deserialize, Serialize};
        #[derive(Serialize)]
        struct TokenReq {
            action: &'static str,
            meta: &'static str,
            #[serde(rename = "type")]
            type_: &'static str,
            format: &'static str,
        }
        #[derive(Deserialize)]
        struct TokenResp<T> {
            tokens: T,
        }
        pub(super) mod login {
            use crate::Config;
            use serde::Deserialize;

            use super::{TokenReq, TokenResp};

            impl TokenReq {
                fn login() -> TokenReq {
                    TokenReq {
                        action: "query",
                        meta: "tokens",
                        type_: "login",
                        format: "json",
                    }
                }
            }
            #[derive(Deserialize)]
            struct LoginTokenResp {
                logintoken: String,
            }
            #[derive(Deserialize)]
            struct QueryLoginTokenResp {
                query: TokenResp<LoginTokenResp>,
            }
            pub(in crate::api::query::meta) fn get() -> String {
                let config = Config::get();
                let resp = config
                    .client
                    .post(&config.end_point)
                    .form(&TokenReq::login())
                    .send()
                    .unwrap();
                resp.json::<QueryLoginTokenResp>()
                    .unwrap()
                    .query
                    .tokens
                    .logintoken
            }
        }
        pub(super) mod csrf {
            use super::{TokenReq, TokenResp};
            use crate::Config;
            use serde::Deserialize;
            #[derive(Deserialize)]
            struct CsrfTokenResp {
                csrftoken: String,
            }
            impl TokenReq {
                fn csrf() -> TokenReq {
                    TokenReq {
                        action: "query",
                        meta: "tokens",
                        type_: "csrf",
                        format: "json",
                    }
                }
            }
            #[derive(Deserialize)]
            struct QueryCsrfTokenResp {
                query: TokenResp<CsrfTokenResp>,
            }
            pub(in crate::api::query::meta) fn get() -> String {
                let config = Config::get();
                let resp = config
                    .client
                    .post(&config.end_point)
                    .form(&TokenReq::csrf())
                    .send()
                    .unwrap();
                resp.json::<QueryCsrfTokenResp>()
                    .unwrap()
                    .query
                    .tokens
                    .csrftoken
            }
        }
    }
}
