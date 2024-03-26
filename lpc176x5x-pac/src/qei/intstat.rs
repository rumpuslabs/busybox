#[doc = "Register `INTSTAT` reader"]
pub type R = crate::R<INTSTAT_SPEC>;
#[doc = "Field `INX_INT` reader - Indicates that an index pulse was detected."]
pub type INX_INT_R = crate::BitReader;
#[doc = "Field `TIM_INT` reader - Indicates that a velocity timer overflow occurred"]
pub type TIM_INT_R = crate::BitReader;
#[doc = "Field `VELC_INT` reader - Indicates that captured velocity is less than compare velocity."]
pub type VELC_INT_R = crate::BitReader;
#[doc = "Field `DIR_INT` reader - Indicates that a change of direction was detected."]
pub type DIR_INT_R = crate::BitReader;
#[doc = "Field `ERR_INT` reader - Indicates that an encoder phase error was detected."]
pub type ERR_INT_R = crate::BitReader;
#[doc = "Field `ENCLK_INT` reader - Indicates that and encoder clock pulse was detected."]
pub type ENCLK_INT_R = crate::BitReader;
#[doc = "Field `POS0_INT` reader - Indicates that the position 0 compare value is equal to the current position."]
pub type POS0_INT_R = crate::BitReader;
#[doc = "Field `POS1_INT` reader - Indicates that the position 1compare value is equal to the current position."]
pub type POS1_INT_R = crate::BitReader;
#[doc = "Field `POS2_INT` reader - Indicates that the position 2 compare value is equal to the current position."]
pub type POS2_INT_R = crate::BitReader;
#[doc = "Field `REV0_INT` reader - Indicates that the index compare 0 value is equal to the current index count."]
pub type REV0_INT_R = crate::BitReader;
#[doc = "Field `POS0REV_INT` reader - Combined position 0 and revolution count interrupt. Set when both the POS0_Int bit is set and the REV0_Int is set."]
pub type POS0REV_INT_R = crate::BitReader;
#[doc = "Field `POS1REV_INT` reader - Combined position 1 and revolution count interrupt. Set when both the POS1_Int bit is set and the REV1_Int is set."]
pub type POS1REV_INT_R = crate::BitReader;
#[doc = "Field `POS2REV_INT` reader - Combined position 2 and revolution count interrupt. Set when both the POS2_Int bit is set and the REV2_Int is set."]
pub type POS2REV_INT_R = crate::BitReader;
#[doc = "Field `REV1_INT` reader - Indicates that the index compare 1value is equal to the current index count."]
pub type REV1_INT_R = crate::BitReader;
#[doc = "Field `REV2_INT` reader - Indicates that the index compare 2 value is equal to the current index count."]
pub type REV2_INT_R = crate::BitReader;
#[doc = "Field `MAXPOS_INT` reader - Indicates that the current position count goes through the MAXPOS value to zero in the forward direction, or through zero to MAXPOS in the reverse direction."]
pub type MAXPOS_INT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates that an index pulse was detected."]
    #[inline(always)]
    pub fn inx_int(&self) -> INX_INT_R {
        INX_INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates that a velocity timer overflow occurred"]
    #[inline(always)]
    pub fn tim_int(&self) -> TIM_INT_R {
        TIM_INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates that captured velocity is less than compare velocity."]
    #[inline(always)]
    pub fn velc_int(&self) -> VELC_INT_R {
        VELC_INT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates that a change of direction was detected."]
    #[inline(always)]
    pub fn dir_int(&self) -> DIR_INT_R {
        DIR_INT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates that an encoder phase error was detected."]
    #[inline(always)]
    pub fn err_int(&self) -> ERR_INT_R {
        ERR_INT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates that and encoder clock pulse was detected."]
    #[inline(always)]
    pub fn enclk_int(&self) -> ENCLK_INT_R {
        ENCLK_INT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates that the position 0 compare value is equal to the current position."]
    #[inline(always)]
    pub fn pos0_int(&self) -> POS0_INT_R {
        POS0_INT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates that the position 1compare value is equal to the current position."]
    #[inline(always)]
    pub fn pos1_int(&self) -> POS1_INT_R {
        POS1_INT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates that the position 2 compare value is equal to the current position."]
    #[inline(always)]
    pub fn pos2_int(&self) -> POS2_INT_R {
        POS2_INT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates that the index compare 0 value is equal to the current index count."]
    #[inline(always)]
    pub fn rev0_int(&self) -> REV0_INT_R {
        REV0_INT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Combined position 0 and revolution count interrupt. Set when both the POS0_Int bit is set and the REV0_Int is set."]
    #[inline(always)]
    pub fn pos0rev_int(&self) -> POS0REV_INT_R {
        POS0REV_INT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Combined position 1 and revolution count interrupt. Set when both the POS1_Int bit is set and the REV1_Int is set."]
    #[inline(always)]
    pub fn pos1rev_int(&self) -> POS1REV_INT_R {
        POS1REV_INT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Combined position 2 and revolution count interrupt. Set when both the POS2_Int bit is set and the REV2_Int is set."]
    #[inline(always)]
    pub fn pos2rev_int(&self) -> POS2REV_INT_R {
        POS2REV_INT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Indicates that the index compare 1value is equal to the current index count."]
    #[inline(always)]
    pub fn rev1_int(&self) -> REV1_INT_R {
        REV1_INT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Indicates that the index compare 2 value is equal to the current index count."]
    #[inline(always)]
    pub fn rev2_int(&self) -> REV2_INT_R {
        REV2_INT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Indicates that the current position count goes through the MAXPOS value to zero in the forward direction, or through zero to MAXPOS in the reverse direction."]
    #[inline(always)]
    pub fn maxpos_int(&self) -> MAXPOS_INT_R {
        MAXPOS_INT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTAT")
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
impl core::fmt::Debug for crate::generic::Reg<INTSTAT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTSTAT_SPEC;
impl crate::RegisterSpec for INTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstat::R`](R) reader structure"]
impl crate::Readable for INTSTAT_SPEC {}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for INTSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
