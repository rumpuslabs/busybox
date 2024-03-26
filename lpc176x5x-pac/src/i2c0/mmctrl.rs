#[doc = "Register `MMCTRL` reader"]
pub type R = crate::R<MMCTRL_SPEC>;
#[doc = "Register `MMCTRL` writer"]
pub type W = crate::W<MMCTRL_SPEC>;
#[doc = "Field `MM_ENA` reader - Monitor mode enable."]
pub type MM_ENA_R = crate::BitReader<MM_ENA_A>;
#[doc = "Monitor mode enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MM_ENA_A {
    #[doc = "0: Monitor mode disabled."]
    DISABLED = 0,
    #[doc = "1: The I 2C module will enter monitor mode. In this mode the SDA output will be forced high. This will prevent the I2C module from outputting data of any kind (including ACK) onto the I2C data bus. Depending on the state of the ENA_SCL bit, the output may be also forced high, preventing the module from having control over the I2C clock line."]
    ENABLED = 1,
}
impl From<MM_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: MM_ENA_A) -> Self {
        variant as u8 != 0
    }
}
impl MM_ENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MM_ENA_A {
        match self.bits {
            false => MM_ENA_A::DISABLED,
            true => MM_ENA_A::ENABLED,
        }
    }
    #[doc = "Monitor mode disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MM_ENA_A::DISABLED
    }
    #[doc = "The I 2C module will enter monitor mode. In this mode the SDA output will be forced high. This will prevent the I2C module from outputting data of any kind (including ACK) onto the I2C data bus. Depending on the state of the ENA_SCL bit, the output may be also forced high, preventing the module from having control over the I2C clock line."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MM_ENA_A::ENABLED
    }
}
#[doc = "Field `MM_ENA` writer - Monitor mode enable."]
pub type MM_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MM_ENA_A>;
impl<'a, REG, const O: u8> MM_ENA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Monitor mode disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MM_ENA_A::DISABLED)
    }
    #[doc = "The I 2C module will enter monitor mode. In this mode the SDA output will be forced high. This will prevent the I2C module from outputting data of any kind (including ACK) onto the I2C data bus. Depending on the state of the ENA_SCL bit, the output may be also forced high, preventing the module from having control over the I2C clock line."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MM_ENA_A::ENABLED)
    }
}
#[doc = "Field `ENA_SCL` reader - SCL output enable."]
pub type ENA_SCL_R = crate::BitReader<ENA_SCL_A>;
#[doc = "SCL output enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENA_SCL_A {
    #[doc = "0: When this bit is cleared to 0, the SCL output will be forced high when the module is in monitor mode. As described above, this will prevent the module from having any control over the I2C clock line."]
    DISABLED = 0,
    #[doc = "1: When this bit is set, the I2C module may exercise the same control over the clock line that it would in normal operation. This means that, acting as a slave peripheral, the I2C module can stretch the clock line (hold it low) until it has had time to respond to an I2C interrupt.\\[1\\]"]
    ENABLED = 1,
}
impl From<ENA_SCL_A> for bool {
    #[inline(always)]
    fn from(variant: ENA_SCL_A) -> Self {
        variant as u8 != 0
    }
}
impl ENA_SCL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENA_SCL_A {
        match self.bits {
            false => ENA_SCL_A::DISABLED,
            true => ENA_SCL_A::ENABLED,
        }
    }
    #[doc = "When this bit is cleared to 0, the SCL output will be forced high when the module is in monitor mode. As described above, this will prevent the module from having any control over the I2C clock line."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENA_SCL_A::DISABLED
    }
    #[doc = "When this bit is set, the I2C module may exercise the same control over the clock line that it would in normal operation. This means that, acting as a slave peripheral, the I2C module can stretch the clock line (hold it low) until it has had time to respond to an I2C interrupt.\\[1\\]"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENA_SCL_A::ENABLED
    }
}
#[doc = "Field `ENA_SCL` writer - SCL output enable."]
pub type ENA_SCL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ENA_SCL_A>;
impl<'a, REG, const O: u8> ENA_SCL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When this bit is cleared to 0, the SCL output will be forced high when the module is in monitor mode. As described above, this will prevent the module from having any control over the I2C clock line."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENA_SCL_A::DISABLED)
    }
    #[doc = "When this bit is set, the I2C module may exercise the same control over the clock line that it would in normal operation. This means that, acting as a slave peripheral, the I2C module can stretch the clock line (hold it low) until it has had time to respond to an I2C interrupt.\\[1\\]"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENA_SCL_A::ENABLED)
    }
}
#[doc = "Field `MATCH_ALL` reader - Select interrupt register match."]
pub type MATCH_ALL_R = crate::BitReader<MATCH_ALL_A>;
#[doc = "Select interrupt register match.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MATCH_ALL_A {
    #[doc = "0: When this bit is cleared, an interrupt will only be generated when a match occurs to one of the (up-to) four address registers described above. That is, the module will respond as a normal slave as far as address-recognition is concerned."]
    DISABLED = 0,
    #[doc = "1: When this bit is set to 1 and the I2C is in monitor mode, an interrupt will be generated on ANY address received. This will enable the part to monitor all traffic on the bus."]
    ENABLED = 1,
}
impl From<MATCH_ALL_A> for bool {
    #[inline(always)]
    fn from(variant: MATCH_ALL_A) -> Self {
        variant as u8 != 0
    }
}
impl MATCH_ALL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MATCH_ALL_A {
        match self.bits {
            false => MATCH_ALL_A::DISABLED,
            true => MATCH_ALL_A::ENABLED,
        }
    }
    #[doc = "When this bit is cleared, an interrupt will only be generated when a match occurs to one of the (up-to) four address registers described above. That is, the module will respond as a normal slave as far as address-recognition is concerned."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MATCH_ALL_A::DISABLED
    }
    #[doc = "When this bit is set to 1 and the I2C is in monitor mode, an interrupt will be generated on ANY address received. This will enable the part to monitor all traffic on the bus."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MATCH_ALL_A::ENABLED
    }
}
#[doc = "Field `MATCH_ALL` writer - Select interrupt register match."]
pub type MATCH_ALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MATCH_ALL_A>;
impl<'a, REG, const O: u8> MATCH_ALL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When this bit is cleared, an interrupt will only be generated when a match occurs to one of the (up-to) four address registers described above. That is, the module will respond as a normal slave as far as address-recognition is concerned."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MATCH_ALL_A::DISABLED)
    }
    #[doc = "When this bit is set to 1 and the I2C is in monitor mode, an interrupt will be generated on ANY address received. This will enable the part to monitor all traffic on the bus."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MATCH_ALL_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Monitor mode enable."]
    #[inline(always)]
    pub fn mm_ena(&self) -> MM_ENA_R {
        MM_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCL output enable."]
    #[inline(always)]
    pub fn ena_scl(&self) -> ENA_SCL_R {
        ENA_SCL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select interrupt register match."]
    #[inline(always)]
    pub fn match_all(&self) -> MATCH_ALL_R {
        MATCH_ALL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMCTRL")
            .field("mm_ena", &format_args!("{}", self.mm_ena().bit()))
            .field("ena_scl", &format_args!("{}", self.ena_scl().bit()))
            .field("match_all", &format_args!("{}", self.match_all().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<MMCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Monitor mode enable."]
    #[inline(always)]
    #[must_use]
    pub fn mm_ena(&mut self) -> MM_ENA_W<MMCTRL_SPEC, 0> {
        MM_ENA_W::new(self)
    }
    #[doc = "Bit 1 - SCL output enable."]
    #[inline(always)]
    #[must_use]
    pub fn ena_scl(&mut self) -> ENA_SCL_W<MMCTRL_SPEC, 1> {
        ENA_SCL_W::new(self)
    }
    #[doc = "Bit 2 - Select interrupt register match."]
    #[inline(always)]
    #[must_use]
    pub fn match_all(&mut self) -> MATCH_ALL_W<MMCTRL_SPEC, 2> {
        MATCH_ALL_W::new(self)
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
#[doc = "Monitor mode control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCTRL_SPEC;
impl crate::RegisterSpec for MMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmctrl::R`](R) reader structure"]
impl crate::Readable for MMCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmctrl::W`](W) writer structure"]
impl crate::Writable for MMCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMCTRL to value 0"]
impl crate::Resettable for MMCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
