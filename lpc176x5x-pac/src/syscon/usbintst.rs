#[doc = "Register `USBINTST` reader"]
pub type R = crate::R<USBINTST_SPEC>;
#[doc = "Register `USBINTST` writer"]
pub type W = crate::W<USBINTST_SPEC>;
#[doc = "Field `USB_INT_REQ_LP` reader - Low priority interrupt line status. This bit is read-only."]
pub type USB_INT_REQ_LP_R = crate::BitReader;
#[doc = "Field `USB_INT_REQ_LP` writer - Low priority interrupt line status. This bit is read-only."]
pub type USB_INT_REQ_LP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_INT_REQ_HP` reader - High priority interrupt line status. This bit is read-only."]
pub type USB_INT_REQ_HP_R = crate::BitReader;
#[doc = "Field `USB_INT_REQ_HP` writer - High priority interrupt line status. This bit is read-only."]
pub type USB_INT_REQ_HP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_INT_REQ_DMA` reader - DMA interrupt line status. This bit is read-only."]
pub type USB_INT_REQ_DMA_R = crate::BitReader;
#[doc = "Field `USB_INT_REQ_DMA` writer - DMA interrupt line status. This bit is read-only."]
pub type USB_INT_REQ_DMA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_HOST_INT` reader - USB host interrupt line status. This bit is read-only."]
pub type USB_HOST_INT_R = crate::BitReader;
#[doc = "Field `USB_HOST_INT` writer - USB host interrupt line status. This bit is read-only."]
pub type USB_HOST_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_ATX_INT` reader - External ATX interrupt line status. This bit is read-only."]
pub type USB_ATX_INT_R = crate::BitReader;
#[doc = "Field `USB_ATX_INT` writer - External ATX interrupt line status. This bit is read-only."]
pub type USB_ATX_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_OTG_INT` reader - OTG interrupt line status. This bit is read-only."]
pub type USB_OTG_INT_R = crate::BitReader;
#[doc = "Field `USB_OTG_INT` writer - OTG interrupt line status. This bit is read-only."]
pub type USB_OTG_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_I2C_INT` reader - I2C module interrupt line status. This bit is read-only."]
pub type USB_I2C_INT_R = crate::BitReader;
#[doc = "Field `USB_I2C_INT` writer - I2C module interrupt line status. This bit is read-only."]
pub type USB_I2C_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_NEED_CLK` reader - USB need clock indicator. This bit is read-only. This bit is set to 1 when USB activity or a change of state on the USB data pins is detected, and it indicates that a PLL supplied clock of 48 MHz is needed. Once USB_NEED_CLK becomes one, it resets to zero 5 ms after the last packet has been received/sent, or 2 ms after the Suspend Change (SUS_CH) interrupt has occurred. A change of this bit from 0 to 1 can wake up the microcontroller if activity on the USB bus is selected to wake up the part from the Power-down mode (see Section 4.7.9 Wake-up from Reduced Power Modes for details). Also see Section 4.5.8 PLLs and Power-down mode and Section 4.7.10 Power Control for Peripherals register (PCONP - 0x400F C0C4) for considerations about the PLL and invoking the Power-down mode. This bit is read-only."]
pub type USB_NEED_CLK_R = crate::BitReader;
#[doc = "Field `USB_NEED_CLK` writer - USB need clock indicator. This bit is read-only. This bit is set to 1 when USB activity or a change of state on the USB data pins is detected, and it indicates that a PLL supplied clock of 48 MHz is needed. Once USB_NEED_CLK becomes one, it resets to zero 5 ms after the last packet has been received/sent, or 2 ms after the Suspend Change (SUS_CH) interrupt has occurred. A change of this bit from 0 to 1 can wake up the microcontroller if activity on the USB bus is selected to wake up the part from the Power-down mode (see Section 4.7.9 Wake-up from Reduced Power Modes for details). Also see Section 4.5.8 PLLs and Power-down mode and Section 4.7.10 Power Control for Peripherals register (PCONP - 0x400F C0C4) for considerations about the PLL and invoking the Power-down mode. This bit is read-only."]
pub type USB_NEED_CLK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN_USB_INTS` reader - Enable all USB interrupts. When this bit is cleared, the NVIC does not see the ORed output of the USB interrupt lines."]
pub type EN_USB_INTS_R = crate::BitReader;
#[doc = "Field `EN_USB_INTS` writer - Enable all USB interrupts. When this bit is cleared, the NVIC does not see the ORed output of the USB interrupt lines."]
pub type EN_USB_INTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Low priority interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_lp(&self) -> USB_INT_REQ_LP_R {
        USB_INT_REQ_LP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High priority interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_hp(&self) -> USB_INT_REQ_HP_R {
        USB_INT_REQ_HP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_dma(&self) -> USB_INT_REQ_DMA_R {
        USB_INT_REQ_DMA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB host interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_host_int(&self) -> USB_HOST_INT_R {
        USB_HOST_INT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External ATX interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_atx_int(&self) -> USB_ATX_INT_R {
        USB_ATX_INT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OTG interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_otg_int(&self) -> USB_OTG_INT_R {
        USB_OTG_INT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C module interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_i2c_int(&self) -> USB_I2C_INT_R {
        USB_I2C_INT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - USB need clock indicator. This bit is read-only. This bit is set to 1 when USB activity or a change of state on the USB data pins is detected, and it indicates that a PLL supplied clock of 48 MHz is needed. Once USB_NEED_CLK becomes one, it resets to zero 5 ms after the last packet has been received/sent, or 2 ms after the Suspend Change (SUS_CH) interrupt has occurred. A change of this bit from 0 to 1 can wake up the microcontroller if activity on the USB bus is selected to wake up the part from the Power-down mode (see Section 4.7.9 Wake-up from Reduced Power Modes for details). Also see Section 4.5.8 PLLs and Power-down mode and Section 4.7.10 Power Control for Peripherals register (PCONP - 0x400F C0C4) for considerations about the PLL and invoking the Power-down mode. This bit is read-only."]
    #[inline(always)]
    pub fn usb_need_clk(&self) -> USB_NEED_CLK_R {
        USB_NEED_CLK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable all USB interrupts. When this bit is cleared, the NVIC does not see the ORed output of the USB interrupt lines."]
    #[inline(always)]
    pub fn en_usb_ints(&self) -> EN_USB_INTS_R {
        EN_USB_INTS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBINTST")
            .field(
                "usb_int_req_lp",
                &format_args!("{}", self.usb_int_req_lp().bit()),
            )
            .field(
                "usb_int_req_hp",
                &format_args!("{}", self.usb_int_req_hp().bit()),
            )
            .field(
                "usb_int_req_dma",
                &format_args!("{}", self.usb_int_req_dma().bit()),
            )
            .field(
                "usb_host_int",
                &format_args!("{}", self.usb_host_int().bit()),
            )
            .field("usb_atx_int", &format_args!("{}", self.usb_atx_int().bit()))
            .field("usb_otg_int", &format_args!("{}", self.usb_otg_int().bit()))
            .field("usb_i2c_int", &format_args!("{}", self.usb_i2c_int().bit()))
            .field(
                "usb_need_clk",
                &format_args!("{}", self.usb_need_clk().bit()),
            )
            .field("en_usb_ints", &format_args!("{}", self.en_usb_ints().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<USBINTST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Low priority interrupt line status. This bit is read-only."]
    #[inline(always)]
    #[must_use]
    pub fn usb_int_req_lp(&mut self) -> USB_INT_REQ_LP_W<USBINTST_SPEC, 0> {
        USB_INT_REQ_LP_W::new(self)
    }
    #[doc = "Bit 1 - High priority interrupt line status. This bit is read-only."]
    #[inline(always)]
    #[must_use]
    pub fn usb_int_req_hp(&mut self) -> USB_INT_REQ_HP_W<USBINTST_SPEC, 1> {
        USB_INT_REQ_HP_W::new(self)
    }
    #[doc = "Bit 2 - DMA interrupt line status. This bit is read-only."]
    #[inline(always)]
    #[must_use]
    pub fn usb_int_req_dma(&mut self) -> USB_INT_REQ_DMA_W<USBINTST_SPEC, 2> {
        USB_INT_REQ_DMA_W::new(self)
    }
    #[doc = "Bit 3 - USB host interrupt line status. This bit is read-only."]
    #[inline(always)]
    #[must_use]
    pub fn usb_host_int(&mut self) -> USB_HOST_INT_W<USBINTST_SPEC, 3> {
        USB_HOST_INT_W::new(self)
    }
    #[doc = "Bit 4 - External ATX interrupt line status. This bit is read-only."]
    #[inline(always)]
    #[must_use]
    pub fn usb_atx_int(&mut self) -> USB_ATX_INT_W<USBINTST_SPEC, 4> {
        USB_ATX_INT_W::new(self)
    }
    #[doc = "Bit 5 - OTG interrupt line status. This bit is read-only."]
    #[inline(always)]
    #[must_use]
    pub fn usb_otg_int(&mut self) -> USB_OTG_INT_W<USBINTST_SPEC, 5> {
        USB_OTG_INT_W::new(self)
    }
    #[doc = "Bit 6 - I2C module interrupt line status. This bit is read-only."]
    #[inline(always)]
    #[must_use]
    pub fn usb_i2c_int(&mut self) -> USB_I2C_INT_W<USBINTST_SPEC, 6> {
        USB_I2C_INT_W::new(self)
    }
    #[doc = "Bit 8 - USB need clock indicator. This bit is read-only. This bit is set to 1 when USB activity or a change of state on the USB data pins is detected, and it indicates that a PLL supplied clock of 48 MHz is needed. Once USB_NEED_CLK becomes one, it resets to zero 5 ms after the last packet has been received/sent, or 2 ms after the Suspend Change (SUS_CH) interrupt has occurred. A change of this bit from 0 to 1 can wake up the microcontroller if activity on the USB bus is selected to wake up the part from the Power-down mode (see Section 4.7.9 Wake-up from Reduced Power Modes for details). Also see Section 4.5.8 PLLs and Power-down mode and Section 4.7.10 Power Control for Peripherals register (PCONP - 0x400F C0C4) for considerations about the PLL and invoking the Power-down mode. This bit is read-only."]
    #[inline(always)]
    #[must_use]
    pub fn usb_need_clk(&mut self) -> USB_NEED_CLK_W<USBINTST_SPEC, 8> {
        USB_NEED_CLK_W::new(self)
    }
    #[doc = "Bit 31 - Enable all USB interrupts. When this bit is cleared, the NVIC does not see the ORed output of the USB interrupt lines."]
    #[inline(always)]
    #[must_use]
    pub fn en_usb_ints(&mut self) -> EN_USB_INTS_W<USBINTST_SPEC, 31> {
        EN_USB_INTS_W::new(self)
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
#[doc = "USB Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbintst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbintst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBINTST_SPEC;
impl crate::RegisterSpec for USBINTST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbintst::R`](R) reader structure"]
impl crate::Readable for USBINTST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbintst::W`](W) writer structure"]
impl crate::Writable for USBINTST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBINTST to value 0x8000_0000"]
impl crate::Resettable for USBINTST_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
