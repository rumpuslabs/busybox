#[doc = "Register `CALIBRATION` reader"]
pub type R = crate::R<CALIBRATION_SPEC>;
#[doc = "Register `CALIBRATION` writer"]
pub type W = crate::W<CALIBRATION_SPEC>;
#[doc = "Field `CALVAL` reader - If enabled, the calibration counter counts up to this value. The maximum value is 131, 072 corresponding to about 36.4 hours. Calibration is disabled if CALVAL = 0."]
pub type CALVAL_R = crate::FieldReader<u32>;
#[doc = "Field `CALVAL` writer - If enabled, the calibration counter counts up to this value. The maximum value is 131, 072 corresponding to about 36.4 hours. Calibration is disabled if CALVAL = 0."]
pub type CALVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 17, O, u32>;
#[doc = "Field `CALDIR` reader - Calibration direction"]
pub type CALDIR_R = crate::BitReader<CALDIR_A>;
#[doc = "Calibration direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALDIR_A {
    #[doc = "1: Backward calibration. When CALVAL is equal to the calibration counter, the RTC timers will stop incrementing for 1 second."]
    BACKWARD_CALIBRATION = 1,
    #[doc = "0: Forward calibration. When CALVAL is equal to the calibration counter, the RTC timers will jump by 2 seconds."]
    FORWARD_CALIBRATION = 0,
}
impl From<CALDIR_A> for bool {
    #[inline(always)]
    fn from(variant: CALDIR_A) -> Self {
        variant as u8 != 0
    }
}
impl CALDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CALDIR_A {
        match self.bits {
            true => CALDIR_A::BACKWARD_CALIBRATION,
            false => CALDIR_A::FORWARD_CALIBRATION,
        }
    }
    #[doc = "Backward calibration. When CALVAL is equal to the calibration counter, the RTC timers will stop incrementing for 1 second."]
    #[inline(always)]
    pub fn is_backward_calibration(&self) -> bool {
        *self == CALDIR_A::BACKWARD_CALIBRATION
    }
    #[doc = "Forward calibration. When CALVAL is equal to the calibration counter, the RTC timers will jump by 2 seconds."]
    #[inline(always)]
    pub fn is_forward_calibration(&self) -> bool {
        *self == CALDIR_A::FORWARD_CALIBRATION
    }
}
#[doc = "Field `CALDIR` writer - Calibration direction"]
pub type CALDIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CALDIR_A>;
impl<'a, REG, const O: u8> CALDIR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Backward calibration. When CALVAL is equal to the calibration counter, the RTC timers will stop incrementing for 1 second."]
    #[inline(always)]
    pub fn backward_calibration(self) -> &'a mut crate::W<REG> {
        self.variant(CALDIR_A::BACKWARD_CALIBRATION)
    }
    #[doc = "Forward calibration. When CALVAL is equal to the calibration counter, the RTC timers will jump by 2 seconds."]
    #[inline(always)]
    pub fn forward_calibration(self) -> &'a mut crate::W<REG> {
        self.variant(CALDIR_A::FORWARD_CALIBRATION)
    }
}
impl R {
    #[doc = "Bits 0:16 - If enabled, the calibration counter counts up to this value. The maximum value is 131, 072 corresponding to about 36.4 hours. Calibration is disabled if CALVAL = 0."]
    #[inline(always)]
    pub fn calval(&self) -> CALVAL_R {
        CALVAL_R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bit 17 - Calibration direction"]
    #[inline(always)]
    pub fn caldir(&self) -> CALDIR_R {
        CALDIR_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALIBRATION")
            .field("calval", &format_args!("{}", self.calval().bits()))
            .field("caldir", &format_args!("{}", self.caldir().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CALIBRATION_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:16 - If enabled, the calibration counter counts up to this value. The maximum value is 131, 072 corresponding to about 36.4 hours. Calibration is disabled if CALVAL = 0."]
    #[inline(always)]
    #[must_use]
    pub fn calval(&mut self) -> CALVAL_W<CALIBRATION_SPEC, 0> {
        CALVAL_W::new(self)
    }
    #[doc = "Bit 17 - Calibration direction"]
    #[inline(always)]
    #[must_use]
    pub fn caldir(&mut self) -> CALDIR_W<CALIBRATION_SPEC, 17> {
        CALDIR_W::new(self)
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
#[doc = "Calibration Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calibration::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calibration::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALIBRATION_SPEC;
impl crate::RegisterSpec for CALIBRATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calibration::R`](R) reader structure"]
impl crate::Readable for CALIBRATION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`calibration::W`](W) writer structure"]
impl crate::Writable for CALIBRATION_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALIBRATION to value 0"]
impl crate::Resettable for CALIBRATION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
