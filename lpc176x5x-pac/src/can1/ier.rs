#[doc = "Register `IER` reader"]
pub type R = crate::R<IER_SPEC>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Field `RIE` reader - Receiver Interrupt Enable. When the Receive Buffer Status is 'full', the CAN Controller requests the respective interrupt."]
pub type RIE_R = crate::BitReader;
#[doc = "Field `RIE` writer - Receiver Interrupt Enable. When the Receive Buffer Status is 'full', the CAN Controller requests the respective interrupt."]
pub type RIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIE1` reader - Transmit Interrupt Enable for Buffer1. When a message has been successfully transmitted out of TXB1 or Transmit Buffer 1 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
pub type TIE1_R = crate::BitReader;
#[doc = "Field `TIE1` writer - Transmit Interrupt Enable for Buffer1. When a message has been successfully transmitted out of TXB1 or Transmit Buffer 1 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
pub type TIE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EIE` reader - Error Warning Interrupt Enable. If the Error or Bus Status change (see Status Register), the CAN Controller requests the respective interrupt."]
pub type EIE_R = crate::BitReader;
#[doc = "Field `EIE` writer - Error Warning Interrupt Enable. If the Error or Bus Status change (see Status Register), the CAN Controller requests the respective interrupt."]
pub type EIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DOIE` reader - Data Overrun Interrupt Enable. If the Data Overrun Status bit is set (see Status Register), the CAN Controller requests the respective interrupt."]
pub type DOIE_R = crate::BitReader;
#[doc = "Field `DOIE` writer - Data Overrun Interrupt Enable. If the Data Overrun Status bit is set (see Status Register), the CAN Controller requests the respective interrupt."]
pub type DOIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WUIE` reader - Wake-Up Interrupt Enable. If the sleeping CAN controller wakes up, the respective interrupt is requested."]
pub type WUIE_R = crate::BitReader;
#[doc = "Field `WUIE` writer - Wake-Up Interrupt Enable. If the sleeping CAN controller wakes up, the respective interrupt is requested."]
pub type WUIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPIE` reader - Error Passive Interrupt Enable. If the error status of the CAN Controller changes from error active to error passive or vice versa, the respective interrupt is requested."]
pub type EPIE_R = crate::BitReader;
#[doc = "Field `EPIE` writer - Error Passive Interrupt Enable. If the error status of the CAN Controller changes from error active to error passive or vice versa, the respective interrupt is requested."]
pub type EPIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ALIE` reader - Arbitration Lost Interrupt Enable. If the CAN Controller has lost arbitration, the respective interrupt is requested."]
pub type ALIE_R = crate::BitReader;
#[doc = "Field `ALIE` writer - Arbitration Lost Interrupt Enable. If the CAN Controller has lost arbitration, the respective interrupt is requested."]
pub type ALIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BEIE` reader - Bus Error Interrupt Enable. If a bus error has been detected, the CAN Controller requests the respective interrupt."]
pub type BEIE_R = crate::BitReader;
#[doc = "Field `BEIE` writer - Bus Error Interrupt Enable. If a bus error has been detected, the CAN Controller requests the respective interrupt."]
pub type BEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IDIE` reader - ID Ready Interrupt Enable. When a CAN identifier has been received, the CAN Controller requests the respective interrupt."]
pub type IDIE_R = crate::BitReader;
#[doc = "Field `IDIE` writer - ID Ready Interrupt Enable. When a CAN identifier has been received, the CAN Controller requests the respective interrupt."]
pub type IDIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIE2` reader - Transmit Interrupt Enable for Buffer2. When a message has been successfully transmitted out of TXB2 or Transmit Buffer 2 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
pub type TIE2_R = crate::BitReader;
#[doc = "Field `TIE2` writer - Transmit Interrupt Enable for Buffer2. When a message has been successfully transmitted out of TXB2 or Transmit Buffer 2 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
pub type TIE2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIE3` reader - Transmit Interrupt Enable for Buffer3. When a message has been successfully transmitted out of TXB3 or Transmit Buffer 3 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
pub type TIE3_R = crate::BitReader;
#[doc = "Field `TIE3` writer - Transmit Interrupt Enable for Buffer3. When a message has been successfully transmitted out of TXB3 or Transmit Buffer 3 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
pub type TIE3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Receiver Interrupt Enable. When the Receive Buffer Status is 'full', the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Interrupt Enable for Buffer1. When a message has been successfully transmitted out of TXB1 or Transmit Buffer 1 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn tie1(&self) -> TIE1_R {
        TIE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error Warning Interrupt Enable. If the Error or Bus Status change (see Status Register), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Overrun Interrupt Enable. If the Data Overrun Status bit is set (see Status Register), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn doie(&self) -> DOIE_R {
        DOIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake-Up Interrupt Enable. If the sleeping CAN controller wakes up, the respective interrupt is requested."]
    #[inline(always)]
    pub fn wuie(&self) -> WUIE_R {
        WUIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error Passive Interrupt Enable. If the error status of the CAN Controller changes from error active to error passive or vice versa, the respective interrupt is requested."]
    #[inline(always)]
    pub fn epie(&self) -> EPIE_R {
        EPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Arbitration Lost Interrupt Enable. If the CAN Controller has lost arbitration, the respective interrupt is requested."]
    #[inline(always)]
    pub fn alie(&self) -> ALIE_R {
        ALIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus Error Interrupt Enable. If a bus error has been detected, the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn beie(&self) -> BEIE_R {
        BEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ID Ready Interrupt Enable. When a CAN identifier has been received, the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn idie(&self) -> IDIE_R {
        IDIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit Interrupt Enable for Buffer2. When a message has been successfully transmitted out of TXB2 or Transmit Buffer 2 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn tie2(&self) -> TIE2_R {
        TIE2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit Interrupt Enable for Buffer3. When a message has been successfully transmitted out of TXB3 or Transmit Buffer 3 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn tie3(&self) -> TIE3_R {
        TIE3_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("rie", &format_args!("{}", self.rie().bit()))
            .field("tie1", &format_args!("{}", self.tie1().bit()))
            .field("eie", &format_args!("{}", self.eie().bit()))
            .field("doie", &format_args!("{}", self.doie().bit()))
            .field("wuie", &format_args!("{}", self.wuie().bit()))
            .field("epie", &format_args!("{}", self.epie().bit()))
            .field("alie", &format_args!("{}", self.alie().bit()))
            .field("beie", &format_args!("{}", self.beie().bit()))
            .field("idie", &format_args!("{}", self.idie().bit()))
            .field("tie2", &format_args!("{}", self.tie2().bit()))
            .field("tie3", &format_args!("{}", self.tie3().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Interrupt Enable. When the Receive Buffer Status is 'full', the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<IER_SPEC, 0> {
        RIE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Interrupt Enable for Buffer1. When a message has been successfully transmitted out of TXB1 or Transmit Buffer 1 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tie1(&mut self) -> TIE1_W<IER_SPEC, 1> {
        TIE1_W::new(self)
    }
    #[doc = "Bit 2 - Error Warning Interrupt Enable. If the Error or Bus Status change (see Status Register), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn eie(&mut self) -> EIE_W<IER_SPEC, 2> {
        EIE_W::new(self)
    }
    #[doc = "Bit 3 - Data Overrun Interrupt Enable. If the Data Overrun Status bit is set (see Status Register), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn doie(&mut self) -> DOIE_W<IER_SPEC, 3> {
        DOIE_W::new(self)
    }
    #[doc = "Bit 4 - Wake-Up Interrupt Enable. If the sleeping CAN controller wakes up, the respective interrupt is requested."]
    #[inline(always)]
    #[must_use]
    pub fn wuie(&mut self) -> WUIE_W<IER_SPEC, 4> {
        WUIE_W::new(self)
    }
    #[doc = "Bit 5 - Error Passive Interrupt Enable. If the error status of the CAN Controller changes from error active to error passive or vice versa, the respective interrupt is requested."]
    #[inline(always)]
    #[must_use]
    pub fn epie(&mut self) -> EPIE_W<IER_SPEC, 5> {
        EPIE_W::new(self)
    }
    #[doc = "Bit 6 - Arbitration Lost Interrupt Enable. If the CAN Controller has lost arbitration, the respective interrupt is requested."]
    #[inline(always)]
    #[must_use]
    pub fn alie(&mut self) -> ALIE_W<IER_SPEC, 6> {
        ALIE_W::new(self)
    }
    #[doc = "Bit 7 - Bus Error Interrupt Enable. If a bus error has been detected, the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn beie(&mut self) -> BEIE_W<IER_SPEC, 7> {
        BEIE_W::new(self)
    }
    #[doc = "Bit 8 - ID Ready Interrupt Enable. When a CAN identifier has been received, the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn idie(&mut self) -> IDIE_W<IER_SPEC, 8> {
        IDIE_W::new(self)
    }
    #[doc = "Bit 9 - Transmit Interrupt Enable for Buffer2. When a message has been successfully transmitted out of TXB2 or Transmit Buffer 2 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tie2(&mut self) -> TIE2_W<IER_SPEC, 9> {
        TIE2_W::new(self)
    }
    #[doc = "Bit 10 - Transmit Interrupt Enable for Buffer3. When a message has been successfully transmitted out of TXB3 or Transmit Buffer 3 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tie3(&mut self) -> TIE3_W<IER_SPEC, 10> {
        TIE3_W::new(self)
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
#[doc = "Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
