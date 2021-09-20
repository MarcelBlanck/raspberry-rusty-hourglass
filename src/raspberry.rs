use rppal::spi::Spi;
use rppal::gpio::OutputPin;

#[cfg_attr(test, mockall::automock)]
pub trait SpiInterface {
    fn send_bytes(&mut self, bytes: &[u8]);
}

#[cfg_attr(test, mockall::automock)]
pub trait PinInterface {
    fn set_pin(&mut self, value: bool);
}

impl SpiInterface for Spi {
    fn send_bytes(&mut self, bytes: &[u8]) {
        self.write(bytes).unwrap();
    }
}

impl PinInterface for OutputPin {
    fn set_pin(&mut self, value: bool) {
        match value {
            true => self.set_high(),
            false => self.set_low()
        };
    }
}
