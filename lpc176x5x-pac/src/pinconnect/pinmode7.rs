#[doc = "Register `PINMODE7` reader"]
pub type R = crate::R<PINMODE7_SPEC>;
#[doc = "Register `PINMODE7` writer"]
pub type W = crate::W<PINMODE7_SPEC>;
#[doc = "Field `P3_25MODE` reader - Port 3 pin 25 control."]
pub use super::pinmode0::P0_00MODE_R as P3_25MODE_R;
#[doc = "Field `P3_26MODE` reader - Port 3 pin 26 control."]
pub use super::pinmode0::P0_00MODE_R as P3_26MODE_R;
#[doc = "Field `P3_25MODE` writer - Port 3 pin 25 control."]
pub use super::pinmode0::P0_00MODE_W as P3_25MODE_W;
#[doc = "Field `P3_26MODE` writer - Port 3 pin 26 control."]
pub use super::pinmode0::P0_00MODE_W as P3_26MODE_W;
#[doc = "Port 3 pin 25 control."]
pub use super::pinmode0::PINMODE_A;
impl R {
    #[doc = "Bits 18:19 - Port 3 pin 25 control."]
    #[inline(always)]
    pub fn p3_25mode(&self) -> P3_25MODE_R {
        P3_25MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port 3 pin 26 control."]
    #[inline(always)]
    pub fn p3_26mode(&self) -> P3_26MODE_R {
        P3_26MODE_R::new(((self.bits >> 20) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PINMODE7")
            .field("p3_25mode", &format_args!("{}", self.p3_25mode().bits()))
            .field("p3_26mode", &format_args!("{}", self.p3_26mode().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PINMODE7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 18:19 - Port 3 pin 25 control."]
    #[inline(always)]
    #[must_use]
    pub fn p3_25mode(&mut self) -> P3_25MODE_W<PINMODE7_SPEC, 18> {
        P3_25MODE_W::new(self)
    }
    #[doc = "Bits 20:21 - Port 3 pin 26 control."]
    #[inline(always)]
    #[must_use]
    pub fn p3_26mode(&mut self) -> P3_26MODE_W<PINMODE7_SPEC, 20> {
        P3_26MODE_W::new(self)
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
#[doc = "Pin mode select register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinmode7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinmode7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINMODE7_SPEC;
impl crate::RegisterSpec for PINMODE7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode7::R`](R) reader structure"]
impl crate::Readable for PINMODE7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pinmode7::W`](W) writer structure"]
impl crate::Writable for PINMODE7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINMODE7 to value 0"]
impl crate::Resettable for PINMODE7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
