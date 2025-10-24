use std::process::Command;

pub fn get_focused_monitor_id() -> Option<u32> {
    let output = Command::new("aerospace")
        .args(&[
            "list-monitors",
            "--focused",
            "--format",
            "%{monitor-appkit-nsscreen-screens-id}",
        ])
        .output()
        .ok()?;

    let id_str = String::from_utf8(output.stdout).ok()?;
    let id_str = id_str.trim();
    id_str.parse::<u32>().ok()
}

pub fn get_focused_window_id() -> Option<i32> {
    let output = Command::new("aerospace")
        .args(&["list-windows", "--focused", "--format", "%{window-id}"])
        .output()
        .ok()?;

    let id_str = String::from_utf8(output.stdout).ok()?;
    let id_str = id_str.trim();

    id_str.parse::<i32>().ok()
}
