extern crate spidev;
use self::spidev::{Spidev, SpidevOptions, SpidevTransfer, SPI_MODE_0};

use std::io::Error;

const CMD_GET_PROTOCOL_VERSION : u8 = 0x01;
const CMD_SET_SENSOR_COUNT     : u8 = 0x02;
const CMD_GET_SENSOR_COUNT     : u8 = 0x03;
const CMD_GET_SENSOR_READING   : u8 = 0x04;
const CMD_SET_INTERVAL         : u8 = 0x05;
const CMD_TOGGLE_LED           : u8 = 0x06;
const CMD_SET_MAX_DISTANCE     : u8 = 0x07;
const CMD_GET_MAX_DISTANCE     : u8 = 0x08;

pub struct Octasonic {
    spi: Spidev,
}

impl Octasonic {

    /// Create an Octasonic struct for the specified sensor count
    pub fn new(sensor_count: usize) -> Result<Self, Error> {
        let mut spi = try!(Spidev::open("/dev/spidev0.0"));
        let mut options = SpidevOptions::new();

        options.bits_per_word(8)
            .max_speed_hz(20_000)
            .mode(SPI_MODE_0);

        try!(spi.configure(&options));

        let o = Octasonic { spi: spi };

        o.set_sensor_count(sensor_count as u8);

        Ok(o)
    }

    pub fn get_protocol_version(&self) -> u8 {
        self.send_receive(CMD_GET_PROTOCOL_VERSION, 0)
    }

    pub fn get_sensor_count(&self) -> u8 {
        self.send_receive(CMD_GET_SENSOR_COUNT, 0)
    }

    pub fn get_sensor_reading(&self, n: u8) -> u8 {
        self.send_receive(CMD_GET_SENSOR_READING, n)
    }

    pub fn set_sensor_count(&self, n: u8) {
        self.send(CMD_SET_SENSOR_COUNT, n)
    }

    pub fn set_interval(&self, n: u8) {
        self.send(CMD_SET_INTERVAL, n)
    }

    pub fn set_max_distance(&self, n: u8) {
        self.send(CMD_SET_MAX_DISTANCE, n )
    }

    pub fn get_max_distance(&self) -> u8 {
        self.send_receive(CMD_GET_MAX_DISTANCE, 0)
    }

    pub fn toggle_led(&self) {
        self.send(CMD_TOGGLE_LED, 0)
    }

    /// send a command and read the result byte
    fn send_receive(&self, cmd: u8, param: u8) -> u8 {
        let _ = self.transfer(cmd << 4 | param);
        self.transfer(0x00)
    }

    /// send a single byte containing a command and a parameter
    fn send(&self, cmd: u8, param: u8) {
        let _ = self.transfer(cmd << 4 | param);
    }

    fn transfer(&self, b: u8) -> u8 {
        let mut transfer = SpidevTransfer::write(&[b]);
        self.spi.transfer(&mut transfer).unwrap();
        // println!("Sent: {:?}, Received: {:?}", b, transfer.rx_buf);
        let b = transfer.rx_buf.unwrap();
        (*b)[0]
    }
}
