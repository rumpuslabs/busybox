#[doc = "Register `RSID` reader"]
pub type R = crate::R<RSID_SPEC>;
#[doc = "Register `RSID` writer"]
pub type W = crate::W<RSID_SPEC>;
#[doc = "Field `POR` reader - Assertion of the POR signal sets this bit, and clears all of the other bits in this register. But if another Reset signal (e.g., External Reset) remains asserted after the POR signal is negated, then its bit is set. This bit is not affected by any of the other sources of Reset."]
pub type POR_R = crate::BitReader;
#[doc = "Field `POR` writer - Assertion of the POR signal sets this bit, and clears all of the other bits in this register. But if another Reset signal (e.g., External Reset) remains asserted after the POR signal is negated, then its bit is set. This bit is not affected by any of the other sources of Reset."]
pub type POR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTR` reader - Assertion of the RESET signal sets this bit. This bit is cleared only by software or POR."]
pub type EXTR_R = crate::BitReader;
#[doc = "Field `EXTR` writer - Assertion of the RESET signal sets this bit. This bit is cleared only by software or POR."]
pub type EXTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDTR` reader - This bit is set when the Watchdog Timer times out and the WDTRESET bit in the Watchdog Mode Register is 1. This bit is cleared only by software or POR."]
pub type WDTR_R = crate::BitReader;
#[doc = "Field `WDTR` writer - This bit is set when the Watchdog Timer times out and the WDTRESET bit in the Watchdog Mode Register is 1. This bit is cleared only by software or POR."]
pub type WDTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BODR` reader - This bit is set when the VDD(REG)(3V3) voltage reaches a level below the BOD reset trip level (typically 1.85 V under nominal room temperature conditions). If the VDD(REG)(3V3) voltage dips from the normal operating range to below the BOD reset trip level and recovers, the BODR bit will be set to 1. If the VDD(REG)(3V3) voltage dips from the normal operating range to below the BOD reset trip level and continues to decline to the level at which POR is asserted (nominally 1 V), the BODR bit is cleared. If the VDD(REG)(3V3) voltage rises continuously from below 1 V to a level above the BOD reset trip level, the BODR will be set to 1. This bit is cleared only by software or POR. Note: Only in the case where a reset occurs and the POR = 0, the BODR bit indicates if the VDD(REG)(3V3) voltage was below the BOD reset trip level or not."]
pub type BODR_R = crate::BitReader;
#[doc = "Field `BODR` writer - This bit is set when the VDD(REG)(3V3) voltage reaches a level below the BOD reset trip level (typically 1.85 V under nominal room temperature conditions). If the VDD(REG)(3V3) voltage dips from the normal operating range to below the BOD reset trip level and recovers, the BODR bit will be set to 1. If the VDD(REG)(3V3) voltage dips from the normal operating range to below the BOD reset trip level and continues to decline to the level at which POR is asserted (nominally 1 V), the BODR bit is cleared. If the VDD(REG)(3V3) voltage rises continuously from below 1 V to a level above the BOD reset trip level, the BODR will be set to 1. This bit is cleared only by software or POR. Note: Only in the case where a reset occurs and the POR = 0, the BODR bit indicates if the VDD(REG)(3V3) voltage was below the BOD reset trip level or not."]
pub type BODR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Assertion of the POR signal sets this bit, and clears all of the other bits in this register. But if another Reset signal (e.g., External Reset) remains asserted after the POR signal is negated, then its bit is set. This bit is not affected by any of the other sources of Reset."]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Assertion of the RESET signal sets this bit. This bit is cleared only by software or POR."]
    #[inline(always)]
    pub fn extr(&self) -> EXTR_R {
        EXTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is set when the Watchdog Timer times out and the WDTRESET bit in the Watchdog Mode Register is 1. This bit is cleared only by software or POR."]
    #[inline(always)]
    pub fn wdtr(&self) -> WDTR_R {
        WDTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is set when the VDD(REG)(3V3) voltage reaches a level below the BOD reset trip level (typically 1.85 V under nominal room temperature conditions). If the VDD(REG)(3V3) voltage dips from the normal operating range to below the BOD reset trip level and recovers, the BODR bit will be set to 1. If the VDD(REG)(3V3) voltage dips from the normal operating range to below the BOD reset trip level and continues to decline to the level at which POR is asserted (nominally 1 V), the BODR bit is cleared. If the VDD(REG)(3V3) voltage rises continuously from below 1 V to a level above the BOD reset trip level, the BODR will be set to 1. This bit is cleared only by software or POR. Note: Only in the case where a reset occurs and the POR = 0, the BODR bit indicates if the VDD(REG)(3V3) voltage was below the BOD reset trip level or not."]
    #[inline(always)]
    pub fn bodr(&self) -> BODR_R {
        BODR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSID")
            .field("por", &format_args!("{}", self.por().bit()))
            .field("extr", &format_args!("{}", self.extr().bit()))
            .field("wdtr", &format_args!("{}", self.wdtr().bit()))
            .field("bodr", &format_args!("{}", self.bodr().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RSID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Assertion of the POR signal sets this bit, and clears all of the other bits in this register. But if another Reset signal (e.g., External Reset) remains asserted after the POR signal is negated, then its bit is set. This bit is not affected by any of the other sources of Reset."]
    #[inline(always)]
    #[must_use]
    pub fn por(&mut self) -> POR_W<RSID_SPEC, 0> {
        POR_W::new(self)
    }
    #[doc = "Bit 1 - Assertion of the RESET signal sets this bit. This bit is cleared only by software or POR."]
    #[inline(always)]
    #[must_use]
    pub fn extr(&mut self) -> EXTR_W<RSID_SPEC, 1> {
        EXTR_W::new(self)
    }
    #[doc = "Bit 2 - This bit is set when the Watchdog Timer times out and the WDTRESET bit in the Watchdog Mode Register is 1. This bit is cleared only by software or POR."]
    #[inline(always)]
    #[must_use]
    pub fn wdtr(&mut self) -> WDTR_W<RSID_SPEC, 2> {
        WDTR_W::new(self)
    }
    #[doc = "Bit 3 - This bit is set when the VDD(REG)(3V3) voltage reaches a level below the BOD reset trip level (typically 1.85 V under nominal room temperature conditions). If the VDD(REG)(3V3) voltage dips from the normal operating range to below the BOD reset trip level and recovers, the BODR bit will be set to 1. If the VDD(REG)(3V3) voltage dips from the normal operating range to below the BOD reset trip level and continues to decline to the level at which POR is asserted (nominally 1 V), the BODR bit is cleared. If the VDD(REG)(3V3) voltage rises continuously from below 1 V to a level above the BOD reset trip level, the BODR will be set to 1. This bit is cleared only by software or POR. Note: Only in the case where a reset occurs and the POR = 0, the BODR bit indicates if the VDD(REG)(3V3) voltage was below the BOD reset trip level or not."]
    #[inline(always)]
    #[must_use]
    pub fn bodr(&mut self) -> BODR_W<RSID_SPEC, 3> {
        BODR_W::new(self)
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
#[doc = "Reset Source Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSID_SPEC;
impl crate::RegisterSpec for RSID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsid::R`](R) reader structure"]
impl crate::Readable for RSID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rsid::W`](W) writer structure"]
impl crate::Writable for RSID_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSID to value 0"]
impl crate::Resettable for RSID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
