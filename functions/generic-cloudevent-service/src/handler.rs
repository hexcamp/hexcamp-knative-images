use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

use crate::config::HandlerConfig;
use actix_web::web;
use actix_web::{http::header::ContentType, HttpResponse};
use cloudevents::Data;
use cloudevents::Event;
use duct::cmd;
use serde_json::{from_slice, from_str, json};

pub async fn handle(event: Event, config: web::Data<HandlerConfig>) -> HttpResponse {
    println!("event: {}", event);

    let input = match event.data() {
        Some(Data::Binary(v)) => from_slice(v).unwrap(),
        Some(Data::String(v)) => from_str(v).unwrap(),
        Some(Data::Json(v)) => v.to_owned(),
        None => json!({ "name": config.name }),
    };
    let j = serde_json::to_string(&input).unwrap();

    let path = Path::new("/tmp/cloudevent.json");
    let display = path.display();
    let file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    let mut writer = BufWriter::new(file);
    write!(writer, "{}", j).unwrap();
    let _ = writer.flush();

    println!("Running /scripts/process-cloudevent.sh...");
    let update_zones_cmd = cmd!("sh", "/scripts/process-cloudevent.sh");
    let reader_result = update_zones_cmd.stderr_to_stdout().reader();

    match reader_result {
        Ok(reader) => {
            let lines = BufReader::new(reader).lines();
            for line_result in lines {
                match line_result {
                    Ok(line) => {
                        println!("{}", line);
                    }
                    Err(e) => {
                        println!("Error! {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            println!("Error! {}", e);
        }
    }
    println!("Finished running /scripts/process-cloudevent.sh");

    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("Event processed")
}
