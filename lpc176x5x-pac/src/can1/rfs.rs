#[doc = "Register `RFS` reader"]
pub type R = crate::R<RFS_SPEC>;
#[doc = "Register `RFS` writer"]
pub type W = crate::W<RFS_SPEC>;
#[doc = "Field `IDINDEX` reader - ID Index. If the BP bit (below) is 0, this value is the zero-based number of the Lookup Table RAM entry at which the Acceptance Filter matched the received Identifier. Disabled entries in the Standard tables are included in this numbering, but will not be matched. See Section 21.17 Examples of acceptance filter tables and ID index values on page 587 for examples of ID Index values."]
pub type IDINDEX_R = crate::FieldReader<u16>;
#[doc = "Field `IDINDEX` writer - ID Index. If the BP bit (below) is 0, this value is the zero-based number of the Lookup Table RAM entry at which the Acceptance Filter matched the received Identifier. Disabled entries in the Standard tables are included in this numbering, but will not be matched. See Section 21.17 Examples of acceptance filter tables and ID index values on page 587 for examples of ID Index values."]
pub type IDINDEX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `BP` reader - If this bit is 1, the current message was received in AF Bypass mode, and the ID Index field (above) is meaningless."]
pub type BP_R = crate::BitReader;
#[doc = "Field `BP` writer - If this bit is 1, the current message was received in AF Bypass mode, and the ID Index field (above) is meaningless."]
pub type BP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DLC` reader - The field contains the Data Length Code (DLC) field of the current received message. When RTR = 0, this is related to the number of data bytes available in the CANRDA and CANRDB registers as follows: 0000-0111 = 0 to 7 bytes1000-1111 = 8 bytes With RTR = 1, this value indicates the number of data bytes requested to be sent back, with the same encoding."]
pub type DLC_R = crate::FieldReader;
#[doc = "Field `DLC` writer - The field contains the Data Length Code (DLC) field of the current received message. When RTR = 0, this is related to the number of data bytes available in the CANRDA and CANRDB registers as follows: 0000-0111 = 0 to 7 bytes1000-1111 = 8 bytes With RTR = 1, this value indicates the number of data bytes requested to be sent back, with the same encoding."]
pub type DLC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `RTR` reader - This bit contains the Remote Transmission Request bit of the current received message. 0 indicates a Data Frame, in which (if DLC is non-zero) data can be read from the CANRDA and possibly the CANRDB registers. 1 indicates a Remote frame, in which case the DLC value identifies the number of data bytes requested to be sent using the same Identifier."]
pub type RTR_R = crate::BitReader;
#[doc = "Field `RTR` writer - This bit contains the Remote Transmission Request bit of the current received message. 0 indicates a Data Frame, in which (if DLC is non-zero) data can be read from the CANRDA and possibly the CANRDB registers. 1 indicates a Remote frame, in which case the DLC value identifies the number of data bytes requested to be sent using the same Identifier."]
pub type RTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FF` reader - A 0 in this bit indicates that the current received message included an 11-bit Identifier, while a 1 indicates a 29-bit Identifier. This affects the contents of the CANid register described below."]
pub type FF_R = crate::BitReader;
#[doc = "Field `FF` writer - A 0 in this bit indicates that the current received message included an 11-bit Identifier, while a 1 indicates a 29-bit Identifier. This affects the contents of the CANid register described below."]
pub type FF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:9 - ID Index. If the BP bit (below) is 0, this value is the zero-based number of the Lookup Table RAM entry at which the Acceptance Filter matched the received Identifier. Disabled entries in the Standard tables are included in this numbering, but will not be matched. See Section 21.17 Examples of acceptance filter tables and ID index values on page 587 for examples of ID Index values."]
    #[inline(always)]
    pub fn idindex(&self) -> IDINDEX_R {
        IDINDEX_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - If this bit is 1, the current message was received in AF Bypass mode, and the ID Index field (above) is meaningless."]
    #[inline(always)]
    pub fn bp(&self) -> BP_R {
        BP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:19 - The field contains the Data Length Code (DLC) field of the current received message. When RTR = 0, this is related to the number of data bytes available in the CANRDA and CANRDB registers as follows: 0000-0111 = 0 to 7 bytes1000-1111 = 8 bytes With RTR = 1, this value indicates the number of data bytes requested to be sent back, with the same encoding."]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - This bit contains the Remote Transmission Request bit of the current received message. 0 indicates a Data Frame, in which (if DLC is non-zero) data can be read from the CANRDA and possibly the CANRDB registers. 1 indicates a Remote frame, in which case the DLC value identifies the number of data bytes requested to be sent using the same Identifier."]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - A 0 in this bit indicates that the current received message included an 11-bit Identifier, while a 1 indicates a 29-bit Identifier. This affects the contents of the CANid register described below."]
    #[inline(always)]
    pub fn ff(&self) -> FF_R {
        FF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFS")
            .field("idindex", &format_args!("{}", self.idindex().bits()))
            .field("bp", &format_args!("{}", self.bp().bit()))
            .field("dlc", &format_args!("{}", self.dlc().bits()))
            .field("rtr", &format_args!("{}", self.rtr().bit()))
            .field("ff", &format_args!("{}", self.ff().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RFS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9 - ID Index. If the BP bit (below) is 0, this value is the zero-based number of the Lookup Table RAM entry at which the Acceptance Filter matched the received Identifier. Disabled entries in the Standard tables are included in this numbering, but will not be matched. See Section 21.17 Examples of acceptance filter tables and ID index values on page 587 for examples of ID Index values."]
    #[inline(always)]
    #[must_use]
    pub fn idindex(&mut self) -> IDINDEX_W<RFS_SPEC, 0> {
        IDINDEX_W::new(self)
    }
    #[doc = "Bit 10 - If this bit is 1, the current message was received in AF Bypass mode, and the ID Index field (above) is meaningless."]
    #[inline(always)]
    #[must_use]
    pub fn bp(&mut self) -> BP_W<RFS_SPEC, 10> {
        BP_W::new(self)
    }
    #[doc = "Bits 16:19 - The field contains the Data Length Code (DLC) field of the current received message. When RTR = 0, this is related to the number of data bytes available in the CANRDA and CANRDB registers as follows: 0000-0111 = 0 to 7 bytes1000-1111 = 8 bytes With RTR = 1, this value indicates the number of data bytes requested to be sent back, with the same encoding."]
    #[inline(always)]
    #[must_use]
    pub fn dlc(&mut self) -> DLC_W<RFS_SPEC, 16> {
        DLC_W::new(self)
    }
    #[doc = "Bit 30 - This bit contains the Remote Transmission Request bit of the current received message. 0 indicates a Data Frame, in which (if DLC is non-zero) data can be read from the CANRDA and possibly the CANRDB registers. 1 indicates a Remote frame, in which case the DLC value identifies the number of data bytes requested to be sent using the same Identifier."]
    #[inline(always)]
    #[must_use]
    pub fn rtr(&mut self) -> RTR_W<RFS_SPEC, 30> {
        RTR_W::new(self)
    }
    #[doc = "Bit 31 - A 0 in this bit indicates that the current received message included an 11-bit Identifier, while a 1 indicates a 29-bit Identifier. This affects the contents of the CANid register described below."]
    #[inline(always)]
    #[must_use]
    pub fn ff(&mut self) -> FF_W<RFS_SPEC, 31> {
        FF_W::new(self)
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
#[doc = "Receive frame status. Can only be written when RM in CANMOD is 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFS_SPEC;
impl crate::RegisterSpec for RFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfs::R`](R) reader structure"]
impl crate::Readable for RFS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rfs::W`](W) writer structure"]
impl crate::Writable for RFS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFS to value 0"]
impl crate::Resettable for RFS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
