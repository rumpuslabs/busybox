#[doc = "Register `DR[%s]` reader"]
pub type R = crate::R<DR_SPEC>;
#[doc = "Field `RESULT` reader - When DONE is 1, this field contains a binary fraction representing the voltage on the AD0\\[n\\]
pin, as it falls within the range of VREFP to V SS. Zero in the field indicates that the voltage on the input pin was less than, equal to, or close to that on VSS, while 0xFFF indicates that the voltage on the input was close to, equal to, or greater than that on VREFP."]
pub type RESULT_R = crate::FieldReader<u16>;
#[doc = "Field `OVERRUN` reader - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the RESULT bits.This bit is cleared by reading this register."]
pub type OVERRUN_R = crate::BitReader;
#[doc = "Field `DONE` reader - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read."]
pub type DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 4:15 - When DONE is 1, this field contains a binary fraction representing the voltage on the AD0\\[n\\]
pin, as it falls within the range of VREFP to V SS. Zero in the field indicates that the voltage on the input pin was less than, equal to, or close to that on VSS, while 0xFFF indicates that the voltage on the input was close to, equal to, or greater than that on VREFP."]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bit 30 - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the RESULT bits.This bit is cleared by reading this register."]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR")
            .field("result", &format_args!("{}", self.result().bits()))
            .field("overrun", &format_args!("{}", self.overrun().bit()))
            .field("done", &format_args!("{}", self.done().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "A/D Channel 0 Data Register. This register contains the result of the most recent conversion completed on channel 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DR_SPEC {}
#[doc = "`reset()` method sets DR[%s]
to value 0"]
impl crate::Resettable for DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
