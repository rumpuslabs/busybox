#[doc = "Register `PINMODE_OD2` reader"]
pub type R = crate::R<PINMODE_OD2_SPEC>;
#[doc = "Register `PINMODE_OD2` writer"]
pub type W = crate::W<PINMODE_OD2_SPEC>;
#[doc = "Field `P2_00OD` reader - Port 2 pin 0 open drain mode control."]
pub use super::pinmode_od0::P0_00OD_R as P2_00OD_R;
#[doc = "Field `P2_01OD` reader - Port 2 pin 1 open drain mode control, see P2.00OD"]
pub use super::pinmode_od0::P0_00OD_R as P2_01OD_R;
#[doc = "Field `P2_02OD` reader - Port 2 pin 2 open drain mode control, see P2.00OD"]
pub use super::pinmode_od0::P0_00OD_R as P2_02OD_R;
#[doc = "Field `P2_03OD` reader - Port 2 pin 3 open drain mode control, see P2.00OD"]
pub use super::pinmode_od0::P0_00OD_R as P2_03OD_R;
#[doc = "Field `P2_04OD` reader - Port 2 pin 4 open drain mode control, see P2.00OD"]
pub use super::pinmode_od0::P0_00OD_R as P2_04OD_R;
#[doc = "Field `P2_05OD` reader - Port 2 pin 5 open drain mode control, see P2.00OD"]
pub use super::pinmode_od0::P0_00OD_R as P2_05OD_R;
#[doc = "Field `P2_06OD` reader - Port 2 pin 6 open drain mode control, see P2.00OD"]
pub use super::pinmode_od0::P0_00OD_R as P2_06OD_R;
#[doc = "Field `P2_07OD` reader - Port 2 pin 7 open drain mode control, see P2.00OD"]
pub use super::pinmode_od0::P0_00OD_R as P2_07OD_R;
#[doc = "Field `P2_08OD` reader - Port 2 pin 8 open drain mode control, see P2.00OD"]
pub use super::pinmode_od0::P0_00OD_R as P2_08OD_R;
#[doc = "Field `P2_09OD` reader - Port 2 pin 9 open drain mode control, see P2.00OD"]
pub use super::pinmode_od0::P0_00OD_R as P2_09OD_R;
#[doc = "Field `P2_10OD` reader - Port 2 pin 10 open drain mode control, see P2.00OD"]
pub use super::pinmode_od0::P0_00OD_R as P2_10OD_R;
#[doc = "Field `P2_11OD` reader - Port 2 pin 11 open drain mode control, see P2.00OD"]
pub use super::pinmode_od0::P0_00OD_R as P2_11OD_R;
#[doc = "Field `P2_12OD` reader - Port 2 pin 12 open drain mode control, see P2.00OD"]
pub use super::pinmode_od0::P0_00OD_R as P2_12OD_R;
#[doc = "Field `P2_13OD` reader - Port 2 pin 13 open drain mode control, see P2.00OD"]
pub use super::pinmode_od0::P0_00OD_R as P2_13OD_R;
#[doc = "Field `P2_00OD` writer - Port 2 pin 0 open drain mode control."]
pub use super::pinmode_od0::P0_00OD_W as P2_00OD_W;
#[doc = "Field `P2_01OD` writer - Port 2 pin 1 open drain mode control, see P2.00OD"]
pub use super::pinmode_od0::P0_00OD_W as P2_01OD_W;
#[doc = "Field `P2_02OD` writer - Port 2 pin 2 open drain mode control, see P2.00OD"]
pub use super::pinmode_od0::P0_00OD_W as P2_02OD_W;
#[doc = "Field `P2_03OD` writer - Port 2 pin 3 open drain mode control, see P2.00OD"]
pub use super::pinmode_od0::P0_00OD_W as P2_03OD_W;
#[doc = "Field `P2_04OD` writer - Port 2 pin 4 open drain mode control, see P2.00OD"]
pub use super::pinmode_od0::P0_00OD_W as P2_04OD_W;
#[doc = "Field `P2_05OD` writer - Port 2 pin 5 open drain mode control, see P2.00OD"]
pub use super::pinmode_od0::P0_00OD_W as P2_05OD_W;
#[doc = "Field `P2_06OD` writer - Port 2 pin 6 open drain mode control, see P2.00OD"]
pub use super::pinmode_od0::P0_00OD_W as P2_06OD_W;
#[doc = "Field `P2_07OD` writer - Port 2 pin 7 open drain mode control, see P2.00OD"]
pub use super::pinmode_od0::P0_00OD_W as P2_07OD_W;
#[doc = "Field `P2_08OD` writer - Port 2 pin 8 open drain mode control, see P2.00OD"]
pub use super::pinmode_od0::P0_00OD_W as P2_08OD_W;
#[doc = "Field `P2_09OD` writer - Port 2 pin 9 open drain mode control, see P2.00OD"]
pub use super::pinmode_od0::P0_00OD_W as P2_09OD_W;
#[doc = "Field `P2_10OD` writer - Port 2 pin 10 open drain mode control, see P2.00OD"]
pub use super::pinmode_od0::P0_00OD_W as P2_10OD_W;
#[doc = "Field `P2_11OD` writer - Port 2 pin 11 open drain mode control, see P2.00OD"]
pub use super::pinmode_od0::P0_00OD_W as P2_11OD_W;
#[doc = "Field `P2_12OD` writer - Port 2 pin 12 open drain mode control, see P2.00OD"]
pub use super::pinmode_od0::P0_00OD_W as P2_12OD_W;
#[doc = "Field `P2_13OD` writer - Port 2 pin 13 open drain mode control, see P2.00OD"]
pub use super::pinmode_od0::P0_00OD_W as P2_13OD_W;
#[doc = "Port 2 pin 0 open drain mode control."]
pub use super::pinmode_od0::PINMODE_OD_A;
impl R {
    #[doc = "Bit 0 - Port 2 pin 0 open drain mode control."]
    #[inline(always)]
    pub fn p2_00od(&self) -> P2_00OD_R {
        P2_00OD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port 2 pin 1 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_01od(&self) -> P2_01OD_R {
        P2_01OD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port 2 pin 2 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_02od(&self) -> P2_02OD_R {
        P2_02OD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port 2 pin 3 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_03od(&self) -> P2_03OD_R {
        P2_03OD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port 2 pin 4 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_04od(&self) -> P2_04OD_R {
        P2_04OD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port 2 pin 5 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_05od(&self) -> P2_05OD_R {
        P2_05OD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port 2 pin 6 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_06od(&self) -> P2_06OD_R {
        P2_06OD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port 2 pin 7 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_07od(&self) -> P2_07OD_R {
        P2_07OD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port 2 pin 8 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_08od(&self) -> P2_08OD_R {
        P2_08OD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port 2 pin 9 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_09od(&self) -> P2_09OD_R {
        P2_09OD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port 2 pin 10 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_10od(&self) -> P2_10OD_R {
        P2_10OD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port 2 pin 11 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_11od(&self) -> P2_11OD_R {
        P2_11OD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port 2 pin 12 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_12od(&self) -> P2_12OD_R {
        P2_12OD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port 2 pin 13 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_13od(&self) -> P2_13OD_R {
        P2_13OD_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PINMODE_OD2")
            .field("p2_00od", &format_args!("{}", self.p2_00od().bit()))
            .field("p2_01od", &format_args!("{}", self.p2_01od().bit()))
            .field("p2_02od", &format_args!("{}", self.p2_02od().bit()))
            .field("p2_03od", &format_args!("{}", self.p2_03od().bit()))
            .field("p2_04od", &format_args!("{}", self.p2_04od().bit()))
            .field("p2_05od", &format_args!("{}", self.p2_05od().bit()))
            .field("p2_06od", &format_args!("{}", self.p2_06od().bit()))
            .field("p2_07od", &format_args!("{}", self.p2_07od().bit()))
            .field("p2_08od", &format_args!("{}", self.p2_08od().bit()))
            .field("p2_09od", &format_args!("{}", self.p2_09od().bit()))
            .field("p2_10od", &format_args!("{}", self.p2_10od().bit()))
            .field("p2_11od", &format_args!("{}", self.p2_11od().bit()))
            .field("p2_12od", &format_args!("{}", self.p2_12od().bit()))
            .field("p2_13od", &format_args!("{}", self.p2_13od().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PINMODE_OD2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Port 2 pin 0 open drain mode control."]
    #[inline(always)]
    #[must_use]
    pub fn p2_00od(&mut self) -> P2_00OD_W<PINMODE_OD2_SPEC, 0> {
        P2_00OD_W::new(self)
    }
    #[doc = "Bit 1 - Port 2 pin 1 open drain mode control, see P2.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p2_01od(&mut self) -> P2_01OD_W<PINMODE_OD2_SPEC, 1> {
        P2_01OD_W::new(self)
    }
    #[doc = "Bit 2 - Port 2 pin 2 open drain mode control, see P2.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p2_02od(&mut self) -> P2_02OD_W<PINMODE_OD2_SPEC, 2> {
        P2_02OD_W::new(self)
    }
    #[doc = "Bit 3 - Port 2 pin 3 open drain mode control, see P2.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p2_03od(&mut self) -> P2_03OD_W<PINMODE_OD2_SPEC, 3> {
        P2_03OD_W::new(self)
    }
    #[doc = "Bit 4 - Port 2 pin 4 open drain mode control, see P2.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p2_04od(&mut self) -> P2_04OD_W<PINMODE_OD2_SPEC, 4> {
        P2_04OD_W::new(self)
    }
    #[doc = "Bit 5 - Port 2 pin 5 open drain mode control, see P2.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p2_05od(&mut self) -> P2_05OD_W<PINMODE_OD2_SPEC, 5> {
        P2_05OD_W::new(self)
    }
    #[doc = "Bit 6 - Port 2 pin 6 open drain mode control, see P2.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p2_06od(&mut self) -> P2_06OD_W<PINMODE_OD2_SPEC, 6> {
        P2_06OD_W::new(self)
    }
    #[doc = "Bit 7 - Port 2 pin 7 open drain mode control, see P2.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p2_07od(&mut self) -> P2_07OD_W<PINMODE_OD2_SPEC, 7> {
        P2_07OD_W::new(self)
    }
    #[doc = "Bit 8 - Port 2 pin 8 open drain mode control, see P2.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p2_08od(&mut self) -> P2_08OD_W<PINMODE_OD2_SPEC, 8> {
        P2_08OD_W::new(self)
    }
    #[doc = "Bit 9 - Port 2 pin 9 open drain mode control, see P2.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p2_09od(&mut self) -> P2_09OD_W<PINMODE_OD2_SPEC, 9> {
        P2_09OD_W::new(self)
    }
    #[doc = "Bit 10 - Port 2 pin 10 open drain mode control, see P2.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p2_10od(&mut self) -> P2_10OD_W<PINMODE_OD2_SPEC, 10> {
        P2_10OD_W::new(self)
    }
    #[doc = "Bit 11 - Port 2 pin 11 open drain mode control, see P2.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p2_11od(&mut self) -> P2_11OD_W<PINMODE_OD2_SPEC, 11> {
        P2_11OD_W::new(self)
    }
    #[doc = "Bit 12 - Port 2 pin 12 open drain mode control, see P2.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p2_12od(&mut self) -> P2_12OD_W<PINMODE_OD2_SPEC, 12> {
        P2_12OD_W::new(self)
    }
    #[doc = "Bit 13 - Port 2 pin 13 open drain mode control, see P2.00OD"]
    #[inline(always)]
    #[must_use]
    pub fn p2_13od(&mut self) -> P2_13OD_W<PINMODE_OD2_SPEC, 13> {
        P2_13OD_W::new(self)
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
#[doc = "Open drain mode control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinmode_od2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinmode_od2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINMODE_OD2_SPEC;
impl crate::RegisterSpec for PINMODE_OD2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode_od2::R`](R) reader structure"]
impl crate::Readable for PINMODE_OD2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pinmode_od2::W`](W) writer structure"]
impl crate::Writable for PINMODE_OD2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINMODE_OD2 to value 0"]
impl crate::Resettable for PINMODE_OD2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
