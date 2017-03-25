extern crate spidev;
use self::spidev::{Spidev, SpidevOptions, SpidevTransfer, SPI_MODE_0};

use std::io::Error;

const CMD_GET_PROTOCOL_VERSION : u8 = 0x01;
const CMD_SET_SENSOR_COUNT     : u8 = 0x02;
const CMD_GET_SENSOR_COUNT     : u8 = 0x03;
const CMD_GET_SENSOR_READING   : u8 = 0x04;
const CMD_SET_INTERVAL         : u8 = 0x05;
const CMD_TOGGLE_LED           : u8 = 0x06;

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

    pub fn get_sensor_count(&self) -> u8 {
        let _ = self.transfer(CMD_GET_SENSOR_COUNT << 4);
        self.transfer(0x00)
    }

    pub fn get_sensor_reading(&self, n: u8) -> u8 {
        let _ = self.transfer(CMD_GET_SENSOR_READING << 4 | n);
        self.transfer(0x00)
    }

    pub fn set_sensor_count(&self, n: u8) {
        assert!(n > 0 && n < 9);
        let _ = self.transfer(CMD_SET_SENSOR_COUNT << 4 | n);
    }

    pub fn toggle_led(&self) {
        let _ = self.transfer(CMD_TOGGLE_LED << 4);
    }

    fn transfer(&self, b: u8) -> u8 {
        let mut transfer = SpidevTransfer::write(&[b]);
        self.spi.transfer(&mut transfer).unwrap();
        // println!("Sent: {:?}, Received: {:?}", b, transfer.rx_buf);
        let b = transfer.rx_buf.unwrap();
        (*b)[0]
    }
}
