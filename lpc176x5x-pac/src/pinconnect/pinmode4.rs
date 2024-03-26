#[doc = "Register `PINMODE4` reader"]
pub type R = crate::R<PINMODE4_SPEC>;
#[doc = "Register `PINMODE4` writer"]
pub type W = crate::W<PINMODE4_SPEC>;
#[doc = "Field `P2_00MODE` reader - Port 2 pin 0 control."]
pub use super::pinmode0::P0_00MODE_R as P2_00MODE_R;
#[doc = "Field `P2_01MODE` reader - Port 2 pin 1 control."]
pub use super::pinmode0::P0_00MODE_R as P2_01MODE_R;
#[doc = "Field `P2_02MODE` reader - Port 2 pin 2 control."]
pub use super::pinmode0::P0_00MODE_R as P2_02MODE_R;
#[doc = "Field `P2_03MODE` reader - Port 2 pin 3 control."]
pub use super::pinmode0::P0_00MODE_R as P2_03MODE_R;
#[doc = "Field `P2_04MODE` reader - Port 2 pin 4 control."]
pub use super::pinmode0::P0_00MODE_R as P2_04MODE_R;
#[doc = "Field `P2_05MODE` reader - Port 2 pin 5 control."]
pub use super::pinmode0::P0_00MODE_R as P2_05MODE_R;
#[doc = "Field `P2_06MODE` reader - Port 2 pin 6 control."]
pub use super::pinmode0::P0_00MODE_R as P2_06MODE_R;
#[doc = "Field `P2_07MODE` reader - Port 2 pin 7 control."]
pub use super::pinmode0::P0_00MODE_R as P2_07MODE_R;
#[doc = "Field `P2_08MODE` reader - Port 2 pin 8 control."]
pub use super::pinmode0::P0_00MODE_R as P2_08MODE_R;
#[doc = "Field `P2_09MODE` reader - Port 2 pin 9 control."]
pub use super::pinmode0::P0_00MODE_R as P2_09MODE_R;
#[doc = "Field `P2_10MODE` reader - Port 2 pin 10 control."]
pub use super::pinmode0::P0_00MODE_R as P2_10MODE_R;
#[doc = "Field `P2_11MODE` reader - Port 2 pin 11 control."]
pub use super::pinmode0::P0_00MODE_R as P2_11MODE_R;
#[doc = "Field `P2_12MODE` reader - Port 2 pin 12 control."]
pub use super::pinmode0::P0_00MODE_R as P2_12MODE_R;
#[doc = "Field `P2_13MODE` reader - Port 2 pin 13 control."]
pub use super::pinmode0::P0_00MODE_R as P2_13MODE_R;
#[doc = "Field `P2_00MODE` writer - Port 2 pin 0 control."]
pub use super::pinmode0::P0_00MODE_W as P2_00MODE_W;
#[doc = "Field `P2_01MODE` writer - Port 2 pin 1 control."]
pub use super::pinmode0::P0_00MODE_W as P2_01MODE_W;
#[doc = "Field `P2_02MODE` writer - Port 2 pin 2 control."]
pub use super::pinmode0::P0_00MODE_W as P2_02MODE_W;
#[doc = "Field `P2_03MODE` writer - Port 2 pin 3 control."]
pub use super::pinmode0::P0_00MODE_W as P2_03MODE_W;
#[doc = "Field `P2_04MODE` writer - Port 2 pin 4 control."]
pub use super::pinmode0::P0_00MODE_W as P2_04MODE_W;
#[doc = "Field `P2_05MODE` writer - Port 2 pin 5 control."]
pub use super::pinmode0::P0_00MODE_W as P2_05MODE_W;
#[doc = "Field `P2_06MODE` writer - Port 2 pin 6 control."]
pub use super::pinmode0::P0_00MODE_W as P2_06MODE_W;
#[doc = "Field `P2_07MODE` writer - Port 2 pin 7 control."]
pub use super::pinmode0::P0_00MODE_W as P2_07MODE_W;
#[doc = "Field `P2_08MODE` writer - Port 2 pin 8 control."]
pub use super::pinmode0::P0_00MODE_W as P2_08MODE_W;
#[doc = "Field `P2_09MODE` writer - Port 2 pin 9 control."]
pub use super::pinmode0::P0_00MODE_W as P2_09MODE_W;
#[doc = "Field `P2_10MODE` writer - Port 2 pin 10 control."]
pub use super::pinmode0::P0_00MODE_W as P2_10MODE_W;
#[doc = "Field `P2_11MODE` writer - Port 2 pin 11 control."]
pub use super::pinmode0::P0_00MODE_W as P2_11MODE_W;
#[doc = "Field `P2_12MODE` writer - Port 2 pin 12 control."]
pub use super::pinmode0::P0_00MODE_W as P2_12MODE_W;
#[doc = "Field `P2_13MODE` writer - Port 2 pin 13 control."]
pub use super::pinmode0::P0_00MODE_W as P2_13MODE_W;
#[doc = "Port 2 pin 0 control."]
pub use super::pinmode0::PINMODE_A;
impl R {
    #[doc = "Bits 0:1 - Port 2 pin 0 control."]
    #[inline(always)]
    pub fn p2_00mode(&self) -> P2_00MODE_R {
        P2_00MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port 2 pin 1 control."]
    #[inline(always)]
    pub fn p2_01mode(&self) -> P2_01MODE_R {
        P2_01MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port 2 pin 2 control."]
    #[inline(always)]
    pub fn p2_02mode(&self) -> P2_02MODE_R {
        P2_02MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port 2 pin 3 control."]
    #[inline(always)]
    pub fn p2_03mode(&self) -> P2_03MODE_R {
        P2_03MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port 2 pin 4 control."]
    #[inline(always)]
    pub fn p2_04mode(&self) -> P2_04MODE_R {
        P2_04MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port 2 pin 5 control."]
    #[inline(always)]
    pub fn p2_05mode(&self) -> P2_05MODE_R {
        P2_05MODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port 2 pin 6 control."]
    #[inline(always)]
    pub fn p2_06mode(&self) -> P2_06MODE_R {
        P2_06MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port 2 pin 7 control."]
    #[inline(always)]
    pub fn p2_07mode(&self) -> P2_07MODE_R {
        P2_07MODE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port 2 pin 8 control."]
    #[inline(always)]
    pub fn p2_08mode(&self) -> P2_08MODE_R {
        P2_08MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port 2 pin 9 control."]
    #[inline(always)]
    pub fn p2_09mode(&self) -> P2_09MODE_R {
        P2_09MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port 2 pin 10 control."]
    #[inline(always)]
    pub fn p2_10mode(&self) -> P2_10MODE_R {
        P2_10MODE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port 2 pin 11 control."]
    #[inline(always)]
    pub fn p2_11mode(&self) -> P2_11MODE_R {
        P2_11MODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port 2 pin 12 control."]
    #[inline(always)]
    pub fn p2_12mode(&self) -> P2_12MODE_R {
        P2_12MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port 2 pin 13 control."]
    #[inline(always)]
    pub fn p2_13mode(&self) -> P2_13MODE_R {
        P2_13MODE_R::new(((self.bits >> 26) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PINMODE4")
            .field("p2_00mode", &format_args!("{}", self.p2_00mode().bits()))
            .field("p2_01mode", &format_args!("{}", self.p2_01mode().bits()))
            .field("p2_02mode", &format_args!("{}", self.p2_02mode().bits()))
            .field("p2_03mode", &format_args!("{}", self.p2_03mode().bits()))
            .field("p2_04mode", &format_args!("{}", self.p2_04mode().bits()))
            .field("p2_05mode", &format_args!("{}", self.p2_05mode().bits()))
            .field("p2_06mode", &format_args!("{}", self.p2_06mode().bits()))
            .field("p2_07mode", &format_args!("{}", self.p2_07mode().bits()))
            .field("p2_08mode", &format_args!("{}", self.p2_08mode().bits()))
            .field("p2_09mode", &format_args!("{}", self.p2_09mode().bits()))
            .field("p2_10mode", &format_args!("{}", self.p2_10mode().bits()))
            .field("p2_11mode", &format_args!("{}", self.p2_11mode().bits()))
            .field("p2_12mode", &format_args!("{}", self.p2_12mode().bits()))
            .field("p2_13mode", &format_args!("{}", self.p2_13mode().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PINMODE4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port 2 pin 0 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_00mode(&mut self) -> P2_00MODE_W<PINMODE4_SPEC, 0> {
        P2_00MODE_W::new(self)
    }
    #[doc = "Bits 2:3 - Port 2 pin 1 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_01mode(&mut self) -> P2_01MODE_W<PINMODE4_SPEC, 2> {
        P2_01MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - Port 2 pin 2 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_02mode(&mut self) -> P2_02MODE_W<PINMODE4_SPEC, 4> {
        P2_02MODE_W::new(self)
    }
    #[doc = "Bits 6:7 - Port 2 pin 3 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_03mode(&mut self) -> P2_03MODE_W<PINMODE4_SPEC, 6> {
        P2_03MODE_W::new(self)
    }
    #[doc = "Bits 8:9 - Port 2 pin 4 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_04mode(&mut self) -> P2_04MODE_W<PINMODE4_SPEC, 8> {
        P2_04MODE_W::new(self)
    }
    #[doc = "Bits 10:11 - Port 2 pin 5 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_05mode(&mut self) -> P2_05MODE_W<PINMODE4_SPEC, 10> {
        P2_05MODE_W::new(self)
    }
    #[doc = "Bits 12:13 - Port 2 pin 6 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_06mode(&mut self) -> P2_06MODE_W<PINMODE4_SPEC, 12> {
        P2_06MODE_W::new(self)
    }
    #[doc = "Bits 14:15 - Port 2 pin 7 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_07mode(&mut self) -> P2_07MODE_W<PINMODE4_SPEC, 14> {
        P2_07MODE_W::new(self)
    }
    #[doc = "Bits 16:17 - Port 2 pin 8 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_08mode(&mut self) -> P2_08MODE_W<PINMODE4_SPEC, 16> {
        P2_08MODE_W::new(self)
    }
    #[doc = "Bits 18:19 - Port 2 pin 9 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_09mode(&mut self) -> P2_09MODE_W<PINMODE4_SPEC, 18> {
        P2_09MODE_W::new(self)
    }
    #[doc = "Bits 20:21 - Port 2 pin 10 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_10mode(&mut self) -> P2_10MODE_W<PINMODE4_SPEC, 20> {
        P2_10MODE_W::new(self)
    }
    #[doc = "Bits 22:23 - Port 2 pin 11 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_11mode(&mut self) -> P2_11MODE_W<PINMODE4_SPEC, 22> {
        P2_11MODE_W::new(self)
    }
    #[doc = "Bits 24:25 - Port 2 pin 12 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_12mode(&mut self) -> P2_12MODE_W<PINMODE4_SPEC, 24> {
        P2_12MODE_W::new(self)
    }
    #[doc = "Bits 26:27 - Port 2 pin 13 control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_13mode(&mut self) -> P2_13MODE_W<PINMODE4_SPEC, 26> {
        P2_13MODE_W::new(self)
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
#[doc = "Pin mode select register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinmode4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinmode4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINMODE4_SPEC;
impl crate::RegisterSpec for PINMODE4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode4::R`](R) reader structure"]
impl crate::Readable for PINMODE4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pinmode4::W`](W) writer structure"]
impl crate::Writable for PINMODE4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINMODE4 to value 0"]
impl crate::Resettable for PINMODE4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
