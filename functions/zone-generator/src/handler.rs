use cloudevents::Event;

pub async fn handle(
    event: Event
) -> String {
    println!("event: {}", event);

    "Hello world!".to_owned()
}

