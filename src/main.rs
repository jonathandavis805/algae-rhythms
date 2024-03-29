use std::borrow::BorrowMut;
use std::cmp::min;
use std::fmt::format;
use std::fs::File;
use std::io::Read;
use std::ops::{Deref, Index};
use std::slice::SliceIndex;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
use actix_web::web::Data;
use rand::Rng;

use crate::sorting::{insertion, selection};

mod sorting;

#[post("save")]
async fn save_sorts(mut state: Data<State>) -> impl Responder {
    let mut insertion_rng = rand::thread_rng();
    let mut selection_rng = rand::thread_rng();
    let selection_file: File = File::create("./sorted/selection.json").unwrap();
    let insertion_file: File = File::create("./sorted/insertion.json").unwrap();
    let length = 0..1000;
    let mut rand_vec_for_selection: Vec<i32> = (length.clone()).map(|_| selection_rng.gen_range(length.clone())).collect();
    let mut rand_vec_for_insertion: Vec<i32> = (length.clone()).map(|_| insertion_rng.gen_range(length.clone())).collect();
    selection(&mut rand_vec_for_selection, Some(selection_file));
    insertion(&mut rand_vec_for_insertion, Some(insertion_file));
    state.update_state();
    HttpResponse::Ok().append_header(("Access-Control-Allow-Origin", "*")).body(format!("new sorts saved"))
}

#[derive(Serialize, Deserialize)]
struct ApiReturn {
    values: Vec<i32>,
    steps: i32
}

#[get("{sort_type}/{index}")]
async fn get_sort(params: web::Path<(String, usize)>, state: Data<State>) -> impl Responder {
    print!("request received");
    let params_inner = params.into_inner();
    println!("sort_type: {}", params_inner.0);
    println!("index: {}", params_inner.1);
    let mut index = params_inner.1;
    match params_inner.0.as_str() {
        "selection" => {
            let selection = state.selection.lock().unwrap();
            if selection.len() <= index {
                index = selection.len() - 1;
            }
            let val = selection[index].clone();
            let ret = ApiReturn { values: val, steps: selection.len() as i32 };
            let str_val = serde_json::to_string(&ret).unwrap();
            HttpResponse::Ok().append_header(("Access-Control-Allow-Origin", "*")).body(str_val)
        }
        "insertion" => {
            let insertion = state.insertion.lock().unwrap();
            if insertion.len() <= index {
                index = insertion.len() - 1;
            }
            let val = insertion[index].clone();
            let ret = ApiReturn { values: val, steps: insertion.len() as i32 };
            let str_val = serde_json::to_string(&ret).unwrap();
            HttpResponse::Ok().append_header(("Access-Control-Allow-Origin", "*")).body(str_val)
        }
        _ => {
            HttpResponse::BadRequest().append_header(("Access-Control-Allow-Origin", "*")).body(format!("these aren't the droids you're looking for"))
        }
    }
}

#[derive(Clone)]
struct State {
    selection: Arc<Mutex<Vec<Vec<i32>>>>,
    insertion: Arc<Mutex<Vec<Vec<i32>>>>,
}

impl State {
    fn update_state(&self) {
        let mut selection_data = Vec::new();
        File::open("/Users/jd/repos/algae-rhythms/sorted/selection.json").unwrap().read_to_end(&mut selection_data).unwrap();
        let mut insertion_data = Vec::new();
        File::open("/Users/jd/repos/algae-rhythms/sorted/insertion.json").unwrap().read_to_end(&mut insertion_data).unwrap();
        let mut ins = self.insertion.lock().unwrap();
        let mut ins_vec: Vec<Vec<i32>> = serde_json::from_slice(insertion_data.as_ref()).expect("TODO: panic message");
        *ins = ins_vec;
        let mut sel = self.selection.lock().unwrap();
        let mut sel_vec: Vec<Vec<i32>> = serde_json::from_slice(selection_data.as_ref()).expect("TODO: panic message");
        *sel = sel_vec;
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = State {
        selection: Arc::new(Mutex::new(Vec::new())),
        insertion: Arc::new(Mutex::new(Vec::new())),
    };
    state.update_state();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .service(get_sort)
            .service(save_sorts)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
