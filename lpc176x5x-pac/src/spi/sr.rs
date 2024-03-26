#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Field `ABRT` reader - Slave abort. When 1, this bit indicates that a slave abort has occurred. This bit is cleared by reading this register."]
pub type ABRT_R = crate::BitReader;
#[doc = "Field `MODF` reader - Mode fault. when 1, this bit indicates that a Mode fault error has occurred. This bit is cleared by reading this register, then writing the SPI0 control register."]
pub type MODF_R = crate::BitReader;
#[doc = "Field `ROVR` reader - Read overrun. When 1, this bit indicates that a read overrun has occurred. This bit is cleared by reading this register."]
pub type ROVR_R = crate::BitReader;
#[doc = "Field `WCOL` reader - Write collision. When 1, this bit indicates that a write collision has occurred. This bit is cleared by reading this register, then accessing the SPI Data Register."]
pub type WCOL_R = crate::BitReader;
#[doc = "Field `SPIF` reader - SPI transfer complete flag. When 1, this bit indicates when a SPI data transfer is complete. When a master, this bit is set at the end of the last cycle of the transfer. When a slave, this bit is set on the last data sampling edge of the SCK. This bit is cleared by first reading this register, then accessing the SPI Data Register. Note: this is not the SPI interrupt flag. This flag is found in the SPINT register."]
pub type SPIF_R = crate::BitReader;
impl R {
    #[doc = "Bit 3 - Slave abort. When 1, this bit indicates that a slave abort has occurred. This bit is cleared by reading this register."]
    #[inline(always)]
    pub fn abrt(&self) -> ABRT_R {
        ABRT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mode fault. when 1, this bit indicates that a Mode fault error has occurred. This bit is cleared by reading this register, then writing the SPI0 control register."]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read overrun. When 1, this bit indicates that a read overrun has occurred. This bit is cleared by reading this register."]
    #[inline(always)]
    pub fn rovr(&self) -> ROVR_R {
        ROVR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write collision. When 1, this bit indicates that a write collision has occurred. This bit is cleared by reading this register, then accessing the SPI Data Register."]
    #[inline(always)]
    pub fn wcol(&self) -> WCOL_R {
        WCOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI transfer complete flag. When 1, this bit indicates when a SPI data transfer is complete. When a master, this bit is set at the end of the last cycle of the transfer. When a slave, this bit is set on the last data sampling edge of the SCK. This bit is cleared by first reading this register, then accessing the SPI Data Register. Note: this is not the SPI interrupt flag. This flag is found in the SPINT register."]
    #[inline(always)]
    pub fn spif(&self) -> SPIF_R {
        SPIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("abrt", &format_args!("{}", self.abrt().bit()))
            .field("modf", &format_args!("{}", self.modf().bit()))
            .field("rovr", &format_args!("{}", self.rovr().bit()))
            .field("wcol", &format_args!("{}", self.wcol().bit()))
            .field("spif", &format_args!("{}", self.spif().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<SR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "SPI Status Register. This register shows the status of the SPI.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
