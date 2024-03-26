#[doc = "Register `SET` writer"]
pub type W = crate::W<SET_SPEC>;
#[doc = "Field `INX_INT` writer - Writing a 1 sets the INX_Int bit in QEIINTSTAT."]
pub type INX_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM_INT` writer - Writing a 1 sets the TIN_Int bit in QEIINTSTAT."]
pub type TIM_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VELC_INT` writer - Writing a 1 sets the VELC_Int bit in QEIINTSTAT."]
pub type VELC_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIR_INT` writer - Writing a 1 sets the DIR_Int bit in QEIINTSTAT."]
pub type DIR_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERR_INT` writer - Writing a 1 sets the ERR_Int bit in QEIINTSTAT."]
pub type ERR_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENCLK_INT` writer - Writing a 1 sets the ENCLK_Int bit in QEIINTSTAT."]
pub type ENCLK_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `POS0_INT` writer - Writing a 1 sets the POS0_Int bit in QEIINTSTAT."]
pub type POS0_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `POS1_INT` writer - Writing a 1 sets the POS1_Int bit in QEIINTSTAT."]
pub type POS1_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `POS2_INT` writer - Writing a 1 sets the POS2_Int bit in QEIINTSTAT."]
pub type POS2_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REV0_INT` writer - Writing a 1 sets the REV0_Int bit in QEIINTSTAT."]
pub type REV0_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `POS0REV_INT` writer - Writing a 1 sets the POS0REV_Int bit in QEIINTSTAT."]
pub type POS0REV_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `POS1REV_INT` writer - Writing a 1 sets the POS1REV_Int bit in QEIINTSTAT."]
pub type POS1REV_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `POS2REV_INT` writer - Writing a 1 sets the POS2REV_Int bit in QEIINTSTAT."]
pub type POS2REV_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REV1_INT` writer - Writing a 1 sets the REV1_Int bit in QEIINTSTAT."]
pub type REV1_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REV2_INT` writer - Writing a 1 sets the REV2_Int bit in QEIINTSTAT."]
pub type REV2_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MAXPOS_INT` writer - Writing a 1 sets the MAXPOS_Int bit in QEIINTSTAT."]
pub type MAXPOS_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<SET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Writing a 1 sets the INX_Int bit in QEIINTSTAT."]
    #[inline(always)]
    #[must_use]
    pub fn inx_int(&mut self) -> INX_INT_W<SET_SPEC, 0> {
        INX_INT_W::new(self)
    }
    #[doc = "Bit 1 - Writing a 1 sets the TIN_Int bit in QEIINTSTAT."]
    #[inline(always)]
    #[must_use]
    pub fn tim_int(&mut self) -> TIM_INT_W<SET_SPEC, 1> {
        TIM_INT_W::new(self)
    }
    #[doc = "Bit 2 - Writing a 1 sets the VELC_Int bit in QEIINTSTAT."]
    #[inline(always)]
    #[must_use]
    pub fn velc_int(&mut self) -> VELC_INT_W<SET_SPEC, 2> {
        VELC_INT_W::new(self)
    }
    #[doc = "Bit 3 - Writing a 1 sets the DIR_Int bit in QEIINTSTAT."]
    #[inline(always)]
    #[must_use]
    pub fn dir_int(&mut self) -> DIR_INT_W<SET_SPEC, 3> {
        DIR_INT_W::new(self)
    }
    #[doc = "Bit 4 - Writing a 1 sets the ERR_Int bit in QEIINTSTAT."]
    #[inline(always)]
    #[must_use]
    pub fn err_int(&mut self) -> ERR_INT_W<SET_SPEC, 4> {
        ERR_INT_W::new(self)
    }
    #[doc = "Bit 5 - Writing a 1 sets the ENCLK_Int bit in QEIINTSTAT."]
    #[inline(always)]
    #[must_use]
    pub fn enclk_int(&mut self) -> ENCLK_INT_W<SET_SPEC, 5> {
        ENCLK_INT_W::new(self)
    }
    #[doc = "Bit 6 - Writing a 1 sets the POS0_Int bit in QEIINTSTAT."]
    #[inline(always)]
    #[must_use]
    pub fn pos0_int(&mut self) -> POS0_INT_W<SET_SPEC, 6> {
        POS0_INT_W::new(self)
    }
    #[doc = "Bit 7 - Writing a 1 sets the POS1_Int bit in QEIINTSTAT."]
    #[inline(always)]
    #[must_use]
    pub fn pos1_int(&mut self) -> POS1_INT_W<SET_SPEC, 7> {
        POS1_INT_W::new(self)
    }
    #[doc = "Bit 8 - Writing a 1 sets the POS2_Int bit in QEIINTSTAT."]
    #[inline(always)]
    #[must_use]
    pub fn pos2_int(&mut self) -> POS2_INT_W<SET_SPEC, 8> {
        POS2_INT_W::new(self)
    }
    #[doc = "Bit 9 - Writing a 1 sets the REV0_Int bit in QEIINTSTAT."]
    #[inline(always)]
    #[must_use]
    pub fn rev0_int(&mut self) -> REV0_INT_W<SET_SPEC, 9> {
        REV0_INT_W::new(self)
    }
    #[doc = "Bit 10 - Writing a 1 sets the POS0REV_Int bit in QEIINTSTAT."]
    #[inline(always)]
    #[must_use]
    pub fn pos0rev_int(&mut self) -> POS0REV_INT_W<SET_SPEC, 10> {
        POS0REV_INT_W::new(self)
    }
    #[doc = "Bit 11 - Writing a 1 sets the POS1REV_Int bit in QEIINTSTAT."]
    #[inline(always)]
    #[must_use]
    pub fn pos1rev_int(&mut self) -> POS1REV_INT_W<SET_SPEC, 11> {
        POS1REV_INT_W::new(self)
    }
    #[doc = "Bit 12 - Writing a 1 sets the POS2REV_Int bit in QEIINTSTAT."]
    #[inline(always)]
    #[must_use]
    pub fn pos2rev_int(&mut self) -> POS2REV_INT_W<SET_SPEC, 12> {
        POS2REV_INT_W::new(self)
    }
    #[doc = "Bit 13 - Writing a 1 sets the REV1_Int bit in QEIINTSTAT."]
    #[inline(always)]
    #[must_use]
    pub fn rev1_int(&mut self) -> REV1_INT_W<SET_SPEC, 13> {
        REV1_INT_W::new(self)
    }
    #[doc = "Bit 14 - Writing a 1 sets the REV2_Int bit in QEIINTSTAT."]
    #[inline(always)]
    #[must_use]
    pub fn rev2_int(&mut self) -> REV2_INT_W<SET_SPEC, 14> {
        REV2_INT_W::new(self)
    }
    #[doc = "Bit 15 - Writing a 1 sets the MAXPOS_Int bit in QEIINTSTAT."]
    #[inline(always)]
    #[must_use]
    pub fn maxpos_int(&mut self) -> MAXPOS_INT_W<SET_SPEC, 15> {
        MAXPOS_INT_W::new(self)
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
#[doc = "Interrupt status set register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SET_SPEC;
impl crate::RegisterSpec for SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set::W`](W) writer structure"]
impl crate::Writable for SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SET to value 0"]
impl crate::Resettable for SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
