extern crate octasonic;
use octasonic::Octasonic;

use std::thread::sleep;
use std::time::Duration;

fn main() {
  let octasonic = Octasonic::new(4, 0).unwrap();
  loop {
    octasonic.toggle_led();
    sleep(Duration::from_millis(500));
  }
}
