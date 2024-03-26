#[doc = "Register `RXPLEN` reader"]
pub type R = crate::R<RXPLEN_SPEC>;
#[doc = "Field `PKT_LNGTH` reader - The remaining number of bytes to be read from the currently selected endpoint's buffer. When this field decrements to 0, the RxENDPKT bit will be set in USBDevIntSt."]
pub type PKT_LNGTH_R = crate::FieldReader<u16>;
#[doc = "Field `DV` reader - Data valid. This bit is useful for isochronous endpoints. Non-isochronous endpoints do not raise an interrupt when an erroneous data packet is received. But invalid data packet can be produced with a bus reset. For isochronous endpoints, data transfer will happen even if an erroneous packet is received. In this case DV bit will not be set for the packet."]
pub type DV_R = crate::BitReader<DV_A>;
#[doc = "Data valid. This bit is useful for isochronous endpoints. Non-isochronous endpoints do not raise an interrupt when an erroneous data packet is received. But invalid data packet can be produced with a bus reset. For isochronous endpoints, data transfer will happen even if an erroneous packet is received. In this case DV bit will not be set for the packet.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DV_A {
    #[doc = "0: Data is invalid."]
    INVALID = 0,
    #[doc = "1: Data is valid."]
    VALID = 1,
}
impl From<DV_A> for bool {
    #[inline(always)]
    fn from(variant: DV_A) -> Self {
        variant as u8 != 0
    }
}
impl DV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DV_A {
        match self.bits {
            false => DV_A::INVALID,
            true => DV_A::VALID,
        }
    }
    #[doc = "Data is invalid."]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == DV_A::INVALID
    }
    #[doc = "Data is valid."]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == DV_A::VALID
    }
}
#[doc = "Field `PKT_RDY` reader - The PKT_LNGTH field is valid and the packet is ready for reading."]
pub type PKT_RDY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:9 - The remaining number of bytes to be read from the currently selected endpoint's buffer. When this field decrements to 0, the RxENDPKT bit will be set in USBDevIntSt."]
    #[inline(always)]
    pub fn pkt_lngth(&self) -> PKT_LNGTH_R {
        PKT_LNGTH_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Data valid. This bit is useful for isochronous endpoints. Non-isochronous endpoints do not raise an interrupt when an erroneous data packet is received. But invalid data packet can be produced with a bus reset. For isochronous endpoints, data transfer will happen even if an erroneous packet is received. In this case DV bit will not be set for the packet."]
    #[inline(always)]
    pub fn dv(&self) -> DV_R {
        DV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The PKT_LNGTH field is valid and the packet is ready for reading."]
    #[inline(always)]
    pub fn pkt_rdy(&self) -> PKT_RDY_R {
        PKT_RDY_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXPLEN")
            .field("pkt_lngth", &format_args!("{}", self.pkt_lngth().bits()))
            .field("dv", &format_args!("{}", self.dv().bit()))
            .field("pkt_rdy", &format_args!("{}", self.pkt_rdy().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RXPLEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "USB Receive Packet Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxplen::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXPLEN_SPEC;
impl crate::RegisterSpec for RXPLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxplen::R`](R) reader structure"]
impl crate::Readable for RXPLEN_SPEC {}
#[doc = "`reset()` method sets RXPLEN to value 0"]
impl crate::Resettable for RXPLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
