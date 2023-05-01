use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use reqwest::Url;
use std::fs;

#[get("/")]
async fn index() -> impl Responder {
    let html_content = fs::read_to_string("serve/index.html").unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html_content)
}

#[get("/proxy/{url}")]
async fn proxy(url: web::Path<String>) -> impl Responder {
    let encoded_url = url.into_inner();
    let decoded_url = base64::decode(&encoded_url).unwrap();
    let full_url = String::from_utf8(decoded_url).unwrap();
    let url = Url::parse(&format!("http://{}", full_url)).unwrap();
    println!("Decoded URL: {}", url); // debug purposes you can remove
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36")
        .build()
        .unwrap();

    let response = client.get(url).send().await.unwrap();
    let body = response.text().await.unwrap();
    HttpResponse::Ok().body(body)
}

#[get("/script.js")]
async fn script(req: HttpRequest) -> HttpResponse {
    let ipaddr = req.connection_info().remote_addr().unwrap().to_string();
    let script_path = "serve/script.js";
    // this is the ip address thing, i may or may not use later for url encryption
    let script_content = fs::read_to_string(script_path).unwrap();
    let modified_script_content = format!("const ipaddr = \"{}\";\n{}", ipaddr, script_content);

    HttpResponse::Ok()
        .content_type("text/javascript")
        .body(modified_script_content)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(proxy)
            .service(index)
            .service(script)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
