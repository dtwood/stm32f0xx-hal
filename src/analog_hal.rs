#![allow(missing_docs)]

pub trait Adc<Word> {
    fn read(&self) -> u16;
}
