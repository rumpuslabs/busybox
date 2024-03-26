#[doc = "Register `MOD` reader"]
pub type R = crate::R<MOD_SPEC>;
#[doc = "Register `MOD` writer"]
pub type W = crate::W<MOD_SPEC>;
#[doc = "Field `RM` reader - Reset Mode."]
pub type RM_R = crate::BitReader<RM_A>;
#[doc = "Reset Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RM_A {
    #[doc = "0: Normal.The CAN Controller is in the Operating Mode, and certain registers can not be written."]
    NORMAL = 0,
    #[doc = "1: Reset. CAN operation is disabled, writable registers can be written and the current transmission/reception of a message is aborted."]
    RESET = 1,
}
impl From<RM_A> for bool {
    #[inline(always)]
    fn from(variant: RM_A) -> Self {
        variant as u8 != 0
    }
}
impl RM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RM_A {
        match self.bits {
            false => RM_A::NORMAL,
            true => RM_A::RESET,
        }
    }
    #[doc = "Normal.The CAN Controller is in the Operating Mode, and certain registers can not be written."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == RM_A::NORMAL
    }
    #[doc = "Reset. CAN operation is disabled, writable registers can be written and the current transmission/reception of a message is aborted."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RM_A::RESET
    }
}
#[doc = "Field `RM` writer - Reset Mode."]
pub type RM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RM_A>;
impl<'a, REG, const O: u8> RM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal.The CAN Controller is in the Operating Mode, and certain registers can not be written."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(RM_A::NORMAL)
    }
    #[doc = "Reset. CAN operation is disabled, writable registers can be written and the current transmission/reception of a message is aborted."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(RM_A::RESET)
    }
}
#[doc = "Field `LOM` reader - Listen Only Mode."]
pub type LOM_R = crate::BitReader<LOM_A>;
#[doc = "Listen Only Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOM_A {
    #[doc = "0: Normal. The CAN controller acknowledges a successfully received message on the CAN bus. The error counters are stopped at the current value."]
    NORMAL = 0,
    #[doc = "1: Listen only. The controller gives no acknowledgment, even if a message is successfully received. Messages cannot be sent, and the controller operates in error passive mode. This mode is intended for software bit rate detection and hot plugging."]
    LISTEN_ONLY = 1,
}
impl From<LOM_A> for bool {
    #[inline(always)]
    fn from(variant: LOM_A) -> Self {
        variant as u8 != 0
    }
}
impl LOM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOM_A {
        match self.bits {
            false => LOM_A::NORMAL,
            true => LOM_A::LISTEN_ONLY,
        }
    }
    #[doc = "Normal. The CAN controller acknowledges a successfully received message on the CAN bus. The error counters are stopped at the current value."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == LOM_A::NORMAL
    }
    #[doc = "Listen only. The controller gives no acknowledgment, even if a message is successfully received. Messages cannot be sent, and the controller operates in error passive mode. This mode is intended for software bit rate detection and hot plugging."]
    #[inline(always)]
    pub fn is_listen_only(&self) -> bool {
        *self == LOM_A::LISTEN_ONLY
    }
}
#[doc = "Field `LOM` writer - Listen Only Mode."]
pub type LOM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LOM_A>;
impl<'a, REG, const O: u8> LOM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. The CAN controller acknowledges a successfully received message on the CAN bus. The error counters are stopped at the current value."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(LOM_A::NORMAL)
    }
    #[doc = "Listen only. The controller gives no acknowledgment, even if a message is successfully received. Messages cannot be sent, and the controller operates in error passive mode. This mode is intended for software bit rate detection and hot plugging."]
    #[inline(always)]
    pub fn listen_only(self) -> &'a mut crate::W<REG> {
        self.variant(LOM_A::LISTEN_ONLY)
    }
}
#[doc = "Field `STM` reader - Self Test Mode."]
pub type STM_R = crate::BitReader<STM_A>;
#[doc = "Self Test Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STM_A {
    #[doc = "0: Normal. A transmitted message must be acknowledged to be considered successful."]
    NORMAL = 0,
    #[doc = "1: Self test. The controller will consider a Tx message successful even if there is no acknowledgment received. In this mode a full node test is possible without any other active node on the bus using the SRR bit in CANxCMR."]
    SELF_TEST = 1,
}
impl From<STM_A> for bool {
    #[inline(always)]
    fn from(variant: STM_A) -> Self {
        variant as u8 != 0
    }
}
impl STM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STM_A {
        match self.bits {
            false => STM_A::NORMAL,
            true => STM_A::SELF_TEST,
        }
    }
    #[doc = "Normal. A transmitted message must be acknowledged to be considered successful."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == STM_A::NORMAL
    }
    #[doc = "Self test. The controller will consider a Tx message successful even if there is no acknowledgment received. In this mode a full node test is possible without any other active node on the bus using the SRR bit in CANxCMR."]
    #[inline(always)]
    pub fn is_self_test(&self) -> bool {
        *self == STM_A::SELF_TEST
    }
}
#[doc = "Field `STM` writer - Self Test Mode."]
pub type STM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STM_A>;
impl<'a, REG, const O: u8> STM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. A transmitted message must be acknowledged to be considered successful."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(STM_A::NORMAL)
    }
    #[doc = "Self test. The controller will consider a Tx message successful even if there is no acknowledgment received. In this mode a full node test is possible without any other active node on the bus using the SRR bit in CANxCMR."]
    #[inline(always)]
    pub fn self_test(self) -> &'a mut crate::W<REG> {
        self.variant(STM_A::SELF_TEST)
    }
}
#[doc = "Field `TPM` reader - Transmit Priority Mode."]
pub type TPM_R = crate::BitReader<TPM_A>;
#[doc = "Transmit Priority Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPM_A {
    #[doc = "0: CAN ID. The transmit priority for 3 Transmit Buffers depends on the CAN Identifier."]
    CAN_ID = 0,
    #[doc = "1: Local priority. The transmit priority for 3 Transmit Buffers depends on the contents of the Tx Priority register within the Transmit Buffer."]
    LOCAL_PRIORITY = 1,
}
impl From<TPM_A> for bool {
    #[inline(always)]
    fn from(variant: TPM_A) -> Self {
        variant as u8 != 0
    }
}
impl TPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TPM_A {
        match self.bits {
            false => TPM_A::CAN_ID,
            true => TPM_A::LOCAL_PRIORITY,
        }
    }
    #[doc = "CAN ID. The transmit priority for 3 Transmit Buffers depends on the CAN Identifier."]
    #[inline(always)]
    pub fn is_can_id(&self) -> bool {
        *self == TPM_A::CAN_ID
    }
    #[doc = "Local priority. The transmit priority for 3 Transmit Buffers depends on the contents of the Tx Priority register within the Transmit Buffer."]
    #[inline(always)]
    pub fn is_local_priority(&self) -> bool {
        *self == TPM_A::LOCAL_PRIORITY
    }
}
#[doc = "Field `TPM` writer - Transmit Priority Mode."]
pub type TPM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TPM_A>;
impl<'a, REG, const O: u8> TPM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CAN ID. The transmit priority for 3 Transmit Buffers depends on the CAN Identifier."]
    #[inline(always)]
    pub fn can_id(self) -> &'a mut crate::W<REG> {
        self.variant(TPM_A::CAN_ID)
    }
    #[doc = "Local priority. The transmit priority for 3 Transmit Buffers depends on the contents of the Tx Priority register within the Transmit Buffer."]
    #[inline(always)]
    pub fn local_priority(self) -> &'a mut crate::W<REG> {
        self.variant(TPM_A::LOCAL_PRIORITY)
    }
}
#[doc = "Field `SM` reader - Sleep Mode."]
pub type SM_R = crate::BitReader<SM_A>;
#[doc = "Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SM_A {
    #[doc = "0: Wake-up. Normal operation."]
    WAKE_UP = 0,
    #[doc = "1: Sleep. The CAN controller enters Sleep Mode if no CAN interrupt is pending and there is no bus activity. See the Sleep Mode description Section 21.8.2 on page 565."]
    SLEEP = 1,
}
impl From<SM_A> for bool {
    #[inline(always)]
    fn from(variant: SM_A) -> Self {
        variant as u8 != 0
    }
}
impl SM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SM_A {
        match self.bits {
            false => SM_A::WAKE_UP,
            true => SM_A::SLEEP,
        }
    }
    #[doc = "Wake-up. Normal operation."]
    #[inline(always)]
    pub fn is_wake_up(&self) -> bool {
        *self == SM_A::WAKE_UP
    }
    #[doc = "Sleep. The CAN controller enters Sleep Mode if no CAN interrupt is pending and there is no bus activity. See the Sleep Mode description Section 21.8.2 on page 565."]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == SM_A::SLEEP
    }
}
#[doc = "Field `SM` writer - Sleep Mode."]
pub type SM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SM_A>;
impl<'a, REG, const O: u8> SM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up. Normal operation."]
    #[inline(always)]
    pub fn wake_up(self) -> &'a mut crate::W<REG> {
        self.variant(SM_A::WAKE_UP)
    }
    #[doc = "Sleep. The CAN controller enters Sleep Mode if no CAN interrupt is pending and there is no bus activity. See the Sleep Mode description Section 21.8.2 on page 565."]
    #[inline(always)]
    pub fn sleep(self) -> &'a mut crate::W<REG> {
        self.variant(SM_A::SLEEP)
    }
}
#[doc = "Field `RPM` reader - Receive Polarity Mode."]
pub type RPM_R = crate::BitReader<RPM_A>;
#[doc = "Receive Polarity Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPM_A {
    #[doc = "0: Low active. RD input is active Low (dominant bit = 0)."]
    LOW_ACTIVE = 0,
    #[doc = "1: High active. RD input is active High (dominant bit = 1) -- reverse polarity."]
    HIGH_ACTIVE = 1,
}
impl From<RPM_A> for bool {
    #[inline(always)]
    fn from(variant: RPM_A) -> Self {
        variant as u8 != 0
    }
}
impl RPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPM_A {
        match self.bits {
            false => RPM_A::LOW_ACTIVE,
            true => RPM_A::HIGH_ACTIVE,
        }
    }
    #[doc = "Low active. RD input is active Low (dominant bit = 0)."]
    #[inline(always)]
    pub fn is_low_active(&self) -> bool {
        *self == RPM_A::LOW_ACTIVE
    }
    #[doc = "High active. RD input is active High (dominant bit = 1) -- reverse polarity."]
    #[inline(always)]
    pub fn is_high_active(&self) -> bool {
        *self == RPM_A::HIGH_ACTIVE
    }
}
#[doc = "Field `RPM` writer - Receive Polarity Mode."]
pub type RPM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RPM_A>;
impl<'a, REG, const O: u8> RPM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low active. RD input is active Low (dominant bit = 0)."]
    #[inline(always)]
    pub fn low_active(self) -> &'a mut crate::W<REG> {
        self.variant(RPM_A::LOW_ACTIVE)
    }
    #[doc = "High active. RD input is active High (dominant bit = 1) -- reverse polarity."]
    #[inline(always)]
    pub fn high_active(self) -> &'a mut crate::W<REG> {
        self.variant(RPM_A::HIGH_ACTIVE)
    }
}
#[doc = "Field `TM` reader - Test Mode."]
pub type TM_R = crate::BitReader<TM_A>;
#[doc = "Test Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TM_A {
    #[doc = "0: Disabled. Normal operation."]
    DISABLED_NORMAL_OPE = 0,
    #[doc = "1: Enabled. The TD pin will reflect the bit, detected on RD pin, with the next positive edge of the system clock."]
    ENABLED = 1,
}
impl From<TM_A> for bool {
    #[inline(always)]
    fn from(variant: TM_A) -> Self {
        variant as u8 != 0
    }
}
impl TM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TM_A {
        match self.bits {
            false => TM_A::DISABLED_NORMAL_OPE,
            true => TM_A::ENABLED,
        }
    }
    #[doc = "Disabled. Normal operation."]
    #[inline(always)]
    pub fn is_disabled_normal_ope(&self) -> bool {
        *self == TM_A::DISABLED_NORMAL_OPE
    }
    #[doc = "Enabled. The TD pin will reflect the bit, detected on RD pin, with the next positive edge of the system clock."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TM_A::ENABLED
    }
}
#[doc = "Field `TM` writer - Test Mode."]
pub type TM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TM_A>;
impl<'a, REG, const O: u8> TM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. Normal operation."]
    #[inline(always)]
    pub fn disabled_normal_ope(self) -> &'a mut crate::W<REG> {
        self.variant(TM_A::DISABLED_NORMAL_OPE)
    }
    #[doc = "Enabled. The TD pin will reflect the bit, detected on RD pin, with the next positive edge of the system clock."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TM_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Reset Mode."]
    #[inline(always)]
    pub fn rm(&self) -> RM_R {
        RM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Listen Only Mode."]
    #[inline(always)]
    pub fn lom(&self) -> LOM_R {
        LOM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Self Test Mode."]
    #[inline(always)]
    pub fn stm(&self) -> STM_R {
        STM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Priority Mode."]
    #[inline(always)]
    pub fn tpm(&self) -> TPM_R {
        TPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sleep Mode."]
    #[inline(always)]
    pub fn sm(&self) -> SM_R {
        SM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Polarity Mode."]
    #[inline(always)]
    pub fn rpm(&self) -> RPM_R {
        RPM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Test Mode."]
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MOD")
            .field("rm", &format_args!("{}", self.rm().bit()))
            .field("lom", &format_args!("{}", self.lom().bit()))
            .field("stm", &format_args!("{}", self.stm().bit()))
            .field("tpm", &format_args!("{}", self.tpm().bit()))
            .field("sm", &format_args!("{}", self.sm().bit()))
            .field("rpm", &format_args!("{}", self.rpm().bit()))
            .field("tm", &format_args!("{}", self.tm().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<MOD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Reset Mode."]
    #[inline(always)]
    #[must_use]
    pub fn rm(&mut self) -> RM_W<MOD_SPEC, 0> {
        RM_W::new(self)
    }
    #[doc = "Bit 1 - Listen Only Mode."]
    #[inline(always)]
    #[must_use]
    pub fn lom(&mut self) -> LOM_W<MOD_SPEC, 1> {
        LOM_W::new(self)
    }
    #[doc = "Bit 2 - Self Test Mode."]
    #[inline(always)]
    #[must_use]
    pub fn stm(&mut self) -> STM_W<MOD_SPEC, 2> {
        STM_W::new(self)
    }
    #[doc = "Bit 3 - Transmit Priority Mode."]
    #[inline(always)]
    #[must_use]
    pub fn tpm(&mut self) -> TPM_W<MOD_SPEC, 3> {
        TPM_W::new(self)
    }
    #[doc = "Bit 4 - Sleep Mode."]
    #[inline(always)]
    #[must_use]
    pub fn sm(&mut self) -> SM_W<MOD_SPEC, 4> {
        SM_W::new(self)
    }
    #[doc = "Bit 5 - Receive Polarity Mode."]
    #[inline(always)]
    #[must_use]
    pub fn rpm(&mut self) -> RPM_W<MOD_SPEC, 5> {
        RPM_W::new(self)
    }
    #[doc = "Bit 7 - Test Mode."]
    #[inline(always)]
    #[must_use]
    pub fn tm(&mut self) -> TM_W<MOD_SPEC, 7> {
        TM_W::new(self)
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
#[doc = "Controls the operating mode of the CAN Controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mod_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mod_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MOD_SPEC;
impl crate::RegisterSpec for MOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mod_::R`](R) reader structure"]
impl crate::Readable for MOD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mod_::W`](W) writer structure"]
impl crate::Writable for MOD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MOD to value 0"]
impl crate::Resettable for MOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
