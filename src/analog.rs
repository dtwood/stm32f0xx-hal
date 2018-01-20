#![allow(missing_docs)]

use analog_hal;
use stm32f0xx::{ADC, DAC};
use gpio::Analog;
use gpio::gpioa::{PA0, PA4, PA5, PA6, PA7};
use gpio::gpiob::{PB0, PB1};

pub struct Adc<ADC> {
    adc: ADC,
}

pub unsafe trait AdcPin {
    fn get_channel(&self) -> u8 {
        unimplemented!()
    }
}

macro_rules! adc_pin {
    ($port: ident, $channel: expr) => {
        unsafe impl AdcPin for $port<Analog> {
            fn get_channel(&self) -> u8 {
                $channel
            }
        }
    }
}

adc_pin!(PA0, 0);
adc_pin!(PA5, 5);
adc_pin!(PA6, 6);
adc_pin!(PA7, 7);
adc_pin!(PB0, 8);
adc_pin!(PB1, 9);

impl Adc<ADC> {
    pub fn adc(adc: ADC) -> Adc<ADC> {
        adc.cr.write(|w| w.addis().set_bit());
        while adc.cr.read().aden().bit_is_set() { /* Wait for the ADC to be disabled */ }

        adc.cfgr2.reset();

        adc.cr.write(|w| w.adcal().set_bit());
        while adc.cr.read().adcal().bit_is_set() { /* Wait for calibration to finish */ }

        unsafe {
            adc.cfgr1.write(|w| {
                w.cont()
                    .clear_bit()
                    .discen()
                    .set_bit()
                    .align()
                    .set_bit()
                    .res()
                    .bits(0)
            });

            adc.smpr.write(|w| w.smpr().bits(6));
        }

        adc.cr.write(|w| w.aden().set_bit());
        while adc.cr.read().aden().bit_is_clear() { /* Wait for the ADC to be enabled */ }

        Adc { adc }
    }

    pub fn free(self) -> ADC {
        self.adc.cr.write(|w| w.addis().set_bit());
        self.adc
    }
}

impl<'a, PIN> analog_hal::Adc<u16> for (&'a mut Adc<ADC>, &'a mut PIN)
where
    PIN: AdcPin,
{
    fn read(&self) -> u16 {
        let adc = &self.0.adc;
        let pin = &self.1;

        adc.chselr
            .write(|w| unsafe { w.bits(1 << pin.get_channel()) });

        adc.cr.write(|w| w.adstart().set_bit());
        while adc.isr.read().eoc().bit_is_clear() { /* Wait for the conversion to finish */ }

        adc.dr.read().data().bits()
    }
}

pub struct Dac<DAC, PINS> {
    dac: DAC,
    pins: PINS,
}

impl<LEFT, RIGHT> Dac<DAC, (LEFT, RIGHT)>
where
    LEFT: DacLeftPin,
    RIGHT: DacRightPin,
{
    pub fn dac(dac: DAC, pins: (LEFT, RIGHT)) -> Self {
        Self { dac, pins }
    }

    pub fn free(self) -> (DAC, (LEFT, RIGHT)) {
        (self.dac, self.pins)
    }
}

impl<PINS> analog_hal::Dac for Dac<DAC, PINS> {
    fn set_right_u8(&self, value: u8) {
        self.dac
            .dhr8r1
            .write(|w| unsafe { w.dacc1dhr().bits(value) });
    }
}

pub unsafe trait DacLeftPin {}
pub unsafe trait DacRightPin {}

unsafe impl DacLeftPin for PA4<Analog> {}
unsafe impl DacRightPin for PA5<Analog> {}
