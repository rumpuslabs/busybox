#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Field `DONE0` reader - This bit mirrors the DONE status flag from the result register for A/D channel 0."]
pub type DONE0_R = crate::BitReader;
#[doc = "Field `DONE1` reader - This bit mirrors the DONE status flag from the result register for A/D channel 1."]
pub type DONE1_R = crate::BitReader;
#[doc = "Field `DONE2` reader - This bit mirrors the DONE status flag from the result register for A/D channel 2."]
pub type DONE2_R = crate::BitReader;
#[doc = "Field `DONE3` reader - This bit mirrors the DONE status flag from the result register for A/D channel 3."]
pub type DONE3_R = crate::BitReader;
#[doc = "Field `DONE4` reader - This bit mirrors the DONE status flag from the result register for A/D channel 4."]
pub type DONE4_R = crate::BitReader;
#[doc = "Field `DONE5` reader - This bit mirrors the DONE status flag from the result register for A/D channel 5."]
pub type DONE5_R = crate::BitReader;
#[doc = "Field `DONE6` reader - This bit mirrors the DONE status flag from the result register for A/D channel 6."]
pub type DONE6_R = crate::BitReader;
#[doc = "Field `DONE7` reader - This bit mirrors the DONE status flag from the result register for A/D channel 7."]
pub type DONE7_R = crate::BitReader;
#[doc = "Field `OVERRUN0` reader - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 0."]
pub type OVERRUN0_R = crate::BitReader;
#[doc = "Field `OVERRUN1` reader - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 1."]
pub type OVERRUN1_R = crate::BitReader;
#[doc = "Field `OVERRUN2` reader - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 2."]
pub type OVERRUN2_R = crate::BitReader;
#[doc = "Field `OVERRUN3` reader - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 3."]
pub type OVERRUN3_R = crate::BitReader;
#[doc = "Field `OVERRUN4` reader - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 4."]
pub type OVERRUN4_R = crate::BitReader;
#[doc = "Field `OVERRUN5` reader - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 5."]
pub type OVERRUN5_R = crate::BitReader;
#[doc = "Field `OVERRUN6` reader - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 6."]
pub type OVERRUN6_R = crate::BitReader;
#[doc = "Field `OVERRUN7` reader - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 7."]
pub type OVERRUN7_R = crate::BitReader;
#[doc = "Field `ADINT` reader - This bit is the A/D interrupt flag. It is one when any of the individual A/D channel Done flags is asserted and enabled to contribute to the A/D interrupt via the ADINTEN register."]
pub type ADINT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit mirrors the DONE status flag from the result register for A/D channel 0."]
    #[inline(always)]
    pub fn done0(&self) -> DONE0_R {
        DONE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit mirrors the DONE status flag from the result register for A/D channel 1."]
    #[inline(always)]
    pub fn done1(&self) -> DONE1_R {
        DONE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit mirrors the DONE status flag from the result register for A/D channel 2."]
    #[inline(always)]
    pub fn done2(&self) -> DONE2_R {
        DONE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit mirrors the DONE status flag from the result register for A/D channel 3."]
    #[inline(always)]
    pub fn done3(&self) -> DONE3_R {
        DONE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit mirrors the DONE status flag from the result register for A/D channel 4."]
    #[inline(always)]
    pub fn done4(&self) -> DONE4_R {
        DONE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit mirrors the DONE status flag from the result register for A/D channel 5."]
    #[inline(always)]
    pub fn done5(&self) -> DONE5_R {
        DONE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit mirrors the DONE status flag from the result register for A/D channel 6."]
    #[inline(always)]
    pub fn done6(&self) -> DONE6_R {
        DONE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit mirrors the DONE status flag from the result register for A/D channel 7."]
    #[inline(always)]
    pub fn done7(&self) -> DONE7_R {
        DONE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 0."]
    #[inline(always)]
    pub fn overrun0(&self) -> OVERRUN0_R {
        OVERRUN0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 1."]
    #[inline(always)]
    pub fn overrun1(&self) -> OVERRUN1_R {
        OVERRUN1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 2."]
    #[inline(always)]
    pub fn overrun2(&self) -> OVERRUN2_R {
        OVERRUN2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 3."]
    #[inline(always)]
    pub fn overrun3(&self) -> OVERRUN3_R {
        OVERRUN3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 4."]
    #[inline(always)]
    pub fn overrun4(&self) -> OVERRUN4_R {
        OVERRUN4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 5."]
    #[inline(always)]
    pub fn overrun5(&self) -> OVERRUN5_R {
        OVERRUN5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 6."]
    #[inline(always)]
    pub fn overrun6(&self) -> OVERRUN6_R {
        OVERRUN6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 7."]
    #[inline(always)]
    pub fn overrun7(&self) -> OVERRUN7_R {
        OVERRUN7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit is the A/D interrupt flag. It is one when any of the individual A/D channel Done flags is asserted and enabled to contribute to the A/D interrupt via the ADINTEN register."]
    #[inline(always)]
    pub fn adint(&self) -> ADINT_R {
        ADINT_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT")
            .field("done0", &format_args!("{}", self.done0().bit()))
            .field("done1", &format_args!("{}", self.done1().bit()))
            .field("done2", &format_args!("{}", self.done2().bit()))
            .field("done3", &format_args!("{}", self.done3().bit()))
            .field("done4", &format_args!("{}", self.done4().bit()))
            .field("done5", &format_args!("{}", self.done5().bit()))
            .field("done6", &format_args!("{}", self.done6().bit()))
            .field("done7", &format_args!("{}", self.done7().bit()))
            .field("overrun0", &format_args!("{}", self.overrun0().bit()))
            .field("overrun1", &format_args!("{}", self.overrun1().bit()))
            .field("overrun2", &format_args!("{}", self.overrun2().bit()))
            .field("overrun3", &format_args!("{}", self.overrun3().bit()))
            .field("overrun4", &format_args!("{}", self.overrun4().bit()))
            .field("overrun5", &format_args!("{}", self.overrun5().bit()))
            .field("overrun6", &format_args!("{}", self.overrun6().bit()))
            .field("overrun7", &format_args!("{}", self.overrun7().bit()))
            .field("adint", &format_args!("{}", self.adint().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<STAT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "A/D Status Register. This register contains DONE and OVERRUN flags for all of the A/D channels, as well as the A/D interrupt/DMA flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
