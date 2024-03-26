#[doc = "Register `PCONP` reader"]
pub type R = crate::R<PCONP_SPEC>;
#[doc = "Register `PCONP` writer"]
pub type W = crate::W<PCONP_SPEC>;
#[doc = "Field `PCTIM0` reader - Timer/Counter 0 power/clock control bit."]
pub type PCTIM0_R = crate::BitReader;
#[doc = "Field `PCTIM0` writer - Timer/Counter 0 power/clock control bit."]
pub type PCTIM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCTIM1` reader - Timer/Counter 1 power/clock control bit."]
pub type PCTIM1_R = crate::BitReader;
#[doc = "Field `PCTIM1` writer - Timer/Counter 1 power/clock control bit."]
pub type PCTIM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCUART0` reader - UART0 power/clock control bit."]
pub type PCUART0_R = crate::BitReader;
#[doc = "Field `PCUART0` writer - UART0 power/clock control bit."]
pub type PCUART0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCUART1` reader - UART1 power/clock control bit."]
pub type PCUART1_R = crate::BitReader;
#[doc = "Field `PCUART1` writer - UART1 power/clock control bit."]
pub type PCUART1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCPWM1` reader - PWM1 power/clock control bit."]
pub type PCPWM1_R = crate::BitReader;
#[doc = "Field `PCPWM1` writer - PWM1 power/clock control bit."]
pub type PCPWM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCI2C0` reader - The I2C0 interface power/clock control bit."]
pub type PCI2C0_R = crate::BitReader;
#[doc = "Field `PCI2C0` writer - The I2C0 interface power/clock control bit."]
pub type PCI2C0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCSPI` reader - The SPI interface power/clock control bit."]
pub type PCSPI_R = crate::BitReader;
#[doc = "Field `PCSPI` writer - The SPI interface power/clock control bit."]
pub type PCSPI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCRTC` reader - The RTC power/clock control bit."]
pub type PCRTC_R = crate::BitReader;
#[doc = "Field `PCRTC` writer - The RTC power/clock control bit."]
pub type PCRTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCSSP1` reader - The SSP 1 interface power/clock control bit."]
pub type PCSSP1_R = crate::BitReader;
#[doc = "Field `PCSSP1` writer - The SSP 1 interface power/clock control bit."]
pub type PCSSP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCADC` reader - A/D converter (ADC) power/clock control bit. Note: Clear the PDN bit in the AD0CR before clearing this bit, and set this bit before setting PDN."]
pub type PCADC_R = crate::BitReader;
#[doc = "Field `PCADC` writer - A/D converter (ADC) power/clock control bit. Note: Clear the PDN bit in the AD0CR before clearing this bit, and set this bit before setting PDN."]
pub type PCADC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCCAN1` reader - CAN Controller 1 power/clock control bit."]
pub type PCCAN1_R = crate::BitReader;
#[doc = "Field `PCCAN1` writer - CAN Controller 1 power/clock control bit."]
pub type PCCAN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCCAN2` reader - CAN Controller 2 power/clock control bit."]
pub type PCCAN2_R = crate::BitReader;
#[doc = "Field `PCCAN2` writer - CAN Controller 2 power/clock control bit."]
pub type PCCAN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCGPIO` reader - Power/clock control bit for IOCON, GPIO, and GPIO interrupts."]
pub type PCGPIO_R = crate::BitReader;
#[doc = "Field `PCGPIO` writer - Power/clock control bit for IOCON, GPIO, and GPIO interrupts."]
pub type PCGPIO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCRIT` reader - Repetitive Interrupt Timer power/clock control bit."]
pub type PCRIT_R = crate::BitReader;
#[doc = "Field `PCRIT` writer - Repetitive Interrupt Timer power/clock control bit."]
pub type PCRIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCMCPWM` reader - Motor Control PWM"]
pub type PCMCPWM_R = crate::BitReader;
#[doc = "Field `PCMCPWM` writer - Motor Control PWM"]
pub type PCMCPWM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCQEI` reader - Quadrature Encoder Interface power/clock control bit."]
pub type PCQEI_R = crate::BitReader;
#[doc = "Field `PCQEI` writer - Quadrature Encoder Interface power/clock control bit."]
pub type PCQEI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCI2C1` reader - The I2C1 interface power/clock control bit."]
pub type PCI2C1_R = crate::BitReader;
#[doc = "Field `PCI2C1` writer - The I2C1 interface power/clock control bit."]
pub type PCI2C1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCSSP0` reader - The SSP0 interface power/clock control bit."]
pub type PCSSP0_R = crate::BitReader;
#[doc = "Field `PCSSP0` writer - The SSP0 interface power/clock control bit."]
pub type PCSSP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCTIM2` reader - Timer 2 power/clock control bit."]
pub type PCTIM2_R = crate::BitReader;
#[doc = "Field `PCTIM2` writer - Timer 2 power/clock control bit."]
pub type PCTIM2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCTIM3` reader - Timer 3 power/clock control bit."]
pub type PCTIM3_R = crate::BitReader;
#[doc = "Field `PCTIM3` writer - Timer 3 power/clock control bit."]
pub type PCTIM3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCUART2` reader - UART 2 power/clock control bit."]
pub type PCUART2_R = crate::BitReader;
#[doc = "Field `PCUART2` writer - UART 2 power/clock control bit."]
pub type PCUART2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCUART3` reader - UART 3 power/clock control bit."]
pub type PCUART3_R = crate::BitReader;
#[doc = "Field `PCUART3` writer - UART 3 power/clock control bit."]
pub type PCUART3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCI2C2` reader - I2C interface 2 power/clock control bit."]
pub type PCI2C2_R = crate::BitReader;
#[doc = "Field `PCI2C2` writer - I2C interface 2 power/clock control bit."]
pub type PCI2C2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCI2S` reader - I2S interface power/clock control bit."]
pub type PCI2S_R = crate::BitReader;
#[doc = "Field `PCI2S` writer - I2S interface power/clock control bit."]
pub type PCI2S_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCGPDMA` reader - GPDMA function power/clock control bit."]
pub type PCGPDMA_R = crate::BitReader;
#[doc = "Field `PCGPDMA` writer - GPDMA function power/clock control bit."]
pub type PCGPDMA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCENET` reader - Ethernet block power/clock control bit."]
pub type PCENET_R = crate::BitReader;
#[doc = "Field `PCENET` writer - Ethernet block power/clock control bit."]
pub type PCENET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCUSB` reader - USB interface power/clock control bit."]
pub type PCUSB_R = crate::BitReader;
#[doc = "Field `PCUSB` writer - USB interface power/clock control bit."]
pub type PCUSB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Timer/Counter 0 power/clock control bit."]
    #[inline(always)]
    pub fn pctim0(&self) -> PCTIM0_R {
        PCTIM0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter 1 power/clock control bit."]
    #[inline(always)]
    pub fn pctim1(&self) -> PCTIM1_R {
        PCTIM1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART0 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart0(&self) -> PCUART0_R {
        PCUART0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART1 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart1(&self) -> PCUART1_R {
        PCUART1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - PWM1 power/clock control bit."]
    #[inline(always)]
    pub fn pcpwm1(&self) -> PCPWM1_R {
        PCPWM1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The I2C0 interface power/clock control bit."]
    #[inline(always)]
    pub fn pci2c0(&self) -> PCI2C0_R {
        PCI2C0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The SPI interface power/clock control bit."]
    #[inline(always)]
    pub fn pcspi(&self) -> PCSPI_R {
        PCSPI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The RTC power/clock control bit."]
    #[inline(always)]
    pub fn pcrtc(&self) -> PCRTC_R {
        PCRTC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The SSP 1 interface power/clock control bit."]
    #[inline(always)]
    pub fn pcssp1(&self) -> PCSSP1_R {
        PCSSP1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - A/D converter (ADC) power/clock control bit. Note: Clear the PDN bit in the AD0CR before clearing this bit, and set this bit before setting PDN."]
    #[inline(always)]
    pub fn pcadc(&self) -> PCADC_R {
        PCADC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CAN Controller 1 power/clock control bit."]
    #[inline(always)]
    pub fn pccan1(&self) -> PCCAN1_R {
        PCCAN1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CAN Controller 2 power/clock control bit."]
    #[inline(always)]
    pub fn pccan2(&self) -> PCCAN2_R {
        PCCAN2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Power/clock control bit for IOCON, GPIO, and GPIO interrupts."]
    #[inline(always)]
    pub fn pcgpio(&self) -> PCGPIO_R {
        PCGPIO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Repetitive Interrupt Timer power/clock control bit."]
    #[inline(always)]
    pub fn pcrit(&self) -> PCRIT_R {
        PCRIT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Motor Control PWM"]
    #[inline(always)]
    pub fn pcmcpwm(&self) -> PCMCPWM_R {
        PCMCPWM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Quadrature Encoder Interface power/clock control bit."]
    #[inline(always)]
    pub fn pcqei(&self) -> PCQEI_R {
        PCQEI_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The I2C1 interface power/clock control bit."]
    #[inline(always)]
    pub fn pci2c1(&self) -> PCI2C1_R {
        PCI2C1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - The SSP0 interface power/clock control bit."]
    #[inline(always)]
    pub fn pcssp0(&self) -> PCSSP0_R {
        PCSSP0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Timer 2 power/clock control bit."]
    #[inline(always)]
    pub fn pctim2(&self) -> PCTIM2_R {
        PCTIM2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Timer 3 power/clock control bit."]
    #[inline(always)]
    pub fn pctim3(&self) -> PCTIM3_R {
        PCTIM3_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - UART 2 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart2(&self) -> PCUART2_R {
        PCUART2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - UART 3 power/clock control bit."]
    #[inline(always)]
    pub fn pcuart3(&self) -> PCUART3_R {
        PCUART3_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - I2C interface 2 power/clock control bit."]
    #[inline(always)]
    pub fn pci2c2(&self) -> PCI2C2_R {
        PCI2C2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - I2S interface power/clock control bit."]
    #[inline(always)]
    pub fn pci2s(&self) -> PCI2S_R {
        PCI2S_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - GPDMA function power/clock control bit."]
    #[inline(always)]
    pub fn pcgpdma(&self) -> PCGPDMA_R {
        PCGPDMA_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Ethernet block power/clock control bit."]
    #[inline(always)]
    pub fn pcenet(&self) -> PCENET_R {
        PCENET_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - USB interface power/clock control bit."]
    #[inline(always)]
    pub fn pcusb(&self) -> PCUSB_R {
        PCUSB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCONP")
            .field("pctim0", &format_args!("{}", self.pctim0().bit()))
            .field("pctim1", &format_args!("{}", self.pctim1().bit()))
            .field("pcuart0", &format_args!("{}", self.pcuart0().bit()))
            .field("pcuart1", &format_args!("{}", self.pcuart1().bit()))
            .field("pcpwm1", &format_args!("{}", self.pcpwm1().bit()))
            .field("pci2c0", &format_args!("{}", self.pci2c0().bit()))
            .field("pcspi", &format_args!("{}", self.pcspi().bit()))
            .field("pcrtc", &format_args!("{}", self.pcrtc().bit()))
            .field("pcssp1", &format_args!("{}", self.pcssp1().bit()))
            .field("pcadc", &format_args!("{}", self.pcadc().bit()))
            .field("pccan1", &format_args!("{}", self.pccan1().bit()))
            .field("pccan2", &format_args!("{}", self.pccan2().bit()))
            .field("pcgpio", &format_args!("{}", self.pcgpio().bit()))
            .field("pcrit", &format_args!("{}", self.pcrit().bit()))
            .field("pcmcpwm", &format_args!("{}", self.pcmcpwm().bit()))
            .field("pcqei", &format_args!("{}", self.pcqei().bit()))
            .field("pci2c1", &format_args!("{}", self.pci2c1().bit()))
            .field("pcssp0", &format_args!("{}", self.pcssp0().bit()))
            .field("pctim2", &format_args!("{}", self.pctim2().bit()))
            .field("pctim3", &format_args!("{}", self.pctim3().bit()))
            .field("pcuart2", &format_args!("{}", self.pcuart2().bit()))
            .field("pcuart3", &format_args!("{}", self.pcuart3().bit()))
            .field("pci2c2", &format_args!("{}", self.pci2c2().bit()))
            .field("pci2s", &format_args!("{}", self.pci2s().bit()))
            .field("pcgpdma", &format_args!("{}", self.pcgpdma().bit()))
            .field("pcenet", &format_args!("{}", self.pcenet().bit()))
            .field("pcusb", &format_args!("{}", self.pcusb().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PCONP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1 - Timer/Counter 0 power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pctim0(&mut self) -> PCTIM0_W<PCONP_SPEC, 1> {
        PCTIM0_W::new(self)
    }
    #[doc = "Bit 2 - Timer/Counter 1 power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pctim1(&mut self) -> PCTIM1_W<PCONP_SPEC, 2> {
        PCTIM1_W::new(self)
    }
    #[doc = "Bit 3 - UART0 power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcuart0(&mut self) -> PCUART0_W<PCONP_SPEC, 3> {
        PCUART0_W::new(self)
    }
    #[doc = "Bit 4 - UART1 power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcuart1(&mut self) -> PCUART1_W<PCONP_SPEC, 4> {
        PCUART1_W::new(self)
    }
    #[doc = "Bit 6 - PWM1 power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcpwm1(&mut self) -> PCPWM1_W<PCONP_SPEC, 6> {
        PCPWM1_W::new(self)
    }
    #[doc = "Bit 7 - The I2C0 interface power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pci2c0(&mut self) -> PCI2C0_W<PCONP_SPEC, 7> {
        PCI2C0_W::new(self)
    }
    #[doc = "Bit 8 - The SPI interface power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcspi(&mut self) -> PCSPI_W<PCONP_SPEC, 8> {
        PCSPI_W::new(self)
    }
    #[doc = "Bit 9 - The RTC power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcrtc(&mut self) -> PCRTC_W<PCONP_SPEC, 9> {
        PCRTC_W::new(self)
    }
    #[doc = "Bit 10 - The SSP 1 interface power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcssp1(&mut self) -> PCSSP1_W<PCONP_SPEC, 10> {
        PCSSP1_W::new(self)
    }
    #[doc = "Bit 12 - A/D converter (ADC) power/clock control bit. Note: Clear the PDN bit in the AD0CR before clearing this bit, and set this bit before setting PDN."]
    #[inline(always)]
    #[must_use]
    pub fn pcadc(&mut self) -> PCADC_W<PCONP_SPEC, 12> {
        PCADC_W::new(self)
    }
    #[doc = "Bit 13 - CAN Controller 1 power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pccan1(&mut self) -> PCCAN1_W<PCONP_SPEC, 13> {
        PCCAN1_W::new(self)
    }
    #[doc = "Bit 14 - CAN Controller 2 power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pccan2(&mut self) -> PCCAN2_W<PCONP_SPEC, 14> {
        PCCAN2_W::new(self)
    }
    #[doc = "Bit 15 - Power/clock control bit for IOCON, GPIO, and GPIO interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn pcgpio(&mut self) -> PCGPIO_W<PCONP_SPEC, 15> {
        PCGPIO_W::new(self)
    }
    #[doc = "Bit 16 - Repetitive Interrupt Timer power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcrit(&mut self) -> PCRIT_W<PCONP_SPEC, 16> {
        PCRIT_W::new(self)
    }
    #[doc = "Bit 17 - Motor Control PWM"]
    #[inline(always)]
    #[must_use]
    pub fn pcmcpwm(&mut self) -> PCMCPWM_W<PCONP_SPEC, 17> {
        PCMCPWM_W::new(self)
    }
    #[doc = "Bit 18 - Quadrature Encoder Interface power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcqei(&mut self) -> PCQEI_W<PCONP_SPEC, 18> {
        PCQEI_W::new(self)
    }
    #[doc = "Bit 19 - The I2C1 interface power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pci2c1(&mut self) -> PCI2C1_W<PCONP_SPEC, 19> {
        PCI2C1_W::new(self)
    }
    #[doc = "Bit 21 - The SSP0 interface power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcssp0(&mut self) -> PCSSP0_W<PCONP_SPEC, 21> {
        PCSSP0_W::new(self)
    }
    #[doc = "Bit 22 - Timer 2 power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pctim2(&mut self) -> PCTIM2_W<PCONP_SPEC, 22> {
        PCTIM2_W::new(self)
    }
    #[doc = "Bit 23 - Timer 3 power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pctim3(&mut self) -> PCTIM3_W<PCONP_SPEC, 23> {
        PCTIM3_W::new(self)
    }
    #[doc = "Bit 24 - UART 2 power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcuart2(&mut self) -> PCUART2_W<PCONP_SPEC, 24> {
        PCUART2_W::new(self)
    }
    #[doc = "Bit 25 - UART 3 power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcuart3(&mut self) -> PCUART3_W<PCONP_SPEC, 25> {
        PCUART3_W::new(self)
    }
    #[doc = "Bit 26 - I2C interface 2 power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pci2c2(&mut self) -> PCI2C2_W<PCONP_SPEC, 26> {
        PCI2C2_W::new(self)
    }
    #[doc = "Bit 27 - I2S interface power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pci2s(&mut self) -> PCI2S_W<PCONP_SPEC, 27> {
        PCI2S_W::new(self)
    }
    #[doc = "Bit 29 - GPDMA function power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcgpdma(&mut self) -> PCGPDMA_W<PCONP_SPEC, 29> {
        PCGPDMA_W::new(self)
    }
    #[doc = "Bit 30 - Ethernet block power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcenet(&mut self) -> PCENET_W<PCONP_SPEC, 30> {
        PCENET_W::new(self)
    }
    #[doc = "Bit 31 - USB interface power/clock control bit."]
    #[inline(always)]
    #[must_use]
    pub fn pcusb(&mut self) -> PCUSB_W<PCONP_SPEC, 31> {
        PCUSB_W::new(self)
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
#[doc = "Power Control for Peripherals Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pconp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pconp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCONP_SPEC;
impl crate::RegisterSpec for PCONP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pconp::R`](R) reader structure"]
impl crate::Readable for PCONP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pconp::W`](W) writer structure"]
impl crate::Writable for PCONP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCONP to value 0x03be"]
impl crate::Resettable for PCONP_SPEC {
    const RESET_VALUE: Self::Ux = 0x03be;
}
