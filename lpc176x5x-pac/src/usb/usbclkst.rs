#[doc = "Register `USBCLKST` reader"]
pub type R = crate::R<USBCLKST_SPEC>;
#[doc = "Field `DEV_CLK_ON` reader - Device clock on. The usbclk input to the device controller is active ."]
pub type DEV_CLK_ON_R = crate::BitReader;
#[doc = "Field `PORTSEL_CLK_ON` reader - Port select register clock on."]
pub type PORTSEL_CLK_ON_R = crate::BitReader;
#[doc = "Field `AHB_CLK_ON` reader - AHB clock on."]
pub type AHB_CLK_ON_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Device clock on. The usbclk input to the device controller is active ."]
    #[inline(always)]
    pub fn dev_clk_on(&self) -> DEV_CLK_ON_R {
        DEV_CLK_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Port select register clock on."]
    #[inline(always)]
    pub fn portsel_clk_on(&self) -> PORTSEL_CLK_ON_R {
        PORTSEL_CLK_ON_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB clock on."]
    #[inline(always)]
    pub fn ahb_clk_on(&self) -> AHB_CLK_ON_R {
        AHB_CLK_ON_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBCLKST")
            .field("dev_clk_on", &format_args!("{}", self.dev_clk_on().bit()))
            .field(
                "portsel_clk_on",
                &format_args!("{}", self.portsel_clk_on().bit()),
            )
            .field("ahb_clk_on", &format_args!("{}", self.ahb_clk_on().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<USBCLKST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "USB Clock Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbclkst::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBCLKST_SPEC;
impl crate::RegisterSpec for USBCLKST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbclkst::R`](R) reader structure"]
impl crate::Readable for USBCLKST_SPEC {}
#[doc = "`reset()` method sets USBCLKST to value 0"]
impl crate::Resettable for USBCLKST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
