use flume::Receiver;

use crate::keyboard::NavigationEvent;

pub fn listen_for_navigation_events(navigation_events_rx: Receiver<NavigationEvent>) {}
