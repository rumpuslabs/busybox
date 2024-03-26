#[doc = "Register `PINMODE_OD3` reader"]
pub type R = crate::R<PINMODE_OD3_SPEC>;
#[doc = "Register `PINMODE_OD3` writer"]
pub type W = crate::W<PINMODE_OD3_SPEC>;
#[doc = "Field `P3_25OD` reader - Port 3 pin 25 open drain mode control."]
pub use super::pinmode_od0::P0_00OD_R as P3_25OD_R;
#[doc = "Field `P3_25OD` writer - Port 3 pin 25 open drain mode control."]
pub use super::pinmode_od0::P0_00OD_W as P3_25OD_W;
#[doc = "Port 3 pin 25 open drain mode control."]
pub use super::pinmode_od0::PINMODE_OD_A;
#[doc = "Field `P3_26OD` reader - Port 3 pin 26 open drain mode control, see P3.25OD"]
pub type P3_26OD_R = crate::BitReader;
#[doc = "Field `P3_26OD` writer - Port 3 pin 26 open drain mode control, see P3.25OD"]
pub type P3_26OD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 25 - Port 3 pin 25 open drain mode control."]
    #[inline(always)]
    pub fn p3_25od(&self) -> P3_25OD_R {
        P3_25OD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Port 3 pin 26 open drain mode control, see P3.25OD"]
    #[inline(always)]
    pub fn p3_26od(&self) -> P3_26OD_R {
        P3_26OD_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PINMODE_OD3")
            .field("p3_25od", &format_args!("{}", self.p3_25od().bit()))
            .field("p3_26od", &format_args!("{}", self.p3_26od().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PINMODE_OD3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 25 - Port 3 pin 25 open drain mode control."]
    #[inline(always)]
    #[must_use]
    pub fn p3_25od(&mut self) -> P3_25OD_W<PINMODE_OD3_SPEC, 25> {
        P3_25OD_W::new(self)
    }
    #[doc = "Bit 26 - Port 3 pin 26 open drain mode control, see P3.25OD"]
    #[inline(always)]
    #[must_use]
    pub fn p3_26od(&mut self) -> P3_26OD_W<PINMODE_OD3_SPEC, 26> {
        P3_26OD_W::new(self)
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
#[doc = "Open drain mode control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinmode_od3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinmode_od3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINMODE_OD3_SPEC;
impl crate::RegisterSpec for PINMODE_OD3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode_od3::R`](R) reader structure"]
impl crate::Readable for PINMODE_OD3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pinmode_od3::W`](W) writer structure"]
impl crate::Writable for PINMODE_OD3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINMODE_OD3 to value 0"]
impl crate::Resettable for PINMODE_OD3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
