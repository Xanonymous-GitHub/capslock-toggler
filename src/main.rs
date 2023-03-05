use std::{thread, time};

use rdev::Event;
use rdev::EventType;
use rdev::EventType::KeyPress;
use rdev::Key::CapsLock;
use rdev::listen;
use rdev::simulate;
use rdev::SimulateError;

fn send_event(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(_) => {}
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    thread::sleep(delay);
}

fn callback(event: Event) {
    // filter out non-keyboard events.
    match event.event_type {
        KeyPress(_) => (),
        _ => {
            return;
        }
    }

    match event.name {
        Some(_) => println!("{:?}", event.event_type),
        None => (),
    }
}

fn main() {
    // Listen to keyboard event and send caps-lock event at the send time.
    // main --> [loop] send caps-lock event...
    //   \
    //    --> listen keyboard event...

    let _event_listener_thread = thread::spawn(|| {
        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error);
        }
    });

    let mut sent_amount = 0;

    // Automatically send `Caps-Lock` event to OS.
    loop {
        sent_amount += 1;
        println!("{:?}", sent_amount);
        thread::sleep(time::Duration::from_secs(1));
        send_event(&KeyPress(CapsLock))
    }

    // If not using unlimited loop, don't forget to use this:
    // _event_listener_thread.join().unwrap();
}
