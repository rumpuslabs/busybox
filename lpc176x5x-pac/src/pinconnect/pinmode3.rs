#[doc = "Register `PINMODE3` reader"]
pub type R = crate::R<PINMODE3_SPEC>;
#[doc = "Register `PINMODE3` writer"]
pub type W = crate::W<PINMODE3_SPEC>;
#[doc = "Field `P1_16MODE` reader - Port 1 pin 16 control."]
pub use super::pinmode0::P0_00MODE_R as P1_16MODE_R;
#[doc = "Field `P1_17MODE` reader - Port 1 pin 17 control."]
pub use super::pinmode0::P0_00MODE_R as P1_17MODE_R;
#[doc = "Field `P1_18MODE` reader - Port 1 pin 18 control."]
pub use super::pinmode0::P0_00MODE_R as P1_18MODE_R;
#[doc = "Field `P1_19MODE` reader - Port 1 pin 19 control."]
pub use super::pinmode0::P0_00MODE_R as P1_19MODE_R;
#[doc = "Field `P1_20MODE` reader - Port 1 pin 20 control."]
pub use super::pinmode0::P0_00MODE_R as P1_20MODE_R;
#[doc = "Field `P1_21MODE` reader - Port 1 pin 21 control."]
pub use super::pinmode0::P0_00MODE_R as P1_21MODE_R;
#[doc = "Field `P1_22MODE` reader - Port 1 pin 22 control."]
pub use super::pinmode0::P0_00MODE_R as P1_22MODE_R;
#[doc = "Field `P1_23MODE` reader - Port 1 pin 23 control."]
pub use super::pinmode0::P0_00MODE_R as P1_23MODE_R;
#[doc = "Field `P1_24MODE` reader - Port 1 pin 24 control."]
pub use super::pinmode0::P0_00MODE_R as P1_24MODE_R;
#[doc = "Field `P1_25MODE` reader - Port 1 pin 25 control."]
pub use super::pinmode0::P0_00MODE_R as P1_25MODE_R;
#[doc = "Field `P1_26MODE` reader - Port 1 pin 26 control."]
pub use super::pinmode0::P0_00MODE_R as P1_26MODE_R;
#[doc = "Field `P1_27MODE` reader - Port 1 pin 27 control."]
pub use super::pinmode0::P0_00MODE_R as P1_27MODE_R;
#[doc = "Field `P1_28MODE` reader - Port 1 pin 28 control."]
pub use super::pinmode0::P0_00MODE_R as P1_28MODE_R;
#[doc = "Field `P1_29MODE` reader - Port 1 pin 29 control."]
pub use super::pinmode0::P0_00MODE_R as P1_29MODE_R;
#[doc = "Field `P1_30MODE` reader - Port 1 pin 30 control."]
pub use super::pinmode0::P0_00MODE_R as P1_30MODE_R;
#[doc = "Field `P1_31MODE` reader - Port 1 pin 31 control."]
pub use super::pinmode0::P0_00MODE_R as P1_31MODE_R;
#[doc = "Field `P1_16MODE` writer - Port 1 pin 16 control."]
pub use super::pinmode0::P0_00MODE_W as P1_16MODE_W;
#[doc = "Field `P1_17MODE` writer - Port 1 pin 17 control."]
pub use super::pinmode0::P0_00MODE_W as P1_17MODE_W;
#[doc = "Field `P1_18MODE` writer - Port 1 pin 18 control."]
pub use super::pinmode0::P0_00MODE_W as P1_18MODE_W;
#[doc = "Field `P1_19MODE` writer - Port 1 pin 19 control."]
pub use super::pinmode0::P0_00MODE_W as P1_19MODE_W;
#[doc = "Field `P1_20MODE` writer - Port 1 pin 20 control."]
pub use super::pinmode0::P0_00MODE_W as P1_20MODE_W;
#[doc = "Field `P1_21MODE` writer - Port 1 pin 21 control."]
pub use super::pinmode0::P0_00MODE_W as P1_21MODE_W;
#[doc = "Field `P1_22MODE` writer - Port 1 pin 22 control."]
pub use super::pinmode0::P0_00MODE_W as P1_22MODE_W;
#[doc = "Field `P1_23MODE` writer - Port 1 pin 23 control."]
pub use super::pinmode0::P0_00MODE_W as P1_23MODE_W;
#[doc = "Field `P1_24MODE` writer - Port 1 pin 24 control."]
pub use super::pinmode0::P0_00MODE_W as P1_24MODE_W;
#[doc = "Field `P1_25MODE` writer - Port 1 pin 25 control."]
pub use super::pinmode0::P0_00MODE_W as P1_25MODE_W;
#[doc = "Field `P1_26MODE` writer - Port 1 pin 26 control."]
pub use super::pinmode0::P0_00MODE_W as P1_26MODE_W;
#[doc = "Field `P1_27MODE` writer - Port 1 pin 27 control."]
pub use super::pinmode0::P0_00MODE_W as P1_27MODE_W;
#[doc = "Field `P1_28MODE` writer - Port 1 pin 28 control."]
pub use super::pinmode0::P0_00MODE_W as P1_28MODE_W;
#[doc = "Field `P1_29MODE` writer - Port 1 pin 29 control."]
pub use super::pinmode0::P0_00MODE_W as P1_29MODE_W;
#[doc = "Field `P1_30MODE` writer - Port 1 pin 30 control."]
pub use super::pinmode0::P0_00MODE_W as P1_30MODE_W;
#[doc = "Field `P1_31MODE` writer - Port 1 pin 31 control."]
pub use super::pinmode0::P0_00MODE_W as P1_31MODE_W;
#[doc = "Port 1 pin 16 control."]
pub use super::pinmode0::PINMODE_A;
impl R {
    #[doc = "Bits 0:1 - Port 1 pin 16 control."]
    #[inline(always)]
    pub fn p1_16mode(&self) -> P1_16MODE_R {
        P1_16MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port 1 pin 17 control."]
    #[inline(always)]
    pub fn p1_17mode(&self) -> P1_17MODE_R {
        P1_17MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port 1 pin 18 control."]
    #[inline(always)]
    pub fn p1_18mode(&self) -> P1_18MODE_R {
        P1_18MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port 1 pin 19 control."]
    #[inline(always)]
    pub fn p1_19mode(&self) -> P1_19MODE_R {
        P1_19MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port 1 pin 20 control."]
    #[inline(always)]
    pub fn p1_20mode(&self) -> P1_20MODE_R {
        P1_20MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port 1 pin 21 control."]
    #[inline(always)]
    pub fn p1_21mode(&self) -> P1_21MODE_R {
        P1_21MODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port 1 pin 22 control."]
    #[inline(always)]
    pub fn p1_22mode(&self) -> P1_22MODE_R {
        P1_22MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port 1 pin 23 control."]
    #[inline(always)]
    pub fn p1_23mode(&self) -> P1_23MODE_R {
        P1_23MODE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port 1 pin 24 control."]
    #[inline(always)]
    pub fn p1_24mode(&self) -> P1_24MODE_R {
        P1_24MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port 1 pin 25 control."]
    #[inline(always)]
    pub fn p1_25mode(&self) -> P1_25MODE_R {
        P1_25MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port 1 pin 26 control."]
    #[inline(always)]
    pub fn p1_26mode(&self) -> P1_26MODE_R {
        P1_26MODE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port 1 pin 27 control."]
    #[inline(always)]
    pub fn p1_27mode(&self) -> P1_27MODE_R {
        P1_27MODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port 1 pin 28 control."]
    #[inline(always)]
    pub fn p1_28mode(&self) -> P1_28MODE_R {
        P1_28MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port 1 pin 29 control."]
    #[inline(always)]
    pub fn p1_29mode(&self) -> P1_29MODE_R {
        P1_29MODE_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port 1 pin 30 control."]
    #[inline(always)]
    pub fn p1_30mode(&self) -> P1_30MODE_R {
        P1_30MODE_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port 1 pin 31 control."]
    #[inline(always)]
    pub fn p1_31mode(&self) -> P1_31MODE_R {
        P1_31MODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PINMODE3")
            .field("p1_16mode", &format_args!("{}", self.p1_16mode().bits()))
            .field("p1_17mode", &format_args!("{}", self.p1_17mode().bits()))
            .field("p1_18mode", &format_args!("{}", self.p1_18mode().bits()))
            .field("p1_19mode", &format_args!("{}", self.p1_19mode().bits()))
            .field("p1_20mode", &format_args!("{}", self.p1_20mode().bits()))
            .field("p1_21mode", &format_args!("{}", self.p1_21mode().bits()))
            .field("p1_22mode", &format_args!("{}", self.p1_22mode().bits()))
            .field("p1_23mode", &format_args!("{}", self.p1_23mode().bits()))
            .field("p1_24mode", &format_args!("{}", self.p1_24mode().bits()))
            .field("p1_25mode", &format_args!("{}", self.p1_25mode().bits()))
            .field("p1_26mode", &format_args!("{}", self.p1_26mode().bits()))
            .field("p1_27mode", &format_args!("{}", self.p1_27mode().bits()))
            .field("p1_28mode", &format_args!("{}", self.p1_28mode().bits()))
            .field("p1_29mode", &format_args!("{}", self.p1_29mode().bits()))
            .field("p1_30mode", &format_args!("{}", self.p1_30mode().bits()))
            .field("p1_31mode", &format_args!("{}", self.p1_31mode().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PINMODE3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port 1 pin 16 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_16mode(&mut self) -> P1_16MODE_W<PINMODE3_SPEC, 0> {
        P1_16MODE_W::new(self)
    }
    #[doc = "Bits 2:3 - Port 1 pin 17 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_17mode(&mut self) -> P1_17MODE_W<PINMODE3_SPEC, 2> {
        P1_17MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - Port 1 pin 18 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_18mode(&mut self) -> P1_18MODE_W<PINMODE3_SPEC, 4> {
        P1_18MODE_W::new(self)
    }
    #[doc = "Bits 6:7 - Port 1 pin 19 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_19mode(&mut self) -> P1_19MODE_W<PINMODE3_SPEC, 6> {
        P1_19MODE_W::new(self)
    }
    #[doc = "Bits 8:9 - Port 1 pin 20 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_20mode(&mut self) -> P1_20MODE_W<PINMODE3_SPEC, 8> {
        P1_20MODE_W::new(self)
    }
    #[doc = "Bits 10:11 - Port 1 pin 21 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_21mode(&mut self) -> P1_21MODE_W<PINMODE3_SPEC, 10> {
        P1_21MODE_W::new(self)
    }
    #[doc = "Bits 12:13 - Port 1 pin 22 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_22mode(&mut self) -> P1_22MODE_W<PINMODE3_SPEC, 12> {
        P1_22MODE_W::new(self)
    }
    #[doc = "Bits 14:15 - Port 1 pin 23 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_23mode(&mut self) -> P1_23MODE_W<PINMODE3_SPEC, 14> {
        P1_23MODE_W::new(self)
    }
    #[doc = "Bits 16:17 - Port 1 pin 24 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_24mode(&mut self) -> P1_24MODE_W<PINMODE3_SPEC, 16> {
        P1_24MODE_W::new(self)
    }
    #[doc = "Bits 18:19 - Port 1 pin 25 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_25mode(&mut self) -> P1_25MODE_W<PINMODE3_SPEC, 18> {
        P1_25MODE_W::new(self)
    }
    #[doc = "Bits 20:21 - Port 1 pin 26 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_26mode(&mut self) -> P1_26MODE_W<PINMODE3_SPEC, 20> {
        P1_26MODE_W::new(self)
    }
    #[doc = "Bits 22:23 - Port 1 pin 27 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_27mode(&mut self) -> P1_27MODE_W<PINMODE3_SPEC, 22> {
        P1_27MODE_W::new(self)
    }
    #[doc = "Bits 24:25 - Port 1 pin 28 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_28mode(&mut self) -> P1_28MODE_W<PINMODE3_SPEC, 24> {
        P1_28MODE_W::new(self)
    }
    #[doc = "Bits 26:27 - Port 1 pin 29 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_29mode(&mut self) -> P1_29MODE_W<PINMODE3_SPEC, 26> {
        P1_29MODE_W::new(self)
    }
    #[doc = "Bits 28:29 - Port 1 pin 30 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_30mode(&mut self) -> P1_30MODE_W<PINMODE3_SPEC, 28> {
        P1_30MODE_W::new(self)
    }
    #[doc = "Bits 30:31 - Port 1 pin 31 control."]
    #[inline(always)]
    #[must_use]
    pub fn p1_31mode(&mut self) -> P1_31MODE_W<PINMODE3_SPEC, 30> {
        P1_31MODE_W::new(self)
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
#[doc = "Pin mode select register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinmode3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinmode3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINMODE3_SPEC;
impl crate::RegisterSpec for PINMODE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode3::R`](R) reader structure"]
impl crate::Readable for PINMODE3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pinmode3::W`](W) writer structure"]
impl crate::Writable for PINMODE3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINMODE3 to value 0"]
impl crate::Resettable for PINMODE3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
