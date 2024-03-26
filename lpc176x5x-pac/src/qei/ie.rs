#[doc = "Register `IE` reader"]
pub type R = crate::R<IE_SPEC>;
#[doc = "Field `INX_INT` reader - When 1, the INX_Int interrupt is enabled."]
pub type INX_INT_R = crate::BitReader;
#[doc = "Field `TIM_INT` reader - When 1, the TIN_Int interrupt is enabled."]
pub type TIM_INT_R = crate::BitReader;
#[doc = "Field `VELC_INT` reader - When 1, the VELC_Int interrupt is enabled."]
pub type VELC_INT_R = crate::BitReader;
#[doc = "Field `DIR_INT` reader - When 1, the DIR_Int interrupt is enabled."]
pub type DIR_INT_R = crate::BitReader;
#[doc = "Field `ERR_INT` reader - When 1, the ERR_Int interrupt is enabled."]
pub type ERR_INT_R = crate::BitReader;
#[doc = "Field `ENCLK_INT` reader - When 1, the ENCLK_Int interrupt is enabled."]
pub type ENCLK_INT_R = crate::BitReader;
#[doc = "Field `POS0_INT` reader - When 1, the POS0_Int interrupt is enabled."]
pub type POS0_INT_R = crate::BitReader;
#[doc = "Field `POS1_INT` reader - When 1, the POS1_Int interrupt is enabled."]
pub type POS1_INT_R = crate::BitReader;
#[doc = "Field `POS2_INT` reader - When 1, the POS2_Int interrupt is enabled."]
pub type POS2_INT_R = crate::BitReader;
#[doc = "Field `REV0_INT` reader - When 1, the REV0_Int interrupt is enabled."]
pub type REV0_INT_R = crate::BitReader;
#[doc = "Field `POS0REV_INT` reader - When 1, the POS0REV_Int interrupt is enabled."]
pub type POS0REV_INT_R = crate::BitReader;
#[doc = "Field `POS1REV_INT` reader - When 1, the POS1REV_Int interrupt is enabled."]
pub type POS1REV_INT_R = crate::BitReader;
#[doc = "Field `POS2REV_INT` reader - When 1, the POS2REV_Int interrupt is enabled."]
pub type POS2REV_INT_R = crate::BitReader;
#[doc = "Field `REV1_INT` reader - When 1, the REV1_Int interrupt is enabled."]
pub type REV1_INT_R = crate::BitReader;
#[doc = "Field `REV2_INT` reader - When 1, the REV2_Int interrupt is enabled."]
pub type REV2_INT_R = crate::BitReader;
#[doc = "Field `MAXPOS_INT` reader - When 1, the MAXPOS_Int interrupt is enabled."]
pub type MAXPOS_INT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - When 1, the INX_Int interrupt is enabled."]
    #[inline(always)]
    pub fn inx_int(&self) -> INX_INT_R {
        INX_INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, the TIN_Int interrupt is enabled."]
    #[inline(always)]
    pub fn tim_int(&self) -> TIM_INT_R {
        TIM_INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When 1, the VELC_Int interrupt is enabled."]
    #[inline(always)]
    pub fn velc_int(&self) -> VELC_INT_R {
        VELC_INT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When 1, the DIR_Int interrupt is enabled."]
    #[inline(always)]
    pub fn dir_int(&self) -> DIR_INT_R {
        DIR_INT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When 1, the ERR_Int interrupt is enabled."]
    #[inline(always)]
    pub fn err_int(&self) -> ERR_INT_R {
        ERR_INT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When 1, the ENCLK_Int interrupt is enabled."]
    #[inline(always)]
    pub fn enclk_int(&self) -> ENCLK_INT_R {
        ENCLK_INT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When 1, the POS0_Int interrupt is enabled."]
    #[inline(always)]
    pub fn pos0_int(&self) -> POS0_INT_R {
        POS0_INT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When 1, the POS1_Int interrupt is enabled."]
    #[inline(always)]
    pub fn pos1_int(&self) -> POS1_INT_R {
        POS1_INT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - When 1, the POS2_Int interrupt is enabled."]
    #[inline(always)]
    pub fn pos2_int(&self) -> POS2_INT_R {
        POS2_INT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When 1, the REV0_Int interrupt is enabled."]
    #[inline(always)]
    pub fn rev0_int(&self) -> REV0_INT_R {
        REV0_INT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When 1, the POS0REV_Int interrupt is enabled."]
    #[inline(always)]
    pub fn pos0rev_int(&self) -> POS0REV_INT_R {
        POS0REV_INT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When 1, the POS1REV_Int interrupt is enabled."]
    #[inline(always)]
    pub fn pos1rev_int(&self) -> POS1REV_INT_R {
        POS1REV_INT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When 1, the POS2REV_Int interrupt is enabled."]
    #[inline(always)]
    pub fn pos2rev_int(&self) -> POS2REV_INT_R {
        POS2REV_INT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When 1, the REV1_Int interrupt is enabled."]
    #[inline(always)]
    pub fn rev1_int(&self) -> REV1_INT_R {
        REV1_INT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - When 1, the REV2_Int interrupt is enabled."]
    #[inline(always)]
    pub fn rev2_int(&self) -> REV2_INT_R {
        REV2_INT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - When 1, the MAXPOS_Int interrupt is enabled."]
    #[inline(always)]
    pub fn maxpos_int(&self) -> MAXPOS_INT_R {
        MAXPOS_INT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IE")
            .field("inx_int", &format_args!("{}", self.inx_int().bit()))
            .field("tim_int", &format_args!("{}", self.tim_int().bit()))
            .field("velc_int", &format_args!("{}", self.velc_int().bit()))
            .field("dir_int", &format_args!("{}", self.dir_int().bit()))
            .field("err_int", &format_args!("{}", self.err_int().bit()))
            .field("enclk_int", &format_args!("{}", self.enclk_int().bit()))
            .field("pos0_int", &format_args!("{}", self.pos0_int().bit()))
            .field("pos1_int", &format_args!("{}", self.pos1_int().bit()))
            .field("pos2_int", &format_args!("{}", self.pos2_int().bit()))
            .field("rev0_int", &format_args!("{}", self.rev0_int().bit()))
            .field("pos0rev_int", &format_args!("{}", self.pos0rev_int().bit()))
            .field("pos1rev_int", &format_args!("{}", self.pos1rev_int().bit()))
            .field("pos2rev_int", &format_args!("{}", self.pos2rev_int().bit()))
            .field("rev1_int", &format_args!("{}", self.rev1_int().bit()))
            .field("rev2_int", &format_args!("{}", self.rev2_int().bit()))
            .field("maxpos_int", &format_args!("{}", self.maxpos_int().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IE_SPEC;
impl crate::RegisterSpec for IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ie::R`](R) reader structure"]
impl crate::Readable for IE_SPEC {}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
