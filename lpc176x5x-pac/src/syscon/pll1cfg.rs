#[doc = "Register `PLL1CFG` reader"]
pub type R = crate::R<PLL1CFG_SPEC>;
#[doc = "Register `PLL1CFG` writer"]
pub type W = crate::W<PLL1CFG_SPEC>;
#[doc = "Field `MSEL1` reader - PLL1 Multiplier value. Supplies the value M in the PLL1 frequency calculations."]
pub type MSEL1_R = crate::FieldReader;
#[doc = "Field `MSEL1` writer - PLL1 Multiplier value. Supplies the value M in the PLL1 frequency calculations."]
pub type MSEL1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `PSEL1` reader - PLL1 Divider value. Supplies the value P in the PLL1 frequency calculations."]
pub type PSEL1_R = crate::FieldReader;
#[doc = "Field `PSEL1` writer - PLL1 Divider value. Supplies the value P in the PLL1 frequency calculations."]
pub type PSEL1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:4 - PLL1 Multiplier value. Supplies the value M in the PLL1 frequency calculations."]
    #[inline(always)]
    pub fn msel1(&self) -> MSEL1_R {
        MSEL1_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - PLL1 Divider value. Supplies the value P in the PLL1 frequency calculations."]
    #[inline(always)]
    pub fn psel1(&self) -> PSEL1_R {
        PSEL1_R::new(((self.bits >> 5) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL1CFG")
            .field("msel1", &format_args!("{}", self.msel1().bits()))
            .field("psel1", &format_args!("{}", self.psel1().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PLL1CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - PLL1 Multiplier value. Supplies the value M in the PLL1 frequency calculations."]
    #[inline(always)]
    #[must_use]
    pub fn msel1(&mut self) -> MSEL1_W<PLL1CFG_SPEC, 0> {
        MSEL1_W::new(self)
    }
    #[doc = "Bits 5:6 - PLL1 Divider value. Supplies the value P in the PLL1 frequency calculations."]
    #[inline(always)]
    #[must_use]
    pub fn psel1(&mut self) -> PSEL1_W<PLL1CFG_SPEC, 5> {
        PSEL1_W::new(self)
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
#[doc = "PLL1 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll1cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll1cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL1CFG_SPEC;
impl crate::RegisterSpec for PLL1CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll1cfg::R`](R) reader structure"]
impl crate::Readable for PLL1CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pll1cfg::W`](W) writer structure"]
impl crate::Writable for PLL1CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL1CFG to value 0"]
impl crate::Resettable for PLL1CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
