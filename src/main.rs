use enigo::*;
use std::{thread, time};
use rand::prelude::*;


fn main() {
    let mut enigo = Enigo::new();
    let five_secs = time::Duration::from_secs(5);
    let one_sec = time::Duration::from_millis(400);
    thread::sleep(five_secs);
    for i in 0..243 {
      println!("Starting");
      let mut rng = thread_rng();
      let n: u64 = rng.gen_range(0, 1000);
      let fuzz = time::Duration::from_millis(n);
      println!("Fuzz: {:?}", fuzz);
      thread::sleep(one_sec + fuzz);
      println!("clicking!");
      enigo.mouse_down(MouseButton::Left);
      enigo.mouse_up(MouseButton::Left);
    }
}
