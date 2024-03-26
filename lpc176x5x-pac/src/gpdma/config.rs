#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<CONFIG_SPEC>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<CONFIG_SPEC>;
#[doc = "Field `E` reader - DMA Controller enable: 0 = disabled (default). Disabling the DMA Controller reduces power consumption. 1 = enabled."]
pub type E_R = crate::BitReader;
#[doc = "Field `E` writer - DMA Controller enable: 0 = disabled (default). Disabling the DMA Controller reduces power consumption. 1 = enabled."]
pub type E_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `M` reader - AHB Master endianness configuration: 0 = little-endian mode (default). 1 = big-endian mode."]
pub type M_R = crate::BitReader;
#[doc = "Field `M` writer - AHB Master endianness configuration: 0 = little-endian mode (default). 1 = big-endian mode."]
pub type M_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - DMA Controller enable: 0 = disabled (default). Disabling the DMA Controller reduces power consumption. 1 = enabled."]
    #[inline(always)]
    pub fn e(&self) -> E_R {
        E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AHB Master endianness configuration: 0 = little-endian mode (default). 1 = big-endian mode."]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG")
            .field("e", &format_args!("{}", self.e().bit()))
            .field("m", &format_args!("{}", self.m().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CONFIG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Controller enable: 0 = disabled (default). Disabling the DMA Controller reduces power consumption. 1 = enabled."]
    #[inline(always)]
    #[must_use]
    pub fn e(&mut self) -> E_W<CONFIG_SPEC, 0> {
        E_W::new(self)
    }
    #[doc = "Bit 1 - AHB Master endianness configuration: 0 = little-endian mode (default). 1 = big-endian mode."]
    #[inline(always)]
    #[must_use]
    pub fn m(&mut self) -> M_W<CONFIG_SPEC, 1> {
        M_W::new(self)
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
#[doc = "DMA Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
