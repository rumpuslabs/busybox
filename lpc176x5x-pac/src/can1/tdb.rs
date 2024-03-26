#[doc = "Register `TDB%s` reader"]
pub type R = crate::R<TDB_SPEC>;
#[doc = "Register `TDB%s` writer"]
pub type W = crate::W<TDB_SPEC>;
#[doc = "Field `DATA5` reader - Data 5. If RTR = 0 and DLC >= 0101 in the corresponding CANTFI, this byte is sent as the 5th Data byte of the next transmit message."]
pub type DATA5_R = crate::FieldReader;
#[doc = "Field `DATA5` writer - Data 5. If RTR = 0 and DLC >= 0101 in the corresponding CANTFI, this byte is sent as the 5th Data byte of the next transmit message."]
pub type DATA5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DATA6` reader - Data 6. If RTR = 0 and DLC >= 0110 in the corresponding CANTFI, this byte is sent as the 6th Data byte of the next transmit message."]
pub type DATA6_R = crate::FieldReader;
#[doc = "Field `DATA6` writer - Data 6. If RTR = 0 and DLC >= 0110 in the corresponding CANTFI, this byte is sent as the 6th Data byte of the next transmit message."]
pub type DATA6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DATA7` reader - Data 7. If RTR = 0 and DLC >= 0111 in the corresponding CANTFI, this byte is sent as the 7th Data byte of the next transmit message."]
pub type DATA7_R = crate::FieldReader;
#[doc = "Field `DATA7` writer - Data 7. If RTR = 0 and DLC >= 0111 in the corresponding CANTFI, this byte is sent as the 7th Data byte of the next transmit message."]
pub type DATA7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DATA8` reader - Data 8. If RTR = 0 and DLC >= 1000 in the corresponding CANTFI, this byte is sent as the 8th Data byte of the next transmit message."]
pub type DATA8_R = crate::FieldReader;
#[doc = "Field `DATA8` writer - Data 8. If RTR = 0 and DLC >= 1000 in the corresponding CANTFI, this byte is sent as the 8th Data byte of the next transmit message."]
pub type DATA8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data 5. If RTR = 0 and DLC >= 0101 in the corresponding CANTFI, this byte is sent as the 5th Data byte of the next transmit message."]
    #[inline(always)]
    pub fn data5(&self) -> DATA5_R {
        DATA5_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data 6. If RTR = 0 and DLC >= 0110 in the corresponding CANTFI, this byte is sent as the 6th Data byte of the next transmit message."]
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data 7. If RTR = 0 and DLC >= 0111 in the corresponding CANTFI, this byte is sent as the 7th Data byte of the next transmit message."]
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data 8. If RTR = 0 and DLC >= 1000 in the corresponding CANTFI, this byte is sent as the 8th Data byte of the next transmit message."]
    #[inline(always)]
    pub fn data8(&self) -> DATA8_R {
        DATA8_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TDB")
            .field("data5", &format_args!("{}", self.data5().bits()))
            .field("data6", &format_args!("{}", self.data6().bits()))
            .field("data7", &format_args!("{}", self.data7().bits()))
            .field("data8", &format_args!("{}", self.data8().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TDB_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 5. If RTR = 0 and DLC >= 0101 in the corresponding CANTFI, this byte is sent as the 5th Data byte of the next transmit message."]
    #[inline(always)]
    #[must_use]
    pub fn data5(&mut self) -> DATA5_W<TDB_SPEC, 0> {
        DATA5_W::new(self)
    }
    #[doc = "Bits 8:15 - Data 6. If RTR = 0 and DLC >= 0110 in the corresponding CANTFI, this byte is sent as the 6th Data byte of the next transmit message."]
    #[inline(always)]
    #[must_use]
    pub fn data6(&mut self) -> DATA6_W<TDB_SPEC, 8> {
        DATA6_W::new(self)
    }
    #[doc = "Bits 16:23 - Data 7. If RTR = 0 and DLC >= 0111 in the corresponding CANTFI, this byte is sent as the 7th Data byte of the next transmit message."]
    #[inline(always)]
    #[must_use]
    pub fn data7(&mut self) -> DATA7_W<TDB_SPEC, 16> {
        DATA7_W::new(self)
    }
    #[doc = "Bits 24:31 - Data 8. If RTR = 0 and DLC >= 1000 in the corresponding CANTFI, this byte is sent as the 8th Data byte of the next transmit message."]
    #[inline(always)]
    #[must_use]
    pub fn data8(&mut self) -> DATA8_W<TDB_SPEC, 24> {
        DATA8_W::new(self)
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
#[doc = "Transmit data bytes 5-8 (Tx Buffer )\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TDB_SPEC;
impl crate::RegisterSpec for TDB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdb::R`](R) reader structure"]
impl crate::Readable for TDB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tdb::W`](W) writer structure"]
impl crate::Writable for TDB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TDB%s to value 0"]
impl crate::Resettable for TDB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
