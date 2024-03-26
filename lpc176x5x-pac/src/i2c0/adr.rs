#[doc = "Register `ADR%s` reader"]
pub type R = crate::R<ADR_SPEC>;
#[doc = "Register `ADR%s` writer"]
pub type W = crate::W<ADR_SPEC>;
#[doc = "Field `GC` reader - General Call enable bit."]
pub type GC_R = crate::BitReader;
#[doc = "Field `GC` writer - General Call enable bit."]
pub type GC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `Address` reader - The I2C device address for slave mode."]
pub type ADDRESS_R = crate::FieldReader;
#[doc = "Field `Address` writer - The I2C device address for slave mode."]
pub type ADDRESS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bit 0 - General Call enable bit."]
    #[inline(always)]
    pub fn gc(&self) -> GC_R {
        GC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - The I2C device address for slave mode."]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADR")
            .field("gc", &format_args!("{}", self.gc().bit()))
            .field("address", &format_args!("{}", self.address().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<ADR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - General Call enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn gc(&mut self) -> GC_W<ADR_SPEC, 0> {
        GC_W::new(self)
    }
    #[doc = "Bits 1:7 - The I2C device address for slave mode."]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<ADR_SPEC, 1> {
        ADDRESS_W::new(self)
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
#[doc = "I2C Slave Address Register. Contains the 7-bit slave address for operation of the I2C interface in slave mode, and is not used in master mode. The least significant bit determines whether a slave responds to the General Call address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADR_SPEC;
impl crate::RegisterSpec for ADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adr::R`](R) reader structure"]
impl crate::Readable for ADR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adr::W`](W) writer structure"]
impl crate::Writable for ADR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADR%s to value 0"]
impl crate::Resettable for ADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
