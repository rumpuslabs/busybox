#[doc = "Register `PLL0CFG` reader"]
pub type R = crate::R<PLL0CFG_SPEC>;
#[doc = "Register `PLL0CFG` writer"]
pub type W = crate::W<PLL0CFG_SPEC>;
#[doc = "Field `MSEL0` reader - PLL0 Multiplier value. Supplies the value M in PLL0 frequency calculations. The value stored here is M - 1. Note: Not all values of M are needed, and therefore some are not supported by hardware."]
pub type MSEL0_R = crate::FieldReader<u16>;
#[doc = "Field `MSEL0` writer - PLL0 Multiplier value. Supplies the value M in PLL0 frequency calculations. The value stored here is M - 1. Note: Not all values of M are needed, and therefore some are not supported by hardware."]
pub type MSEL0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 15, O, u16>;
#[doc = "Field `NSEL0` reader - PLL0 Pre-Divider value. Supplies the value N in PLL0 frequency calculations. The value stored here is N - 1. Supported values for N are 1 through 32."]
pub type NSEL0_R = crate::FieldReader;
#[doc = "Field `NSEL0` writer - PLL0 Pre-Divider value. Supplies the value N in PLL0 frequency calculations. The value stored here is N - 1. Supported values for N are 1 through 32."]
pub type NSEL0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:14 - PLL0 Multiplier value. Supplies the value M in PLL0 frequency calculations. The value stored here is M - 1. Note: Not all values of M are needed, and therefore some are not supported by hardware."]
    #[inline(always)]
    pub fn msel0(&self) -> MSEL0_R {
        MSEL0_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:23 - PLL0 Pre-Divider value. Supplies the value N in PLL0 frequency calculations. The value stored here is N - 1. Supported values for N are 1 through 32."]
    #[inline(always)]
    pub fn nsel0(&self) -> NSEL0_R {
        NSEL0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL0CFG")
            .field("msel0", &format_args!("{}", self.msel0().bits()))
            .field("nsel0", &format_args!("{}", self.nsel0().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PLL0CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:14 - PLL0 Multiplier value. Supplies the value M in PLL0 frequency calculations. The value stored here is M - 1. Note: Not all values of M are needed, and therefore some are not supported by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn msel0(&mut self) -> MSEL0_W<PLL0CFG_SPEC, 0> {
        MSEL0_W::new(self)
    }
    #[doc = "Bits 16:23 - PLL0 Pre-Divider value. Supplies the value N in PLL0 frequency calculations. The value stored here is N - 1. Supported values for N are 1 through 32."]
    #[inline(always)]
    #[must_use]
    pub fn nsel0(&mut self) -> NSEL0_W<PLL0CFG_SPEC, 16> {
        NSEL0_W::new(self)
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
#[doc = "PLL0 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll0cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll0cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL0CFG_SPEC;
impl crate::RegisterSpec for PLL0CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll0cfg::R`](R) reader structure"]
impl crate::Readable for PLL0CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pll0cfg::W`](W) writer structure"]
impl crate::Writable for PLL0CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL0CFG to value 0"]
impl crate::Resettable for PLL0CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
