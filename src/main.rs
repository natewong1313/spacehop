use clap::Parser;
use core_foundation::base::{TCFType, ToVoid};
use core_foundation::dictionary::CFDictionary;
use core_foundation::number::{CFNumber, CFNumberRef};
use core_graphics::display::{
    CFDictionaryRef, CGDisplay, CGRect, kCGWindowListExcludeDesktopElements,
};
use core_graphics::window::{self, kCGWindowListOptionOnScreenOnly};
use rdev::{Event, listen};
use std::ffi::c_void;

use crate::keyboard::NavigationEvent;
mod aerospace;
mod keyboard;
mod movement;

fn get_focused_monitor_width(monitor_id: u32) -> f64 {
    let display = CGDisplay::new(monitor_id);
    display.bounds().size.width
}

// returns x, y, width, height
fn get_focused_window_bounds(window_id: i32) -> Option<()> {
    let windows = CGDisplay::window_list_info(
        kCGWindowListExcludeDesktopElements | kCGWindowListOptionOnScreenOnly,
        None,
    )?;
    unsafe {
        for window in windows.iter() {
            let window: CFDictionary<*const c_void, *const c_void> =
                CFDictionary::wrap_under_get_rule(*window as CFDictionaryRef);
            let window_id_ref = window.get(window::kCGWindowNumber.to_void());
            let id = CFNumber::wrap_under_get_rule(*window_id_ref as CFNumberRef);
            // if id.to_i32() != Some(window_id) {
            //     continue;
            // }
            let bounds = window.get(window::kCGWindowBounds.to_void());
            let bounds = CFDictionary::wrap_under_get_rule(*bounds as CFDictionaryRef);
            let bounds = CGRect::from_dict_representation(&bounds)?;
            println!("{}", bounds.origin.x);
            println!("{}", bounds.origin.y);
        }
    }

    None
}

/// TODO: parse from .aerospace.toml instead
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short)]
    left_padding: u64,
    #[arg(short)]
    right_padding: u64,
}

fn main() {
    let args = Args::parse();

    let (mac_events_tx, mac_events_rx) = flume::unbounded();
    let (navigation_events_tx, navigation_events_rx) = flume::unbounded::<NavigationEvent>();

    std::thread::spawn(move || {
        keyboard::listen_for_navigation_keys(mac_events_rx, navigation_events_tx);
    });
    std::thread::spawn(move || {
        movement::listen_for_navigation_events(navigation_events_rx);
    });

    // starts listening for all macos events
    listen(move |mac_event| {
        if let Err(e) = mac_events_tx.send(mac_event) {
            eprintln!("Unexpected error sending event {e}");
        };
    })
    .expect("Failed to start listener, might need to go into macos security settings");

    // let monitor_id = aerospace::get_focused_monitor_id()
    //     .expect("Failed to get focused monitor id, is aerospace installed?");
    // let display_width = get_focused_monitor_width(monitor_id);
    //
    // let window_id = aerospace::get_focused_window_id().expect("Failed to get focused window id");
    // get_focused_window_bounds(window_id);

    // let window_bounds = get_focused_window_bounds(window_id)
    //     .expect(format!("Failed to get focused window bounds for {window_id}").as_str());
    // let (window_x, width) = window_bounds;
    //
    // let x = window_x - monitor_origin_x;
    //
    // let actual_display_width = display_width - args.left_padding - args.right_padding;
    //
    // println!("{actual_display_width}");
    // println!("{x} {width}");
}
