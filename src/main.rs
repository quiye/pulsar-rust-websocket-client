extern crate base64;
extern crate jsonway;
extern crate ws;

fn main() {
    let payload = base64::encode(r#"Hello """Rust""" World"#);
    let json = jsonway::object(|json| {
        json.set("payload", payload);
        json.object("properties", |json| {
            json.set("key1", "value1");
            json.set("key2", "value2");
        });
        json.set("context", 5);
    }).unwrap();
    let topic = "ws://localhost:8080/ws/v2/producer/persistent/public/default/my-topic";
    ws::connect(topic, |out| {
        out.send(json.to_string()).unwrap();

        move |msg| {
            println!("Got message: {}", msg);
            out.close(ws::CloseCode::Normal)
        }
    }).unwrap()
}
