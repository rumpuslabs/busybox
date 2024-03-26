#[doc = "Register `BTR` reader"]
pub type R = crate::R<BTR_SPEC>;
#[doc = "Register `BTR` writer"]
pub type W = crate::W<BTR_SPEC>;
#[doc = "Field `BRP` reader - Baud Rate Prescaler. The APB clock is divided by (this value plus one) to produce the CAN clock."]
pub type BRP_R = crate::FieldReader<u16>;
#[doc = "Field `BRP` writer - Baud Rate Prescaler. The APB clock is divided by (this value plus one) to produce the CAN clock."]
pub type BRP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `SJW` reader - The Synchronization Jump Width is (this value plus one) CAN clocks."]
pub type SJW_R = crate::FieldReader;
#[doc = "Field `SJW` writer - The Synchronization Jump Width is (this value plus one) CAN clocks."]
pub type SJW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TESG1` reader - The delay from the nominal Sync point to the sample point is (this value plus one) CAN clocks."]
pub type TESG1_R = crate::FieldReader;
#[doc = "Field `TESG1` writer - The delay from the nominal Sync point to the sample point is (this value plus one) CAN clocks."]
pub type TESG1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TESG2` reader - The delay from the sample point to the next nominal sync point is (this value plus one) CAN clocks. The nominal CAN bit time is (this value plus the value in TSEG1 plus 3) CAN clocks."]
pub type TESG2_R = crate::FieldReader;
#[doc = "Field `TESG2` writer - The delay from the sample point to the next nominal sync point is (this value plus one) CAN clocks. The nominal CAN bit time is (this value plus the value in TSEG1 plus 3) CAN clocks."]
pub type TESG2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SAM` reader - Sampling"]
pub type SAM_R = crate::BitReader<SAM_A>;
#[doc = "Sampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAM_A {
    #[doc = "0: The bus is sampled once (recommended for high speed buses)"]
    SAMPLE_1 = 0,
    #[doc = "1: The bus is sampled 3 times (recommended for low to medium speed buses to filter spikes on the bus-line)"]
    SAMPLE_3 = 1,
}
impl From<SAM_A> for bool {
    #[inline(always)]
    fn from(variant: SAM_A) -> Self {
        variant as u8 != 0
    }
}
impl SAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SAM_A {
        match self.bits {
            false => SAM_A::SAMPLE_1,
            true => SAM_A::SAMPLE_3,
        }
    }
    #[doc = "The bus is sampled once (recommended for high speed buses)"]
    #[inline(always)]
    pub fn is_sample_1(&self) -> bool {
        *self == SAM_A::SAMPLE_1
    }
    #[doc = "The bus is sampled 3 times (recommended for low to medium speed buses to filter spikes on the bus-line)"]
    #[inline(always)]
    pub fn is_sample_3(&self) -> bool {
        *self == SAM_A::SAMPLE_3
    }
}
#[doc = "Field `SAM` writer - Sampling"]
pub type SAM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SAM_A>;
impl<'a, REG, const O: u8> SAM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The bus is sampled once (recommended for high speed buses)"]
    #[inline(always)]
    pub fn sample_1(self) -> &'a mut crate::W<REG> {
        self.variant(SAM_A::SAMPLE_1)
    }
    #[doc = "The bus is sampled 3 times (recommended for low to medium speed buses to filter spikes on the bus-line)"]
    #[inline(always)]
    pub fn sample_3(self) -> &'a mut crate::W<REG> {
        self.variant(SAM_A::SAMPLE_3)
    }
}
impl R {
    #[doc = "Bits 0:9 - Baud Rate Prescaler. The APB clock is divided by (this value plus one) to produce the CAN clock."]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 14:15 - The Synchronization Jump Width is (this value plus one) CAN clocks."]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - The delay from the nominal Sync point to the sample point is (this value plus one) CAN clocks."]
    #[inline(always)]
    pub fn tesg1(&self) -> TESG1_R {
        TESG1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - The delay from the sample point to the next nominal sync point is (this value plus one) CAN clocks. The nominal CAN bit time is (this value plus the value in TSEG1 plus 3) CAN clocks."]
    #[inline(always)]
    pub fn tesg2(&self) -> TESG2_R {
        TESG2_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Sampling"]
    #[inline(always)]
    pub fn sam(&self) -> SAM_R {
        SAM_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BTR")
            .field("brp", &format_args!("{}", self.brp().bits()))
            .field("sjw", &format_args!("{}", self.sjw().bits()))
            .field("tesg1", &format_args!("{}", self.tesg1().bits()))
            .field("tesg2", &format_args!("{}", self.tesg2().bits()))
            .field("sam", &format_args!("{}", self.sam().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<BTR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9 - Baud Rate Prescaler. The APB clock is divided by (this value plus one) to produce the CAN clock."]
    #[inline(always)]
    #[must_use]
    pub fn brp(&mut self) -> BRP_W<BTR_SPEC, 0> {
        BRP_W::new(self)
    }
    #[doc = "Bits 14:15 - The Synchronization Jump Width is (this value plus one) CAN clocks."]
    #[inline(always)]
    #[must_use]
    pub fn sjw(&mut self) -> SJW_W<BTR_SPEC, 14> {
        SJW_W::new(self)
    }
    #[doc = "Bits 16:19 - The delay from the nominal Sync point to the sample point is (this value plus one) CAN clocks."]
    #[inline(always)]
    #[must_use]
    pub fn tesg1(&mut self) -> TESG1_W<BTR_SPEC, 16> {
        TESG1_W::new(self)
    }
    #[doc = "Bits 20:22 - The delay from the sample point to the next nominal sync point is (this value plus one) CAN clocks. The nominal CAN bit time is (this value plus the value in TSEG1 plus 3) CAN clocks."]
    #[inline(always)]
    #[must_use]
    pub fn tesg2(&mut self) -> TESG2_W<BTR_SPEC, 20> {
        TESG2_W::new(self)
    }
    #[doc = "Bit 23 - Sampling"]
    #[inline(always)]
    #[must_use]
    pub fn sam(&mut self) -> SAM_W<BTR_SPEC, 23> {
        SAM_W::new(self)
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
#[doc = "Bus Timing. Can only be written when RM in CANMOD is 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTR_SPEC;
impl crate::RegisterSpec for BTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btr::R`](R) reader structure"]
impl crate::Readable for BTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`btr::W`](W) writer structure"]
impl crate::Writable for BTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BTR to value 0x001c_0000"]
impl crate::Resettable for BTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x001c_0000;
}
