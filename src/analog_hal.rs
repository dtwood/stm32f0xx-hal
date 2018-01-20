#![allow(missing_docs)]

pub trait Adc<Word> {
    fn read(&mut self) -> u16;
}

pub trait Dac<Word> {
    fn set(&mut self, value: Word);
}
