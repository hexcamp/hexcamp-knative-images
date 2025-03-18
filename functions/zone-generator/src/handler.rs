use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::io::Write;

use crate::config::HandlerConfig;
use actix_web::web;
use cloudevents::Data;
use cloudevents::Event;
use serde_json::{from_slice, from_str, json};

pub async fn handle(event: Event, config: web::Data<HandlerConfig>) -> String {
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

    "Event processed".to_owned()
}
