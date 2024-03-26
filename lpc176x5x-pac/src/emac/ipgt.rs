#[doc = "Register `IPGT` reader"]
pub type R = crate::R<IPGT_SPEC>;
#[doc = "Register `IPGT` writer"]
pub type W = crate::W<IPGT_SPEC>;
#[doc = "Field `BTOBINTEGAP` reader - BACK-TO-BACK INTER-PACKET-GAP.This is a programmable field representing the nibble time offset of the minimum possible period between the end of any transmitted packet to the beginning of the next. In Full-Duplex mode, the register value should be the desired period in nibble times minus 3. In Half-Duplex mode, the register value should be the desired period in nibble times minus 6. In Full-Duplex the recommended setting is 0x15 (21d), which represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode). In Half-Duplex the recommended setting is 0x12 (18d), which also represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode)."]
pub type BTOBINTEGAP_R = crate::FieldReader;
#[doc = "Field `BTOBINTEGAP` writer - BACK-TO-BACK INTER-PACKET-GAP.This is a programmable field representing the nibble time offset of the minimum possible period between the end of any transmitted packet to the beginning of the next. In Full-Duplex mode, the register value should be the desired period in nibble times minus 3. In Half-Duplex mode, the register value should be the desired period in nibble times minus 6. In Full-Duplex the recommended setting is 0x15 (21d), which represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode). In Half-Duplex the recommended setting is 0x12 (18d), which also represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode)."]
pub type BTOBINTEGAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - BACK-TO-BACK INTER-PACKET-GAP.This is a programmable field representing the nibble time offset of the minimum possible period between the end of any transmitted packet to the beginning of the next. In Full-Duplex mode, the register value should be the desired period in nibble times minus 3. In Half-Duplex mode, the register value should be the desired period in nibble times minus 6. In Full-Duplex the recommended setting is 0x15 (21d), which represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode). In Half-Duplex the recommended setting is 0x12 (18d), which also represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode)."]
    #[inline(always)]
    pub fn btobintegap(&self) -> BTOBINTEGAP_R {
        BTOBINTEGAP_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPGT")
            .field(
                "btobintegap",
                &format_args!("{}", self.btobintegap().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IPGT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6 - BACK-TO-BACK INTER-PACKET-GAP.This is a programmable field representing the nibble time offset of the minimum possible period between the end of any transmitted packet to the beginning of the next. In Full-Duplex mode, the register value should be the desired period in nibble times minus 3. In Half-Duplex mode, the register value should be the desired period in nibble times minus 6. In Full-Duplex the recommended setting is 0x15 (21d), which represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode). In Half-Duplex the recommended setting is 0x12 (18d), which also represents the minimum IPG of 960 ns (in 100 Mbps mode) or 9.6 us (in 10 Mbps mode)."]
    #[inline(always)]
    #[must_use]
    pub fn btobintegap(&mut self) -> BTOBINTEGAP_W<IPGT_SPEC, 0> {
        BTOBINTEGAP_W::new(self)
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
#[doc = "Back-to-Back Inter-Packet-Gap register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipgt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipgt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPGT_SPEC;
impl crate::RegisterSpec for IPGT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipgt::R`](R) reader structure"]
impl crate::Readable for IPGT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ipgt::W`](W) writer structure"]
impl crate::Writable for IPGT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPGT to value 0"]
impl crate::Resettable for IPGT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
