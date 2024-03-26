#[doc = "Register `INTCLEAR` writer"]
pub type W = crate::W<INTCLEAR_SPEC>;
#[doc = "Field `RXOVERRUNINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub type RXOVERRUNINTCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXERRORINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub type RXERRORINTCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXFINISHEDINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub type RXFINISHEDINTCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXDONEINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub type RXDONEINTCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXUNDERRUNINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub type TXUNDERRUNINTCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXERRORINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub type TXERRORINTCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXFINISHEDINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub type TXFINISHEDINTCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXDONEINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub type TXDONEINTCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFTINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub type SOFTINTCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAKEUPINTCLR` writer - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
pub type WAKEUPINTCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<INTCLEAR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn rxoverrunintclr(&mut self) -> RXOVERRUNINTCLR_W<INTCLEAR_SPEC, 0> {
        RXOVERRUNINTCLR_W::new(self)
    }
    #[doc = "Bit 1 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn rxerrorintclr(&mut self) -> RXERRORINTCLR_W<INTCLEAR_SPEC, 1> {
        RXERRORINTCLR_W::new(self)
    }
    #[doc = "Bit 2 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn rxfinishedintclr(&mut self) -> RXFINISHEDINTCLR_W<INTCLEAR_SPEC, 2> {
        RXFINISHEDINTCLR_W::new(self)
    }
    #[doc = "Bit 3 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn rxdoneintclr(&mut self) -> RXDONEINTCLR_W<INTCLEAR_SPEC, 3> {
        RXDONEINTCLR_W::new(self)
    }
    #[doc = "Bit 4 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn txunderrunintclr(&mut self) -> TXUNDERRUNINTCLR_W<INTCLEAR_SPEC, 4> {
        TXUNDERRUNINTCLR_W::new(self)
    }
    #[doc = "Bit 5 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn txerrorintclr(&mut self) -> TXERRORINTCLR_W<INTCLEAR_SPEC, 5> {
        TXERRORINTCLR_W::new(self)
    }
    #[doc = "Bit 6 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn txfinishedintclr(&mut self) -> TXFINISHEDINTCLR_W<INTCLEAR_SPEC, 6> {
        TXFINISHEDINTCLR_W::new(self)
    }
    #[doc = "Bit 7 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn txdoneintclr(&mut self) -> TXDONEINTCLR_W<INTCLEAR_SPEC, 7> {
        TXDONEINTCLR_W::new(self)
    }
    #[doc = "Bit 12 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn softintclr(&mut self) -> SOFTINTCLR_W<INTCLEAR_SPEC, 12> {
        SOFTINTCLR_W::new(self)
    }
    #[doc = "Bit 13 - Writing a 1 clears the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn wakeupintclr(&mut self) -> WAKEUPINTCLR_W<INTCLEAR_SPEC, 13> {
        WAKEUPINTCLR_W::new(self)
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
#[doc = "Interrupt clear register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intclear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTCLEAR_SPEC;
impl crate::RegisterSpec for INTCLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intclear::W`](W) writer structure"]
impl crate::Writable for INTCLEAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTCLEAR to value 0"]
impl crate::Resettable for INTCLEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
