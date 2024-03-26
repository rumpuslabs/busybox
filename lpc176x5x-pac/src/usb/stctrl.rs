#[doc = "Register `STCTRL` reader"]
pub type R = crate::R<STCTRL_SPEC>;
#[doc = "Register `STCTRL` writer"]
pub type W = crate::W<STCTRL_SPEC>;
#[doc = "Field `PORT_FUNC` reader - Controls connection of USB functions (see Figure 51). Bit 0 is set or cleared by hardware when B_HNP_TRACK or A_HNP_TRACK is set and HNP succeeds. See Section 14.9. 00: U1 = device (OTG), U2 = host 01: U1 = host (OTG), U2 = host 10: Reserved 11: U1 = host, U2 = device In a device-only configuration, the following values are allowed: 00: U1 = device. The USB device controller signals are mapped to the U1 port: USB_CONNECT1, USB_UP_LED1, USB_D+1, USB_D-1. 11: U2 = device. The USB device controller signals are mapped to the U2 port: USB_CONNECT2, USB_UP_LED2, USB_D+2, USB_D-2."]
pub type PORT_FUNC_R = crate::FieldReader;
#[doc = "Field `PORT_FUNC` writer - Controls connection of USB functions (see Figure 51). Bit 0 is set or cleared by hardware when B_HNP_TRACK or A_HNP_TRACK is set and HNP succeeds. See Section 14.9. 00: U1 = device (OTG), U2 = host 01: U1 = host (OTG), U2 = host 10: Reserved 11: U1 = host, U2 = device In a device-only configuration, the following values are allowed: 00: U1 = device. The USB device controller signals are mapped to the U1 port: USB_CONNECT1, USB_UP_LED1, USB_D+1, USB_D-1. 11: U2 = device. The USB device controller signals are mapped to the U2 port: USB_CONNECT2, USB_UP_LED2, USB_D+2, USB_D-2."]
pub type PORT_FUNC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TMR_SCALE` reader - Timer scale selection. This field determines the duration of each timer count. 00: 10 ms (100 KHz) 01: 100 ms (10 KHz) 10: 1000 ms (1 KHz) 11: Reserved"]
pub type TMR_SCALE_R = crate::FieldReader;
#[doc = "Field `TMR_SCALE` writer - Timer scale selection. This field determines the duration of each timer count. 00: 10 ms (100 KHz) 01: 100 ms (10 KHz) 10: 1000 ms (1 KHz) 11: Reserved"]
pub type TMR_SCALE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TMR_MODE` reader - Timer mode selection. 0: monoshot 1: free running"]
pub type TMR_MODE_R = crate::BitReader;
#[doc = "Field `TMR_MODE` writer - Timer mode selection. 0: monoshot 1: free running"]
pub type TMR_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR_EN` reader - Timer enable. When set, TMR_CNT increments. When cleared, TMR_CNT is reset to 0."]
pub type TMR_EN_R = crate::BitReader;
#[doc = "Field `TMR_EN` writer - Timer enable. When set, TMR_CNT increments. When cleared, TMR_CNT is reset to 0."]
pub type TMR_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR_RST` reader - Timer reset. Writing one to this bit resets TMR_CNT to 0. This provides a single bit control for the software to restart the timer when the timer is enabled."]
pub type TMR_RST_R = crate::BitReader;
#[doc = "Field `TMR_RST` writer - Timer reset. Writing one to this bit resets TMR_CNT to 0. This provides a single bit control for the software to restart the timer when the timer is enabled."]
pub type TMR_RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `B_HNP_TRACK` reader - Enable HNP tracking for B-device (peripheral), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
pub type B_HNP_TRACK_R = crate::BitReader;
#[doc = "Field `B_HNP_TRACK` writer - Enable HNP tracking for B-device (peripheral), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
pub type B_HNP_TRACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `A_HNP_TRACK` reader - Enable HNP tracking for A-device (host), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
pub type A_HNP_TRACK_R = crate::BitReader;
#[doc = "Field `A_HNP_TRACK` writer - Enable HNP tracking for A-device (host), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
pub type A_HNP_TRACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PU_REMOVED` reader - When the B-device changes its role from peripheral to host, software sets this bit when it removes the D+ pull-up, see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
pub type PU_REMOVED_R = crate::BitReader;
#[doc = "Field `PU_REMOVED` writer - When the B-device changes its role from peripheral to host, software sets this bit when it removes the D+ pull-up, see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
pub type PU_REMOVED_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMR_CNT` reader - Current timer count value."]
pub type TMR_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `TMR_CNT` writer - Current timer count value."]
pub type TMR_CNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:1 - Controls connection of USB functions (see Figure 51). Bit 0 is set or cleared by hardware when B_HNP_TRACK or A_HNP_TRACK is set and HNP succeeds. See Section 14.9. 00: U1 = device (OTG), U2 = host 01: U1 = host (OTG), U2 = host 10: Reserved 11: U1 = host, U2 = device In a device-only configuration, the following values are allowed: 00: U1 = device. The USB device controller signals are mapped to the U1 port: USB_CONNECT1, USB_UP_LED1, USB_D+1, USB_D-1. 11: U2 = device. The USB device controller signals are mapped to the U2 port: USB_CONNECT2, USB_UP_LED2, USB_D+2, USB_D-2."]
    #[inline(always)]
    pub fn port_func(&self) -> PORT_FUNC_R {
        PORT_FUNC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Timer scale selection. This field determines the duration of each timer count. 00: 10 ms (100 KHz) 01: 100 ms (10 KHz) 10: 1000 ms (1 KHz) 11: Reserved"]
    #[inline(always)]
    pub fn tmr_scale(&self) -> TMR_SCALE_R {
        TMR_SCALE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Timer mode selection. 0: monoshot 1: free running"]
    #[inline(always)]
    pub fn tmr_mode(&self) -> TMR_MODE_R {
        TMR_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer enable. When set, TMR_CNT increments. When cleared, TMR_CNT is reset to 0."]
    #[inline(always)]
    pub fn tmr_en(&self) -> TMR_EN_R {
        TMR_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer reset. Writing one to this bit resets TMR_CNT to 0. This provides a single bit control for the software to restart the timer when the timer is enabled."]
    #[inline(always)]
    pub fn tmr_rst(&self) -> TMR_RST_R {
        TMR_RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable HNP tracking for B-device (peripheral), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn b_hnp_track(&self) -> B_HNP_TRACK_R {
        B_HNP_TRACK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable HNP tracking for A-device (host), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn a_hnp_track(&self) -> A_HNP_TRACK_R {
        A_HNP_TRACK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When the B-device changes its role from peripheral to host, software sets this bit when it removes the D+ pull-up, see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn pu_removed(&self) -> PU_REMOVED_R {
        PU_REMOVED_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Current timer count value."]
    #[inline(always)]
    pub fn tmr_cnt(&self) -> TMR_CNT_R {
        TMR_CNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STCTRL")
            .field("port_func", &format_args!("{}", self.port_func().bits()))
            .field("tmr_scale", &format_args!("{}", self.tmr_scale().bits()))
            .field("tmr_mode", &format_args!("{}", self.tmr_mode().bit()))
            .field("tmr_en", &format_args!("{}", self.tmr_en().bit()))
            .field("tmr_rst", &format_args!("{}", self.tmr_rst().bit()))
            .field("b_hnp_track", &format_args!("{}", self.b_hnp_track().bit()))
            .field("a_hnp_track", &format_args!("{}", self.a_hnp_track().bit()))
            .field("pu_removed", &format_args!("{}", self.pu_removed().bit()))
            .field("tmr_cnt", &format_args!("{}", self.tmr_cnt().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<STCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Controls connection of USB functions (see Figure 51). Bit 0 is set or cleared by hardware when B_HNP_TRACK or A_HNP_TRACK is set and HNP succeeds. See Section 14.9. 00: U1 = device (OTG), U2 = host 01: U1 = host (OTG), U2 = host 10: Reserved 11: U1 = host, U2 = device In a device-only configuration, the following values are allowed: 00: U1 = device. The USB device controller signals are mapped to the U1 port: USB_CONNECT1, USB_UP_LED1, USB_D+1, USB_D-1. 11: U2 = device. The USB device controller signals are mapped to the U2 port: USB_CONNECT2, USB_UP_LED2, USB_D+2, USB_D-2."]
    #[inline(always)]
    #[must_use]
    pub fn port_func(&mut self) -> PORT_FUNC_W<STCTRL_SPEC, 0> {
        PORT_FUNC_W::new(self)
    }
    #[doc = "Bits 2:3 - Timer scale selection. This field determines the duration of each timer count. 00: 10 ms (100 KHz) 01: 100 ms (10 KHz) 10: 1000 ms (1 KHz) 11: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn tmr_scale(&mut self) -> TMR_SCALE_W<STCTRL_SPEC, 2> {
        TMR_SCALE_W::new(self)
    }
    #[doc = "Bit 4 - Timer mode selection. 0: monoshot 1: free running"]
    #[inline(always)]
    #[must_use]
    pub fn tmr_mode(&mut self) -> TMR_MODE_W<STCTRL_SPEC, 4> {
        TMR_MODE_W::new(self)
    }
    #[doc = "Bit 5 - Timer enable. When set, TMR_CNT increments. When cleared, TMR_CNT is reset to 0."]
    #[inline(always)]
    #[must_use]
    pub fn tmr_en(&mut self) -> TMR_EN_W<STCTRL_SPEC, 5> {
        TMR_EN_W::new(self)
    }
    #[doc = "Bit 6 - Timer reset. Writing one to this bit resets TMR_CNT to 0. This provides a single bit control for the software to restart the timer when the timer is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn tmr_rst(&mut self) -> TMR_RST_W<STCTRL_SPEC, 6> {
        TMR_RST_W::new(self)
    }
    #[doc = "Bit 8 - Enable HNP tracking for B-device (peripheral), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    #[must_use]
    pub fn b_hnp_track(&mut self) -> B_HNP_TRACK_W<STCTRL_SPEC, 8> {
        B_HNP_TRACK_W::new(self)
    }
    #[doc = "Bit 9 - Enable HNP tracking for A-device (host), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    #[must_use]
    pub fn a_hnp_track(&mut self) -> A_HNP_TRACK_W<STCTRL_SPEC, 9> {
        A_HNP_TRACK_W::new(self)
    }
    #[doc = "Bit 10 - When the B-device changes its role from peripheral to host, software sets this bit when it removes the D+ pull-up, see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    #[must_use]
    pub fn pu_removed(&mut self) -> PU_REMOVED_W<STCTRL_SPEC, 10> {
        PU_REMOVED_W::new(self)
    }
    #[doc = "Bits 16:31 - Current timer count value."]
    #[inline(always)]
    #[must_use]
    pub fn tmr_cnt(&mut self) -> TMR_CNT_W<STCTRL_SPEC, 16> {
        TMR_CNT_W::new(self)
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
#[doc = "OTG Status and Control and USB port select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STCTRL_SPEC;
impl crate::RegisterSpec for STCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stctrl::R`](R) reader structure"]
impl crate::Readable for STCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stctrl::W`](W) writer structure"]
impl crate::Writable for STCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STCTRL to value 0"]
impl crate::Resettable for STCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
