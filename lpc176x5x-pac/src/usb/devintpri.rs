#[doc = "Register `DEVINTPRI` writer"]
pub type W = crate::W<DEVINTPRI_SPEC>;
#[doc = "Frame interrupt routing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRAME_AW {
    #[doc = "0: FRAME interrupt is routed to USB_INT_REQ_LP."]
    LP = 0,
    #[doc = "1: FRAME interrupt is routed to USB_INT_REQ_HP."]
    HP = 1,
}
impl From<FRAME_AW> for bool {
    #[inline(always)]
    fn from(variant: FRAME_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRAME` writer - Frame interrupt routing"]
pub type FRAME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FRAME_AW>;
impl<'a, REG, const O: u8> FRAME_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FRAME interrupt is routed to USB_INT_REQ_LP."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut crate::W<REG> {
        self.variant(FRAME_AW::LP)
    }
    #[doc = "FRAME interrupt is routed to USB_INT_REQ_HP."]
    #[inline(always)]
    pub fn hp(self) -> &'a mut crate::W<REG> {
        self.variant(FRAME_AW::HP)
    }
}
#[doc = "Fast endpoint interrupt routing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EP_FAST_AW {
    #[doc = "0: EP_FAST interrupt is routed to USB_INT_REQ_LP."]
    LP = 0,
    #[doc = "1: EP_FAST interrupt is routed to USB_INT_REQ_HP."]
    HP = 1,
}
impl From<EP_FAST_AW> for bool {
    #[inline(always)]
    fn from(variant: EP_FAST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP_FAST` writer - Fast endpoint interrupt routing"]
pub type EP_FAST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EP_FAST_AW>;
impl<'a, REG, const O: u8> EP_FAST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EP_FAST interrupt is routed to USB_INT_REQ_LP."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut crate::W<REG> {
        self.variant(EP_FAST_AW::LP)
    }
    #[doc = "EP_FAST interrupt is routed to USB_INT_REQ_HP."]
    #[inline(always)]
    pub fn hp(self) -> &'a mut crate::W<REG> {
        self.variant(EP_FAST_AW::HP)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DEVINTPRI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Frame interrupt routing"]
    #[inline(always)]
    #[must_use]
    pub fn frame(&mut self) -> FRAME_W<DEVINTPRI_SPEC, 0> {
        FRAME_W::new(self)
    }
    #[doc = "Bit 1 - Fast endpoint interrupt routing"]
    #[inline(always)]
    #[must_use]
    pub fn ep_fast(&mut self) -> EP_FAST_W<DEVINTPRI_SPEC, 1> {
        EP_FAST_W::new(self)
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
#[doc = "USB Device Interrupt Priority\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devintpri::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVINTPRI_SPEC;
impl crate::RegisterSpec for DEVINTPRI_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`devintpri::W`](W) writer structure"]
impl crate::Writable for DEVINTPRI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVINTPRI to value 0"]
impl crate::Resettable for DEVINTPRI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
