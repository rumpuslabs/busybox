#[doc = "Register `PLL1STAT` reader"]
pub type R = crate::R<PLL1STAT_SPEC>;
#[doc = "Field `MSEL1` reader - Read-back for the PLL1 Multiplier value. This is the value currently used by PLL1."]
pub type MSEL1_R = crate::FieldReader;
#[doc = "Field `PSEL1` reader - Read-back for the PLL1 Divider value. This is the value currently used by PLL1."]
pub type PSEL1_R = crate::FieldReader;
#[doc = "Field `PLLE1_STAT` reader - Read-back for the PLL1 Enable bit. When one, PLL1 is currently activated. When zero, PLL1 is turned off. This bit is automatically cleared when Power-down mode is activated."]
pub type PLLE1_STAT_R = crate::BitReader;
#[doc = "Field `PLLC1_STAT` reader - Read-back for the PLL1 Connect bit. When PLLC and PLLE are both one, PLL1 is connected as the clock source for the microcontroller. When either PLLC or PLLE is zero, PLL1 is bypassed and the oscillator clock is used directly by the microcontroller. This bit is automatically cleared when Power-down mode is activated."]
pub type PLLC1_STAT_R = crate::BitReader;
#[doc = "Field `PLOCK1` reader - Reflects the PLL1 Lock status. When zero, PLL1 is not locked. When one, PLL1 is locked onto the requested frequency."]
pub type PLOCK1_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - Read-back for the PLL1 Multiplier value. This is the value currently used by PLL1."]
    #[inline(always)]
    pub fn msel1(&self) -> MSEL1_R {
        MSEL1_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Read-back for the PLL1 Divider value. This is the value currently used by PLL1."]
    #[inline(always)]
    pub fn psel1(&self) -> PSEL1_R {
        PSEL1_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - Read-back for the PLL1 Enable bit. When one, PLL1 is currently activated. When zero, PLL1 is turned off. This bit is automatically cleared when Power-down mode is activated."]
    #[inline(always)]
    pub fn plle1_stat(&self) -> PLLE1_STAT_R {
        PLLE1_STAT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read-back for the PLL1 Connect bit. When PLLC and PLLE are both one, PLL1 is connected as the clock source for the microcontroller. When either PLLC or PLLE is zero, PLL1 is bypassed and the oscillator clock is used directly by the microcontroller. This bit is automatically cleared when Power-down mode is activated."]
    #[inline(always)]
    pub fn pllc1_stat(&self) -> PLLC1_STAT_R {
        PLLC1_STAT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reflects the PLL1 Lock status. When zero, PLL1 is not locked. When one, PLL1 is locked onto the requested frequency."]
    #[inline(always)]
    pub fn plock1(&self) -> PLOCK1_R {
        PLOCK1_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL1STAT")
            .field("msel1", &format_args!("{}", self.msel1().bits()))
            .field("psel1", &format_args!("{}", self.psel1().bits()))
            .field("plle1_stat", &format_args!("{}", self.plle1_stat().bit()))
            .field("pllc1_stat", &format_args!("{}", self.pllc1_stat().bit()))
            .field("plock1", &format_args!("{}", self.plock1().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PLL1STAT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "PLL1 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll1stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL1STAT_SPEC;
impl crate::RegisterSpec for PLL1STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll1stat::R`](R) reader structure"]
impl crate::Readable for PLL1STAT_SPEC {}
#[doc = "`reset()` method sets PLL1STAT to value 0"]
impl crate::Resettable for PLL1STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
