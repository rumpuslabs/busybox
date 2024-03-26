#[doc = "Register `AFMR` reader"]
pub type R = crate::R<AFMR_SPEC>;
#[doc = "Register `AFMR` writer"]
pub type W = crate::W<AFMR_SPEC>;
#[doc = "Field `ACCOFF` reader - if AccBP is 0, the Acceptance Filter is not operational. All Rx messages on all CAN buses are ignored."]
pub type ACCOFF_R = crate::BitReader;
#[doc = "Field `ACCOFF` writer - if AccBP is 0, the Acceptance Filter is not operational. All Rx messages on all CAN buses are ignored."]
pub type ACCOFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACCBP` reader - All Rx messages are accepted on enabled CAN controllers. Software must set this bit before modifying the contents of any of the registers described below, and before modifying the contents of Lookup Table RAM in any way other than setting or clearing Disable bits in Standard Identifier entries. When both this bit and AccOff are 0, the Acceptance filter operates to screen received CAN Identifiers."]
pub type ACCBP_R = crate::BitReader;
#[doc = "Field `ACCBP` writer - All Rx messages are accepted on enabled CAN controllers. Software must set this bit before modifying the contents of any of the registers described below, and before modifying the contents of Lookup Table RAM in any way other than setting or clearing Disable bits in Standard Identifier entries. When both this bit and AccOff are 0, the Acceptance filter operates to screen received CAN Identifiers."]
pub type ACCBP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EFCAN` reader - FullCAN mode"]
pub type EFCAN_R = crate::BitReader<EFCAN_A>;
#[doc = "FullCAN mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EFCAN_A {
    #[doc = "0: Software must read all messages for all enabled IDs on all enabled CAN buses, from the receiving CAN controllers."]
    ENABLED = 0,
    #[doc = "1: The Acceptance Filter itself will take care of receiving and storing messages for selected Standard ID values on selected CAN buses. See Section 21.16 FullCAN mode on page 576."]
    DISABLED = 1,
}
impl From<EFCAN_A> for bool {
    #[inline(always)]
    fn from(variant: EFCAN_A) -> Self {
        variant as u8 != 0
    }
}
impl EFCAN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EFCAN_A {
        match self.bits {
            false => EFCAN_A::ENABLED,
            true => EFCAN_A::DISABLED,
        }
    }
    #[doc = "Software must read all messages for all enabled IDs on all enabled CAN buses, from the receiving CAN controllers."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EFCAN_A::ENABLED
    }
    #[doc = "The Acceptance Filter itself will take care of receiving and storing messages for selected Standard ID values on selected CAN buses. See Section 21.16 FullCAN mode on page 576."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EFCAN_A::DISABLED
    }
}
#[doc = "Field `EFCAN` writer - FullCAN mode"]
pub type EFCAN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EFCAN_A>;
impl<'a, REG, const O: u8> EFCAN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software must read all messages for all enabled IDs on all enabled CAN buses, from the receiving CAN controllers."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EFCAN_A::ENABLED)
    }
    #[doc = "The Acceptance Filter itself will take care of receiving and storing messages for selected Standard ID values on selected CAN buses. See Section 21.16 FullCAN mode on page 576."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EFCAN_A::DISABLED)
    }
}
impl R {
    #[doc = "Bit 0 - if AccBP is 0, the Acceptance Filter is not operational. All Rx messages on all CAN buses are ignored."]
    #[inline(always)]
    pub fn accoff(&self) -> ACCOFF_R {
        ACCOFF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - All Rx messages are accepted on enabled CAN controllers. Software must set this bit before modifying the contents of any of the registers described below, and before modifying the contents of Lookup Table RAM in any way other than setting or clearing Disable bits in Standard Identifier entries. When both this bit and AccOff are 0, the Acceptance filter operates to screen received CAN Identifiers."]
    #[inline(always)]
    pub fn accbp(&self) -> ACCBP_R {
        ACCBP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FullCAN mode"]
    #[inline(always)]
    pub fn efcan(&self) -> EFCAN_R {
        EFCAN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFMR")
            .field("accoff", &format_args!("{}", self.accoff().bit()))
            .field("accbp", &format_args!("{}", self.accbp().bit()))
            .field("efcan", &format_args!("{}", self.efcan().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<AFMR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - if AccBP is 0, the Acceptance Filter is not operational. All Rx messages on all CAN buses are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn accoff(&mut self) -> ACCOFF_W<AFMR_SPEC, 0> {
        ACCOFF_W::new(self)
    }
    #[doc = "Bit 1 - All Rx messages are accepted on enabled CAN controllers. Software must set this bit before modifying the contents of any of the registers described below, and before modifying the contents of Lookup Table RAM in any way other than setting or clearing Disable bits in Standard Identifier entries. When both this bit and AccOff are 0, the Acceptance filter operates to screen received CAN Identifiers."]
    #[inline(always)]
    #[must_use]
    pub fn accbp(&mut self) -> ACCBP_W<AFMR_SPEC, 1> {
        ACCBP_W::new(self)
    }
    #[doc = "Bit 2 - FullCAN mode"]
    #[inline(always)]
    #[must_use]
    pub fn efcan(&mut self) -> EFCAN_W<AFMR_SPEC, 2> {
        EFCAN_W::new(self)
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
#[doc = "Acceptance Filter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AFMR_SPEC;
impl crate::RegisterSpec for AFMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afmr::R`](R) reader structure"]
impl crate::Readable for AFMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`afmr::W`](W) writer structure"]
impl crate::Writable for AFMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFMR to value 0"]
impl crate::Resettable for AFMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
