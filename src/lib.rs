use serde::{Deserialize, Serialize};
use worker::*;

fn raw_html_response(html: &str) -> Result<Response> {
    Response::from_html(html)
}

#[derive(Deserialize, Serialize, Debug)]
struct Payload {
    msg: String,
}

async fn read_request_body(mut req: Request) -> String {
    let ctype = req.headers().get("content-type").unwrap().unwrap();
    match ctype.as_str() {
        "application/json" => format!("{:?}", req.json::<Payload>().await.unwrap()),
        "text/html" => req.text().await.unwrap(),
        "multipart/form-data" => format!("{:?}", req.form_data().await.unwrap()),
        _ => String::from("a file"),
    }
}

#[event(fetch)]
async fn fetch(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    if String::from(req.url()?).contains("form") {
        return raw_html_response("some html form");
    }

    match req.method() {
        Method::Post => {
            let req_body = read_request_body(req).await;
            Response::ok(format!("The request body sent in was {}", req_body))
        }
        Method::Get => {
            let html = include_str!("index.html");
            raw_html_response(html)
        }
        _ => Response::ok(format!("The resut was a {:?}", req.method())),
    }
}
