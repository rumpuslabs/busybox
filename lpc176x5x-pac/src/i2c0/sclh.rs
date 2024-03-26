#[doc = "Register `SCLH` reader"]
pub type R = crate::R<SCLH_SPEC>;
#[doc = "Register `SCLH` writer"]
pub type W = crate::W<SCLH_SPEC>;
#[doc = "Field `SCLH` reader - Count for SCL HIGH time period selection."]
pub type SCLH_R = crate::FieldReader<u16>;
#[doc = "Field `SCLH` writer - Count for SCL HIGH time period selection."]
pub type SCLH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Count for SCL HIGH time period selection."]
    #[inline(always)]
    pub fn sclh(&self) -> SCLH_R {
        SCLH_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCLH")
            .field("sclh", &format_args!("{}", self.sclh().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<SCLH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Count for SCL HIGH time period selection."]
    #[inline(always)]
    #[must_use]
    pub fn sclh(&mut self) -> SCLH_W<SCLH_SPEC, 0> {
        SCLH_W::new(self)
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
#[doc = "SCH Duty Cycle Register High Half Word. Determines the high time of the I2C clock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sclh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sclh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCLH_SPEC;
impl crate::RegisterSpec for SCLH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sclh::R`](R) reader structure"]
impl crate::Readable for SCLH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sclh::W`](W) writer structure"]
impl crate::Writable for SCLH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCLH to value 0x04"]
impl crate::Resettable for SCLH_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
