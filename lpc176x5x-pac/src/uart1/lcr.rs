#[doc = "Register `LCR` reader"]
pub type R = crate::R<LCR_SPEC>;
#[doc = "Register `LCR` writer"]
pub type W = crate::W<LCR_SPEC>;
#[doc = "Field `WLS` reader - Word Length Select."]
pub type WLS_R = crate::FieldReader<WLS_A>;
#[doc = "Word Length Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WLS_A {
    #[doc = "0: 5-bit character length."]
    _5_BITS = 0,
    #[doc = "1: 6-bit character length."]
    _6_BITS = 1,
    #[doc = "2: 7-bit character length."]
    _7_BITS = 2,
    #[doc = "3: 8-bit character length."]
    _8_BITS = 3,
}
impl From<WLS_A> for u8 {
    #[inline(always)]
    fn from(variant: WLS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WLS_A {
    type Ux = u8;
}
impl WLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WLS_A {
        match self.bits {
            0 => WLS_A::_5_BITS,
            1 => WLS_A::_6_BITS,
            2 => WLS_A::_7_BITS,
            3 => WLS_A::_8_BITS,
            _ => unreachable!(),
        }
    }
    #[doc = "5-bit character length."]
    #[inline(always)]
    pub fn is_5_bits(&self) -> bool {
        *self == WLS_A::_5_BITS
    }
    #[doc = "6-bit character length."]
    #[inline(always)]
    pub fn is_6_bits(&self) -> bool {
        *self == WLS_A::_6_BITS
    }
    #[doc = "7-bit character length."]
    #[inline(always)]
    pub fn is_7_bits(&self) -> bool {
        *self == WLS_A::_7_BITS
    }
    #[doc = "8-bit character length."]
    #[inline(always)]
    pub fn is_8_bits(&self) -> bool {
        *self == WLS_A::_8_BITS
    }
}
#[doc = "Field `WLS` writer - Word Length Select."]
pub type WLS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, WLS_A>;
impl<'a, REG, const O: u8> WLS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "5-bit character length."]
    #[inline(always)]
    pub fn _5_bits(self) -> &'a mut crate::W<REG> {
        self.variant(WLS_A::_5_BITS)
    }
    #[doc = "6-bit character length."]
    #[inline(always)]
    pub fn _6_bits(self) -> &'a mut crate::W<REG> {
        self.variant(WLS_A::_6_BITS)
    }
    #[doc = "7-bit character length."]
    #[inline(always)]
    pub fn _7_bits(self) -> &'a mut crate::W<REG> {
        self.variant(WLS_A::_7_BITS)
    }
    #[doc = "8-bit character length."]
    #[inline(always)]
    pub fn _8_bits(self) -> &'a mut crate::W<REG> {
        self.variant(WLS_A::_8_BITS)
    }
}
#[doc = "Field `SBS` reader - Stop Bit Select."]
pub type SBS_R = crate::BitReader<SBS_A>;
#[doc = "Stop Bit Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBS_A {
    #[doc = "0: 1 stop bit."]
    _1_STOP_BIT = 0,
    #[doc = "1: 2 stop bits (1.5 if LCR\\[1:0\\]=00)."]
    _2_STOP_BITS = 1,
}
impl From<SBS_A> for bool {
    #[inline(always)]
    fn from(variant: SBS_A) -> Self {
        variant as u8 != 0
    }
}
impl SBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SBS_A {
        match self.bits {
            false => SBS_A::_1_STOP_BIT,
            true => SBS_A::_2_STOP_BITS,
        }
    }
    #[doc = "1 stop bit."]
    #[inline(always)]
    pub fn is_1_stop_bit(&self) -> bool {
        *self == SBS_A::_1_STOP_BIT
    }
    #[doc = "2 stop bits (1.5 if LCR\\[1:0\\]=00)."]
    #[inline(always)]
    pub fn is_2_stop_bits(&self) -> bool {
        *self == SBS_A::_2_STOP_BITS
    }
}
#[doc = "Field `SBS` writer - Stop Bit Select."]
pub type SBS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SBS_A>;
impl<'a, REG, const O: u8> SBS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1 stop bit."]
    #[inline(always)]
    pub fn _1_stop_bit(self) -> &'a mut crate::W<REG> {
        self.variant(SBS_A::_1_STOP_BIT)
    }
    #[doc = "2 stop bits (1.5 if LCR\\[1:0\\]=00)."]
    #[inline(always)]
    pub fn _2_stop_bits(self) -> &'a mut crate::W<REG> {
        self.variant(SBS_A::_2_STOP_BITS)
    }
}
#[doc = "Field `PE` reader - Parity Enable."]
pub type PE_R = crate::BitReader<PE_A>;
#[doc = "Parity Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PE_A {
    #[doc = "0: Disable parity generation and checking."]
    DISABLE_PARITY_GENER = 0,
    #[doc = "1: Enable parity generation and checking."]
    ENABLE_PARITY_GENERA = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
