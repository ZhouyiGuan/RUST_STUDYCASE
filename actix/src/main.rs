#![allow(non_snake_case)]

use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

mod router;
use router::*;
mod func;
use func::*;


#[actix_web::main]
pub async fn main() -> std::io::Result<()> {

    // 创建全局共享app状态 这里的counter代表的是可变状态成员变量
    let app_status =web::Data::new(AppStatus {
        app_name: String::from("ZD-server"),
        app_counter:Mutex::new(0)
    });

    let server = HttpServer::new(move || App::new()
        .app_data(app_status.clone())// 全局状态注册在这里

        .service(reply_string)
        .service(
            web::scope("/reqForm")
            .route("/path/{A}/{B}",web::get().to(req_in_path))
            .route("/json",web::get().to(req_in_json))
        )
        .service(
            web::scope("/app")
            .route("/status",web::get().to(get_appstatus))
            .route("/counter",web::post().to(post_appstatus_counter))
        )


    )
    .bind(("0.0.0.0", 7878))?;

    println!("app is listening on port 7878");                           

    server.run().await
}