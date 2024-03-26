#[doc = "Register `INTENABLE` reader"]
pub type R = crate::R<INTENABLE_SPEC>;
#[doc = "Register `INTENABLE` writer"]
pub type W = crate::W<INTENABLE_SPEC>;
#[doc = "Field `RXOVERRUNINTEN` reader - Enable for interrupt trigger on receive buffer overrun or descriptor underrun situations."]
pub type RXOVERRUNINTEN_R = crate::BitReader;
#[doc = "Field `RXOVERRUNINTEN` writer - Enable for interrupt trigger on receive buffer overrun or descriptor underrun situations."]
pub type RXOVERRUNINTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXERRORINTEN` reader - Enable for interrupt trigger on receive errors."]
pub type RXERRORINTEN_R = crate::BitReader;
#[doc = "Field `RXERRORINTEN` writer - Enable for interrupt trigger on receive errors."]
pub type RXERRORINTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXFINISHEDINTEN` reader - Enable for interrupt triggered when all receive descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
pub type RXFINISHEDINTEN_R = crate::BitReader;
#[doc = "Field `RXFINISHEDINTEN` writer - Enable for interrupt triggered when all receive descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
pub type RXFINISHEDINTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXDONEINTEN` reader - Enable for interrupt triggered when a receive descriptor has been processed while the Interrupt bit in the Control field of the descriptor was set."]
pub type RXDONEINTEN_R = crate::BitReader;
#[doc = "Field `RXDONEINTEN` writer - Enable for interrupt triggered when a receive descriptor has been processed while the Interrupt bit in the Control field of the descriptor was set."]
pub type RXDONEINTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXUNDERRUNINTEN` reader - Enable for interrupt trigger on transmit buffer or descriptor underrun situations."]
pub type TXUNDERRUNINTEN_R = crate::BitReader;
#[doc = "Field `TXUNDERRUNINTEN` writer - Enable for interrupt trigger on transmit buffer or descriptor underrun situations."]
pub type TXUNDERRUNINTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXERRORINTEN` reader - Enable for interrupt trigger on transmit errors."]
pub type TXERRORINTEN_R = crate::BitReader;
#[doc = "Field `TXERRORINTEN` writer - Enable for interrupt trigger on transmit errors."]
pub type TXERRORINTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXFINISHEDINTEN` reader - Enable for interrupt triggered when all transmit descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
pub type TXFINISHEDINTEN_R = crate::BitReader;
#[doc = "Field `TXFINISHEDINTEN` writer - Enable for interrupt triggered when all transmit descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
pub type TXFINISHEDINTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXDONEINTEN` reader - Enable for interrupt triggered when a descriptor has been transmitted while the Interrupt bit in the Control field of the descriptor was set."]
pub type TXDONEINTEN_R = crate::BitReader;
#[doc = "Field `TXDONEINTEN` writer - Enable for interrupt triggered when a descriptor has been transmitted while the Interrupt bit in the Control field of the descriptor was set."]
pub type TXDONEINTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFTINTEN` reader - Enable for interrupt triggered by the SoftInt bit in the IntStatus register, caused by software writing a 1 to the SoftIntSet bit in the IntSet register."]
pub type SOFTINTEN_R = crate::BitReader;
#[doc = "Field `SOFTINTEN` writer - Enable for interrupt triggered by the SoftInt bit in the IntStatus register, caused by software writing a 1 to the SoftIntSet bit in the IntSet register."]
pub type SOFTINTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAKEUPINTEN` reader - Enable for interrupt triggered by a Wake-up event detected by the receive filter."]
pub type WAKEUPINTEN_R = crate::BitReader;
#[doc = "Field `WAKEUPINTEN` writer - Enable for interrupt triggered by a Wake-up event detected by the receive filter."]
pub type WAKEUPINTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Enable for interrupt trigger on receive buffer overrun or descriptor underrun situations."]
    #[inline(always)]
    pub fn rxoverruninten(&self) -> RXOVERRUNINTEN_R {
        RXOVERRUNINTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable for interrupt trigger on receive errors."]
    #[inline(always)]
    pub fn rxerrorinten(&self) -> RXERRORINTEN_R {
        RXERRORINTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable for interrupt triggered when all receive descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    pub fn rxfinishedinten(&self) -> RXFINISHEDINTEN_R {
        RXFINISHEDINTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable for interrupt triggered when a receive descriptor has been processed while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    pub fn rxdoneinten(&self) -> RXDONEINTEN_R {
        RXDONEINTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable for interrupt trigger on transmit buffer or descriptor underrun situations."]
    #[inline(always)]
    pub fn txunderruninten(&self) -> TXUNDERRUNINTEN_R {
        TXUNDERRUNINTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable for interrupt trigger on transmit errors."]
    #[inline(always)]
    pub fn txerrorinten(&self) -> TXERRORINTEN_R {
        TXERRORINTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable for interrupt triggered when all transmit descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    pub fn txfinishedinten(&self) -> TXFINISHEDINTEN_R {
        TXFINISHEDINTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable for interrupt triggered when a descriptor has been transmitted while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    pub fn txdoneinten(&self) -> TXDONEINTEN_R {
        TXDONEINTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable for interrupt triggered by the SoftInt bit in the IntStatus register, caused by software writing a 1 to the SoftIntSet bit in the IntSet register."]
    #[inline(always)]
    pub fn softinten(&self) -> SOFTINTEN_R {
        SOFTINTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable for interrupt triggered by a Wake-up event detected by the receive filter."]
    #[inline(always)]
    pub fn wakeupinten(&self) -> WAKEUPINTEN_R {
        WAKEUPINTEN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTENABLE")
            .field(
                "rxoverruninten",
                &format_args!("{}", self.rxoverruninten().bit()),
            )
            .field(
                "rxerrorinten",
                &format_args!("{}", self.rxerrorinten().bit()),
            )
            .field(
                "rxfinishedinten",
                &format_args!("{}", self.rxfinishedinten().bit()),
            )
            .field("rxdoneinten", &format_args!("{}", self.rxdoneinten().bit()))
            .field(
                "txunderruninten",
                &format_args!("{}", self.txunderruninten().bit()),
            )
            .field(
                "txerrorinten",
                &format_args!("{}", self.txerrorinten().bit()),
            )
            .field(
                "txfinishedinten",
                &format_args!("{}", self.txfinishedinten().bit()),
            )
            .field("txdoneinten", &format_args!("{}", self.txdoneinten().bit()))
            .field("softinten", &format_args!("{}", self.softinten().bit()))
            .field("wakeupinten", &format_args!("{}", self.wakeupinten().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<INTENABLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Enable for interrupt trigger on receive buffer overrun or descriptor underrun situations."]
    #[inline(always)]
    #[must_use]
    pub fn rxoverruninten(&mut self) -> RXOVERRUNINTEN_W<INTENABLE_SPEC, 0> {
        RXOVERRUNINTEN_W::new(self)
    }
    #[doc = "Bit 1 - Enable for interrupt trigger on receive errors."]
    #[inline(always)]
    #[must_use]
    pub fn rxerrorinten(&mut self) -> RXERRORINTEN_W<INTENABLE_SPEC, 1> {
        RXERRORINTEN_W::new(self)
    }
    #[doc = "Bit 2 - Enable for interrupt triggered when all receive descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    #[must_use]
    pub fn rxfinishedinten(&mut self) -> RXFINISHEDINTEN_W<INTENABLE_SPEC, 2> {
        RXFINISHEDINTEN_W::new(self)
    }
    #[doc = "Bit 3 - Enable for interrupt triggered when a receive descriptor has been processed while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    #[must_use]
    pub fn rxdoneinten(&mut self) -> RXDONEINTEN_W<INTENABLE_SPEC, 3> {
        RXDONEINTEN_W::new(self)
    }
    #[doc = "Bit 4 - Enable for interrupt trigger on transmit buffer or descriptor underrun situations."]
    #[inline(always)]
    #[must_use]
    pub fn txunderruninten(&mut self) -> TXUNDERRUNINTEN_W<INTENABLE_SPEC, 4> {
        TXUNDERRUNINTEN_W::new(self)
    }
    #[doc = "Bit 5 - Enable for interrupt trigger on transmit errors."]
    #[inline(always)]
    #[must_use]
    pub fn txerrorinten(&mut self) -> TXERRORINTEN_W<INTENABLE_SPEC, 5> {
        TXERRORINTEN_W::new(self)
    }
    #[doc = "Bit 6 - Enable for interrupt triggered when all transmit descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    #[must_use]
    pub fn txfinishedinten(&mut self) -> TXFINISHEDINTEN_W<INTENABLE_SPEC, 6> {
        TXFINISHEDINTEN_W::new(self)
    }
    #[doc = "Bit 7 - Enable for interrupt triggered when a descriptor has been transmitted while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    #[must_use]
    pub fn txdoneinten(&mut self) -> TXDONEINTEN_W<INTENABLE_SPEC, 7> {
        TXDONEINTEN_W::new(self)
    }
    #[doc = "Bit 12 - Enable for interrupt triggered by the SoftInt bit in the IntStatus register, caused by software writing a 1 to the SoftIntSet bit in the IntSet register."]
    #[inline(always)]
    #[must_use]
    pub fn softinten(&mut self) -> SOFTINTEN_W<INTENABLE_SPEC, 12> {
        SOFTINTEN_W::new(self)
    }
    #[doc = "Bit 13 - Enable for interrupt triggered by a Wake-up event detected by the receive filter."]
    #[inline(always)]
    #[must_use]
    pub fn wakeupinten(&mut self) -> WAKEUPINTEN_W<INTENABLE_SPEC, 13> {
        WAKEUPINTEN_W::new(self)
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
#[doc = "Interrupt enable register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTENABLE_SPEC;
impl crate::RegisterSpec for INTENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenable::R`](R) reader structure"]
impl crate::Readable for INTENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intenable::W`](W) writer structure"]
impl crate::Writable for INTENABLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENABLE to value 0"]
impl crate::Resettable for INTENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
