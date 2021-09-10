#[cfg(target_arch = "arm-unknown-linux-gnueabihf")]
use rppal::spi::Spi;
#[cfg(target_arch = "arm-unknown-linux-gnueabihf")]
use wiringpi::pin::OutputPin;

#[cfg_attr(test, mockall::automock)]
pub trait SpiInterface {
    fn send_bytes(&mut self, bytes: &[u8]);
}

#[cfg_attr(test, mockall::automock)]
pub trait PinInterface {
    fn set_pin(&self, value: bool);
}

#[cfg(all(not(test), target_arch = "arm-unknown-linux-gnueabihf"))]
impl SpiInterface for Spi {
    fn send_bytes(&mut self, bytes: &[u8]) {
        if let Err(e) = self.write(bytes) {
    		println!("Error: {} for sending spi bytes", e);
		}
    }
}

#[cfg(all(not(test), target_arch = "arm-unknown-linux-gnueabihf"))]
impl PinInterface for OutputPin<wiringpi::pin::WiringPi> {
    fn set_pin(&mut self, value: bool) {
        let pin_value = if value { 
            wiringpi::pin::Value::High 
        } else { 
            wiringpi::pin::Value::Low
        };
        self.digital_write(pin_value);
    }
}
