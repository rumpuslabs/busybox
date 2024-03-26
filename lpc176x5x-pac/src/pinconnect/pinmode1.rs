#[doc = "Register `PINMODE1` reader"]
pub type R = crate::R<PINMODE1_SPEC>;
#[doc = "Register `PINMODE1` writer"]
pub type W = crate::W<PINMODE1_SPEC>;
#[doc = "Field `P0_16MODE` reader - Port 1 pin 16 control."]
pub use super::pinmode0::P0_00MODE_R as P0_16MODE_R;
#[doc = "Field `P0_17MODE` reader - Port 1 pin 17 control."]
pub use super::pinmode0::P0_00MODE_R as P0_17MODE_R;
#[doc = "Field `P0_18MODE` reader - Port 1 pin 18 control."]
pub use super::pinmode0::P0_00MODE_R as P0_18MODE_R;
#[doc = "Field `P0_19MODE` reader - Port 1 pin 19 control."]
pub use super::pinmode0::P0_00MODE_R as P0_19MODE_R;
#[doc = "Field `P0_20MODE` reader - Port 1 pin 20 control."]
pub use super::pinmode0::P0_00MODE_R as P0_20MODE_R;
#[doc = "Field `P0_21MODE` reader - Port 1 pin 21 control."]
pub use super::pinmode0::P0_00MODE_R as P0_21MODE_R;
#[doc = "Field `P0_22MODE` reader - Port 1 pin 22 control."]
pub use super::pinmode0::P0_00MODE_R as P0_22MODE_R;
#[doc = "Field `P0_23MODE` reader - Port 1 pin 23 control."]
pub use super::pinmode0::P0_00MODE_R as P0_23MODE_R;
#[doc = "Field `P0_24MODE` reader - Port 1 pin 24 control."]
pub use super::pinmode0::P0_00MODE_R as P0_24MODE_R;
#[doc = "Field `P0_25MODE` reader - Port 1 pin 25 control."]
pub use super::pinmode0::P0_00MODE_R as P0_25MODE_R;
#[doc = "Field `P0_26MODE` reader - Port 1 pin 26 control."]
pub use super::pinmode0::P0_00MODE_R as P0_26MODE_R;
#[doc = "Field `P0_16MODE` writer - Port 1 pin 16 control."]
pub use super::pinmode0::P0_00MODE_W as P0_16MODE_W;
#[doc = "Field `P0_17MODE` writer - Port 1 pin 17 control."]
pub use super::pinmode0::P0_00MODE_W as P0_17MODE_W;
#[doc = "Field `P0_18MODE` writer - Port 1 pin 18 control."]
pub use super::pinmode0::P0_00MODE_W as P0_18MODE_W;
#[doc = "Field `P0_19MODE` writer - Port 1 pin 19 control."]
pub use super::pinmode0::P0_00MODE_W as P0_19MODE_W;
#[doc = "Field `P0_20MODE` writer - Port 1 pin 20 control."]
pub use super::pinmode0::P0_00MODE_W as P0_20MODE_W;
#[doc = "Field `P0_21MODE` writer - Port 1 pin 21 control."]
pub use super::pinmode0::P0_00MODE_W as P0_21MODE_W;
#[doc = "Field `P0_22MODE` writer - Port 1 pin 22 control."]
pub use super::pinmode0::P0_00MODE_W as P0_22MODE_W;
#[doc = "Field `P0_23MODE` writer - Port 1 pin 23 control."]
pub use super::pinmode0::P0_00MODE_W as P0_23MODE_W;
#[doc = "Field `P0_24MODE` writer - Port 1 pin 24 control."]
pub use super::pinmode0::P0_00MODE_W as P0_24MODE_W;
#[doc = "Field `P0_25MODE` writer - Port 1 pin 25 control."]
pub use super::pinmode0::P0_00MODE_W as P0_25MODE_W;
#[doc = "Field `P0_26MODE` writer - Port 1 pin 26 control."]
pub use super::pinmode0::P0_00MODE_W as P0_26MODE_W;
#[doc = "Port 1 pin 16 control."]
pub use super::pinmode0::PINMODE_A;
impl R {
    #[doc = "Bits 0:1 - Port 1 pin 16 control."]
    #[inline(always)]
    pub fn p0_16mode(&self) -> P0_16MODE_R {
        P0_16MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port 1 pin 17 control."]
    #[inline(always)]
    pub fn p0_17mode(&self) -> P0_17MODE_R {
        P0_17MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port 1 pin 18 control."]
    #[inline(always)]
    pub fn p0_18mode(&self) -> P0_18MODE_R {
        P0_18MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port 1 pin 19 control."]
    #[inline(always)]
    pub fn p0_19mode(&self) -> P0_19MODE_R {
        P0_19MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port 1 pin 20 control."]
    #[inline(always)]
    pub fn p0_20mode(&self) -> P0_20MODE_R {
        P0_20MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port 1 pin 21 control."]
    #[inline(always)]
    pub fn p0_21mode(&self) -> P0_21MODE_R {
        P0_21MODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port 1 pin 22 control."]
    #[inline(always)]
    pub fn p0_22mode(&self) -> P0_22MODE_R {
        P0_22MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port 1 pin 23 control."]
    #[inline(always)]
    pub fn p0_23mode(&self) -> P0_23MODE_R {
        P0_23MODE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port 1 pin 24 control."]
    #[inline(always)]
    pub fn p0_24mode(&self) -> P0_24MODE_R {
        P0_24MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port 1 pin 25 control."]
    #[inline(always)]
    pub fn p0_25mode(&self) -> P0_25MODE_R {
        P0_25MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port 1 pin 26 control."]
    #[inline(always)]
    pub fn p0_26mode(&self) -> P0_26MODE_R {
        P0_26MODE_R::new(((self.bits >> 20) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PINMODE1")
            .field("p0_16mode", &format_args!("{}", self.p0_16mode().bits()))
            .field("p0_17mode", &format_args!("{}", self.p0_17mode().bits()))
            .field("p0_18mode", &format_args!("{}", self.p0_18mode().bits()))
            .field("p0_19mode", &format_args!("{}", self.p0_19mode().bits()))
            .field("p0_20mode", &format_args!("{}", self.p0_20mode().bits()))
            .field("p0_21mode", &format_args!("{}", self.p0_21mode().bits()))
            .field("p0_22mode", &format_args!("{}", self.p0_22mode().bits()))
            .field("p0_23mode", &format_args!("{}", self.p0_23mode().bits()))
            .field("p0_24mode", &format_args!("{}", self.p0_24mode().bits()))
            .field("p0_25mode", &format_args!("{}", self.p0_25mode().bits()))
            .field("p0_26mode", &format_args!("{}", self.p0_26mode().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PINMODE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port 1 pin 16 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_16mode(&mut self) -> P0_16MODE_W<PINMODE1_SPEC, 0> {
        P0_16MODE_W::new(self)
    }
    #[doc = "Bits 2:3 - Port 1 pin 17 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_17mode(&mut self) -> P0_17MODE_W<PINMODE1_SPEC, 2> {
        P0_17MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - Port 1 pin 18 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_18mode(&mut self) -> P0_18MODE_W<PINMODE1_SPEC, 4> {
        P0_18MODE_W::new(self)
    }
    #[doc = "Bits 6:7 - Port 1 pin 19 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_19mode(&mut self) -> P0_19MODE_W<PINMODE1_SPEC, 6> {
        P0_19MODE_W::new(self)
    }
    #[doc = "Bits 8:9 - Port 1 pin 20 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_20mode(&mut self) -> P0_20MODE_W<PINMODE1_SPEC, 8> {
        P0_20MODE_W::new(self)
    }
    #[doc = "Bits 10:11 - Port 1 pin 21 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_21mode(&mut self) -> P0_21MODE_W<PINMODE1_SPEC, 10> {
        P0_21MODE_W::new(self)
    }
    #[doc = "Bits 12:13 - Port 1 pin 22 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_22mode(&mut self) -> P0_22MODE_W<PINMODE1_SPEC, 12> {
        P0_22MODE_W::new(self)
    }
    #[doc = "Bits 14:15 - Port 1 pin 23 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_23mode(&mut self) -> P0_23MODE_W<PINMODE1_SPEC, 14> {
        P0_23MODE_W::new(self)
    }
    #[doc = "Bits 16:17 - Port 1 pin 24 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_24mode(&mut self) -> P0_24MODE_W<PINMODE1_SPEC, 16> {
        P0_24MODE_W::new(self)
    }
    #[doc = "Bits 18:19 - Port 1 pin 25 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_25mode(&mut self) -> P0_25MODE_W<PINMODE1_SPEC, 18> {
        P0_25MODE_W::new(self)
    }
    #[doc = "Bits 20:21 - Port 1 pin 26 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_26mode(&mut self) -> P0_26MODE_W<PINMODE1_SPEC, 20> {
        P0_26MODE_W::new(self)
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
#[doc = "Pin mode select register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinmode1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinmode1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINMODE1_SPEC;
impl crate::RegisterSpec for PINMODE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode1::R`](R) reader structure"]
impl crate::Readable for PINMODE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pinmode1::W`](W) writer structure"]
impl crate::Writable for PINMODE1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINMODE1 to value 0"]
impl crate::Resettable for PINMODE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
