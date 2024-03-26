#[doc = "Register `CR0` reader"]
pub type R = crate::R<CR0_SPEC>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<CR0_SPEC>;
#[doc = "Field `DSS` reader - Data Size Select. This field controls the number of bits transferred in each frame. Values 0000-0010 are not supported and should not be used."]
pub type DSS_R = crate::FieldReader<DSS_A>;
#[doc = "Data Size Select. This field controls the number of bits transferred in each frame. Values 0000-0010 are not supported and should not be used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DSS_A {
    #[doc = "3: 4-bit transfer"]
    _4_BITS = 3,
    #[doc = "4: 5-bit transfer"]
    _5_BITS = 4,
    #[doc = "5: 6-bit transfer"]
    _6_BITS = 5,
    #[doc = "6: 7-bit transfer"]
    _7_BITS = 6,
    #[doc = "7: 8-bit transfer"]
    _8_BITS = 7,
    #[doc = "8: 9-bit transfer"]
    _9_BITS = 8,
    #[doc = "9: 10-bit transfer"]
    _10_BITS = 9,
    #[doc = "10: 11-bit transfer"]
    _11_BITS = 10,
    #[doc = "11: 12-bit transfer"]
    _12_BITS = 11,
    #[doc = "12: 13-bit transfer"]
    _13_BITS = 12,
    #[doc = "13: 14-bit transfer"]
    _14_BITS = 13,
    #[doc = "14: 15-bit transfer"]
    _15_BITS = 14,
    #[doc = "15: 16-bit transfer"]
    _16_BITS = 15,
}
impl From<DSS_A> for u8 {
    #[inline(always)]
    fn from(variant: DSS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DSS_A {
    type Ux = u8;
}
impl DSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DSS_A> {
        match self.bits {
            3 => Some(DSS_A::_4_BITS),
            4 => Some(DSS_A::_5_BITS),
            5 => Some(DSS_A::_6_BITS),
            6 => Some(DSS_A::_7_BITS),
            7 => Some(DSS_A::_8_BITS),
            8 => Some(DSS_A::_9_BITS),
            9 => Some(DSS_A::_10_BITS),
            10 => Some(DSS_A::_11_BITS),
            11 => Some(DSS_A::_12_BITS),
            12 => Some(DSS_A::_13_BITS),
            13 => Some(DSS_A::_14_BITS),
            14 => Some(DSS_A::_15_BITS),
            15 => Some(DSS_A::_16_BITS),
            _ => None,
        }
    }
    #[doc = "4-bit transfer"]
    #[inline(always)]
    pub fn is_4_bits(&self) -> bool {
        *self == DSS_A::_4_BITS
    }
    #[doc = "5-bit transfer"]
    #[inline(always)]
    pub fn is_5_bits(&self) -> bool {
        *self == DSS_A::_5_BITS
    }
    #[doc = "6-bit transfer"]
    #[inline(always)]
    pub fn is_6_bits(&self) -> bool {
        *self == DSS_A::_6_BITS
    }
    #[doc = "7-bit transfer"]
    #[inline(always)]
    pub fn is_7_bits(&self) -> bool {
        *self == DSS_A::_7_BITS
    }
    #[doc = "8-bit transfer"]
    #[inline(always)]
    pub fn is_8_bits(&self) -> bool {
        *self == DSS_A::_8_BITS
    }
    #[doc = "9-bit transfer"]
    #[inline(always)]
    pub fn is_9_bits(&self) -> bool {
        *self == DSS_A::_9_BITS
    }
    #[doc = "10-bit transfer"]
    #[inline(always)]
    pub fn is_10_bits(&self) -> bool {
        *self == DSS_A::_10_BITS
    }
    #[doc = "11-bit transfer"]
    #[inline(always)]
    pub fn is_11_bits(&self) -> bool {
        *self == DSS_A::_11_BITS
    }
    #[doc = "12-bit transfer"]
    #[inline(always)]
    pub fn is_12_bits(&self) -> bool {
        *self == DSS_A::_12_BITS
    }
    #[doc = "13-bit transfer"]
    #[inline(always)]
    pub fn is_13_bits(&self) -> bool {
        *self == DSS_A::_13_BITS
    }
    #[doc = "14-bit transfer"]
    #[inline(always)]
    pub fn is_14_bits(&self) -> bool {
        *self == DSS_A::_14_BITS
    }
    #[doc = "15-bit transfer"]
    #[inline(always)]
    pub fn is_15_bits(&self) -> bool {
        *self == DSS_A::_15_BITS
    }
    #[doc = "16-bit transfer"]
    #[inline(always)]
    pub fn is_16_bits(&self) -> bool {
        *self == DSS_A::_16_BITS
    }
}
#[doc = "Field `DSS` writer - Data Size Select. This field controls the number of bits transferred in each frame. Values 0000-0010 are not supported and should not be used."]
pub type DSS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, DSS_A>;
impl<'a, REG, const O: u8> DSS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4-bit transfer"]
    #[inline(always)]
    pub fn _4_bits(self) -> &'a mut crate::W<REG> {
        self.variant(DSS_A::_4_BITS)
    }
    #[doc = "5-bit transfer"]
    #[inline(always)]
    pub fn _5_bits(self) -> &'a mut crate::W<REG> {
        self.variant(DSS_A::_5_BITS)
    }
    #[doc = "6-bit transfer"]
    #[inline(always)]
    pub fn _6_bits(self) -> &'a mut crate::W<REG> {
        self.variant(DSS_A::_6_BITS)
    }
    #[doc = "7-bit transfer"]
    #[inline(always)]
    pub fn _7_bits(self) -> &'a mut crate::W<REG> {
        self.variant(DSS_A::_7_BITS)
    }
    #[doc = "8-bit transfer"]
    #[inline(always)]
    pub fn _8_bits(self) -> &'a mut crate::W<REG> {
        self.variant(DSS_A::_8_BITS)
    }
    #[doc = "9-bit transfer"]
    #[inline(always)]
    pub fn _9_bits(self) -> &'a mut crate::W<REG> {
        self.variant(DSS_A::_9_BITS)
    }
    #[doc = "10-bit transfer"]
    #[inline(always)]
    pub fn _10_bits(self) -> &'a mut crate::W<REG> {
        self.variant(DSS_A::_10_BITS)
    }
    #[doc = "11-bit transfer"]
    #[inline(always)]
    pub fn _11_bits(self) -> &'a mut crate::W<REG> {
        self.variant(DSS_A::_11_BITS)
    }
    #[doc = "12-bit transfer"]
    #[inline(always)]
    pub fn _12_bits(self) -> &'a mut crate::W<REG> {
        self.variant(DSS_A::_12_BITS)
    }
    #[doc = "13-bit transfer"]
    #[inline(always)]
    pub fn _13_bits(self) -> &'a mut crate::W<REG> {
        self.variant(DSS_A::_13_BITS)
    }
    #[doc = "14-bit transfer"]
    #[inline(always)]
    pub fn _14_bits(self) -> &'a mut crate::W<REG> {
        self.variant(DSS_A::_14_BITS)
    }
    #[doc = "15-bit transfer"]
    #[inline(always)]
    pub fn _15_bits(self) -> &'a mut crate::W<REG> {
        self.variant(DSS_A::_15_BITS)
    }
    #[doc = "16-bit transfer"]
    #[inline(always)]
    pub fn _16_bits(self) -> &'a mut crate::W<REG> {
        self.variant(DSS_A::_16_BITS)
    }
}
#[doc = "Field `FRF` reader - Frame Format."]
pub type FRF_R = crate::FieldReader<FRF_A>;
#[doc = "Frame Format.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FRF_A {
    #[doc = "0: SPI"]
    SPI = 0,
    #[doc = "1: TI"]
    TI = 1,
    #[doc = "2: Microwire"]
    MICROWIRE = 2,
}
impl From<FRF_A> for u8 {
    #[inline(always)]
    fn from(variant: FRF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FRF_A {
    type Ux = u8;
}
impl FRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FRF_A {
        match self.bits {
            0 => FRF_A::SPI,
            1 => FRF_A::TI,
            2 => FRF_A::MICROWIRE,
            _ => unreachable!(),
        }
    }
    #[doc = "SPI"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == FRF_A::SPI
    }
    #[doc = "TI"]
    #[inline(always)]
    pub fn is_ti(&self) -> bool {
        *self == FRF_A::TI
    }
    #[doc = "Microwire"]
    #[inline(always)]
    pub fn is_microwire(&self) -> bool {
        *self == FRF_A::MICROWIRE
    }
}
#[doc = "Field `FRF` writer - Frame Format."]
pub type FRF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, FRF_A>;
impl<'a, REG, const O: u8> FRF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SPI"]
    #[inline(always)]
    pub fn spi(self) -> &'a mut crate::W<REG> {
        self.variant(FRF_A::SPI)
    }
    #[doc = "TI"]
    #[inline(always)]
    pub fn ti(self) -> &'a mut crate::W<REG> {
        self.variant(FRF_A::TI)
    }
    #[doc = "Microwire"]
    #[inline(always)]
    pub fn microwire(self) -> &'a mut crate::W<REG> {
        self.variant(FRF_A::MICROWIRE)
    }
}
#[doc = "Field `CPOL` reader - Clock Out Polarity. This bit is only used in SPI mode."]
pub type CPOL_R = crate::BitReader<CPOL_A>;
#[doc = "Clock Out Polarity. This bit is only used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL_A {
    #[doc = "0: SSP controller maintains the bus clock low between frames."]
    BUS_LOW = 0,
    #[doc = "1: SSP controller maintains the bus clock high between frames."]
    BUS_HIGH = 1,
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
            false => CPOL_A::BUS_LOW,
            true => CPOL_A::BUS_HIGH,
        }
    }
    #[doc = "SSP controller maintains the bus clock low between frames."]
    #[inline(always)]
    pub fn is_bus_low(&self) -> bool {
        *self == CPOL_A::BUS_LOW
    }
    #[doc = "SSP controller maintains the bus clock high between frames."]
    #[inline(always)]
    pub fn is_bus_high(&self) -> bool {
        *self == CPOL_A::BUS_HIGH
    }
}
#[doc = "Field `CPOL` writer - Clock Out Polarity. This bit is only used in SPI mode."]
pub type CPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CPOL_A>;
impl<'a, REG, const O: u8> CPOL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SSP controller maintains the bus clock low between frames."]
    #[inline(always)]
    pub fn bus_low(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL_A::BUS_LOW)
    }
    #[doc = "SSP controller maintains the bus clock high between frames."]
    #[inline(always)]
    pub fn bus_high(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL_A::BUS_HIGH)
    }
}
#[doc = "Field `CPHA` reader - Clock Out Phase. This bit is only used in SPI mode."]
pub type CPHA_R = crate::BitReader<CPHA_A>;
#[doc = "Clock Out Phase. This bit is only used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHA_A {
    #[doc = "0: SSP controller captures serial data on the first clock transition of the frame, that is, the transition away from the inter-frame state of the clock line."]
    FIRST_CLOCK = 0,
    #[doc = "1: SSP controller captures serial data on the second clock transition of the frame, that is, the transition back to the inter-frame state of the clock line."]
    SECOND_CLOCK = 1,
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
            false => CPHA_A::FIRST_CLOCK,
            true => CPHA_A::SECOND_CLOCK,
        }
    }
    #[doc = "SSP controller captures serial data on the first clock transition of the frame, that is, the transition away from the inter-frame state of the clock line."]
    #[inline(always)]
    pub fn is_first_clock(&self) -> bool {
        *self == CPHA_A::FIRST_CLOCK
    }
    #[doc = "SSP controller captures serial data on the second clock transition of the frame, that is, the transition back to the inter-frame state of the clock line."]
    #[inline(always)]
    pub fn is_second_clock(&self) -> bool {
        *self == CPHA_A::SECOND_CLOCK
    }
}
#[doc = "Field `CPHA` writer - Clock Out Phase. This bit is only used in SPI mode."]
pub type CPHA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CPHA_A>;
impl<'a, REG, const O: u8> CPHA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SSP controller captures serial data on the first clock transition of the frame, that is, the transition away from the inter-frame state of the clock line."]
    #[inline(always)]
    pub fn first_clock(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA_A::FIRST_CLOCK)
    }
    #[doc = "SSP controller captures serial data on the second clock transition of the frame, that is, the transition back to the inter-frame state of the clock line."]
    #[inline(always)]
    pub fn second_clock(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA_A::SECOND_CLOCK)
    }
}
#[doc = "Field `SCR` reader - Serial Clock Rate. The number of prescaler-output clocks per bit on the bus, minus one. Given that CPSDVSR is the prescale divider, and the APB clock PCLK clocks the prescaler, the bit frequency is PCLK / (CPSDVSR X \\[SCR+1\\])."]
pub type SCR_R = crate::FieldReader;
#[doc = "Field `SCR` writer - Serial Clock Rate. The number of prescaler-output clocks per bit on the bus, minus one. Given that CPSDVSR is the prescale divider, and the APB clock PCLK clocks the prescaler, the bit frequency is PCLK / (CPSDVSR X \\[SCR+1\\])."]
pub type SCR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:3 - Data Size Select. This field controls the number of bits transferred in each frame. Values 0000-0010 are not supported and should not be used."]
    #[inline(always)]
    pub fn dss(&self) -> DSS_R {
        DSS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Frame Format."]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Clock Out Polarity. This bit is only used in SPI mode."]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock Out Phase. This bit is only used in SPI mode."]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Serial Clock Rate. The number of prescaler-output clocks per bit on the bus, minus one. Given that CPSDVSR is the prescale divider, and the APB clock PCLK clocks the prescaler, the bit frequency is PCLK / (CPSDVSR X \\[SCR+1\\])."]
    #[inline(always)]
    pub fn scr(&self) -> SCR_R {
        SCR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR0")
            .field("dss", &format_args!("{}", self.dss().bits()))
            .field("frf", &format_args!("{}", self.frf().bits()))
            .field("cpol", &format_args!("{}", self.cpol().bit()))
            .field("cpha", &format_args!("{}", self.cpha().bit()))
            .field("scr", &format_args!("{}", self.scr().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CR0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Size Select. This field controls the number of bits transferred in each frame. Values 0000-0010 are not supported and should not be used."]
    #[inline(always)]
    #[must_use]
    pub fn dss(&mut self) -> DSS_W<CR0_SPEC, 0> {
        DSS_W::new(self)
    }
    #[doc = "Bits 4:5 - Frame Format."]
    #[inline(always)]
    #[must_use]
    pub fn frf(&mut self) -> FRF_W<CR0_SPEC, 4> {
        FRF_W::new(self)
    }
    #[doc = "Bit 6 - Clock Out Polarity. This bit is only used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<CR0_SPEC, 6> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 7 - Clock Out Phase. This bit is only used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<CR0_SPEC, 7> {
        CPHA_W::new(self)
    }
    #[doc = "Bits 8:15 - Serial Clock Rate. The number of prescaler-output clocks per bit on the bus, minus one. Given that CPSDVSR is the prescale divider, and the APB clock PCLK clocks the prescaler, the bit frequency is PCLK / (CPSDVSR X \\[SCR+1\\])."]
    #[inline(always)]
    #[must_use]
    pub fn scr(&mut self) -> SCR_W<CR0_SPEC, 8> {
        SCR_W::new(self)
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
#[doc = "Control Register 0. Selects the serial clock rate, bus type, and data size.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR0_SPEC;
impl crate::RegisterSpec for CR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for CR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for CR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for CR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
