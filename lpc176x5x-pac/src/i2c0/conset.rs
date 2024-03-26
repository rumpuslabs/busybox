#[doc = "Register `CONSET` reader"]
pub type R = crate::R<CONSET_SPEC>;
#[doc = "Register `CONSET` writer"]
pub type W = crate::W<CONSET_SPEC>;
#[doc = "Field `AA` reader - Assert acknowledge flag."]
pub type AA_R = crate::BitReader;
#[doc = "Field `AA` writer - Assert acknowledge flag."]
pub type AA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SI` reader - I2C interrupt flag."]
pub type SI_R = crate::BitReader;
#[doc = "Field `SI` writer - I2C interrupt flag."]
pub type SI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STO` reader - STOP flag."]
pub type STO_R = crate::BitReader;
#[doc = "Field `STO` writer - STOP flag."]
pub type STO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STA` reader - START flag."]
pub type STA_R = crate::BitReader;
#[doc = "Field `STA` writer - START flag."]
pub type STA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2EN` reader - I2C interface enable."]
pub type I2EN_R = crate::BitReader;
#[doc = "Field `I2EN` writer - I2C interface enable."]
pub type I2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 2 - Assert acknowledge flag."]
    #[inline(always)]
    pub fn aa(&self) -> AA_R {
        AA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C interrupt flag."]
    #[inline(always)]
    pub fn si(&self) -> SI_R {
        SI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - STOP flag."]
    #[inline(always)]
    pub fn sto(&self) -> STO_R {
        STO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - START flag."]
    #[inline(always)]
    pub fn sta(&self) -> STA_R {
        STA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C interface enable."]
    #[inline(always)]
    pub fn i2en(&self) -> I2EN_R {
        I2EN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONSET")
            .field("aa", &format_args!("{}", self.aa().bit()))
            .field("si", &format_args!("{}", self.si().bit()))
            .field("sto", &format_args!("{}", self.sto().bit()))
            .field("sta", &format_args!("{}", self.sta().bit()))
            .field("i2en", &format_args!("{}", self.i2en().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CONSET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 2 - Assert acknowledge flag."]
    #[inline(always)]
    #[must_use]
    pub fn aa(&mut self) -> AA_W<CONSET_SPEC, 2> {
        AA_W::new(self)
    }
    #[doc = "Bit 3 - I2C interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn si(&mut self) -> SI_W<CONSET_SPEC, 3> {
        SI_W::new(self)
    }
    #[doc = "Bit 4 - STOP flag."]
    #[inline(always)]
    #[must_use]
    pub fn sto(&mut self) -> STO_W<CONSET_SPEC, 4> {
        STO_W::new(self)
    }
    #[doc = "Bit 5 - START flag."]
    #[inline(always)]
    #[must_use]
    pub fn sta(&mut self) -> STA_W<CONSET_SPEC, 5> {
        STA_W::new(self)
    }
    #[doc = "Bit 6 - I2C interface enable."]
    #[inline(always)]
    #[must_use]
    pub fn i2en(&mut self) -> I2EN_W<CONSET_SPEC, 6> {
        I2EN_W::new(self)
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
#[doc = "I2C Control Set Register. When a one is written to a bit of this register, the corresponding bit in the I2C control register is set. Writing a zero has no effect on the corresponding bit in the I2C control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONSET_SPEC;
impl crate::RegisterSpec for CONSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conset::R`](R) reader structure"]
impl crate::Readable for CONSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conset::W`](W) writer structure"]
impl crate::Writable for CONSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONSET to value 0"]
impl crate::Resettable for CONSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
