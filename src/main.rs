use std::cmp::min;
use std::fmt::format;
use std::fs::File;
use std::io::Read;
use std::ops::Index;
use std::slice::SliceIndex;

use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
use actix_web::web::Data;
use rand::Rng;

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


#[get("{sort_type}/{index}")]
async fn get_sort(sort_type: web::Path<String>, index: web::Path<usize>, state: web::Data<State>) -> impl Responder {
    println!("sort_type: {}", sort_type);
    println!("index: {}", index);
    let sort_value = sort_type.into_inner().as_str();
    let index = index.into_inner();
    let e = match sort_value {
        "selection" => {
            Some(state.selection[index].clone())
        }
        "insertion" => {
            Some(state.insertion[index].clone())
        }
        _ => {
            None
        }
    };
    HttpResponse::Ok().append_header(("Access-Control-Allow-Origin", "*")).body(format!("{:?}", index))
}

#[derive(Clone)]
struct State {
    selection: Vec<Vec<i32>>,
    insertion: Vec<Vec<i32>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut selection_data = [];
    File::open("./sorted/selection.json").unwrap().read(&mut selection_data).unwrap();
    let selection_vec: Vec<Vec<i32>> = serde_json::from_slice(selection_data.as_ref()).expect("TODO: panic message");
    let mut insertion_data = [];
    File::open("./sorted/insertion.json").unwrap().read(&mut insertion_data).unwrap();
    let insertion_vec: Vec<Vec<i32>> = serde_json::from_slice(insertion_data.as_ref()).expect("TODO: panic message");
    let state = State {
        selection: selection_vec,
        insertion: insertion_vec,
    };
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(state))
            .service(get_sort)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}