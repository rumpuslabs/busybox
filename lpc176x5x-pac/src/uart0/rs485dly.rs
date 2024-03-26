#[doc = "Register `RS485DLY` reader"]
pub type R = crate::R<RS485DLY_SPEC>;
#[doc = "Register `RS485DLY` writer"]
pub type W = crate::W<RS485DLY_SPEC>;
#[doc = "Field `DLY` reader - Contains the direction control (UnOE) delay value. This register works in conjunction with an 8-bit counter."]
pub type DLY_R = crate::FieldReader;
#[doc = "Field `DLY` writer - Contains the direction control (UnOE) delay value. This register works in conjunction with an 8-bit counter."]
pub type DLY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Contains the direction control (UnOE) delay value. This register works in conjunction with an 8-bit counter."]
    #[inline(always)]
    pub fn dly(&self) -> DLY_R {
        DLY_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RS485DLY")
            .field("dly", &format_args!("{}", self.dly().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RS485DLY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Contains the direction control (UnOE) delay value. This register works in conjunction with an 8-bit counter."]
    #[inline(always)]
    #[must_use]
    pub fn dly(&mut self) -> DLY_W<RS485DLY_SPEC, 0> {
        DLY_W::new(self)
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
#[doc = "RS-485/EIA-485 direction control delay.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rs485dly::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rs485dly::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RS485DLY_SPEC;
impl crate::RegisterSpec for RS485DLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rs485dly::R`](R) reader structure"]
impl crate::Readable for RS485DLY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rs485dly::W`](W) writer structure"]
impl crate::Writable for RS485DLY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RS485DLY to value 0"]
impl crate::Resettable for RS485DLY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
