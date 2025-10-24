use flume::{Receiver, Sender};
use rdev::{Event, EventType, Key};

pub enum NavigationEvent {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

// This assumes navigation shortcuts are vim keys + cmd
// TODO: allow user to configure this
pub fn listen_for_navigation_keys(
    mac_events_rx: Receiver<Event>,
    navigation_events_tx: Sender<NavigationEvent>,
) {
    let mut cmd_pressed = false;
    while let Ok(mac_event) = mac_events_rx.recv() {
        match mac_event.event_type {
            EventType::KeyPress(Key::MetaLeft) => cmd_pressed = true,
            EventType::KeyRelease(Key::MetaLeft) => cmd_pressed = false,
            EventType::KeyPress(Key::KeyH) if cmd_pressed => {
                publish_navigation_event(navigation_events_tx.clone(), NavigationEvent::LEFT)
            }
            EventType::KeyPress(Key::KeyL) if cmd_pressed => {
                publish_navigation_event(navigation_events_tx.clone(), NavigationEvent::RIGHT)
            }
            EventType::KeyPress(Key::KeyJ) if cmd_pressed => {
                publish_navigation_event(navigation_events_tx.clone(), NavigationEvent::DOWN)
            }
            EventType::KeyPress(Key::KeyK) if cmd_pressed => {
                publish_navigation_event(navigation_events_tx.clone(), NavigationEvent::UP)
            }
            _ => {}
        }
    }
}

// helpful utility
pub fn publish_navigation_event(
    navigation_events_tx: Sender<NavigationEvent>,
    event: NavigationEvent,
) {
    if let Err(e) = navigation_events_tx.send(event) {
        eprintln!("Unexpected error sending navigation event {e}");
    }
}
