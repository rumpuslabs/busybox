#[doc = "Register `PCON` reader"]
pub type R = crate::R<PCON_SPEC>;
#[doc = "Register `PCON` writer"]
pub type W = crate::W<PCON_SPEC>;
#[doc = "Field `PM0` reader - Power mode control bit 0. This bit controls entry to the Power-down mode."]
pub type PM0_R = crate::BitReader;
#[doc = "Field `PM0` writer - Power mode control bit 0. This bit controls entry to the Power-down mode."]
pub type PM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PM1` reader - Power mode control bit 1. This bit controls entry to the Deep Power-down mode."]
pub type PM1_R = crate::BitReader;
#[doc = "Field `PM1` writer - Power mode control bit 1. This bit controls entry to the Deep Power-down mode."]
pub type PM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BODRPM` reader - Brown-Out Reduced Power Mode. When BODRPM is 1, the Brown-Out Detect circuitry will be turned off when chip Power-down mode or Deep Sleep mode is entered, resulting in a further reduction in power usage. However, the possibility of using Brown-Out Detect as a wake-up source from the reduced power mode will be lost. When 0, the Brown-Out Detect function remains active during Power-down and Deep Sleep modes. See the System Control Block chapter for details of Brown-Out detection."]
pub type BODRPM_R = crate::BitReader;
#[doc = "Field `BODRPM` writer - Brown-Out Reduced Power Mode. When BODRPM is 1, the Brown-Out Detect circuitry will be turned off when chip Power-down mode or Deep Sleep mode is entered, resulting in a further reduction in power usage. However, the possibility of using Brown-Out Detect as a wake-up source from the reduced power mode will be lost. When 0, the Brown-Out Detect function remains active during Power-down and Deep Sleep modes. See the System Control Block chapter for details of Brown-Out detection."]
pub type BODRPM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BOGD` reader - Brown-Out Global Disable. When BOGD is 1, the Brown-Out Detect circuitry is fully disabled at all times, and does not consume power. When 0, the Brown-Out Detect circuitry is enabled. See the System Control Block chapter for details of Brown-Out detection. Note: the Brown-Out Reset Disable (BORD, in this register) and the Brown-Out Interrupt (xx) must be disabled when software changes the value of this bit."]
pub type BOGD_R = crate::BitReader;
#[doc = "Field `BOGD` writer - Brown-Out Global Disable. When BOGD is 1, the Brown-Out Detect circuitry is fully disabled at all times, and does not consume power. When 0, the Brown-Out Detect circuitry is enabled. See the System Control Block chapter for details of Brown-Out detection. Note: the Brown-Out Reset Disable (BORD, in this register) and the Brown-Out Interrupt (xx) must be disabled when software changes the value of this bit."]
pub type BOGD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BORD` reader - Brown-Out Reset Disable. When BORD is 1, the BOD will not reset the device when the VDD(REG)(3V3) voltage dips goes below the BOD reset trip level. The Brown-Out interrupt is not affected. When BORD is 0, the BOD reset is enabled."]
pub type BORD_R = crate::BitReader;
#[doc = "Field `BORD` writer - Brown-Out Reset Disable. When BORD is 1, the BOD will not reset the device when the VDD(REG)(3V3) voltage dips goes below the BOD reset trip level. The Brown-Out interrupt is not affected. When BORD is 0, the BOD reset is enabled."]
pub type BORD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SMFLAG` reader - Sleep Mode entry flag. Set when the Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
pub type SMFLAG_R = crate::BitReader;
#[doc = "Field `SMFLAG` writer - Sleep Mode entry flag. Set when the Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
pub type SMFLAG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DSFLAG` reader - Deep Sleep entry flag. Set when the Deep Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
pub type DSFLAG_R = crate::BitReader;
#[doc = "Field `DSFLAG` writer - Deep Sleep entry flag. Set when the Deep Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
pub type DSFLAG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDFLAG` reader - Power-down entry flag. Set when the Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
pub type PDFLAG_R = crate::BitReader;
#[doc = "Field `PDFLAG` writer - Power-down entry flag. Set when the Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
pub type PDFLAG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DPDFLAG` reader - Deep Power-down entry flag. Set when the Deep Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
pub type DPDFLAG_R = crate::BitReader;
#[doc = "Field `DPDFLAG` writer - Deep Power-down entry flag. Set when the Deep Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
pub type DPDFLAG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Power mode control bit 0. This bit controls entry to the Power-down mode."]
    #[inline(always)]
    pub fn pm0(&self) -> PM0_R {
        PM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power mode control bit 1. This bit controls entry to the Deep Power-down mode."]
    #[inline(always)]
    pub fn pm1(&self) -> PM1_R {
        PM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Brown-Out Reduced Power Mode. When BODRPM is 1, the Brown-Out Detect circuitry will be turned off when chip Power-down mode or Deep Sleep mode is entered, resulting in a further reduction in power usage. However, the possibility of using Brown-Out Detect as a wake-up source from the reduced power mode will be lost. When 0, the Brown-Out Detect function remains active during Power-down and Deep Sleep modes. See the System Control Block chapter for details of Brown-Out detection."]
    #[inline(always)]
    pub fn bodrpm(&self) -> BODRPM_R {
        BODRPM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Brown-Out Global Disable. When BOGD is 1, the Brown-Out Detect circuitry is fully disabled at all times, and does not consume power. When 0, the Brown-Out Detect circuitry is enabled. See the System Control Block chapter for details of Brown-Out detection. Note: the Brown-Out Reset Disable (BORD, in this register) and the Brown-Out Interrupt (xx) must be disabled when software changes the value of this bit."]
    #[inline(always)]
    pub fn bogd(&self) -> BOGD_R {
        BOGD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Brown-Out Reset Disable. When BORD is 1, the BOD will not reset the device when the VDD(REG)(3V3) voltage dips goes below the BOD reset trip level. The Brown-Out interrupt is not affected. When BORD is 0, the BOD reset is enabled."]
    #[inline(always)]
    pub fn bord(&self) -> BORD_R {
        BORD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Sleep Mode entry flag. Set when the Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    pub fn smflag(&self) -> SMFLAG_R {
        SMFLAG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Deep Sleep entry flag. Set when the Deep Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    pub fn dsflag(&self) -> DSFLAG_R {
        DSFLAG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Power-down entry flag. Set when the Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    pub fn pdflag(&self) -> PDFLAG_R {
        PDFLAG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Deep Power-down entry flag. Set when the Deep Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    pub fn dpdflag(&self) -> DPDFLAG_R {
        DPDFLAG_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCON")
            .field("pm0", &format_args!("{}", self.pm0().bit()))
            .field("pm1", &format_args!("{}", self.pm1().bit()))
            .field("bodrpm", &format_args!("{}", self.bodrpm().bit()))
            .field("bogd", &format_args!("{}", self.bogd().bit()))
            .field("bord", &format_args!("{}", self.bord().bit()))
            .field("smflag", &format_args!("{}", self.smflag().bit()))
            .field("dsflag", &format_args!("{}", self.dsflag().bit()))
            .field("pdflag", &format_args!("{}", self.pdflag().bit()))
            .field("dpdflag", &format_args!("{}", self.dpdflag().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PCON_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Power mode control bit 0. This bit controls entry to the Power-down mode."]
    #[inline(always)]
    #[must_use]
    pub fn pm0(&mut self) -> PM0_W<PCON_SPEC, 0> {
        PM0_W::new(self)
    }
    #[doc = "Bit 1 - Power mode control bit 1. This bit controls entry to the Deep Power-down mode."]
    #[inline(always)]
    #[must_use]
    pub fn pm1(&mut self) -> PM1_W<PCON_SPEC, 1> {
        PM1_W::new(self)
    }
    #[doc = "Bit 2 - Brown-Out Reduced Power Mode. When BODRPM is 1, the Brown-Out Detect circuitry will be turned off when chip Power-down mode or Deep Sleep mode is entered, resulting in a further reduction in power usage. However, the possibility of using Brown-Out Detect as a wake-up source from the reduced power mode will be lost. When 0, the Brown-Out Detect function remains active during Power-down and Deep Sleep modes. See the System Control Block chapter for details of Brown-Out detection."]
    #[inline(always)]
    #[must_use]
    pub fn bodrpm(&mut self) -> BODRPM_W<PCON_SPEC, 2> {
        BODRPM_W::new(self)
    }
    #[doc = "Bit 3 - Brown-Out Global Disable. When BOGD is 1, the Brown-Out Detect circuitry is fully disabled at all times, and does not consume power. When 0, the Brown-Out Detect circuitry is enabled. See the System Control Block chapter for details of Brown-Out detection. Note: the Brown-Out Reset Disable (BORD, in this register) and the Brown-Out Interrupt (xx) must be disabled when software changes the value of this bit."]
    #[inline(always)]
    #[must_use]
    pub fn bogd(&mut self) -> BOGD_W<PCON_SPEC, 3> {
        BOGD_W::new(self)
    }
    #[doc = "Bit 4 - Brown-Out Reset Disable. When BORD is 1, the BOD will not reset the device when the VDD(REG)(3V3) voltage dips goes below the BOD reset trip level. The Brown-Out interrupt is not affected. When BORD is 0, the BOD reset is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn bord(&mut self) -> BORD_W<PCON_SPEC, 4> {
        BORD_W::new(self)
    }
    #[doc = "Bit 8 - Sleep Mode entry flag. Set when the Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn smflag(&mut self) -> SMFLAG_W<PCON_SPEC, 8> {
        SMFLAG_W::new(self)
    }
    #[doc = "Bit 9 - Deep Sleep entry flag. Set when the Deep Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn dsflag(&mut self) -> DSFLAG_W<PCON_SPEC, 9> {
        DSFLAG_W::new(self)
    }
    #[doc = "Bit 10 - Power-down entry flag. Set when the Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn pdflag(&mut self) -> PDFLAG_W<PCON_SPEC, 10> {
        PDFLAG_W::new(self)
    }
    #[doc = "Bit 11 - Deep Power-down entry flag. Set when the Deep Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn dpdflag(&mut self) -> DPDFLAG_W<PCON_SPEC, 11> {
        DPDFLAG_W::new(self)
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
#[doc = "Power Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCON_SPEC;
impl crate::RegisterSpec for PCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcon::R`](R) reader structure"]
impl crate::Readable for PCON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcon::W`](W) writer structure"]
impl crate::Writable for PCON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCON to value 0"]
impl crate::Resettable for PCON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
