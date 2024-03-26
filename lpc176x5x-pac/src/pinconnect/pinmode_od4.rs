#[doc = "Register `PINMODE_OD4` reader"]
pub type R = crate::R<PINMODE_OD4_SPEC>;
#[doc = "Register `PINMODE_OD4` writer"]
pub type W = crate::W<PINMODE_OD4_SPEC>;
#[doc = "Field `P4_28OD` reader - Port 4 pin 28 open drain mode control."]
pub use super::pinmode_od0::P0_00OD_R as P4_28OD_R;
#[doc = "Field `P4_28OD` writer - Port 4 pin 28 open drain mode control."]
pub use super::pinmode_od0::P0_00OD_W as P4_28OD_W;
#[doc = "Port 4 pin 28 open drain mode control."]
pub use super::pinmode_od0::PINMODE_OD_A;
#[doc = "Field `P4_29OD` reader - Port 4 pin 29 open drain mode control, see P4.28OD"]
pub type P4_29OD_R = crate::BitReader;
#[doc = "Field `P4_29OD` writer - Port 4 pin 29 open drain mode control, see P4.28OD"]
pub type P4_29OD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 28 - Port 4 pin 28 open drain mode control."]
    #[inline(always)]
    pub fn p4_28od(&self) -> P4_28OD_R {
        P4_28OD_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Port 4 pin 29 open drain mode control, see P4.28OD"]
    #[inline(always)]
    pub fn p4_29od(&self) -> P4_29OD_R {
        P4_29OD_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PINMODE_OD4")
            .field("p4_28od", &format_args!("{}", self.p4_28od().bit()))
            .field("p4_29od", &format_args!("{}", self.p4_29od().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PINMODE_OD4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 28 - Port 4 pin 28 open drain mode control."]
    #[inline(always)]
    #[must_use]
    pub fn p4_28od(&mut self) -> P4_28OD_W<PINMODE_OD4_SPEC, 28> {
        P4_28OD_W::new(self)
    }
    #[doc = "Bit 29 - Port 4 pin 29 open drain mode control, see P4.28OD"]
    #[inline(always)]
    #[must_use]
    pub fn p4_29od(&mut self) -> P4_29OD_W<PINMODE_OD4_SPEC, 29> {
        P4_29OD_W::new(self)
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
#[doc = "Open drain mode control register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinmode_od4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinmode_od4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINMODE_OD4_SPEC;
impl crate::RegisterSpec for PINMODE_OD4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode_od4::R`](R) reader structure"]
impl crate::Readable for PINMODE_OD4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pinmode_od4::W`](W) writer structure"]
impl crate::Writable for PINMODE_OD4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINMODE_OD4 to value 0"]
impl crate::Resettable for PINMODE_OD4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
