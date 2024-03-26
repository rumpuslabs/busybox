#[doc = "Register `OTGCLKCTRL` reader"]
pub type R = crate::R<OTGCLKCTRL_SPEC>;
#[doc = "Register `OTGCLKCTRL` writer"]
pub type W = crate::W<OTGCLKCTRL_SPEC>;
#[doc = "Field `HOST_CLK_EN` reader - Host clock enable"]
pub type HOST_CLK_EN_R = crate::BitReader<HOST_CLK_EN_A>;
#[doc = "Host clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HOST_CLK_EN_A {
    #[doc = "0: Disable the Host clock."]
    DISABLE = 0,
    #[doc = "1: Enable the Host clock."]
    ENABLE = 1,
}
impl From<HOST_CLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HOST_CLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl HOST_CLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HOST_CLK_EN_A {
        match self.bits {
            false => HOST_CLK_EN_A::DISABLE,
            true => HOST_CLK_EN_A::ENABLE,
        }
    }
    #[doc = "Disable the Host clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HOST_CLK_EN_A::DISABLE
    }
    #[doc = "Enable the Host clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HOST_CLK_EN_A::ENABLE
    }
}
#[doc = "Field `HOST_CLK_EN` writer - Host clock enable"]
pub type HOST_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, HOST_CLK_EN_A>;
impl<'a, REG, const O: u8> HOST_CLK_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the Host clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HOST_CLK_EN_A::DISABLE)
    }
    #[doc = "Enable the Host clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HOST_CLK_EN_A::ENABLE)
    }
}
#[doc = "Field `DEV_CLK_EN` reader - Device clock enable"]
pub type DEV_CLK_EN_R = crate::BitReader<DEV_CLK_EN_A>;
#[doc = "Device clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEV_CLK_EN_A {
    #[doc = "0: Disable the Device clock."]
    DISABLE = 0,
    #[doc = "1: Enable the Device clock."]
    ENABLE = 1,
}
impl From<DEV_CLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DEV_CLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DEV_CLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DEV_CLK_EN_A {
        match self.bits {
            false => DEV_CLK_EN_A::DISABLE,
            true => DEV_CLK_EN_A::ENABLE,
        }
    }
    #[doc = "Disable the Device clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DEV_CLK_EN_A::DISABLE
    }
    #[doc = "Enable the Device clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DEV_CLK_EN_A::ENABLE
    }
}
#[doc = "Field `DEV_CLK_EN` writer - Device clock enable"]
pub type DEV_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DEV_CLK_EN_A>;
impl<'a, REG, const O: u8> DEV_CLK_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the Device clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DEV_CLK_EN_A::DISABLE)
    }
    #[doc = "Enable the Device clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DEV_CLK_EN_A::ENABLE)
    }
}
#[doc = "Field `I2C_CLK_EN` reader - I2C clock enable"]
pub type I2C_CLK_EN_R = crate::BitReader<I2C_CLK_EN_A>;
#[doc = "I2C clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_CLK_EN_A {
    #[doc = "0: Disable the I2C clock."]
    DISABLE = 0,
    #[doc = "1: Enable the I2C clock."]
    ENABLE = 1,
}
impl From<I2C_CLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_CLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C_CLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C_CLK_EN_A {
        match self.bits {
            false => I2C_CLK_EN_A::DISABLE,
            true => I2C_CLK_EN_A::ENABLE,
        }
    }
    #[doc = "Disable the I2C clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == I2C_CLK_EN_A::DISABLE
    }
    #[doc = "Enable the I2C clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == I2C_CLK_EN_A::ENABLE
    }
}
#[doc = "Field `I2C_CLK_EN` writer - I2C clock enable"]
pub type I2C_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, I2C_CLK_EN_A>;
impl<'a, REG, const O: u8> I2C_CLK_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the I2C clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_CLK_EN_A::DISABLE)
    }
    #[doc = "Enable the I2C clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_CLK_EN_A::ENABLE)
    }
}
#[doc = "Field `OTG_CLK_EN` reader - OTG clock enable. In device-only applications, this bit enables access to the PORTSEL register."]
pub type OTG_CLK_EN_R = crate::BitReader<OTG_CLK_EN_A>;
#[doc = "OTG clock enable. In device-only applications, this bit enables access to the PORTSEL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTG_CLK_EN_A {
    #[doc = "0: Disable the OTG clock."]
    DISABLE = 0,
    #[doc = "1: Enable the OTG clock."]
    ENABLE = 1,
}
impl From<OTG_CLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: OTG_CLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl OTG_CLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OTG_CLK_EN_A {
        match self.bits {
            false => OTG_CLK_EN_A::DISABLE,
            true => OTG_CLK_EN_A::ENABLE,
        }
    }
    #[doc = "Disable the OTG clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OTG_CLK_EN_A::DISABLE
    }
    #[doc = "Enable the OTG clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OTG_CLK_EN_A::ENABLE
    }
}
#[doc = "Field `OTG_CLK_EN` writer - OTG clock enable. In device-only applications, this bit enables access to the PORTSEL register."]
pub type OTG_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OTG_CLK_EN_A>;
impl<'a, REG, const O: u8> OTG_CLK_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the OTG clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(OTG_CLK_EN_A::DISABLE)
    }
    #[doc = "Enable the OTG clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(OTG_CLK_EN_A::ENABLE)
    }
}
#[doc = "Field `AHB_CLK_EN` reader - AHB master clock enable"]
pub type AHB_CLK_EN_R = crate::BitReader<AHB_CLK_EN_A>;
#[doc = "AHB master clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHB_CLK_EN_A {
    #[doc = "0: Disable the AHB clock."]
    DISABLE = 0,
    #[doc = "1: Enable the AHB clock."]
    ENABLE = 1,
}
impl From<AHB_CLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: AHB_CLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl AHB_CLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AHB_CLK_EN_A {
        match self.bits {
            false => AHB_CLK_EN_A::DISABLE,
            true => AHB_CLK_EN_A::ENABLE,
        }
    }
    #[doc = "Disable the AHB clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AHB_CLK_EN_A::DISABLE
    }
    #[doc = "Enable the AHB clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AHB_CLK_EN_A::ENABLE
    }
}
#[doc = "Field `AHB_CLK_EN` writer - AHB master clock enable"]
pub type AHB_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AHB_CLK_EN_A>;
impl<'a, REG, const O: u8> AHB_CLK_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the AHB clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(AHB_CLK_EN_A::DISABLE)
    }
    #[doc = "Enable the AHB clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AHB_CLK_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Host clock enable"]
    #[inline(always)]
    pub fn host_clk_en(&self) -> HOST_CLK_EN_R {
        HOST_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Device clock enable"]
    #[inline(always)]
    pub fn dev_clk_en(&self) -> DEV_CLK_EN_R {
        DEV_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C clock enable"]
    #[inline(always)]
    pub fn i2c_clk_en(&self) -> I2C_CLK_EN_R {
        I2C_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OTG clock enable. In device-only applications, this bit enables access to the PORTSEL register."]
    #[inline(always)]
    pub fn otg_clk_en(&self) -> OTG_CLK_EN_R {
        OTG_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB master clock enable"]
    #[inline(always)]
    pub fn ahb_clk_en(&self) -> AHB_CLK_EN_R {
        AHB_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTGCLKCTRL")
            .field("host_clk_en", &format_args!("{}", self.host_clk_en().bit()))
            .field("dev_clk_en", &format_args!("{}", self.dev_clk_en().bit()))
            .field("i2c_clk_en", &format_args!("{}", self.i2c_clk_en().bit()))
            .field("otg_clk_en", &format_args!("{}", self.otg_clk_en().bit()))
            .field("ahb_clk_en", &format_args!("{}", self.ahb_clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<OTGCLKCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Host clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn host_clk_en(&mut self) -> HOST_CLK_EN_W<OTGCLKCTRL_SPEC, 0> {
        HOST_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Device clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dev_clk_en(&mut self) -> DEV_CLK_EN_W<OTGCLKCTRL_SPEC, 1> {
        DEV_CLK_EN_W::new(self)
    }
    #[doc = "Bit 2 - I2C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_clk_en(&mut self) -> I2C_CLK_EN_W<OTGCLKCTRL_SPEC, 2> {
        I2C_CLK_EN_W::new(self)
    }
    #[doc = "Bit 3 - OTG clock enable. In device-only applications, this bit enables access to the PORTSEL register."]
    #[inline(always)]
    #[must_use]
    pub fn otg_clk_en(&mut self) -> OTG_CLK_EN_W<OTGCLKCTRL_SPEC, 3> {
        OTG_CLK_EN_W::new(self)
    }
    #[doc = "Bit 4 - AHB master clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_clk_en(&mut self) -> AHB_CLK_EN_W<OTGCLKCTRL_SPEC, 4> {
        AHB_CLK_EN_W::new(self)
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
#[doc = "OTG clock controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otgclkctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otgclkctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTGCLKCTRL_SPEC;
impl crate::RegisterSpec for OTGCLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otgclkctrl::R`](R) reader structure"]
impl crate::Readable for OTGCLKCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otgclkctrl::W`](W) writer structure"]
impl crate::Writable for OTGCLKCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTGCLKCTRL to value 0"]
impl crate::Resettable for OTGCLKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
