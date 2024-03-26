#[doc = "Register `PLL1CON` reader"]
pub type R = crate::R<PLL1CON_SPEC>;
#[doc = "Register `PLL1CON` writer"]
pub type W = crate::W<PLL1CON_SPEC>;
#[doc = "Field `PLLE1` reader - PLL1 Enable. When one, and after a valid PLL1 feed, this bit will activate PLL1 and allow it to lock to the requested frequency."]
pub type PLLE1_R = crate::BitReader;
#[doc = "Field `PLLE1` writer - PLL1 Enable. When one, and after a valid PLL1 feed, this bit will activate PLL1 and allow it to lock to the requested frequency."]
pub type PLLE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLLC1` reader - PLL1 Connect. Setting PLLC to one after PLL1 has been enabled and locked, then followed by a valid PLL1 feed sequence causes PLL1 to become the clock source for the USB subsystem via the USB clock divider. See PLL1STAT register."]
pub type PLLC1_R = crate::BitReader;
#[doc = "Field `PLLC1` writer - PLL1 Connect. Setting PLLC to one after PLL1 has been enabled and locked, then followed by a valid PLL1 feed sequence causes PLL1 to become the clock source for the USB subsystem via the USB clock divider. See PLL1STAT register."]
pub type PLLC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - PLL1 Enable. When one, and after a valid PLL1 feed, this bit will activate PLL1 and allow it to lock to the requested frequency."]
    #[inline(always)]
    pub fn plle1(&self) -> PLLE1_R {
        PLLE1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL1 Connect. Setting PLLC to one after PLL1 has been enabled and locked, then followed by a valid PLL1 feed sequence causes PLL1 to become the clock source for the USB subsystem via the USB clock divider. See PLL1STAT register."]
    #[inline(always)]
    pub fn pllc1(&self) -> PLLC1_R {
        PLLC1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL1CON")
            .field("plle1", &format_args!("{}", self.plle1().bit()))
            .field("pllc1", &format_args!("{}", self.pllc1().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PLL1CON_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - PLL1 Enable. When one, and after a valid PLL1 feed, this bit will activate PLL1 and allow it to lock to the requested frequency."]
    #[inline(always)]
    #[must_use]
    pub fn plle1(&mut self) -> PLLE1_W<PLL1CON_SPEC, 0> {
        PLLE1_W::new(self)
    }
    #[doc = "Bit 1 - PLL1 Connect. Setting PLLC to one after PLL1 has been enabled and locked, then followed by a valid PLL1 feed sequence causes PLL1 to become the clock source for the USB subsystem via the USB clock divider. See PLL1STAT register."]
    #[inline(always)]
    #[must_use]
    pub fn pllc1(&mut self) -> PLLC1_W<PLL1CON_SPEC, 1> {
        PLLC1_W::new(self)
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
#[doc = "PLL1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll1con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll1con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL1CON_SPEC;
impl crate::RegisterSpec for PLL1CON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll1con::R`](R) reader structure"]
impl crate::Readable for PLL1CON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pll1con::W`](W) writer structure"]
impl crate::Writable for PLL1CON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL1CON to value 0"]
impl crate::Resettable for PLL1CON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
