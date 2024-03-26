#[doc = "Register `PINMODE9` reader"]
pub type R = crate::R<PINMODE9_SPEC>;
#[doc = "Register `PINMODE9` writer"]
pub type W = crate::W<PINMODE9_SPEC>;
#[doc = "Field `P4_28MODE` reader - Port 4 pin 28 control."]
pub use super::pinmode0::P0_00MODE_R as P4_28MODE_R;
#[doc = "Field `P4_29MODE` reader - Port 4 pin 29 control."]
pub use super::pinmode0::P0_00MODE_R as P4_29MODE_R;
#[doc = "Field `P4_28MODE` writer - Port 4 pin 28 control."]
pub use super::pinmode0::P0_00MODE_W as P4_28MODE_W;
#[doc = "Field `P4_29MODE` writer - Port 4 pin 29 control."]
pub use super::pinmode0::P0_00MODE_W as P4_29MODE_W;
#[doc = "Port 4 pin 28 control."]
pub use super::pinmode0::PINMODE_A;
impl R {
    #[doc = "Bits 24:25 - Port 4 pin 28 control."]
    #[inline(always)]
    pub fn p4_28mode(&self) -> P4_28MODE_R {
        P4_28MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port 4 pin 29 control."]
    #[inline(always)]
    pub fn p4_29mode(&self) -> P4_29MODE_R {
        P4_29MODE_R::new(((self.bits >> 26) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PINMODE9")
            .field("p4_28mode", &format_args!("{}", self.p4_28mode().bits()))
            .field("p4_29mode", &format_args!("{}", self.p4_29mode().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PINMODE9_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 24:25 - Port 4 pin 28 control."]
    #[inline(always)]
    #[must_use]
    pub fn p4_28mode(&mut self) -> P4_28MODE_W<PINMODE9_SPEC, 24> {
        P4_28MODE_W::new(self)
    }
    #[doc = "Bits 26:27 - Port 4 pin 29 control."]
    #[inline(always)]
    #[must_use]
    pub fn p4_29mode(&mut self) -> P4_29MODE_W<PINMODE9_SPEC, 26> {
        P4_29MODE_W::new(self)
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
#[doc = "Pin mode select register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinmode9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinmode9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINMODE9_SPEC;
impl crate::RegisterSpec for PINMODE9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode9::R`](R) reader structure"]
impl crate::Readable for PINMODE9_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pinmode9::W`](W) writer structure"]
impl crate::Writable for PINMODE9_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINMODE9 to value 0"]
impl crate::Resettable for PINMODE9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
