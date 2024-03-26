#[doc = "Register `I2CPADCFG` reader"]
pub type R = crate::R<I2CPADCFG_SPEC>;
#[doc = "Register `I2CPADCFG` writer"]
pub type W = crate::W<I2CPADCFG_SPEC>;
#[doc = "Field `SDADRV0` reader - Drive mode control for the SDA0 pin, P0.27."]
pub type SDADRV0_R = crate::BitReader<SDADRV0_A>;
#[doc = "Drive mode control for the SDA0 pin, P0.27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDADRV0_A {
    #[doc = "0: Standard. The SDA0 pin is in the standard drive mode."]
    STANDARD = 0,
    #[doc = "1: Fast-mode plus. The SDA0 pin is in Fast Mode Plus drive mode."]
    FAST_MODE_PLUS = 1,
}
impl From<SDADRV0_A> for bool {
    #[inline(always)]
    fn from(variant: SDADRV0_A) -> Self {
        variant as u8 != 0
    }
}
impl SDADRV0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDADRV0_A {
        match self.bits {
            false => SDADRV0_A::STANDARD,
            true => SDADRV0_A::FAST_MODE_PLUS,
        }
    }
    #[doc = "Standard. The SDA0 pin is in the standard drive mode."]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == SDADRV0_A::STANDARD
    }
    #[doc = "Fast-mode plus. The SDA0 pin is in Fast Mode Plus drive mode."]
    #[inline(always)]
    pub fn is_fast_mode_plus(&self) -> bool {
        *self == SDADRV0_A::FAST_MODE_PLUS
    }
}
#[doc = "Field `SDADRV0` writer - Drive mode control for the SDA0 pin, P0.27."]
pub type SDADRV0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SDADRV0_A>;
impl<'a, REG, const O: u8> SDADRV0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard. The SDA0 pin is in the standard drive mode."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(SDADRV0_A::STANDARD)
    }
    #[doc = "Fast-mode plus. The SDA0 pin is in Fast Mode Plus drive mode."]
    #[inline(always)]
    pub fn fast_mode_plus(self) -> &'a mut crate::W<REG> {
        self.variant(SDADRV0_A::FAST_MODE_PLUS)
    }
}
#[doc = "Field `SDAI2C0` reader - I 2C filter mode control for the SDA0 pin, P0.27."]
pub type SDAI2C0_R = crate::BitReader<SDAI2C0_A>;
#[doc = "I 2C filter mode control for the SDA0 pin, P0.27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDAI2C0_A {
    #[doc = "0: Enabled. The SDA0 pin has I2C glitch filtering and slew rate control enabled."]
    ENABLED = 0,
    #[doc = "1: Disabled. The SDA0 pin has I2C glitch filtering and slew rate control disabled."]
    DISABLED = 1,
}
impl From<SDAI2C0_A> for bool {
    #[inline(always)]
    fn from(variant: SDAI2C0_A) -> Self {
        variant as u8 != 0
    }
}
impl SDAI2C0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDAI2C0_A {
        match self.bits {
            false => SDAI2C0_A::ENABLED,
            true => SDAI2C0_A::DISABLED,
        }
    }
    #[doc = "Enabled. The SDA0 pin has I2C glitch filtering and slew rate control enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDAI2C0_A::ENABLED
    }
    #[doc = "Disabled. The SDA0 pin has I2C glitch filtering and slew rate control disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDAI2C0_A::DISABLED
    }
}
#[doc = "Field `SDAI2C0` writer - I 2C filter mode control for the SDA0 pin, P0.27."]
pub type SDAI2C0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SDAI2C0_A>;
impl<'a, REG, const O: u8> SDAI2C0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled. The SDA0 pin has I2C glitch filtering and slew rate control enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDAI2C0_A::ENABLED)
    }
    #[doc = "Disabled. The SDA0 pin has I2C glitch filtering and slew rate control disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SDAI2C0_A::DISABLED)
    }
}
#[doc = "Field `SCLDRV0` reader - Drive mode control for the SCL0 pin, P0.28."]
pub type SCLDRV0_R = crate::BitReader<SCLDRV0_A>;
#[doc = "Drive mode control for the SCL0 pin, P0.28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCLDRV0_A {
    #[doc = "0: Standard. The SCL0 pin is in the standard drive mode."]
    STANDARD = 0,
    #[doc = "1: Fast-mode plus. The SCL0 pin is in Fast Mode Plus drive mode."]
    FAST_MODE_PLUS = 1,
}
impl From<SCLDRV0_A> for bool {
    #[inline(always)]
    fn from(variant: SCLDRV0_A) -> Self {
        variant as u8 != 0
    }
}
impl SCLDRV0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCLDRV0_A {
        match self.bits {
            false => SCLDRV0_A::STANDARD,
            true => SCLDRV0_A::FAST_MODE_PLUS,
        }
    }
    #[doc = "Standard. The SCL0 pin is in the standard drive mode."]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == SCLDRV0_A::STANDARD
    }
    #[doc = "Fast-mode plus. The SCL0 pin is in Fast Mode Plus drive mode."]
    #[inline(always)]
    pub fn is_fast_mode_plus(&self) -> bool {
        *self == SCLDRV0_A::FAST_MODE_PLUS
    }
}
#[doc = "Field `SCLDRV0` writer - Drive mode control for the SCL0 pin, P0.28."]
pub type SCLDRV0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SCLDRV0_A>;
impl<'a, REG, const O: u8> SCLDRV0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard. The SCL0 pin is in the standard drive mode."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(SCLDRV0_A::STANDARD)
    }
    #[doc = "Fast-mode plus. The SCL0 pin is in Fast Mode Plus drive mode."]
    #[inline(always)]
    pub fn fast_mode_plus(self) -> &'a mut crate::W<REG> {
        self.variant(SCLDRV0_A::FAST_MODE_PLUS)
    }
}
#[doc = "Field `SCLI2C0` reader - I 2C filter mode control for the SCL0 pin, P0.28."]
pub type SCLI2C0_R = crate::BitReader<SCLI2C0_A>;
#[doc = "I 2C filter mode control for the SCL0 pin, P0.28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCLI2C0_A {
    #[doc = "0: Enabled. The SCL0 pin has I2C glitch filtering and slew rate control enabled."]
    ENABLED = 0,
    #[doc = "1: Disabled. The SCL0 pin has I2C glitch filtering and slew rate control disabled."]
    DISABLED = 1,
}
impl From<SCLI2C0_A> for bool {
    #[inline(always)]
    fn from(variant: SCLI2C0_A) -> Self {
        variant as u8 != 0
    }
}
impl SCLI2C0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCLI2C0_A {
        match self.bits {
            false => SCLI2C0_A::ENABLED,
            true => SCLI2C0_A::DISABLED,
        }
    }
    #[doc = "Enabled. The SCL0 pin has I2C glitch filtering and slew rate control enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCLI2C0_A::ENABLED
    }
    #[doc = "Disabled. The SCL0 pin has I2C glitch filtering and slew rate control disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCLI2C0_A::DISABLED
    }
}
#[doc = "Field `SCLI2C0` writer - I 2C filter mode control for the SCL0 pin, P0.28."]
pub type SCLI2C0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SCLI2C0_A>;
impl<'a, REG, const O: u8> SCLI2C0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled. The SCL0 pin has I2C glitch filtering and slew rate control enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCLI2C0_A::ENABLED)
    }
    #[doc = "Disabled. The SCL0 pin has I2C glitch filtering and slew rate control disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCLI2C0_A::DISABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Drive mode control for the SDA0 pin, P0.27."]
    #[inline(always)]
    pub fn sdadrv0(&self) -> SDADRV0_R {
        SDADRV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I 2C filter mode control for the SDA0 pin, P0.27."]
    #[inline(always)]
    pub fn sdai2c0(&self) -> SDAI2C0_R {
        SDAI2C0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Drive mode control for the SCL0 pin, P0.28."]
    #[inline(always)]
    pub fn scldrv0(&self) -> SCLDRV0_R {
        SCLDRV0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I 2C filter mode control for the SCL0 pin, P0.28."]
    #[inline(always)]
    pub fn scli2c0(&self) -> SCLI2C0_R {
        SCLI2C0_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2CPADCFG")
            .field("sdadrv0", &format_args!("{}", self.sdadrv0().bit()))
            .field("sdai2c0", &format_args!("{}", self.sdai2c0().bit()))
            .field("scldrv0", &format_args!("{}", self.scldrv0().bit()))
            .field("scli2c0", &format_args!("{}", self.scli2c0().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<I2CPADCFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Drive mode control for the SDA0 pin, P0.27."]
    #[inline(always)]
    #[must_use]
    pub fn sdadrv0(&mut self) -> SDADRV0_W<I2CPADCFG_SPEC, 0> {
        SDADRV0_W::new(self)
    }
    #[doc = "Bit 1 - I 2C filter mode control for the SDA0 pin, P0.27."]
    #[inline(always)]
    #[must_use]
    pub fn sdai2c0(&mut self) -> SDAI2C0_W<I2CPADCFG_SPEC, 1> {
        SDAI2C0_W::new(self)
    }
    #[doc = "Bit 2 - Drive mode control for the SCL0 pin, P0.28."]
    #[inline(always)]
    #[must_use]
    pub fn scldrv0(&mut self) -> SCLDRV0_W<I2CPADCFG_SPEC, 2> {
        SCLDRV0_W::new(self)
    }
    #[doc = "Bit 3 - I 2C filter mode control for the SCL0 pin, P0.28."]
    #[inline(always)]
    #[must_use]
    pub fn scli2c0(&mut self) -> SCLI2C0_W<I2CPADCFG_SPEC, 3> {
        SCLI2C0_W::new(self)
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
#[doc = "I2C Pin Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cpadcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cpadcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2CPADCFG_SPEC;
impl crate::RegisterSpec for I2CPADCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cpadcfg::R`](R) reader structure"]
impl crate::Readable for I2CPADCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2cpadcfg::W`](W) writer structure"]
impl crate::Writable for I2CPADCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2CPADCFG to value 0"]
impl crate::Resettable for I2CPADCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
