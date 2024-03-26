#[doc = "Register `USBCLKCTRL` reader"]
pub type R = crate::R<USBCLKCTRL_SPEC>;
#[doc = "Register `USBCLKCTRL` writer"]
pub type W = crate::W<USBCLKCTRL_SPEC>;
#[doc = "Field `DEV_CLK_EN` reader - Device clock enable. Enables the usbclk input to the device controller"]
pub type DEV_CLK_EN_R = crate::BitReader;
#[doc = "Field `DEV_CLK_EN` writer - Device clock enable. Enables the usbclk input to the device controller"]
pub type DEV_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PORTSEL_CLK_EN` reader - Port select register clock enable."]
pub type PORTSEL_CLK_EN_R = crate::BitReader;
#[doc = "Field `PORTSEL_CLK_EN` writer - Port select register clock enable."]
pub type PORTSEL_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AHB_CLK_EN` reader - AHB clock enable"]
pub type AHB_CLK_EN_R = crate::BitReader;
#[doc = "Field `AHB_CLK_EN` writer - AHB clock enable"]
pub type AHB_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Device clock enable. Enables the usbclk input to the device controller"]
    #[inline(always)]
    pub fn dev_clk_en(&self) -> DEV_CLK_EN_R {
        DEV_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Port select register clock enable."]
    #[inline(always)]
    pub fn portsel_clk_en(&self) -> PORTSEL_CLK_EN_R {
        PORTSEL_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB clock enable"]
    #[inline(always)]
    pub fn ahb_clk_en(&self) -> AHB_CLK_EN_R {
        AHB_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBCLKCTRL")
            .field("dev_clk_en", &format_args!("{}", self.dev_clk_en().bit()))
            .field(
                "portsel_clk_en",
                &format_args!("{}", self.portsel_clk_en().bit()),
            )
            .field("ahb_clk_en", &format_args!("{}", self.ahb_clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<USBCLKCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1 - Device clock enable. Enables the usbclk input to the device controller"]
    #[inline(always)]
    #[must_use]
    pub fn dev_clk_en(&mut self) -> DEV_CLK_EN_W<USBCLKCTRL_SPEC, 1> {
        DEV_CLK_EN_W::new(self)
    }
    #[doc = "Bit 3 - Port select register clock enable."]
    #[inline(always)]
    #[must_use]
    pub fn portsel_clk_en(&mut self) -> PORTSEL_CLK_EN_W<USBCLKCTRL_SPEC, 3> {
        PORTSEL_CLK_EN_W::new(self)
    }
    #[doc = "Bit 4 - AHB clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_clk_en(&mut self) -> AHB_CLK_EN_W<USBCLKCTRL_SPEC, 4> {
        AHB_CLK_EN_W::new(self)
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
#[doc = "USB Clock Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbclkctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbclkctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBCLKCTRL_SPEC;
impl crate::RegisterSpec for USBCLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbclkctrl::R`](R) reader structure"]
impl crate::Readable for USBCLKCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbclkctrl::W`](W) writer structure"]
impl crate::Writable for USBCLKCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBCLKCTRL to value 0"]
impl crate::Resettable for USBCLKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
