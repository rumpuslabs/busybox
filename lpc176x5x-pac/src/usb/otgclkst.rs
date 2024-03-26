#[doc = "Register `OTGCLKST` reader"]
pub type R = crate::R<OTGCLKST_SPEC>;
#[doc = "Field `HOST_CLK_ON` reader - Host clock status."]
pub type HOST_CLK_ON_R = crate::BitReader<HOST_CLK_ON_A>;
#[doc = "Host clock status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HOST_CLK_ON_A {
    #[doc = "0: Host clock is not available."]
    NOT_AVAILABLE = 0,
    #[doc = "1: Host clock is available."]
    AVAILABLE = 1,
}
impl From<HOST_CLK_ON_A> for bool {
    #[inline(always)]
    fn from(variant: HOST_CLK_ON_A) -> Self {
        variant as u8 != 0
    }
}
impl HOST_CLK_ON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HOST_CLK_ON_A {
        match self.bits {
            false => HOST_CLK_ON_A::NOT_AVAILABLE,
            true => HOST_CLK_ON_A::AVAILABLE,
        }
    }
    #[doc = "Host clock is not available."]
    #[inline(always)]
    pub fn is_not_available(&self) -> bool {
        *self == HOST_CLK_ON_A::NOT_AVAILABLE
    }
    #[doc = "Host clock is available."]
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == HOST_CLK_ON_A::AVAILABLE
    }
}
#[doc = "Field `DEV_CLK_ON` reader - Device clock status."]
pub type DEV_CLK_ON_R = crate::BitReader<DEV_CLK_ON_A>;
#[doc = "Device clock status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEV_CLK_ON_A {
    #[doc = "0: Device clock is not available."]
    NOT_AVAILABLE = 0,
    #[doc = "1: Device clock is available."]
    AVAILABLE = 1,
}
impl From<DEV_CLK_ON_A> for bool {
    #[inline(always)]
    fn from(variant: DEV_CLK_ON_A) -> Self {
        variant as u8 != 0
    }
}
impl DEV_CLK_ON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DEV_CLK_ON_A {
        match self.bits {
            false => DEV_CLK_ON_A::NOT_AVAILABLE,
            true => DEV_CLK_ON_A::AVAILABLE,
        }
    }
    #[doc = "Device clock is not available."]
    #[inline(always)]
    pub fn is_not_available(&self) -> bool {
        *self == DEV_CLK_ON_A::NOT_AVAILABLE
    }
    #[doc = "Device clock is available."]
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == DEV_CLK_ON_A::AVAILABLE
    }
}
#[doc = "Field `I2C_CLK_ON` reader - I2C clock status."]
pub type I2C_CLK_ON_R = crate::BitReader<I2C_CLK_ON_A>;
#[doc = "I2C clock status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_CLK_ON_A {
    #[doc = "0: I2C clock is not available."]
    NOT_AVAILABLE = 0,
    #[doc = "1: I2C clock is available."]
    AVAILABLE = 1,
}
impl From<I2C_CLK_ON_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_CLK_ON_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C_CLK_ON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C_CLK_ON_A {
        match self.bits {
            false => I2C_CLK_ON_A::NOT_AVAILABLE,
            true => I2C_CLK_ON_A::AVAILABLE,
        }
    }
    #[doc = "I2C clock is not available."]
    #[inline(always)]
    pub fn is_not_available(&self) -> bool {
        *self == I2C_CLK_ON_A::NOT_AVAILABLE
    }
    #[doc = "I2C clock is available."]
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == I2C_CLK_ON_A::AVAILABLE
    }
}
#[doc = "Field `OTG_CLK_ON` reader - OTG clock status."]
pub type OTG_CLK_ON_R = crate::BitReader<OTG_CLK_ON_A>;
#[doc = "OTG clock status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTG_CLK_ON_A {
    #[doc = "0: OTG clock is not available."]
    NOT_AVAILABLE = 0,
    #[doc = "1: OTG clock is available."]
    AVAILABLE = 1,
}
impl From<OTG_CLK_ON_A> for bool {
    #[inline(always)]
    fn from(variant: OTG_CLK_ON_A) -> Self {
        variant as u8 != 0
    }
}
impl OTG_CLK_ON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OTG_CLK_ON_A {
        match self.bits {
            false => OTG_CLK_ON_A::NOT_AVAILABLE,
            true => OTG_CLK_ON_A::AVAILABLE,
        }
    }
    #[doc = "OTG clock is not available."]
    #[inline(always)]
    pub fn is_not_available(&self) -> bool {
        *self == OTG_CLK_ON_A::NOT_AVAILABLE
    }
    #[doc = "OTG clock is available."]
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == OTG_CLK_ON_A::AVAILABLE
    }
}
#[doc = "Field `AHB_CLK_ON` reader - AHB master clock status."]
pub type AHB_CLK_ON_R = crate::BitReader<AHB_CLK_ON_A>;
#[doc = "AHB master clock status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHB_CLK_ON_A {
    #[doc = "0: AHB clock is not available."]
    NOT_AVAILABLE = 0,
    #[doc = "1: AHB clock is available."]
    AVAILABLE = 1,
}
impl From<AHB_CLK_ON_A> for bool {
    #[inline(always)]
    fn from(variant: AHB_CLK_ON_A) -> Self {
        variant as u8 != 0
    }
}
impl AHB_CLK_ON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AHB_CLK_ON_A {
        match self.bits {
            false => AHB_CLK_ON_A::NOT_AVAILABLE,
            true => AHB_CLK_ON_A::AVAILABLE,
        }
    }
    #[doc = "AHB clock is not available."]
    #[inline(always)]
    pub fn is_not_available(&self) -> bool {
        *self == AHB_CLK_ON_A::NOT_AVAILABLE
    }
    #[doc = "AHB clock is available."]
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == AHB_CLK_ON_A::AVAILABLE
    }
}
impl R {
    #[doc = "Bit 0 - Host clock status."]
    #[inline(always)]
    pub fn host_clk_on(&self) -> HOST_CLK_ON_R {
        HOST_CLK_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Device clock status."]
    #[inline(always)]
    pub fn dev_clk_on(&self) -> DEV_CLK_ON_R {
        DEV_CLK_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C clock status."]
    #[inline(always)]
    pub fn i2c_clk_on(&self) -> I2C_CLK_ON_R {
        I2C_CLK_ON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OTG clock status."]
    #[inline(always)]
    pub fn otg_clk_on(&self) -> OTG_CLK_ON_R {
        OTG_CLK_ON_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB master clock status."]
    #[inline(always)]
    pub fn ahb_clk_on(&self) -> AHB_CLK_ON_R {
        AHB_CLK_ON_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTGCLKST")
            .field("host_clk_on", &format_args!("{}", self.host_clk_on().bit()))
            .field("dev_clk_on", &format_args!("{}", self.dev_clk_on().bit()))
            .field("i2c_clk_on", &format_args!("{}", self.i2c_clk_on().bit()))
            .field("otg_clk_on", &format_args!("{}", self.otg_clk_on().bit()))
            .field("ahb_clk_on", &format_args!("{}", self.ahb_clk_on().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<OTGCLKST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "OTG clock status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otgclkst::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTGCLKST_SPEC;
impl crate::RegisterSpec for OTGCLKST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otgclkst::R`](R) reader structure"]
impl crate::Readable for OTGCLKST_SPEC {}
#[doc = "`reset()` method sets OTGCLKST to value 0"]
impl crate::Resettable for OTGCLKST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
