use rdev::{listen, Event, EventType};

fn main() {
    listen(event_callback).unwrap()
}

fn event_callback(event: Event) {
    match event.event_type {
        EventType::KeyRelease(k) => {
            println!("{:#?}", k)
        }
        _ => {}
    }
}
