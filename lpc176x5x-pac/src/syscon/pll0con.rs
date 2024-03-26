#[doc = "Register `PLL0CON` reader"]
pub type R = crate::R<PLL0CON_SPEC>;
#[doc = "Register `PLL0CON` writer"]
pub type W = crate::W<PLL0CON_SPEC>;
#[doc = "Field `PLLE0` reader - PLL0 Enable. When one, and after a valid PLL0 feed, this bit will activate PLL0 and allow it to lock to the requested frequency. See PLL0STAT register."]
pub type PLLE0_R = crate::BitReader;
#[doc = "Field `PLLE0` writer - PLL0 Enable. When one, and after a valid PLL0 feed, this bit will activate PLL0 and allow it to lock to the requested frequency. See PLL0STAT register."]
pub type PLLE0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLLC0` reader - PLL0 Connect. Setting PLLC0 to one after PLL0 has been enabled and locked, then followed by a valid PLL0 feed sequence causes PLL0 to become the clock source for the CPU, AHB peripherals, and used to derive the clocks for APB peripherals. The PLL0 output may potentially be used to clock the USB subsystem if the frequency is 48 MHz. See PLL0STAT register."]
pub type PLLC0_R = crate::BitReader;
#[doc = "Field `PLLC0` writer - PLL0 Connect. Setting PLLC0 to one after PLL0 has been enabled and locked, then followed by a valid PLL0 feed sequence causes PLL0 to become the clock source for the CPU, AHB peripherals, and used to derive the clocks for APB peripherals. The PLL0 output may potentially be used to clock the USB subsystem if the frequency is 48 MHz. See PLL0STAT register."]
pub type PLLC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - PLL0 Enable. When one, and after a valid PLL0 feed, this bit will activate PLL0 and allow it to lock to the requested frequency. See PLL0STAT register."]
    #[inline(always)]
    pub fn plle0(&self) -> PLLE0_R {
        PLLE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL0 Connect. Setting PLLC0 to one after PLL0 has been enabled and locked, then followed by a valid PLL0 feed sequence causes PLL0 to become the clock source for the CPU, AHB peripherals, and used to derive the clocks for APB peripherals. The PLL0 output may potentially be used to clock the USB subsystem if the frequency is 48 MHz. See PLL0STAT register."]
    #[inline(always)]
    pub fn pllc0(&self) -> PLLC0_R {
        PLLC0_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL0CON")
            .field("plle0", &format_args!("{}", self.plle0().bit()))
            .field("pllc0", &format_args!("{}", self.pllc0().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PLL0CON_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - PLL0 Enable. When one, and after a valid PLL0 feed, this bit will activate PLL0 and allow it to lock to the requested frequency. See PLL0STAT register."]
    #[inline(always)]
    #[must_use]
    pub fn plle0(&mut self) -> PLLE0_W<PLL0CON_SPEC, 0> {
        PLLE0_W::new(self)
    }
    #[doc = "Bit 1 - PLL0 Connect. Setting PLLC0 to one after PLL0 has been enabled and locked, then followed by a valid PLL0 feed sequence causes PLL0 to become the clock source for the CPU, AHB peripherals, and used to derive the clocks for APB peripherals. The PLL0 output may potentially be used to clock the USB subsystem if the frequency is 48 MHz. See PLL0STAT register."]
    #[inline(always)]
    #[must_use]
    pub fn pllc0(&mut self) -> PLLC0_W<PLL0CON_SPEC, 1> {
        PLLC0_W::new(self)
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
#[doc = "PLL0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll0con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll0con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL0CON_SPEC;
impl crate::RegisterSpec for PLL0CON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll0con::R`](R) reader structure"]
impl crate::Readable for PLL0CON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pll0con::W`](W) writer structure"]
impl crate::Writable for PLL0CON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL0CON to value 0"]
impl crate::Resettable for PLL0CON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
