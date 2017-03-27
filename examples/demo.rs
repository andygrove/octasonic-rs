extern crate octasonic;
use octasonic::Octasonic;

use std::thread::sleep;
use std::time::Duration;

fn main() {
  let o = Octasonic::new(8).unwrap();

  show_settings(&o);

  loop {
    show_readings(&o);
    sleep(Duration::from_millis(1000));
  }

}

fn show_readings(o: &Octasonic) {
  for i in 0 .. 8 {
    print!("{} ", o.get_sensor_reading(i));
  }
  println!("");
}

fn show_settings(o: &Octasonic) {
  println!("Protocol version: {}", o.get_protocol_version());
  println!("Sensor count: {}", o.get_sensor_count());
  println!("Max distance: {}", o.get_max_distance());
  //println!("Interval: {}", o.get_interval());
}
