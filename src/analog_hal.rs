#![allow(missing_docs)]

pub trait Adc<Word> {
    fn read(&mut self) -> u16;
}

pub trait Dac {
    fn set_right_u8(&self, value: u8);
}
