use std::cmp::min;
use std::fmt::format;
use std::fs::File;

use rand::Rng;

use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};

use crate::sorting::{insertion, selection};

mod sorting;

fn save_sorts() {
    let mut rng = rand::thread_rng();
    let selection_file: File = File::create("./sorted/selection.json").unwrap();
    let insertion_file: File = File::create("./sorted/insertion.json").unwrap();
    let length = 0..1000;
    let mut rand_vec: Vec<i32> = (length.clone()).map(|_| rng.gen_range(length.clone())).collect();
    let mut selection_vec = rand_vec.clone();
    selection(&mut selection_vec, Some(selection_file));
    let mut insertion_vec = rand_vec.clone();
    insertion(&mut insertion_vec, Some(insertion_file));
}


#[get("test/{id}")]
async fn hello(id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().append_header(("Access-Control-Allow-Origin", "*")).body(format!("{:?}",id.into_inner()))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().append_header(("Access-Control-Allow-Origin", "*")).body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}