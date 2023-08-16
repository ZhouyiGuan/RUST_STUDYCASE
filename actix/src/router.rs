use std::sync::Mutex;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AppStatus {
    pub app_name: String,
    pub app_counter:Mutex<usize>
}
#[derive(Serialize, Deserialize)]
pub struct Info {
    pub A: u32,
    pub B: String,
}

/// return input string variable
/// 
/// # RestfulAPI
/// GET: 0.0.0.0:7878/string/`var`
/// 
/// # return 
/// hello `var`
/// 
///  # description
/// 用Path可以反序列化路径中的变量
#[get("/string/{var}")]
pub async fn reply_string(var: web::Path<String>) -> String {
    format!("hello {var}")
}

/// req has variables in path form
/// 
/// # RestfulAPI
/// GET: 0.0.0.0:7878/reqForm/path/11/abc
/// 
/// # return
/// ```json
/// {
///     "info": {
///         "A": 11,
///         "B": "abc"
///     },
///     "status": "ok"
/// }
/// ```
/// 
/// # description
/// 这里可以直接把路径输入解析到一个结构体里
pub async fn req_in_path(info: web::Path<Info>) -> impl Responder {
    println!("{},{}",info.A,info.B);

    HttpResponse::Ok().json(json!({
        "status": "ok",
        "info": *info
    }))
}

/// req has variables in json form
/// 
/// # RestfulAPI
/// GET: 0.0.0.0:7878/reqForm/json
/// ```json
/// {
///     "A": 11,
///     "B":"abc"
/// }
/// ```
/// 
/// # return
/// ```json
/// {
///     "info": {
///         "A": 11,
///         "B": "abc"
///     },
///     "status": "ok"
/// }
/// ```
/// 
/// # description
/// 这里可以直接把json格式输入解析到一个结构体里
pub async fn req_in_json(info: web::Json<Info>) -> impl Responder {
    println!("{},{}",info.A,info.B);

    HttpResponse::Ok().json(json!({
        "status": "ok",
        "info": *info
    }))
}

/// return global app status
/// 
/// # RestfulAPI
/// GET: 0.0.0.0:7878/app/status
/// 
/// # return
/// ```json
/// {
///     "app_status": {
///         "app_counter": 4,
///         "app_name": "ZD-server"
///     },
///     "status": "get status success"
/// }
/// ```
/// 
/// # description
/// 展示如何在json回复体中加入反序列化结构体
/// 
pub async fn get_appstatus(data:web::Data<AppStatus>) -> impl Responder {
    let status = &data;

    HttpResponse::Ok().json(json!({
        "status": "get status success",
        "app_status": status
    }))
}

/// change global app status
/// 
/// # RestfulAPI
/// POST: 0.0.0.0:7878/app/status
/// 
/// # return
/// ```json
/// {
///     "app_status": {
///         "app_counter": 4,
///         "app_name": "ZD-server"
///     },
///     "status": "get status success"
/// }
/// ```
/// 
/// # description
/// 展示如何修改全局共享app状态；状态的注册是在main函数中；可变的成员需要用mutex
pub async fn post_appstatus_counter(data:web::Data<AppStatus>) -> impl Responder {
    let mut  counter_guard =data.app_counter.lock().unwrap();
    *counter_guard +=1;
    drop(counter_guard);

    HttpResponse::Ok().json(json!({
        "status": "change status success",
        "app_status": data
    }))
}

