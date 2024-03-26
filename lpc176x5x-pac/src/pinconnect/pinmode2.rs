#[doc = "Register `PINMODE2` reader"]
pub type R = crate::R<PINMODE2_SPEC>;
#[doc = "Register `PINMODE2` writer"]
pub type W = crate::W<PINMODE2_SPEC>;
#[doc = "Field `P1_00MODE` reader - Port 1 pin 0 control."]
pub use super::pinmode0::P0_00MODE_R as P1_00MODE_R;
#[doc = "Field `P1_01MODE` reader - Port 1 pin 1 control."]
pub use super::pinmode0::P0_00MODE_R as P1_01MODE_R;
#[doc = "Field `P1_04MODE` reader - Port 1 pin 4 control."]
pub use super::pinmode0::P0_00MODE_R as P1_04MODE_R;
#[doc = "Field `P1_08MODE` reader - Port 1 pin 8 control."]
pub use super::pinmode0::P0_00MODE_R as P1_08MODE_R;
#[doc = "Field `P1_09MODE` reader - Port 1 pin 9 control."]
pub use super::pinmode0::P0_00MODE_R as P1_09MODE_R;
#[doc = "Field `P1_10MODE` reader - Port 1 pin 10 control."]
pub use super::pinmode0::P0_00MODE_R as P1_10MODE_R;
#[doc = "Field `P1_14MODE` reader - Port 1 pin 14 control."]
pub use super::pinmode0::P0_00MODE_R as P1_14MODE_R;
#[doc = "Field `P1_15MODE` reader - Port 1 pin 15 control."]
pub use super::pinmode0::P0_00MODE_R as P1_15MODE_R;
#[doc = "Field `P1_00MODE` writer - Port 1 pin 0 control."]
pub use super::pinmode0::P0_00MODE_W as P1_00MODE_W;
#[doc = "Field `P1_01MODE` writer - Port 1 pin 1 control."]
pub use super::pinmode0::P0_00MODE_W as P1_01MODE_W;
#[doc = "Field `P1_04MODE` writer - Port 1 pin 4 control."]
pub use super::pinmode0::P0_00MODE_W as P1_04MODE_W;
#[doc = "Field `P1_08MODE` writer - Port 1 pin 8 control."]
pub use super::pinmode0::P0_00MODE_W as P1_08MODE_W;
#[doc = "Field `P1_09MODE` writer - Port 1 pin 9 control."]
pub use super::pinmode0::P0_00MODE_W as P1_09MODE_W;
#[doc = "Field `P1_10MODE` writer - Port 1 pin 10 control."]
pub use super::pinmode0::P0_00MODE_W as P1_10MODE_W;
#[doc = "Field `P1_14MODE` writer - Port 1 pin 14 control."]
pub use super::pinmode0::P0_00MODE_W as P1_14MODE_W;
#[doc = "Field `P1_15MODE` writer - Port 1 pin 15 control."]
pub use super::pinmode0::P0_00MODE_W as P1_15MODE_W;
#[doc = "Port 1 pin 0 control."]
pub use super::pinmode0::PINMODE_A;
impl R {
    #[doc = "Bits 0:1 - Port 1 pin 0 control."]
    #[inline(always)]
    pub fn p1_00mode(&self) -> P1_00MODE_R {
        P1_00MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port 1 pin 1 control."]
    #[inline(always)]
    pub fn p1_01mode(&self) -> P1_01MODE_R {
        P1_01MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port 1 pin 4 control."]
    #[inline(always)]
    pub fn p1_04mode(&self) -> P1_04MODE_R {
        P1_04MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port 1 pin 8 control."]
    #[inline(always)]
    pub fn p1_08mode(&self) -> P1_08MODE_R {
        P1_08MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port 1 pin 9 control."]
    #[inline(always)]
    pub fn p1_09mode(&self) -> P1_09MODE_R {
        P1_09MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port 1 pin 10 control."]
    #[inline(always)]
    pub fn p1_10mode(&self) -> P1_10MODE_R {
        P1_10MODE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port 1 pin 14 control."]
    #[inline(always)]
    pub fn p1_14mode(&self) -> P1_14MODE_R {
        P1_14MODE_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port 1 pin 15 control."]
    #[inline(always)]
    pub fn p1_15mode(&self) -> P1_15MODE_R {
        P1_15MODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PINMODE2")
            .field("p1_00mode", &format_args!("{}", self.p1_00mode().bits()))
            .field("p1_01mode", &format_args!("{}", self.p1_01mode().bits()))
            .field("p1_04mode", &format_args!("{}", self.p1_04mode().bits()))
            .field("p1_08mode", &format_args!("{}", self.p1_08mode().bits()))
            .field("p1_09mode", &format_args!("{}", self.p1_09mode().bits()))
            .field("p1_10mode", &format_args!("{}", self.p1_10mode().bits()))
            .field("p1_14mode", &format_args!("{}", self.p1_14mode().bits()))
            .field("p1_15mode", &format_args!("{}", self.p1_15mode().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PINMODE2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port 1 pin 0 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_00mode(&mut self) -> P1_00MODE_W<PINMODE2_SPEC, 0> {
        P1_00MODE_W::new(self)
    }
    #[doc = "Bits 2:3 - Port 1 pin 1 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_01mode(&mut self) -> P1_01MODE_W<PINMODE2_SPEC, 2> {
        P1_01MODE_W::new(self)
    }
    #[doc = "Bits 8:9 - Port 1 pin 4 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_04mode(&mut self) -> P1_04MODE_W<PINMODE2_SPEC, 8> {
        P1_04MODE_W::new(self)
    }
    #[doc = "Bits 16:17 - Port 1 pin 8 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_08mode(&mut self) -> P1_08MODE_W<PINMODE2_SPEC, 16> {
        P1_08MODE_W::new(self)
    }
    #[doc = "Bits 18:19 - Port 1 pin 9 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_09mode(&mut self) -> P1_09MODE_W<PINMODE2_SPEC, 18> {
        P1_09MODE_W::new(self)
    }
    #[doc = "Bits 20:21 - Port 1 pin 10 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_10mode(&mut self) -> P1_10MODE_W<PINMODE2_SPEC, 20> {
        P1_10MODE_W::new(self)
    }
    #[doc = "Bits 28:29 - Port 1 pin 14 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_14mode(&mut self) -> P1_14MODE_W<PINMODE2_SPEC, 28> {
        P1_14MODE_W::new(self)
    }
    #[doc = "Bits 30:31 - Port 1 pin 15 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_15mode(&mut self) -> P1_15MODE_W<PINMODE2_SPEC, 30> {
        P1_15MODE_W::new(self)
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
#[doc = "Pin mode select register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinmode2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinmode2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINMODE2_SPEC;
impl crate::RegisterSpec for PINMODE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode2::R`](R) reader structure"]
impl crate::Readable for PINMODE2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pinmode2::W`](W) writer structure"]
impl crate::Writable for PINMODE2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINMODE2 to value 0"]
impl crate::Resettable for PINMODE2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