impl PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::DISABLE_PARITY_GENER,
            true => PE_A::ENABLE_PARITY_GENERA,
        }
    }
    #[doc = "Disable parity generation and checking."]
    #[inline(always)]
    pub fn is_disable_parity_gener(&self) -> bool {
        *self == PE_A::DISABLE_PARITY_GENER
    }
    #[doc = "Enable parity generation and checking."]
    #[inline(always)]
    pub fn is_enable_parity_genera(&self) -> bool {
        *self == PE_A::ENABLE_PARITY_GENERA
    }
}
#[doc = "Field `PE` writer - Parity Enable."]
pub type PE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PE_A>;
impl<'a, REG, const O: u8> PE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable parity generation and checking."]
    #[inline(always)]
    pub fn disable_parity_gener(self) -> &'a mut crate::W<REG> {
        self.variant(PE_A::DISABLE_PARITY_GENER)
    }
    #[doc = "Enable parity generation and checking."]
    #[inline(always)]
    pub fn enable_parity_genera(self) -> &'a mut crate::W<REG> {
        self.variant(PE_A::ENABLE_PARITY_GENERA)
    }
}
#[doc = "Field `PS` reader - Parity Select."]
pub type PS_R = crate::FieldReader<PS_A>;
#[doc = "Parity Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PS_A {
    #[doc = "0: Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd."]
    ODD_PARITY = 0,
    #[doc = "1: Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even."]
    EVEN_PARITY = 1,
    #[doc = "2: Forced 1 stick parity."]
    FORCED_1 = 2,
    #[doc = "3: Forced 0 stick parity."]
    FORCED_0 = 3,
}
impl From<PS_A> for u8 {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PS_A {
    type Ux = u8;
}
impl PS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PS_A {
        match self.bits {
            0 => PS_A::ODD_PARITY,
            1 => PS_A::EVEN_PARITY,
            2 => PS_A::FORCED_1,
            3 => PS_A::FORCED_0,
            _ => unreachable!(),
        }
    }
    #[doc = "Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd."]
    #[inline(always)]
    pub fn is_odd_parity(&self) -> bool {
        *self == PS_A::ODD_PARITY
    }
    #[doc = "Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even."]
    #[inline(always)]
    pub fn is_even_parity(&self) -> bool {
        *self == PS_A::EVEN_PARITY
    }
    #[doc = "Forced 1 stick parity."]
    #[inline(always)]
    pub fn is_forced_1(&self) -> bool {
        *self == PS_A::FORCED_1
    }
    #[doc = "Forced 0 stick parity."]
    #[inline(always)]
    pub fn is_forced_0(&self) -> bool {
        *self == PS_A::FORCED_0
    }
}
#[doc = "Field `PS` writer - Parity Select."]
pub type PS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PS_A>;
impl<'a, REG, const O: u8> PS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd."]
    #[inline(always)]
    pub fn odd_parity(self) -> &'a mut crate::W<REG> {
        self.variant(PS_A::ODD_PARITY)
    }
    #[doc = "Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even."]
    #[inline(always)]
    pub fn even_parity(self) -> &'a mut crate::W<REG> {
        self.variant(PS_A::EVEN_PARITY)
    }
    #[doc = "Forced 1 stick parity."]
    #[inline(always)]
    pub fn forced_1(self) -> &'a mut crate::W<REG> {
        self.variant(PS_A::FORCED_1)
    }
    #[doc = "Forced 0 stick parity."]
    #[inline(always)]
    pub fn forced_0(self) -> &'a mut crate::W<REG> {
        self.variant(PS_A::FORCED_0)
    }
}
#[doc = "Field `BC` reader - Break Control."]
pub type BC_R = crate::BitReader<BC_A>;
#[doc = "Break Control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BC_A {
    #[doc = "0: Disable break transmission."]
    DISABLE_BREAK_TRANSM = 0,
    #[doc = "1: Enable break transmission. Output pin UART1 TXD is forced to logic 0 when LCR\\[6\\]
is active high."]
    ENABLE_BREAK_TRANSMI = 1,
}
impl From<BC_A> for bool {
    #[inline(always)]
    fn from(variant: BC_A) -> Self {
        variant as u8 != 0
    }
}
impl BC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BC_A {
        match self.bits {
            false => BC_A::DISABLE_BREAK_TRANSM,
            true => BC_A::ENABLE_BREAK_TRANSMI,
        }
    }
    #[doc = "Disable break transmission."]
    #[inline(always)]
    pub fn is_disable_break_transm(&self) -> bool {
        *self == BC_A::DISABLE_BREAK_TRANSM
    }
    #[doc = "Enable break transmission. Output pin UART1 TXD is forced to logic 0 when LCR\\[6\\]
