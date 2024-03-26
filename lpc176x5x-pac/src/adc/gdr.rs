#[doc = "Register `GDR` reader"]
pub type R = crate::R<GDR_SPEC>;
#[doc = "Register `GDR` writer"]
pub type W = crate::W<GDR_SPEC>;
#[doc = "Field `RESULT` reader - When DONE is 1, this field contains a binary fraction representing the voltage on the AD0\\[n\\]
pin selected by the SEL field, as it falls within the range of VREFP to VSS. Zero in the field indicates that the voltage on the input pin was less than, equal to, or close to that on VSS, while 0xFFF indicates that the voltage on the input was close to, equal to, or greater than that on VREFP."]
pub type RESULT_R = crate::FieldReader<u16>;
#[doc = "Field `RESULT` writer - When DONE is 1, this field contains a binary fraction representing the voltage on the AD0\\[n\\]
pin selected by the SEL field, as it falls within the range of VREFP to VSS. Zero in the field indicates that the voltage on the input pin was less than, equal to, or close to that on VSS, while 0xFFF indicates that the voltage on the input was close to, equal to, or greater than that on VREFP."]
pub type RESULT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `CHN` reader - These bits contain the channel from which the RESULT bits were converted (e.g. 000 identifies channel 0, 001 channel 1...)."]
pub type CHN_R = crate::FieldReader;
#[doc = "Field `CHN` writer - These bits contain the channel from which the RESULT bits were converted (e.g. 000 identifies channel 0, 001 channel 1...)."]
pub type CHN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `OVERRUN` reader - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the RESULT bits. This bit is cleared by reading this register."]
pub type OVERRUN_R = crate::BitReader;
#[doc = "Field `OVERRUN` writer - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the RESULT bits. This bit is cleared by reading this register."]
pub type OVERRUN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DONE` reader - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read and when the ADCR is written. If the ADCR is written while a conversion is still in progress, this bit is set and a new conversion is started."]
pub type DONE_R = crate::BitReader;
#[doc = "Field `DONE` writer - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read and when the ADCR is written. If the ADCR is written while a conversion is still in progress, this bit is set and a new conversion is started."]
pub type DONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 4:15 - When DONE is 1, this field contains a binary fraction representing the voltage on the AD0\\[n\\]
pin selected by the SEL field, as it falls within the range of VREFP to VSS. Zero in the field indicates that the voltage on the input pin was less than, equal to, or close to that on VSS, while 0xFFF indicates that the voltage on the input was close to, equal to, or greater than that on VREFP."]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 24:26 - These bits contain the channel from which the RESULT bits were converted (e.g. 000 identifies channel 0, 001 channel 1...)."]
    #[inline(always)]
    pub fn chn(&self) -> CHN_R {
        CHN_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 30 - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the RESULT bits. This bit is cleared by reading this register."]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read and when the ADCR is written. If the ADCR is written while a conversion is still in progress, this bit is set and a new conversion is started."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GDR")
            .field("result", &format_args!("{}", self.result().bits()))
            .field("chn", &format_args!("{}", self.chn().bits()))
            .field("overrun", &format_args!("{}", self.overrun().bit()))
            .field("done", &format_args!("{}", self.done().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<GDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 4:15 - When DONE is 1, this field contains a binary fraction representing the voltage on the AD0\\[n\\]
pin selected by the SEL field, as it falls within the range of VREFP to VSS. Zero in the field indicates that the voltage on the input pin was less than, equal to, or close to that on VSS, while 0xFFF indicates that the voltage on the input was close to, equal to, or greater than that on VREFP."]
    #[inline(always)]
    #[must_use]
    pub fn result(&mut self) -> RESULT_W<GDR_SPEC, 4> {
        RESULT_W::new(self)
    }
    #[doc = "Bits 24:26 - These bits contain the channel from which the RESULT bits were converted (e.g. 000 identifies channel 0, 001 channel 1...)."]
    #[inline(always)]
    #[must_use]
    pub fn chn(&mut self) -> CHN_W<GDR_SPEC, 24> {
        CHN_W::new(self)
    }
    #[doc = "Bit 30 - This bit is 1 in burst mode if the results of one or more conversions was (were) lost and overwritten before the conversion that produced the result in the RESULT bits. This bit is cleared by reading this register."]
    #[inline(always)]
    #[must_use]
    pub fn overrun(&mut self) -> OVERRUN_W<GDR_SPEC, 30> {
        OVERRUN_W::new(self)
    }
    #[doc = "Bit 31 - This bit is set to 1 when an A/D conversion completes. It is cleared when this register is read and when the ADCR is written. If the ADCR is written while a conversion is still in progress, this bit is set and a new conversion is started."]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<GDR_SPEC, 31> {
        DONE_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "A/D Global Data Register. This register contains the ADC's DONE bit and the result of the most recent A/D conversion.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GDR_SPEC;
impl crate::RegisterSpec for GDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdr::R`](R) reader structure"]
impl crate::Readable for GDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gdr::W`](W) writer structure"]
impl crate::Writable for GDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GDR to value 0"]
impl crate::Resettable for GDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
