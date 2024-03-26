#[doc = "Register `IPGR` reader"]
pub type R = crate::R<IPGR_SPEC>;
#[doc = "Register `IPGR` writer"]
pub type W = crate::W<IPGR_SPEC>;
#[doc = "Field `NBTOBINTEGAP2` reader - NON-BACK-TO-BACK INTER-PACKET-GAP PART2. This is a programmable field representing the Non-Back-to-Back Inter-Packet-Gap. The recommended value is 0x12 (18d), which represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode)."]
pub type NBTOBINTEGAP2_R = crate::FieldReader;
#[doc = "Field `NBTOBINTEGAP2` writer - NON-BACK-TO-BACK INTER-PACKET-GAP PART2. This is a programmable field representing the Non-Back-to-Back Inter-Packet-Gap. The recommended value is 0x12 (18d), which represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode)."]
pub type NBTOBINTEGAP2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `NBTOBINTEGAP1` reader - NON-BACK-TO-BACK INTER-PACKET-GAP PART1. This is a programmable field representing the optional carrierSense window referenced in IEEE 802.3/4.2.3.2.1 'Carrier Deference'. If carrier is detected during the timing of IPGR1, the MAC defers to carrier. If, however, carrier becomes active after IPGR1, the MAC continues timing IPGR2 and transmits, knowingly causing a collision, thus ensuring fair access to medium. Its range of values is 0x0 to IPGR2. The recommended value is 0xC (12d)"]
pub type NBTOBINTEGAP1_R = crate::FieldReader;
#[doc = "Field `NBTOBINTEGAP1` writer - NON-BACK-TO-BACK INTER-PACKET-GAP PART1. This is a programmable field representing the optional carrierSense window referenced in IEEE 802.3/4.2.3.2.1 'Carrier Deference'. If carrier is detected during the timing of IPGR1, the MAC defers to carrier. If, however, carrier becomes active after IPGR1, the MAC continues timing IPGR2 and transmits, knowingly causing a collision, thus ensuring fair access to medium. Its range of values is 0x0 to IPGR2. The recommended value is 0xC (12d)"]
pub type NBTOBINTEGAP1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - NON-BACK-TO-BACK INTER-PACKET-GAP PART2. This is a programmable field representing the Non-Back-to-Back Inter-Packet-Gap. The recommended value is 0x12 (18d), which represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode)."]
    #[inline(always)]
    pub fn nbtobintegap2(&self) -> NBTOBINTEGAP2_R {
        NBTOBINTEGAP2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - NON-BACK-TO-BACK INTER-PACKET-GAP PART1. This is a programmable field representing the optional carrierSense window referenced in IEEE 802.3/4.2.3.2.1 'Carrier Deference'. If carrier is detected during the timing of IPGR1, the MAC defers to carrier. If, however, carrier becomes active after IPGR1, the MAC continues timing IPGR2 and transmits, knowingly causing a collision, thus ensuring fair access to medium. Its range of values is 0x0 to IPGR2. The recommended value is 0xC (12d)"]
    #[inline(always)]
    pub fn nbtobintegap1(&self) -> NBTOBINTEGAP1_R {
        NBTOBINTEGAP1_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPGR")
            .field(
                "nbtobintegap2",
                &format_args!("{}", self.nbtobintegap2().bits()),
            )
            .field(
                "nbtobintegap1",
                &format_args!("{}", self.nbtobintegap1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IPGR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6 - NON-BACK-TO-BACK INTER-PACKET-GAP PART2. This is a programmable field representing the Non-Back-to-Back Inter-Packet-Gap. The recommended value is 0x12 (18d), which represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode)."]
    #[inline(always)]
    #[must_use]
    pub fn nbtobintegap2(&mut self) -> NBTOBINTEGAP2_W<IPGR_SPEC, 0> {
        NBTOBINTEGAP2_W::new(self)
    }
    #[doc = "Bits 8:14 - NON-BACK-TO-BACK INTER-PACKET-GAP PART1. This is a programmable field representing the optional carrierSense window referenced in IEEE 802.3/4.2.3.2.1 'Carrier Deference'. If carrier is detected during the timing of IPGR1, the MAC defers to carrier. If, however, carrier becomes active after IPGR1, the MAC continues timing IPGR2 and transmits, knowingly causing a collision, thus ensuring fair access to medium. Its range of values is 0x0 to IPGR2. The recommended value is 0xC (12d)"]
    #[inline(always)]
    #[must_use]
    pub fn nbtobintegap1(&mut self) -> NBTOBINTEGAP1_W<IPGR_SPEC, 8> {
        NBTOBINTEGAP1_W::new(self)
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
#[doc = "Non Back-to-Back Inter-Packet-Gap register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPGR_SPEC;
impl crate::RegisterSpec for IPGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipgr::R`](R) reader structure"]
impl crate::Readable for IPGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ipgr::W`](W) writer structure"]
impl crate::Writable for IPGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPGR to value 0"]
impl crate::Resettable for IPGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
