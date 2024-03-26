#[doc = "Register `CANWAKEFLAGS` reader"]
pub type R = crate::R<CANWAKEFLAGS_SPEC>;
#[doc = "Register `CANWAKEFLAGS` writer"]
pub type W = crate::W<CANWAKEFLAGS_SPEC>;
#[doc = "Field `CAN1WAKE` reader - Wake-up status for CAN channel 1. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 1. Write: writing a 1 clears this bit."]
pub type CAN1WAKE_R = crate::BitReader;
#[doc = "Field `CAN1WAKE` writer - Wake-up status for CAN channel 1. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 1. Write: writing a 1 clears this bit."]
pub type CAN1WAKE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN2WAKE` reader - Wake-up status for CAN channel 2. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 2. Write: writing a 1 clears this bit."]
pub type CAN2WAKE_R = crate::BitReader;
#[doc = "Field `CAN2WAKE` writer - Wake-up status for CAN channel 2. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 2. Write: writing a 1 clears this bit."]
pub type CAN2WAKE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Wake-up status for CAN channel 1. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 1. Write: writing a 1 clears this bit."]
    #[inline(always)]
    pub fn can1wake(&self) -> CAN1WAKE_R {
        CAN1WAKE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake-up status for CAN channel 2. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 2. Write: writing a 1 clears this bit."]
    #[inline(always)]
    pub fn can2wake(&self) -> CAN2WAKE_R {
        CAN2WAKE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CANWAKEFLAGS")
            .field("can1wake", &format_args!("{}", self.can1wake().bit()))
            .field("can2wake", &format_args!("{}", self.can2wake().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CANWAKEFLAGS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1 - Wake-up status for CAN channel 1. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 1. Write: writing a 1 clears this bit."]
    #[inline(always)]
    #[must_use]
    pub fn can1wake(&mut self) -> CAN1WAKE_W<CANWAKEFLAGS_SPEC, 1> {
        CAN1WAKE_W::new(self)
    }
    #[doc = "Bit 2 - Wake-up status for CAN channel 2. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 2. Write: writing a 1 clears this bit."]
    #[inline(always)]
    #[must_use]
    pub fn can2wake(&mut self) -> CAN2WAKE_W<CANWAKEFLAGS_SPEC, 2> {
        CAN2WAKE_W::new(self)
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
#[doc = "Allows reading the wake-up state of the CAN channels.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`canwakeflags::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`canwakeflags::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CANWAKEFLAGS_SPEC;
impl crate::RegisterSpec for CANWAKEFLAGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`canwakeflags::R`](R) reader structure"]
impl crate::Readable for CANWAKEFLAGS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`canwakeflags::W`](W) writer structure"]
impl crate::Writable for CANWAKEFLAGS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CANWAKEFLAGS to value 0"]
impl crate::Resettable for CANWAKEFLAGS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
