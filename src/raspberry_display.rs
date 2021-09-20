use crate::display::{DisplayControl, DisplayBuffer, Point, Color};
use crate::raspberry::{SpiInterface, PinInterface};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use rppal::gpio::{Gpio, OutputPin};
use std::{thread, time};

pub struct RaspberryDisplay<T, U> {
    fb: DisplayBuffer,
    pub spi: T,
    reset_pin: U,
    dc_pin: U
}

impl RaspberryDisplay<Spi, OutputPin> {
    pub fn new() -> RaspberryDisplay<Spi, OutputPin> {
        RaspberryDisplay {
            fb: DisplayBuffer::new(),
            spi: Spi::new(Bus::Spi0, SlaveSelect::Ss0, 2_000_000, Mode::Mode0).unwrap(),
            reset_pin: Gpio::new().unwrap().get(25).unwrap().into_output(),
            dc_pin: Gpio::new().unwrap().get(24).unwrap().into_output()
        }
    }
}

impl<T: SpiInterface, U: PinInterface> RaspberryDisplay<T, U> {
    pub fn new_generic(spi: T, reset_pin: U, dc_pin: U) -> RaspberryDisplay<T, U> {
        RaspberryDisplay { fb: DisplayBuffer::new(), spi, reset_pin, dc_pin }
    }

    fn send_display_commands(&mut self, commands : &[u8]) {
        self.dc_pin.set_pin(false);
        self.spi.send_bytes(commands);
    }

    fn reset(&mut self) {
        let interval = time::Duration::from_millis(10);
        self.reset_pin.set_pin(true);
        thread::sleep(interval);
        self.reset_pin.set_pin(false);
        thread::sleep(interval);
        self.reset_pin.set_pin(true);
        thread::sleep(interval);
    }
}

impl<T: SpiInterface, U: PinInterface> DisplayControl for RaspberryDisplay<T, U> {
    fn init(&mut self) {
        // Based on https://www.waveshare.com/w/upload/b/b5/SSD1305-Revision_1.8.pdf
        self.reset();
        self.send_display_commands(&[0xAE]); // Display OFF (sleep mode)
        self.send_display_commands(&[0x20, 0x01]); // Set Vertical Addressing Mode
        self.send_display_commands(&[0x21, 0x00, 0x7F]); // Set Column Address range to 0-127
        self.send_display_commands(&[0x22, 0x00, 0x03]); // Set Page Address range to 0-3
        self.send_display_commands(&[0xAC]); // Display ON in dim mode
        self.swap();
    }

    fn deinit(&mut self) {
        self.fb.fill_with_black();
        self.swap();
        thread::sleep(time::Duration::from_millis(10));
        self.reset_pin.set_pin(false);
    }

    fn swap(&mut self) {
        self.dc_pin.set_pin(true);
        self.spi.send_bytes(&self.fb.buffer);
    }

    fn fb<'a>(&'a mut self) -> &'a mut DisplayBuffer {
        &mut self.fb
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::*;
    use mockall::predicate::*;
    use crate::raspberry::*;

    fn get_new_mocked_display() -> RaspberryDisplay<MockSpiInterface, MockPinInterface>  {
        RaspberryDisplay::new_generic(
            MockSpiInterface::new(),
            MockPinInterface::new(),
            MockPinInterface::new()
        )
    }

    fn set_pin_expectation(value: bool, pin: &mut MockPinInterface, sequence: &mut Sequence) {
        pin.expect_set_pin()
           .with(eq(value))
           .return_const(())
           .times(1)
           .in_sequence(sequence);
    }

    fn set_send_bytes_expectation(bytes: Vec<u8>, spi: &mut MockSpiInterface, sequence: &mut Sequence) {
        spi.expect_send_bytes()
           .withf(move |send_bytes: &[u8]| send_bytes == bytes)
           .return_const(())
           .times(1)
           .in_sequence(sequence);
    }

    #[test]
    fn test_reset_sequence() {
        let mut display = get_new_mocked_display();

        let mut sequence = Sequence::new();
        set_pin_expectation(true, &mut display.reset_pin, &mut sequence);
        set_pin_expectation(false, &mut display.reset_pin, &mut sequence);
        set_pin_expectation(true, &mut display.reset_pin, &mut sequence);

        display.reset();
    }

    #[test]
    fn test_swap() {
        let mut display = get_new_mocked_display();

        let mut sequence = Sequence::new();
        set_pin_expectation(true, &mut display.dc_pin, &mut sequence);
        set_send_bytes_expectation(vec![0xAAu8; 512], &mut display.spi, &mut sequence);
        set_pin_expectation(true, &mut display.dc_pin, &mut sequence);
        set_send_bytes_expectation(vec![0xCCu8; 512], &mut display.spi, &mut sequence);

        display.fb.buffer = [0xAAu8; 512];
        display.swap();
        display.fb.buffer = [0xCCu8; 512];
        display.swap();
    }

    #[test]
    fn test_display_command_sending() {
        let mut display = get_new_mocked_display();

        let mut sequence = Sequence::new();
        set_pin_expectation(false, &mut display.dc_pin, &mut sequence);
        set_send_bytes_expectation(vec![0xABu8,0xCDu8], &mut display.spi, &mut sequence);

        display.send_display_commands(&[0xABu8,0xCDu8]);
    }

    #[test]
    fn test_init_sequence() {
        let mut display = get_new_mocked_display();

        let mut sequence = Sequence::new();

        // Toggle reset pin
        set_pin_expectation(true, &mut display.reset_pin, &mut sequence);
        set_pin_expectation(false, &mut display.reset_pin, &mut sequence);
        set_pin_expectation(true, &mut display.reset_pin, &mut sequence);

        // Display OFF (sleep mode)
        set_pin_expectation(false, &mut display.dc_pin, &mut sequence);
        set_send_bytes_expectation(vec![0xAEu8], &mut display.spi, &mut sequence);

        // Set Vertical Addressing Mode
        set_pin_expectation(false, &mut display.dc_pin, &mut sequence);
        set_send_bytes_expectation(vec![0x20, 0x01], &mut display.spi, &mut sequence);

        // Set Column Address range to 0-127
        set_pin_expectation(false, &mut display.dc_pin, &mut sequence);
        set_send_bytes_expectation(vec![0x21, 0x00, 0x7F], &mut display.spi, &mut sequence);

        // Set Page Address range to 0-3
        set_pin_expectation(false, &mut display.dc_pin, &mut sequence);
        set_send_bytes_expectation(vec![0x22, 0x00, 0x03], &mut display.spi, &mut sequence);

        // Display ON in dim mode
        set_pin_expectation(false, &mut display.dc_pin, &mut sequence);
        set_send_bytes_expectation(vec![0xAC], &mut display.spi, &mut sequence);

        // Send the buffer with all 0u8 to clear the screen to black
        set_pin_expectation(true, &mut display.dc_pin, &mut sequence);
        set_send_bytes_expectation(vec![0u8; 512], &mut display.spi, &mut sequence);

        display.init();
    }
}