is active high."]
    #[inline(always)]
    pub fn is_enable_break_transmi(&self) -> bool {
        *self == BC_A::ENABLE_BREAK_TRANSMI
    }
}
#[doc = "Field `BC` writer - Break Control."]
pub type BC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BC_A>;
impl<'a, REG, const O: u8> BC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable break transmission."]
    #[inline(always)]
    pub fn disable_break_transm(self) -> &'a mut crate::W<REG> {
        self.variant(BC_A::DISABLE_BREAK_TRANSM)
    }
    #[doc = "Enable break transmission. Output pin UART1 TXD is forced to logic 0 when LCR\\[6\\]
is active high."]
    #[inline(always)]
    pub fn enable_break_transmi(self) -> &'a mut crate::W<REG> {
        self.variant(BC_A::ENABLE_BREAK_TRANSMI)
    }
}
#[doc = "Field `DLAB` reader - Divisor Latch Access Bit (DLAB)"]
pub type DLAB_R = crate::BitReader<DLAB_A>;
#[doc = "Divisor Latch Access Bit (DLAB)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLAB_A {
    #[doc = "0: Disable access to Divisor Latches."]
    DISABLE_ACCESS_TO_DI = 0,
    #[doc = "1: Enable access to Divisor Latches."]
    ENABLE_ACCESS_TO_DIV = 1,
}
impl From<DLAB_A> for bool {
    #[inline(always)]
    fn from(variant: DLAB_A) -> Self {
        variant as u8 != 0
    }
}
impl DLAB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DLAB_A {
        match self.bits {
            false => DLAB_A::DISABLE_ACCESS_TO_DI,
            true => DLAB_A::ENABLE_ACCESS_TO_DIV,
        }
    }
    #[doc = "Disable access to Divisor Latches."]
    #[inline(always)]
    pub fn is_disable_access_to_di(&self) -> bool {
        *self == DLAB_A::DISABLE_ACCESS_TO_DI
    }
    #[doc = "Enable access to Divisor Latches."]
    #[inline(always)]
    pub fn is_enable_access_to_div(&self) -> bool {
        *self == DLAB_A::ENABLE_ACCESS_TO_DIV
    }
}
#[doc = "Field `DLAB` writer - Divisor Latch Access Bit (DLAB)"]
pub type DLAB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DLAB_A>;
impl<'a, REG, const O: u8> DLAB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable access to Divisor Latches."]
    #[inline(always)]
    pub fn disable_access_to_di(self) -> &'a mut crate::W<REG> {
        self.variant(DLAB_A::DISABLE_ACCESS_TO_DI)
    }
    #[doc = "Enable access to Divisor Latches."]
    #[inline(always)]
    pub fn enable_access_to_div(self) -> &'a mut crate::W<REG> {
        self.variant(DLAB_A::ENABLE_ACCESS_TO_DIV)
    }
}
impl R {
    #[doc = "Bits 0:1 - Word Length Select."]
    #[inline(always)]
    pub fn wls(&self) -> WLS_R {
        WLS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Stop Bit Select."]
    #[inline(always)]
    pub fn sbs(&self) -> SBS_R {
        SBS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity Enable."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Parity Select."]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Break Control."]
    #[inline(always)]
    pub fn bc(&self) -> BC_R {
        BC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit (DLAB)"]
    #[inline(always)]
    pub fn dlab(&self) -> DLAB_R {
        DLAB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCR")
            .field("wls", &format_args!("{}", self.wls().bits()))
            .field("sbs", &format_args!("{}", self.sbs().bit()))
            .field("pe", &format_args!("{}", self.pe().bit()))
            .field("ps", &format_args!("{}", self.ps().bits()))
            .field("bc", &format_args!("{}", self.bc().bit()))
            .field("dlab", &format_args!("{}", self.dlab().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<LCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Word Length Select."]
    #[inline(always)]
    #[must_use]
    pub fn wls(&mut self) -> WLS_W<LCR_SPEC, 0> {
        WLS_W::new(self)
    }
    #[doc = "Bit 2 - Stop Bit Select."]
    #[inline(always)]
    #[must_use]
    pub fn sbs(&mut self) -> SBS_W<LCR_SPEC, 2> {
        SBS_W::new(self)
    }
    #[doc = "Bit 3 - Parity Enable."]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<LCR_SPEC, 3> {
        PE_W::new(self)
    }
    #[doc = "Bits 4:5 - Parity Select."]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<LCR_SPEC, 4> {
        PS_W::new(self)
    }
    #[doc = "Bit 6 - Break Control."]
    #[inline(always)]
    #[must_use]
    pub fn bc(&mut self) -> BC_W<LCR_SPEC, 6> {
        BC_W::new(self)
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit (DLAB)"]
    #[inline(always)]
    #[must_use]
    pub fn dlab(&mut self) -> DLAB_W<LCR_SPEC, 7> {
        DLAB_W::new(self)
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
#[doc = "Line Control Register. Contains controls for frame formatting and break generation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCR_SPEC;
impl crate::RegisterSpec for LCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcr::R`](R) reader structure"]
impl crate::Readable for LCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcr::W`](W) writer structure"]
impl crate::Writable for LCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCR to value 0"]
impl crate::Resettable for LCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
