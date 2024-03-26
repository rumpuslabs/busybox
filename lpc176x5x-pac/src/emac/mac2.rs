#[doc = "Register `MAC2` reader"]
pub type R = crate::R<MAC2_SPEC>;
#[doc = "Register `MAC2` writer"]
pub type W = crate::W<MAC2_SPEC>;
#[doc = "Field `FULLDUPLEX` reader - When enabled (set to 1), the MAC operates in Full-Duplex mode. When disabled, the MAC operates in Half-Duplex mode."]
pub type FULLDUPLEX_R = crate::BitReader;
#[doc = "Field `FULLDUPLEX` writer - When enabled (set to 1), the MAC operates in Full-Duplex mode. When disabled, the MAC operates in Half-Duplex mode."]
pub type FULLDUPLEX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLC` reader - FRAMELENGTH CHECKING. When enabled (set to 1), both transmit and receive frame lengths are compared to the Length/Type field. If the Length/Type field represents a length then the check is performed. Mismatches are reported in the StatusInfo word for each received frame."]
pub type FLC_R = crate::BitReader;
#[doc = "Field `FLC` writer - FRAMELENGTH CHECKING. When enabled (set to 1), both transmit and receive frame lengths are compared to the Length/Type field. If the Length/Type field represents a length then the check is performed. Mismatches are reported in the StatusInfo word for each received frame."]
pub type FLC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HFEN` reader - HUGE FRAME ENABLEWhen enabled (set to 1), frames of any length are transmitted and received."]
pub type HFEN_R = crate::BitReader;
#[doc = "Field `HFEN` writer - HUGE FRAME ENABLEWhen enabled (set to 1), frames of any length are transmitted and received."]
pub type HFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DELAYEDCRC` reader - DELAYED CRC. This bit determines the number of bytes, if any, of proprietary header information that exist on the front of IEEE 802.3 frames. When 1, four bytes of header (ignored by the CRC function) are added. When 0, there is no proprietary header."]
pub type DELAYEDCRC_R = crate::BitReader;
#[doc = "Field `DELAYEDCRC` writer - DELAYED CRC. This bit determines the number of bytes, if any, of proprietary header information that exist on the front of IEEE 802.3 frames. When 1, four bytes of header (ignored by the CRC function) are added. When 0, there is no proprietary header."]
pub type DELAYEDCRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRCEN` reader - CRC ENABLESet this bit to append a CRC to every frame whether padding was required or not. Must be set if PAD/CRC ENABLE is set. Clear this bit if frames presented to the MAC contain a CRC."]
pub type CRCEN_R = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRC ENABLESet this bit to append a CRC to every frame whether padding was required or not. Must be set if PAD/CRC ENABLE is set. Clear this bit if frames presented to the MAC contain a CRC."]
pub type CRCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PADCRCEN` reader - PAD CRC ENABLE. Set this bit to have the MAC pad all short frames. Clear this bit if frames presented to the MAC have a valid length. This bit is used in conjunction with AUTO PAD ENABLE and VLAN PAD ENABLE. See Table 153 - Pad Operation for details on the pad function."]
pub type PADCRCEN_R = crate::BitReader;
#[doc = "Field `PADCRCEN` writer - PAD CRC ENABLE. Set this bit to have the MAC pad all short frames. Clear this bit if frames presented to the MAC have a valid length. This bit is used in conjunction with AUTO PAD ENABLE and VLAN PAD ENABLE. See Table 153 - Pad Operation for details on the pad function."]
pub type PADCRCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VLANPADEN` reader - VLAN PAD ENABLE. Set this bit to cause the MAC to pad all short frames to 64 bytes and append a valid CRC. Consult Table 153 - Pad Operation for more information on the various padding features. Note: This bit is ignored if PAD / CRC ENABLE is cleared."]
pub type VLANPADEN_R = crate::BitReader;
#[doc = "Field `VLANPADEN` writer - VLAN PAD ENABLE. Set this bit to cause the MAC to pad all short frames to 64 bytes and append a valid CRC. Consult Table 153 - Pad Operation for more information on the various padding features. Note: This bit is ignored if PAD / CRC ENABLE is cleared."]
pub type VLANPADEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AUTODETPADEN` reader - AUTODETECTPAD ENABLE. Set this bit to cause the MAC to automatically detect the type of frame, either tagged or un-tagged, by comparing the two octets following the source address with 0x8100 (VLAN Protocol ID) and pad accordingly. Table 153 - Pad Operation provides a description of the pad function based on the configuration of this register. Note: This bit is ignored if PAD / CRC ENABLE is cleared."]
pub type AUTODETPADEN_R = crate::BitReader;
#[doc = "Field `AUTODETPADEN` writer - AUTODETECTPAD ENABLE. Set this bit to cause the MAC to automatically detect the type of frame, either tagged or un-tagged, by comparing the two octets following the source address with 0x8100 (VLAN Protocol ID) and pad accordingly. Table 153 - Pad Operation provides a description of the pad function based on the configuration of this register. Note: This bit is ignored if PAD / CRC ENABLE is cleared."]
pub type AUTODETPADEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PPENF` reader - PURE PREAMBLE ENFORCEMEN. When enabled (set to 1), the MAC will verify the content of the preamble to ensure it contains 0x55 and is error-free. A packet with an incorrect preamble is discarded. When disabled, no preamble checking is performed."]
pub type PPENF_R = crate::BitReader;
#[doc = "Field `PPENF` writer - PURE PREAMBLE ENFORCEMEN. When enabled (set to 1), the MAC will verify the content of the preamble to ensure it contains 0x55 and is error-free. A packet with an incorrect preamble is discarded. When disabled, no preamble checking is performed."]
pub type PPENF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPENF` reader - LONG PREAMBLE ENFORCEMENT. When enabled (set to 1), the MAC only allows receive packets which contain preamble fields less than 12 bytes in length. When disabled, the MAC allows any length preamble as per the Standard."]
pub type LPENF_R = crate::BitReader;
#[doc = "Field `LPENF` writer - LONG PREAMBLE ENFORCEMENT. When enabled (set to 1), the MAC only allows receive packets which contain preamble fields less than 12 bytes in length. When disabled, the MAC allows any length preamble as per the Standard."]
pub type LPENF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NOBACKOFF` reader - When enabled (set to 1), the MAC will immediately retransmit following a collision rather than using the Binary Exponential Backoff algorithm as specified in the Standard."]
pub type NOBACKOFF_R = crate::BitReader;
#[doc = "Field `NOBACKOFF` writer - When enabled (set to 1), the MAC will immediately retransmit following a collision rather than using the Binary Exponential Backoff algorithm as specified in the Standard."]
pub type NOBACKOFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BP_NOBACKOFF` reader - BACK PRESSURE / NO BACKOFF. When enabled (set to 1), after the MAC incidentally causes a collision during back pressure, it will immediately retransmit without backoff, reducing the chance of further collisions and ensuring transmit packets get sent."]
pub type BP_NOBACKOFF_R = crate::BitReader;
#[doc = "Field `BP_NOBACKOFF` writer - BACK PRESSURE / NO BACKOFF. When enabled (set to 1), after the MAC incidentally causes a collision during back pressure, it will immediately retransmit without backoff, reducing the chance of further collisions and ensuring transmit packets get sent."]
pub type BP_NOBACKOFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXCESSDEFER` reader - When enabled (set to 1) the MAC will defer to carrier indefinitely as per the Standard. When disabled, the MAC will abort when the excessive deferral limit is reached."]
pub type EXCESSDEFER_R = crate::BitReader;
#[doc = "Field `EXCESSDEFER` writer - When enabled (set to 1) the MAC will defer to carrier indefinitely as per the Standard. When disabled, the MAC will abort when the excessive deferral limit is reached."]
pub type EXCESSDEFER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - When enabled (set to 1), the MAC operates in Full-Duplex mode. When disabled, the MAC operates in Half-Duplex mode."]
    #[inline(always)]
    pub fn fullduplex(&self) -> FULLDUPLEX_R {
        FULLDUPLEX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FRAMELENGTH CHECKING. When enabled (set to 1), both transmit and receive frame lengths are compared to the Length/Type field. If the Length/Type field represents a length then the check is performed. Mismatches are reported in the StatusInfo word for each received frame."]
    #[inline(always)]
    pub fn flc(&self) -> FLC_R {
        FLC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HUGE FRAME ENABLEWhen enabled (set to 1), frames of any length are transmitted and received."]
    #[inline(always)]
    pub fn hfen(&self) -> HFEN_R {
        HFEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DELAYED CRC. This bit determines the number of bytes, if any, of proprietary header information that exist on the front of IEEE 802.3 frames. When 1, four bytes of header (ignored by the CRC function) are added. When 0, there is no proprietary header."]
    #[inline(always)]
    pub fn delayedcrc(&self) -> DELAYEDCRC_R {
        DELAYEDCRC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC ENABLESet this bit to append a CRC to every frame whether padding was required or not. Must be set if PAD/CRC ENABLE is set. Clear this bit if frames presented to the MAC contain a CRC."]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PAD CRC ENABLE. Set this bit to have the MAC pad all short frames. Clear this bit if frames presented to the MAC have a valid length. This bit is used in conjunction with AUTO PAD ENABLE and VLAN PAD ENABLE. See Table 153 - Pad Operation for details on the pad function."]
    #[inline(always)]
    pub fn padcrcen(&self) -> PADCRCEN_R {
        PADCRCEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VLAN PAD ENABLE. Set this bit to cause the MAC to pad all short frames to 64 bytes and append a valid CRC. Consult Table 153 - Pad Operation for more information on the various padding features. Note: This bit is ignored if PAD / CRC ENABLE is cleared."]
    #[inline(always)]
    pub fn vlanpaden(&self) -> VLANPADEN_R {
        VLANPADEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AUTODETECTPAD ENABLE. Set this bit to cause the MAC to automatically detect the type of frame, either tagged or un-tagged, by comparing the two octets following the source address with 0x8100 (VLAN Protocol ID) and pad accordingly. Table 153 - Pad Operation provides a description of the pad function based on the configuration of this register. Note: This bit is ignored if PAD / CRC ENABLE is cleared."]
    #[inline(always)]
    pub fn autodetpaden(&self) -> AUTODETPADEN_R {
        AUTODETPADEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PURE PREAMBLE ENFORCEMEN. When enabled (set to 1), the MAC will verify the content of the preamble to ensure it contains 0x55 and is error-free. A packet with an incorrect preamble is discarded. When disabled, no preamble checking is performed."]
    #[inline(always)]
    pub fn ppenf(&self) -> PPENF_R {
        PPENF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LONG PREAMBLE ENFORCEMENT. When enabled (set to 1), the MAC only allows receive packets which contain preamble fields less than 12 bytes in length. When disabled, the MAC allows any length preamble as per the Standard."]
    #[inline(always)]
    pub fn lpenf(&self) -> LPENF_R {
        LPENF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - When enabled (set to 1), the MAC will immediately retransmit following a collision rather than using the Binary Exponential Backoff algorithm as specified in the Standard."]
    #[inline(always)]
    pub fn nobackoff(&self) -> NOBACKOFF_R {
        NOBACKOFF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - BACK PRESSURE / NO BACKOFF. When enabled (set to 1), after the MAC incidentally causes a collision during back pressure, it will immediately retransmit without backoff, reducing the chance of further collisions and ensuring transmit packets get sent."]
    #[inline(always)]
    pub fn bp_nobackoff(&self) -> BP_NOBACKOFF_R {
        BP_NOBACKOFF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - When enabled (set to 1) the MAC will defer to carrier indefinitely as per the Standard. When disabled, the MAC will abort when the excessive deferral limit is reached."]
    #[inline(always)]
    pub fn excessdefer(&self) -> EXCESSDEFER_R {
        EXCESSDEFER_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAC2")
            .field("fullduplex", &format_args!("{}", self.fullduplex().bit()))
            .field("flc", &format_args!("{}", self.flc().bit()))
            .field("hfen", &format_args!("{}", self.hfen().bit()))
            .field("delayedcrc", &format_args!("{}", self.delayedcrc().bit()))
            .field("crcen", &format_args!("{}", self.crcen().bit()))
            .field("padcrcen", &format_args!("{}", self.padcrcen().bit()))
            .field("vlanpaden", &format_args!("{}", self.vlanpaden().bit()))
            .field(
                "autodetpaden",
                &format_args!("{}", self.autodetpaden().bit()),
            )
            .field("ppenf", &format_args!("{}", self.ppenf().bit()))
            .field("lpenf", &format_args!("{}", self.lpenf().bit()))
            .field("nobackoff", &format_args!("{}", self.nobackoff().bit()))
            .field(
                "bp_nobackoff",
                &format_args!("{}", self.bp_nobackoff().bit()),
            )
            .field("excessdefer", &format_args!("{}", self.excessdefer().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<MAC2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - When enabled (set to 1), the MAC operates in Full-Duplex mode. When disabled, the MAC operates in Half-Duplex mode."]
    #[inline(always)]
    #[must_use]
    pub fn fullduplex(&mut self) -> FULLDUPLEX_W<MAC2_SPEC, 0> {
        FULLDUPLEX_W::new(self)
    }
    #[doc = "Bit 1 - FRAMELENGTH CHECKING. When enabled (set to 1), both transmit and receive frame lengths are compared to the Length/Type field. If the Length/Type field represents a length then the check is performed. Mismatches are reported in the StatusInfo word for each received frame."]
    #[inline(always)]
    #[must_use]
    pub fn flc(&mut self) -> FLC_W<MAC2_SPEC, 1> {
        FLC_W::new(self)
    }
    #[doc = "Bit 2 - HUGE FRAME ENABLEWhen enabled (set to 1), frames of any length are transmitted and received."]
    #[inline(always)]
    #[must_use]
    pub fn hfen(&mut self) -> HFEN_W<MAC2_SPEC, 2> {
        HFEN_W::new(self)
    }
    #[doc = "Bit 3 - DELAYED CRC. This bit determines the number of bytes, if any, of proprietary header information that exist on the front of IEEE 802.3 frames. When 1, four bytes of header (ignored by the CRC function) are added. When 0, there is no proprietary header."]
    #[inline(always)]
    #[must_use]
    pub fn delayedcrc(&mut self) -> DELAYEDCRC_W<MAC2_SPEC, 3> {
        DELAYEDCRC_W::new(self)
    }
    #[doc = "Bit 4 - CRC ENABLESet this bit to append a CRC to every frame whether padding was required or not. Must be set if PAD/CRC ENABLE is set. Clear this bit if frames presented to the MAC contain a CRC."]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<MAC2_SPEC, 4> {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 5 - PAD CRC ENABLE. Set this bit to have the MAC pad all short frames. Clear this bit if frames presented to the MAC have a valid length. This bit is used in conjunction with AUTO PAD ENABLE and VLAN PAD ENABLE. See Table 153 - Pad Operation for details on the pad function."]
    #[inline(always)]
    #[must_use]
    pub fn padcrcen(&mut self) -> PADCRCEN_W<MAC2_SPEC, 5> {
        PADCRCEN_W::new(self)
    }
    #[doc = "Bit 6 - VLAN PAD ENABLE. Set this bit to cause the MAC to pad all short frames to 64 bytes and append a valid CRC. Consult Table 153 - Pad Operation for more information on the various padding features. Note: This bit is ignored if PAD / CRC ENABLE is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn vlanpaden(&mut self) -> VLANPADEN_W<MAC2_SPEC, 6> {
        VLANPADEN_W::new(self)
    }
    #[doc = "Bit 7 - AUTODETECTPAD ENABLE. Set this bit to cause the MAC to automatically detect the type of frame, either tagged or un-tagged, by comparing the two octets following the source address with 0x8100 (VLAN Protocol ID) and pad accordingly. Table 153 - Pad Operation provides a description of the pad function based on the configuration of this register. Note: This bit is ignored if PAD / CRC ENABLE is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn autodetpaden(&mut self) -> AUTODETPADEN_W<MAC2_SPEC, 7> {
        AUTODETPADEN_W::new(self)
    }
    #[doc = "Bit 8 - PURE PREAMBLE ENFORCEMEN. When enabled (set to 1), the MAC will verify the content of the preamble to ensure it contains 0x55 and is error-free. A packet with an incorrect preamble is discarded. When disabled, no preamble checking is performed."]
    #[inline(always)]
    #[must_use]
    pub fn ppenf(&mut self) -> PPENF_W<MAC2_SPEC, 8> {
        PPENF_W::new(self)
    }
    #[doc = "Bit 9 - LONG PREAMBLE ENFORCEMENT. When enabled (set to 1), the MAC only allows receive packets which contain preamble fields less than 12 bytes in length. When disabled, the MAC allows any length preamble as per the Standard."]
    #[inline(always)]
    #[must_use]
    pub fn lpenf(&mut self) -> LPENF_W<MAC2_SPEC, 9> {
        LPENF_W::new(self)
    }
    #[doc = "Bit 12 - When enabled (set to 1), the MAC will immediately retransmit following a collision rather than using the Binary Exponential Backoff algorithm as specified in the Standard."]
    #[inline(always)]
    #[must_use]
    pub fn nobackoff(&mut self) -> NOBACKOFF_W<MAC2_SPEC, 12> {
        NOBACKOFF_W::new(self)
    }
    #[doc = "Bit 13 - BACK PRESSURE / NO BACKOFF. When enabled (set to 1), after the MAC incidentally causes a collision during back pressure, it will immediately retransmit without backoff, reducing the chance of further collisions and ensuring transmit packets get sent."]
    #[inline(always)]
    #[must_use]
    pub fn bp_nobackoff(&mut self) -> BP_NOBACKOFF_W<MAC2_SPEC, 13> {
        BP_NOBACKOFF_W::new(self)
    }
    #[doc = "Bit 14 - When enabled (set to 1) the MAC will defer to carrier indefinitely as per the Standard. When disabled, the MAC will abort when the excessive deferral limit is reached."]
    #[inline(always)]
    #[must_use]
    pub fn excessdefer(&mut self) -> EXCESSDEFER_W<MAC2_SPEC, 14> {
        EXCESSDEFER_W::new(self)
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
#[doc = "MAC configuration register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC2_SPEC;
impl crate::RegisterSpec for MAC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac2::R`](R) reader structure"]
impl crate::Readable for MAC2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac2::W`](W) writer structure"]
impl crate::Writable for MAC2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC2 to value 0"]
impl crate::Resettable for MAC2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
