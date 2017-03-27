extern crate octasonic;
use octasonic::Octasonic;

use std::thread::sleep;
use std::time::Duration;

struct Key {
  playing: bool,
  counter: u8
}


fn main() {

  let octasonic = Octasonic::new(8).unwrap();

  octasonic.set_max_distance(100);
  octasonic.set_interval(0);

  let note : Vec<u8> = vec![60, 62, 64, 65, 67, 69, 71, 72]; // C4 .. C5

  // init key state
  let mut key : Vec<Key> = vec![];
  for _ in 0 .. 8 {
    key.push(Key { playing: false, counter: 0 });
  }

  let max_distance : u8 = 20;

  let max_note_duration = 50;

  let channel = 1; // single channel for now

  loop {
    for i in 0 .. 8 {

      // get sensor reading
      let distance = octasonic.get_sensor_reading(i as u8);

      if key[i].playing {
        key[i].counter = key[i].counter+1;
        if key[i].counter == max_note_duration {
          key[i].playing = false;
          key[i].counter = 0;
        }
      }

      if !key[i].playing && distance < max_distance {
        key[i].playing = true;
        key[i].counter = 0;
        // velocity is a function of distance .. closer is louder
        let velocity = 127; //50 + max_distance - distance;
        println!("noteon {} {} {}", channel, note[i], velocity);
      }

    } 
//    sleep(Duration::from_millis(50));
  }
}
