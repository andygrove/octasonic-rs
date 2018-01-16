extern crate octasonic;
use octasonic::Octasonic;

use std::thread::sleep;
use std::time::Duration;

fn main() {
  let mut o = Octasonic::new(8).unwrap();
  let n = 1;

  o.set_sensor_count(1);

  for i in 0 .. 10 {
    println!("------------");
    show_settings(&mut o);
  }

  loop {
    show_readings(&mut o);
    sleep(Duration::from_millis(1000));
  }

}

fn show_readings(o: &mut Octasonic) {
  for i in 0 .. 8 {
    print!("{} ", o.get_sensor_reading(i));
  }
  println!("");
}

fn show_settings(o: &mut Octasonic) {
  println!("Protocol version: {}", o.get_protocol_version());
  println!("Sensor count: {}", o.get_sensor_count());
  println!("Max distance: {}", o.get_max_distance());
  //println!("Interval: {}", o.get_interval());
}
