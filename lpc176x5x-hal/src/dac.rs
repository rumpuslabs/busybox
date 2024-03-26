use crate::pac;
use crate::pins::*;
use cortex_m_semihosting::hprintln;

pub struct DacOut<P: PinId> {
    _id: P,
    dac: pac::DAC,
    gain: f32,
    offset: f32,
}

impl<P> Pin<P, Aout>
where
    P: PinId,
{
    pub fn into_dac_out(self, dac: pac::DAC, gain: f32, offset: f32) -> DacOut<P> {
        DacOut {
            _id: self.id,
            dac,
            gain,
            offset,
        }
    }
}

impl<P> DacOut<P>
where
    P: PinId,
{
    pub fn set_value(&mut self, value: f32) {
        //hprintln!(
        //    "value = {}, * gain = {}, + offset = {}, .clamp = {}, * 1024 = {}",
        //    value,
        //    value * self.gain,
        //    (value * self.gain) + self.offset,
        //    ((value * self.gain) + self.offset).clamp(0.0, 1.0),
        //    (((value * self.gain) + self.offset).clamp(0.0, 1.0) * 1024.0) as u16
        //);
        hprintln!(
            "value = {}, dac = {}",
            value,
            (((value * self.gain) + self.offset).clamp(0.0, 1.0) * 1024.0) as u16
        );
        unsafe {
            self.dac.cr.write(|w| {
                w.value()
                    .bits((((value * self.gain) + self.offset).clamp(0.0, 1.0) * 1024.0) as u16)
            });
        }
    }
}
