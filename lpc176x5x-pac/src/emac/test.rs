#[doc = "Register `TEST` reader"]
pub type R = crate::R<TEST_SPEC>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TEST_SPEC>;
#[doc = "Field `SCPQ` reader - SHORTCUT PAUSE QUANTA. This bit reduces the effective PAUSE quanta from 64 byte-times to 1 byte-time."]
pub type SCPQ_R = crate::BitReader;
#[doc = "Field `SCPQ` writer - SHORTCUT PAUSE QUANTA. This bit reduces the effective PAUSE quanta from 64 byte-times to 1 byte-time."]
pub type SCPQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TESTPAUSE` reader - This bit causes the MAC Control sublayer to inhibit transmissions, just as if a PAUSE Receive Control frame with a nonzero pause time parameter was received."]
pub type TESTPAUSE_R = crate::BitReader;
#[doc = "Field `TESTPAUSE` writer - This bit causes the MAC Control sublayer to inhibit transmissions, just as if a PAUSE Receive Control frame with a nonzero pause time parameter was received."]
pub type TESTPAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TESTBP` reader - TEST BACKPRESSURE. Setting this bit will cause the MAC to assert backpressure on the link. Backpressure causes preamble to be transmitted, raising carrier sense. A transmit packet from the system will be sent during backpressure."]
pub type TESTBP_R = crate::BitReader;
#[doc = "Field `TESTBP` writer - TEST BACKPRESSURE. Setting this bit will cause the MAC to assert backpressure on the link. Backpressure causes preamble to be transmitted, raising carrier sense. A transmit packet from the system will be sent during backpressure."]
pub type TESTBP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SHORTCUT PAUSE QUANTA. This bit reduces the effective PAUSE quanta from 64 byte-times to 1 byte-time."]
    #[inline(always)]
    pub fn scpq(&self) -> SCPQ_R {
        SCPQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit causes the MAC Control sublayer to inhibit transmissions, just as if a PAUSE Receive Control frame with a nonzero pause time parameter was received."]
    #[inline(always)]
    pub fn testpause(&self) -> TESTPAUSE_R {
        TESTPAUSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TEST BACKPRESSURE. Setting this bit will cause the MAC to assert backpressure on the link. Backpressure causes preamble to be transmitted, raising carrier sense. A transmit packet from the system will be sent during backpressure."]
    #[inline(always)]
    pub fn testbp(&self) -> TESTBP_R {
        TESTBP_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEST")
            .field("scpq", &format_args!("{}", self.scpq().bit()))
            .field("testpause", &format_args!("{}", self.testpause().bit()))
            .field("testbp", &format_args!("{}", self.testbp().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TEST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - SHORTCUT PAUSE QUANTA. This bit reduces the effective PAUSE quanta from 64 byte-times to 1 byte-time."]
    #[inline(always)]
    #[must_use]
    pub fn scpq(&mut self) -> SCPQ_W<TEST_SPEC, 0> {
        SCPQ_W::new(self)
    }
    #[doc = "Bit 1 - This bit causes the MAC Control sublayer to inhibit transmissions, just as if a PAUSE Receive Control frame with a nonzero pause time parameter was received."]
    #[inline(always)]
    #[must_use]
    pub fn testpause(&mut self) -> TESTPAUSE_W<TEST_SPEC, 1> {
        TESTPAUSE_W::new(self)
    }
    #[doc = "Bit 2 - TEST BACKPRESSURE. Setting this bit will cause the MAC to assert backpressure on the link. Backpressure causes preamble to be transmitted, raising carrier sense. A transmit packet from the system will be sent during backpressure."]
    #[inline(always)]
    #[must_use]
    pub fn testbp(&mut self) -> TESTBP_W<TEST_SPEC, 2> {
        TESTBP_W::new(self)
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
#[doc = "Test register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TEST_SPEC;
impl crate::RegisterSpec for TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TEST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TEST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
