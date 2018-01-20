#![allow(missing_docs)]

use analog_hal;
use stm32f0xx;
use gpio::Analog;
use gpio::gpioa::{PA0, PA5, PA6, PA7};
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

impl Adc<stm32f0xx::ADC> {
    pub fn adc(adc: stm32f0xx::ADC) -> Adc<stm32f0xx::ADC> {
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
}

impl<'a, PIN> analog_hal::Adc<u16> for (&'a mut Adc<stm32f0xx::ADC>, &'a mut PIN)
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
