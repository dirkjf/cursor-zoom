use std::thread::sleep;
use std::time::{Duration, Instant};
use enigo::{Enigo, MouseButton, MouseControllable};

fn main() {
    loop {
        println!("location: {:?}", get_cursor_location());
        sleep(Duration::from_secs(1 / 60));
    }
}

fn get_cursor_location() -> (i32, i32) {
    let mut enigo = Enigo::new();
    return enigo.mouse_location();
}