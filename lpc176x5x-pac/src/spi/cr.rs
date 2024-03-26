#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `BITENABLE` reader - "]
pub type BITENABLE_R = crate::BitReader<BITENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BITENABLE_A {
    #[doc = "0: The SPI controller sends and receives 8 bits of data per transfer."]
    DISABLED = 0,
    #[doc = "1: The SPI controller sends and receives the number of bits selected by bits 11:8."]
    ENABLED = 1,
}
impl From<BITENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: BITENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl BITENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BITENABLE_A {
        match self.bits {
            false => BITENABLE_A::DISABLED,
            true => BITENABLE_A::ENABLED,
        }
    }
    #[doc = "The SPI controller sends and receives 8 bits of data per transfer."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BITENABLE_A::DISABLED
    }
    #[doc = "The SPI controller sends and receives the number of bits selected by bits 11:8."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BITENABLE_A::ENABLED
    }
}
#[doc = "Field `BITENABLE` writer - "]
pub type BITENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BITENABLE_A>;
impl<'a, REG, const O: u8> BITENABLE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The SPI controller sends and receives 8 bits of data per transfer."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BITENABLE_A::DISABLED)
    }
    #[doc = "The SPI controller sends and receives the number of bits selected by bits 11:8."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BITENABLE_A::ENABLED)
    }
}
#[doc = "Field `CPHA` reader - Clock phase control determines the relationship between the data and the clock on SPI transfers, and controls when a slave transfer is defined as starting and ending."]
pub type CPHA_R = crate::BitReader<CPHA_A>;
#[doc = "Clock phase control determines the relationship between the data and the clock on SPI transfers, and controls when a slave transfer is defined as starting and ending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHA_A {
    #[doc = "0: Data is sampled on the first clock edge of SCK. A transfer starts and ends with activation and deactivation of the SSEL signal."]
    FIRST_EDGE = 0,
    #[doc = "1: Data is sampled on the second clock edge of the SCK. A transfer starts with the first clock edge, and ends with the last sampling edge when the SSEL signal is active."]
    SECOND_EDGE = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
