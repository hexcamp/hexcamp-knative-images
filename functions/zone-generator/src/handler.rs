use duct::cmd;
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

    /*
    use std::process::Command;

    let output = Command::new("sh")
        .arg("-c")
        .arg("update-zones.sh")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("command succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("command failed and stderr was:\n{}", s);
        return HttpResponse::InternalServerError()
            .content_type(ContentType::plaintext())
            .body("Error");
    }
    */
    println!("Running update-zones.sh...");
    let update_zones_cmd = cmd!("sh", "-c", "update-zones.sh");
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
        },
        Err(e) => {
            println!("Error! {}", e);
        }
    }
    println!("Finished running update-zones.sh");

    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("Event processed")
}
