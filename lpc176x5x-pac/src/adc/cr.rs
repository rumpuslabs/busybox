#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `SEL` reader - Selects which of the AD0\\[7:0\\]
pins is (are) to be sampled and converted. For AD0, bit 0 selects Pin AD0\\[0\\], and bit 7 selects pin AD0\\[7\\]. In software-controlled mode, only one of these bits should be 1. In hardware scan mode, any value containing 1 to 8 ones is allowed. All zeroes is equivalent to 0x01."]
pub type SEL_R = crate::FieldReader;
#[doc = "Field `SEL` writer - Selects which of the AD0\\[7:0\\]
pins is (are) to be sampled and converted. For AD0, bit 0 selects Pin AD0\\[0\\], and bit 7 selects pin AD0\\[7\\]. In software-controlled mode, only one of these bits should be 1. In hardware scan mode, any value containing 1 to 8 ones is allowed. All zeroes is equivalent to 0x01."]
pub type SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CLKDIV` reader - The APB clock (PCLK) is divided by (this value plus one) to produce the clock for the A/D converter, which should be less than or equal to 12.4 MHz. Typically, software should program the smallest value in this field that yields a clock of 12.4 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
pub type CLKDIV_R = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - The APB clock (PCLK) is divided by (this value plus one) to produce the clock for the A/D converter, which should be less than or equal to 12.4 MHz. Typically, software should program the smallest value in this field that yields a clock of 12.4 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
pub type CLKDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `BURST` reader - Burst mode"]
pub type BURST_R = crate::BitReader<BURST_A>;
#[doc = "Burst mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BURST_A {
    #[doc = "1: The AD converter does repeated conversions at up to 400 kHz, scanning (if necessary) through the pins selected by bits set to ones in the SEL field. The first conversion after the start corresponds to the least-significant 1 in the SEL field, then higher numbered 1-bits (pins) if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion that's in progress when this bit is cleared will be completed. START bits must be 000 when BURST = 1 or conversions will not start."]
    BURST = 1,
    #[doc = "0: Conversions are software controlled and require 31 clocks."]
    SW = 0,
}
impl From<BURST_A> for bool {
    #[inline(always)]
    fn from(variant: BURST_A) -> Self {
        variant as u8 != 0
    }
}
impl BURST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BURST_A {
        match self.bits {
            true => BURST_A::BURST,
            false => BURST_A::SW,
        }
    }
    #[doc = "The AD converter does repeated conversions at up to 400 kHz, scanning (if necessary) through the pins selected by bits set to ones in the SEL field. The first conversion after the start corresponds to the least-significant 1 in the SEL field, then higher numbered 1-bits (pins) if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion that's in progress when this bit is cleared will be completed. START bits must be 000 when BURST = 1 or conversions will not start."]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == BURST_A::BURST
    }
    #[doc = "Conversions are software controlled and require 31 clocks."]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == BURST_A::SW
    }
}
#[doc = "Field `BURST` writer - Burst mode"]
pub type BURST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BURST_A>;
impl<'a, REG, const O: u8> BURST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The AD converter does repeated conversions at up to 400 kHz, scanning (if necessary) through the pins selected by bits set to ones in the SEL field. The first conversion after the start corresponds to the least-significant 1 in the SEL field, then higher numbered 1-bits (pins) if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion that's in progress when this bit is cleared will be completed. START bits must be 000 when BURST = 1 or conversions will not start."]
    #[inline(always)]
    pub fn burst(self) -> &'a mut crate::W<REG> {
        self.variant(BURST_A::BURST)
    }
    #[doc = "Conversions are software controlled and require 31 clocks."]
    #[inline(always)]
    pub fn sw(self) -> &'a mut crate::W<REG> {
        self.variant(BURST_A::SW)
    }
}
#[doc = "Field `PDN` reader - Power down mode"]
pub type PDN_R = crate::BitReader<PDN_A>;
#[doc = "Power down mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDN_A {
    #[doc = "1: The A/D converter is operational."]
    POWERED = 1,
    #[doc = "0: The A/D converter is in power-down mode."]
    POWERDOWN = 0,
}
impl From<PDN_A> for bool {
    #[inline(always)]
    fn from(variant: PDN_A) -> Self {
        variant as u8 != 0
    }
}
impl PDN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDN_A {
        match self.bits {
            true => PDN_A::POWERED,
            false => PDN_A::POWERDOWN,
        }
    }
    #[doc = "The A/D converter is operational."]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == PDN_A::POWERED
    }
    #[doc = "The A/D converter is in power-down mode."]
    #[inline(always)]
    pub fn is_powerdown(&self) -> bool {
        *self == PDN_A::POWERDOWN
    }
}
#[doc = "Field `PDN` writer - Power down mode"]
pub type PDN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PDN_A>;
impl<'a, REG, const O: u8> PDN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The A/D converter is operational."]
    #[inline(always)]
    pub fn powered(self) -> &'a mut crate::W<REG> {
        self.variant(PDN_A::POWERED)
    }
    #[doc = "The A/D converter is in power-down mode."]
    #[inline(always)]
    pub fn powerdown(self) -> &'a mut crate::W<REG> {
        self.variant(PDN_A::POWERDOWN)
    }
}
#[doc = "Field `START` reader - When the BURST bit is 0, these bits control whether and when an A/D conversion is started:"]
pub type START_R = crate::FieldReader<START_A>;
#[doc = "When the BURST bit is 0, these bits control whether and when an A/D conversion is started:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum START_A {
    #[doc = "0: No start (this value should be used when clearing PDN to 0)."]
    NO_START = 0,
    #[doc = "1: Start conversion now."]
    START_CONVERSION_NOW = 1,
    #[doc = "2: Start conversion when the edge selected by bit 27 occurs on the P2\\[10\\]
pin."]
    P2_10 = 2,
    #[doc = "3: Start conversion when the edge selected by bit 27 occurs on the P1\\[27\\]
pin."]
    P1_27 = 3,
    #[doc = "4: Start conversion when the edge selected by bit 27 occurs on MAT0.1. Note that this does not require that the MAT0.1 function appear on a device pin."]
    MAT0_1 = 4,
    #[doc = "5: Start conversion when the edge selected by bit 27 occurs on MAT0.3. Note that it is not possible to cause the MAT0.3 function to appear on a device pin."]
    MAT0_3 = 5,
    #[doc = "6: Start conversion when the edge selected by bit 27 occurs on MAT1.0. Note that this does not require that the MAT1.0 function appear on a device pin."]
    MAT1_0 = 6,
    #[doc = "7: Start conversion when the edge selected by bit 27 occurs on MAT1.1. Note that this does not require that the MAT1.1 function appear on a device pin."]
    MAT1_1 = 7,
}
impl From<START_A> for u8 {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for START_A {
    type Ux = u8;
}
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> START_A {
        match self.bits {
            0 => START_A::NO_START,
            1 => START_A::START_CONVERSION_NOW,
            2 => START_A::P2_10,
            3 => START_A::P1_27,
            4 => START_A::MAT0_1,
            5 => START_A::MAT0_3,
            6 => START_A::MAT1_0,
            7 => START_A::MAT1_1,
            _ => unreachable!(),
        }
    }
    #[doc = "No start (this value should be used when clearing PDN to 0)."]
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == START_A::NO_START
    }
    #[doc = "Start conversion now."]
    #[inline(always)]
    pub fn is_start_conversion_now(&self) -> bool {
        *self == START_A::START_CONVERSION_NOW
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on the P2\\[10\\]
pin."]
    #[inline(always)]
    pub fn is_p2_10(&self) -> bool {
        *self == START_A::P2_10
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on the P1\\[27\\]
pin."]
    #[inline(always)]
    pub fn is_p1_27(&self) -> bool {
        *self == START_A::P1_27
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT0.1. Note that this does not require that the MAT0.1 function appear on a device pin."]
    #[inline(always)]
    pub fn is_mat0_1(&self) -> bool {
        *self == START_A::MAT0_1
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT0.3. Note that it is not possible to cause the MAT0.3 function to appear on a device pin."]
    #[inline(always)]
    pub fn is_mat0_3(&self) -> bool {
        *self == START_A::MAT0_3
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT1.0. Note that this does not require that the MAT1.0 function appear on a device pin."]
    #[inline(always)]
    pub fn is_mat1_0(&self) -> bool {
        *self == START_A::MAT1_0
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT1.1. Note that this does not require that the MAT1.1 function appear on a device pin."]
    #[inline(always)]
    pub fn is_mat1_1(&self) -> bool {
        *self == START_A::MAT1_1
    }
}
#[doc = "Field `START` writer - When the BURST bit is 0, these bits control whether and when an A/D conversion is started:"]
pub type START_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, START_A>;
impl<'a, REG, const O: u8> START_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No start (this value should be used when clearing PDN to 0)."]
    #[inline(always)]
    pub fn no_start(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::NO_START)
    }
    #[doc = "Start conversion now."]
    #[inline(always)]
    pub fn start_conversion_now(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::START_CONVERSION_NOW)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on the P2\\[10\\]
pin."]
    #[inline(always)]
    pub fn p2_10(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::P2_10)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on the P1\\[27\\]
pin."]
    #[inline(always)]
    pub fn p1_27(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::P1_27)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT0.1. Note that this does not require that the MAT0.1 function appear on a device pin."]
    #[inline(always)]
    pub fn mat0_1(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::MAT0_1)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT0.3. Note that it is not possible to cause the MAT0.3 function to appear on a device pin."]
    #[inline(always)]
    pub fn mat0_3(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::MAT0_3)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT1.0. Note that this does not require that the MAT1.0 function appear on a device pin."]
    #[inline(always)]
    pub fn mat1_0(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::MAT1_0)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT1.1. Note that this does not require that the MAT1.1 function appear on a device pin."]
    #[inline(always)]
    pub fn mat1_1(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::MAT1_1)
    }
}
#[doc = "Field `EDGE` reader - This bit is significant only when the START field contains 010-111. In these cases:"]
pub type EDGE_R = crate::BitReader<EDGE_A>;
#[doc = "This bit is significant only when the START field contains 010-111. In these cases:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE_A {
    #[doc = "1: Start conversion on a falling edge on the selected CAP/MAT signal."]
    FALLLING = 1,
    #[doc = "0: Start conversion on a rising edge on the selected CAP/MAT signal."]
    RISING = 0,
}
impl From<EDGE_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EDGE_A {
        match self.bits {
            true => EDGE_A::FALLLING,
            false => EDGE_A::RISING,
        }
    }
    #[doc = "Start conversion on a falling edge on the selected CAP/MAT signal."]
    #[inline(always)]
    pub fn is_fallling(&self) -> bool {
        *self == EDGE_A::FALLLING
    }
    #[doc = "Start conversion on a rising edge on the selected CAP/MAT signal."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == EDGE_A::RISING
    }
}
#[doc = "Field `EDGE` writer - This bit is significant only when the START field contains 010-111. In these cases:"]
pub type EDGE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EDGE_A>;
impl<'a, REG, const O: u8> EDGE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start conversion on a falling edge on the selected CAP/MAT signal."]
    #[inline(always)]
    pub fn fallling(self) -> &'a mut crate::W<REG> {
        self.variant(EDGE_A::FALLLING)
    }
    #[doc = "Start conversion on a rising edge on the selected CAP/MAT signal."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(EDGE_A::RISING)
    }
}
impl R {
    #[doc = "Bits 0:7 - Selects which of the AD0\\[7:0\\]
pins is (are) to be sampled and converted. For AD0, bit 0 selects Pin AD0\\[0\\], and bit 7 selects pin AD0\\[7\\]. In software-controlled mode, only one of these bits should be 1. In hardware scan mode, any value containing 1 to 8 ones is allowed. All zeroes is equivalent to 0x01."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The APB clock (PCLK) is divided by (this value plus one) to produce the clock for the A/D converter, which should be less than or equal to 12.4 MHz. Typically, software should program the smallest value in this field that yields a clock of 12.4 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Burst mode"]
    #[inline(always)]
    pub fn burst(&self) -> BURST_R {
        BURST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - Power down mode"]
    #[inline(always)]
    pub fn pdn(&self) -> PDN_R {
        PDN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:26 - When the BURST bit is 0, these bits control whether and when an A/D conversion is started:"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - This bit is significant only when the START field contains 010-111. In these cases:"]
    #[inline(always)]
    pub fn edge(&self) -> EDGE_R {
        EDGE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("sel", &format_args!("{}", self.sel().bits()))
            .field("clkdiv", &format_args!("{}", self.clkdiv().bits()))
            .field("burst", &format_args!("{}", self.burst().bit()))
            .field("pdn", &format_args!("{}", self.pdn().bit()))
            .field("start", &format_args!("{}", self.start().bits()))
            .field("edge", &format_args!("{}", self.edge().bit()))
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
    #[doc = "Bits 0:7 - Selects which of the AD0\\[7:0\\]
pins is (are) to be sampled and converted. For AD0, bit 0 selects Pin AD0\\[0\\], and bit 7 selects pin AD0\\[7\\]. In software-controlled mode, only one of these bits should be 1. In hardware scan mode, any value containing 1 to 8 ones is allowed. All zeroes is equivalent to 0x01."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<CR_SPEC, 0> {
        SEL_W::new(self)
    }
    #[doc = "Bits 8:15 - The APB clock (PCLK) is divided by (this value plus one) to produce the clock for the A/D converter, which should be less than or equal to 12.4 MHz. Typically, software should program the smallest value in this field that yields a clock of 12.4 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<CR_SPEC, 8> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bit 16 - Burst mode"]
    #[inline(always)]
    #[must_use]
    pub fn burst(&mut self) -> BURST_W<CR_SPEC, 16> {
        BURST_W::new(self)
    }
    #[doc = "Bit 21 - Power down mode"]
    #[inline(always)]
    #[must_use]
    pub fn pdn(&mut self) -> PDN_W<CR_SPEC, 21> {
        PDN_W::new(self)
    }
    #[doc = "Bits 24:26 - When the BURST bit is 0, these bits control whether and when an A/D conversion is started:"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CR_SPEC, 24> {
        START_W::new(self)
    }
    #[doc = "Bit 27 - This bit is significant only when the START field contains 010-111. In these cases:"]
    #[inline(always)]
    #[must_use]
    pub fn edge(&mut self) -> EDGE_W<CR_SPEC, 27> {
        EDGE_W::new(self)
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
#[doc = "A/D Control Register. The ADCR register must be written to select the operating mode before A/D conversion can occur.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