impl CPHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::FIRST_EDGE,
            true => CPHA_A::SECOND_EDGE,
        }
    }
    #[doc = "Data is sampled on the first clock edge of SCK. A transfer starts and ends with activation and deactivation of the SSEL signal."]
    #[inline(always)]
    pub fn is_first_edge(&self) -> bool {
        *self == CPHA_A::FIRST_EDGE
    }
    #[doc = "Data is sampled on the second clock edge of the SCK. A transfer starts with the first clock edge, and ends with the last sampling edge when the SSEL signal is active."]
    #[inline(always)]
    pub fn is_second_edge(&self) -> bool {
        *self == CPHA_A::SECOND_EDGE
    }
}
#[doc = "Field `CPHA` writer - Clock phase control determines the relationship between the data and the clock on SPI transfers, and controls when a slave transfer is defined as starting and ending."]
pub type CPHA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CPHA_A>;
impl<'a, REG, const O: u8> CPHA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is sampled on the first clock edge of SCK. A transfer starts and ends with activation and deactivation of the SSEL signal."]
    #[inline(always)]
    pub fn first_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA_A::FIRST_EDGE)
    }
    #[doc = "Data is sampled on the second clock edge of the SCK. A transfer starts with the first clock edge, and ends with the last sampling edge when the SSEL signal is active."]
    #[inline(always)]
    pub fn second_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA_A::SECOND_EDGE)
    }
}
#[doc = "Field `CPOL` reader - Clock polarity control."]
pub type CPOL_R = crate::BitReader<CPOL_A>;
#[doc = "Clock polarity control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL_A {
    #[doc = "0: SCK is active high."]
    ACTIVE_HIGH = 0,
    #[doc = "1: SCK is active low."]
    ACTIVE_LOW = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl CPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::ACTIVE_HIGH,
            true => CPOL_A::ACTIVE_LOW,
        }
    }
    #[doc = "SCK is active high."]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == CPOL_A::ACTIVE_HIGH
    }
    #[doc = "SCK is active low."]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == CPOL_A::ACTIVE_LOW
    }
}
#[doc = "Field `CPOL` writer - Clock polarity control."]
pub type CPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CPOL_A>;
impl<'a, REG, const O: u8> CPOL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SCK is active high."]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL_A::ACTIVE_HIGH)
    }
    #[doc = "SCK is active low."]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL_A::ACTIVE_LOW)
    }
}
#[doc = "Field `MSTR` reader - Master mode select."]
pub type MSTR_R = crate::BitReader<MSTR_A>;
#[doc = "Master mode select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTR_A {
    #[doc = "0: The SPI operates in Slave mode."]
    SLAVE = 0,
    #[doc = "1: The SPI operates in Master mode."]
    MASTER = 1,
}
impl From<MSTR_A> for bool {
    #[inline(always)]
    fn from(variant: MSTR_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSTR_A {
        match self.bits {
            false => MSTR_A::SLAVE,
            true => MSTR_A::MASTER,
        }
    }
    #[doc = "The SPI operates in Slave mode."]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MSTR_A::SLAVE
    }
    #[doc = "The SPI operates in Master mode."]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MSTR_A::MASTER
    }
}
#[doc = "Field `MSTR` writer - Master mode select."]
pub type MSTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MSTR_A>;
impl<'a, REG, const O: u8> MSTR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The SPI operates in Slave mode."]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(MSTR_A::SLAVE)
    }
    #[doc = "The SPI operates in Master mode."]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(MSTR_A::MASTER)
    }
}
#[doc = "Field `LSBF` reader - LSB First controls which direction each byte is shifted when transferred."]
pub type LSBF_R = crate::BitReader<LSBF_A>;
#[doc = "LSB First controls which direction each byte is shifted when transferred.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSBF_A {
    #[doc = "0: SPI data is transferred MSB (bit 7) first."]
    MSB = 0,
    #[doc = "1: SPI data is transferred LSB (bit 0) first."]
    LSB = 1,
}
impl From<LSBF_A> for bool {
    #[inline(always)]
    fn from(variant: LSBF_A) -> Self {
        variant as u8 != 0
    }
}
impl LSBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSBF_A {
        match self.bits {
            false => LSBF_A::MSB,
            true => LSBF_A::LSB,
        }
    }
    #[doc = "SPI data is transferred MSB (bit 7) first."]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == LSBF_A::MSB
    }
    #[doc = "SPI data is transferred LSB (bit 0) first."]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == LSBF_A::LSB
    }
}
#[doc = "Field `LSBF` writer - LSB First controls which direction each byte is shifted when transferred."]
pub type LSBF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LSBF_A>;
impl<'a, REG, const O: u8> LSBF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI data is transferred MSB (bit 7) first."]
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(LSBF_A::MSB)
    }
    #[doc = "SPI data is transferred LSB (bit 0) first."]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
        self.variant(LSBF_A::LSB)
    }
}
#[doc = "Field `SPIE` reader - Serial peripheral interrupt enable."]
pub type SPIE_R = crate::BitReader<SPIE_A>;
#[doc = "Serial peripheral interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPIE_A {
    #[doc = "0: SPI interrupts are inhibited."]
    INTBLOCK = 0,
    #[doc = "1: A hardware interrupt is generated each time the SPIF or MODF bits are activated."]
    HWINT = 1,
}
impl From<SPIE_A> for bool {
    #[inline(always)]
    fn from(variant: SPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPIE_A {
        match self.bits {
            false => SPIE_A::INTBLOCK,
            true => SPIE_A::HWINT,
        }
    }
    #[doc = "SPI interrupts are inhibited."]
    #[inline(always)]
    pub fn is_intblock(&self) -> bool {
        *self == SPIE_A::INTBLOCK
    }
    #[doc = "A hardware interrupt is generated each time the SPIF or MODF bits are activated."]
    #[inline(always)]
    pub fn is_hwint(&self) -> bool {
        *self == SPIE_A::HWINT
    }
}
#[doc = "Field `SPIE` writer - Serial peripheral interrupt enable."]
pub type SPIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPIE_A>;
impl<'a, REG, const O: u8> SPIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI interrupts are inhibited."]
    #[inline(always)]
    pub fn intblock(self) -> &'a mut crate::W<REG> {
        self.variant(SPIE_A::INTBLOCK)
    }
    #[doc = "A hardware interrupt is generated each time the SPIF or MODF bits are activated."]
    #[inline(always)]
    pub fn hwint(self) -> &'a mut crate::W<REG> {
        self.variant(SPIE_A::HWINT)
    }
}
#[doc = "Field `BITS` reader - When bit 2 of this register is 1, this field controls the number of bits per transfer:"]
pub type BITS_R = crate::FieldReader<BITS_A>;
#[doc = "When bit 2 of this register is 1, this field controls the number of bits per transfer:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BITS_A {
    #[doc = "8: 8 bits per transfer"]
    _8_BITS = 8,
    #[doc = "9: 9 bits per transfer"]
    _9_BITS = 9,
    #[doc = "10: 10 bits per transfer"]
    _10_BITS = 10,
    #[doc = "11: 11 bits per transfer"]
    _11_BITS = 11,
    #[doc = "12: 12 bits per transfer"]
    _12_BITS = 12,
    #[doc = "13: 13 bits per transfer"]
    _13_BITS = 13,
    #[doc = "14: 14 bits per transfer"]
    _14_BITS = 14,
    #[doc = "15: 15 bits per transfer"]
    _15_BITS = 15,
    #[doc = "0: 16 bits per transfer"]
    _16_BITS = 0,
}
impl From<BITS_A> for u8 {
    #[inline(always)]
    fn from(variant: BITS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BITS_A {
    type Ux = u8;
}
impl BITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BITS_A> {
        match self.bits {
            8 => Some(BITS_A::_8_BITS),
            9 => Some(BITS_A::_9_BITS),
            10 => Some(BITS_A::_10_BITS),
            11 => Some(BITS_A::_11_BITS),
            12 => Some(BITS_A::_12_BITS),
            13 => Some(BITS_A::_13_BITS),
            14 => Some(BITS_A::_14_BITS),
            15 => Some(BITS_A::_15_BITS),
            0 => Some(BITS_A::_16_BITS),
            _ => None,
        }
    }
    #[doc = "8 bits per transfer"]
    #[inline(always)]
    pub fn is_8_bits(&self) -> bool {
        *self == BITS_A::_8_BITS
    }
    #[doc = "9 bits per transfer"]
    #[inline(always)]
    pub fn is_9_bits(&self) -> bool {
        *self == BITS_A::_9_BITS
    }
    #[doc = "10 bits per transfer"]
    #[inline(always)]
    pub fn is_10_bits(&self) -> bool {
        *self == BITS_A::_10_BITS
    }
    #[doc = "11 bits per transfer"]
    #[inline(always)]
    pub fn is_11_bits(&self) -> bool {
        *self == BITS_A::_11_BITS
    }
    #[doc = "12 bits per transfer"]
    #[inline(always)]
    pub fn is_12_bits(&self) -> bool {
        *self == BITS_A::_12_BITS
    }
    #[doc = "13 bits per transfer"]
    #[inline(always)]
    pub fn is_13_bits(&self) -> bool {
        *self == BITS_A::_13_BITS
    }
    #[doc = "14 bits per transfer"]
    #[inline(always)]
    pub fn is_14_bits(&self) -> bool {
        *self == BITS_A::_14_BITS
    }
    #[doc = "15 bits per transfer"]
    #[inline(always)]
    pub fn is_15_bits(&self) -> bool {
        *self == BITS_A::_15_BITS
    }
    #[doc = "16 bits per transfer"]
    #[inline(always)]
    pub fn is_16_bits(&self) -> bool {
        *self == BITS_A::_16_BITS
    }
}
#[doc = "Field `BITS` writer - When bit 2 of this register is 1, this field controls the number of bits per transfer:"]
pub type BITS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, BITS_A>;
impl<'a, REG, const O: u8> BITS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits per transfer"]
    #[inline(always)]
    pub fn _8_bits(self) -> &'a mut crate::W<REG> {
        self.variant(BITS_A::_8_BITS)
    }
    #[doc = "9 bits per transfer"]
    #[inline(always)]
    pub fn _9_bits(self) -> &'a mut crate::W<REG> {
        self.variant(BITS_A::_9_BITS)
    }
    #[doc = "10 bits per transfer"]
    #[inline(always)]
    pub fn _10_bits(self) -> &'a mut crate::W<REG> {
        self.variant(BITS_A::_10_BITS)
    }
    #[doc = "11 bits per transfer"]
    #[inline(always)]
    pub fn _11_bits(self) -> &'a mut crate::W<REG> {
        self.variant(BITS_A::_11_BITS)
    }
    #[doc = "12 bits per transfer"]
    #[inline(always)]
    pub fn _12_bits(self) -> &'a mut crate::W<REG> {
        self.variant(BITS_A::_12_BITS)
    }
    #[doc = "13 bits per transfer"]
    #[inline(always)]
    pub fn _13_bits(self) -> &'a mut crate::W<REG> {
        self.variant(BITS_A::_13_BITS)
    }
    #[doc = "14 bits per transfer"]
    #[inline(always)]
    pub fn _14_bits(self) -> &'a mut crate::W<REG> {
        self.variant(BITS_A::_14_BITS)
    }
    #[doc = "15 bits per transfer"]
    #[inline(always)]
    pub fn _15_bits(self) -> &'a mut crate::W<REG> {
        self.variant(BITS_A::_15_BITS)
    }
    #[doc = "16 bits per transfer"]
    #[inline(always)]
    pub fn _16_bits(self) -> &'a mut crate::W<REG> {
        self.variant(BITS_A::_16_BITS)
    }
}
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn bitenable(&self) -> BITENABLE_R {
        BITENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock phase control determines the relationship between the data and the clock on SPI transfers, and controls when a slave transfer is defined as starting and ending."]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock polarity control."]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master mode select."]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LSB First controls which direction each byte is shifted when transferred."]
    #[inline(always)]
    pub fn lsbf(&self) -> LSBF_R {
        LSBF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Serial peripheral interrupt enable."]
    #[inline(always)]
    pub fn spie(&self) -> SPIE_R {
        SPIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - When bit 2 of this register is 1, this field controls the number of bits per transfer:"]
    #[inline(always)]
    pub fn bits_(&self) -> BITS_R {
        BITS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("bitenable", &format_args!("{}", self.bitenable().bit()))
            .field("cpha", &format_args!("{}", self.cpha().bit()))
            .field("cpol", &format_args!("{}", self.cpol().bit()))
            .field("mstr", &format_args!("{}", self.mstr().bit()))
            .field("lsbf", &format_args!("{}", self.lsbf().bit()))
            .field("spie", &format_args!("{}", self.spie().bit()))
            .field("bits_", &format_args!("{}", self.bits_().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn bitenable(&mut self) -> BITENABLE_W<CR_SPEC, 2> {
        BITENABLE_W::new(self)
    }
    #[doc = "Bit 3 - Clock phase control determines the relationship between the data and the clock on SPI transfers, and controls when a slave transfer is defined as starting and ending."]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<CR_SPEC, 3> {
        CPHA_W::new(self)
    }
    #[doc = "Bit 4 - Clock polarity control."]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<CR_SPEC, 4> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 5 - Master mode select."]
    #[inline(always)]
    #[must_use]
    pub fn mstr(&mut self) -> MSTR_W<CR_SPEC, 5> {
        MSTR_W::new(self)
    }
    #[doc = "Bit 6 - LSB First controls which direction each byte is shifted when transferred."]
    #[inline(always)]
    #[must_use]
    pub fn lsbf(&mut self) -> LSBF_W<CR_SPEC, 6> {
        LSBF_W::new(self)
    }
    #[doc = "Bit 7 - Serial peripheral interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn spie(&mut self) -> SPIE_W<CR_SPEC, 7> {
        SPIE_W::new(self)
    }
    #[doc = "Bits 8:11 - When bit 2 of this register is 1, this field controls the number of bits per transfer:"]
    #[inline(always)]
    #[must_use]
    pub fn bits_(&mut self) -> BITS_W<CR_SPEC, 8> {
        BITS_W::new(self)
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
#[doc = "SPI Control Register. This register controls the operation of the SPI.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
