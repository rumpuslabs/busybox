#[doc = "Register `INTSET` writer"]
pub type W = crate::W<INTSET_SPEC>;
#[doc = "Field `RXOVERRUNINTSET` writer - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
pub type RXOVERRUNINTSET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXERRORINTSET` writer - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
pub type RXERRORINTSET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXFINISHEDINTSET` writer - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
pub type RXFINISHEDINTSET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXDONEINTSET` writer - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
pub type RXDONEINTSET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXUNDERRUNINTSET` writer - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
pub type TXUNDERRUNINTSET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXERRORINTSET` writer - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
pub type TXERRORINTSET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXFINISHEDINTSET` writer - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
pub type TXFINISHEDINTSET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXDONEINTSET` writer - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
pub type TXDONEINTSET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFTINTSET` writer - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
pub type SOFTINTSET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAKEUPINTSET` writer - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
pub type WAKEUPINTSET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<INTSET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn rxoverrunintset(&mut self) -> RXOVERRUNINTSET_W<INTSET_SPEC, 0> {
        RXOVERRUNINTSET_W::new(self)
    }
    #[doc = "Bit 1 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn rxerrorintset(&mut self) -> RXERRORINTSET_W<INTSET_SPEC, 1> {
        RXERRORINTSET_W::new(self)
    }
    #[doc = "Bit 2 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn rxfinishedintset(&mut self) -> RXFINISHEDINTSET_W<INTSET_SPEC, 2> {
        RXFINISHEDINTSET_W::new(self)
    }
    #[doc = "Bit 3 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn rxdoneintset(&mut self) -> RXDONEINTSET_W<INTSET_SPEC, 3> {
        RXDONEINTSET_W::new(self)
    }
    #[doc = "Bit 4 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn txunderrunintset(&mut self) -> TXUNDERRUNINTSET_W<INTSET_SPEC, 4> {
        TXUNDERRUNINTSET_W::new(self)
    }
    #[doc = "Bit 5 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn txerrorintset(&mut self) -> TXERRORINTSET_W<INTSET_SPEC, 5> {
        TXERRORINTSET_W::new(self)
    }
    #[doc = "Bit 6 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn txfinishedintset(&mut self) -> TXFINISHEDINTSET_W<INTSET_SPEC, 6> {
        TXFINISHEDINTSET_W::new(self)
    }
    #[doc = "Bit 7 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn txdoneintset(&mut self) -> TXDONEINTSET_W<INTSET_SPEC, 7> {
        TXDONEINTSET_W::new(self)
    }
    #[doc = "Bit 12 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn softintset(&mut self) -> SOFTINTSET_W<INTSET_SPEC, 12> {
        SOFTINTSET_W::new(self)
    }
    #[doc = "Bit 13 - Writing a 1 to one sets the corresponding status bit in interrupt status register IntStatus."]
    #[inline(always)]
    #[must_use]
    pub fn wakeupintset(&mut self) -> WAKEUPINTSET_W<INTSET_SPEC, 13> {
        WAKEUPINTSET_W::new(self)
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
#[doc = "Interrupt set register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTSET_SPEC;
impl crate::RegisterSpec for INTSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intset::W`](W) writer structure"]
impl crate::Writable for INTSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTSET to value 0"]
impl crate::Resettable for INTSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
