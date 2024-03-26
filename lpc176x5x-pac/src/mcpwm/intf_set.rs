#[doc = "Register `INTF_SET` writer"]
pub type W = crate::W<INTF_SET_SPEC>;
#[doc = "Field `ILIM0_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub type ILIM0_F_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IMAT0_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub type IMAT0_F_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ICAP0_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub type ICAP0_F_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ILIM1_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub type ILIM1_F_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IMAT1_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub type IMAT1_F_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ICAP1_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub type ICAP1_F_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ILIM2_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub type ILIM2_F_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IMAT2_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub type IMAT2_F_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ICAP2_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub type ICAP2_F_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ABORT_F_SET` writer - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
pub type ABORT_F_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<INTF_SET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ilim0_f_set(&mut self) -> ILIM0_F_SET_W<INTF_SET_SPEC, 0> {
        ILIM0_F_SET_W::new(self)
    }
    #[doc = "Bit 1 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn imat0_f_set(&mut self) -> IMAT0_F_SET_W<INTF_SET_SPEC, 1> {
        IMAT0_F_SET_W::new(self)
    }
    #[doc = "Bit 2 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn icap0_f_set(&mut self) -> ICAP0_F_SET_W<INTF_SET_SPEC, 2> {
        ICAP0_F_SET_W::new(self)
    }
    #[doc = "Bit 4 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ilim1_f_set(&mut self) -> ILIM1_F_SET_W<INTF_SET_SPEC, 4> {
        ILIM1_F_SET_W::new(self)
    }
    #[doc = "Bit 5 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn imat1_f_set(&mut self) -> IMAT1_F_SET_W<INTF_SET_SPEC, 5> {
        IMAT1_F_SET_W::new(self)
    }
    #[doc = "Bit 6 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn icap1_f_set(&mut self) -> ICAP1_F_SET_W<INTF_SET_SPEC, 6> {
        ICAP1_F_SET_W::new(self)
    }
    #[doc = "Bit 8 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ilim2_f_set(&mut self) -> ILIM2_F_SET_W<INTF_SET_SPEC, 8> {
        ILIM2_F_SET_W::new(self)
    }
    #[doc = "Bit 9 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn imat2_f_set(&mut self) -> IMAT2_F_SET_W<INTF_SET_SPEC, 9> {
        IMAT2_F_SET_W::new(self)
    }
    #[doc = "Bit 10 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn icap2_f_set(&mut self) -> ICAP2_F_SET_W<INTF_SET_SPEC, 10> {
        ICAP2_F_SET_W::new(self)
    }
    #[doc = "Bit 15 - Writing a one sets the corresponding bit in the INTF register, thus possibly simulating hardware interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn abort_f_set(&mut self) -> ABORT_F_SET_W<INTF_SET_SPEC, 15> {
        ABORT_F_SET_W::new(self)
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
#[doc = "Interrupt flags set address\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTF_SET_SPEC;
impl crate::RegisterSpec for INTF_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intf_set::W`](W) writer structure"]
impl crate::Writable for INTF_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTF_SET to value 0"]
impl crate::Resettable for INTF_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
