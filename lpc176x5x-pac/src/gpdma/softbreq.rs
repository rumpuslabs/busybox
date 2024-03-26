#[doc = "Register `SOFTBREQ` reader"]
pub type R = crate::R<SOFTBREQ_SPEC>;
#[doc = "Register `SOFTBREQ` writer"]
pub type W = crate::W<SOFTBREQ_SPEC>;
#[doc = "Field `SOFTBREQ0` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ0_R = crate::BitReader;
#[doc = "Field `SOFTBREQ0` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFTBREQ1` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ1_R = crate::BitReader;
#[doc = "Field `SOFTBREQ1` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFTBREQ2` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ2_R = crate::BitReader;
#[doc = "Field `SOFTBREQ2` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFTBREQ3` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ3_R = crate::BitReader;
#[doc = "Field `SOFTBREQ3` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFTBREQ4` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ4_R = crate::BitReader;
#[doc = "Field `SOFTBREQ4` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFTBREQ5` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ5_R = crate::BitReader;
#[doc = "Field `SOFTBREQ5` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFTBREQ6` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ6_R = crate::BitReader;
#[doc = "Field `SOFTBREQ6` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFTBREQ7` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ7_R = crate::BitReader;
#[doc = "Field `SOFTBREQ7` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFTBREQ8` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ8_R = crate::BitReader;
#[doc = "Field `SOFTBREQ8` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFTBREQ9` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ9_R = crate::BitReader;
#[doc = "Field `SOFTBREQ9` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFTBREQ10` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ10_R = crate::BitReader;
#[doc = "Field `SOFTBREQ10` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFTBREQ11` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ11_R = crate::BitReader;
#[doc = "Field `SOFTBREQ11` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFTBREQ12` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ12_R = crate::BitReader;
#[doc = "Field `SOFTBREQ12` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFTBREQ13` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ13_R = crate::BitReader;
#[doc = "Field `SOFTBREQ13` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFTBREQ14` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ14_R = crate::BitReader;
#[doc = "Field `SOFTBREQ14` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFTBREQ15` reader - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ15_R = crate::BitReader;
#[doc = "Field `SOFTBREQ15` writer - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
pub type SOFTBREQ15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq0(&self) -> SOFTBREQ0_R {
        SOFTBREQ0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq1(&self) -> SOFTBREQ1_R {
        SOFTBREQ1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq2(&self) -> SOFTBREQ2_R {
        SOFTBREQ2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq3(&self) -> SOFTBREQ3_R {
        SOFTBREQ3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq4(&self) -> SOFTBREQ4_R {
        SOFTBREQ4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq5(&self) -> SOFTBREQ5_R {
        SOFTBREQ5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq6(&self) -> SOFTBREQ6_R {
        SOFTBREQ6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq7(&self) -> SOFTBREQ7_R {
        SOFTBREQ7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq8(&self) -> SOFTBREQ8_R {
        SOFTBREQ8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq9(&self) -> SOFTBREQ9_R {
        SOFTBREQ9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq10(&self) -> SOFTBREQ10_R {
        SOFTBREQ10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq11(&self) -> SOFTBREQ11_R {
        SOFTBREQ11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq12(&self) -> SOFTBREQ12_R {
        SOFTBREQ12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq13(&self) -> SOFTBREQ13_R {
        SOFTBREQ13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq14(&self) -> SOFTBREQ14_R {
        SOFTBREQ14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq15(&self) -> SOFTBREQ15_R {
        SOFTBREQ15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOFTBREQ")
            .field("softbreq0", &format_args!("{}", self.softbreq0().bit()))
            .field("softbreq1", &format_args!("{}", self.softbreq1().bit()))
            .field("softbreq2", &format_args!("{}", self.softbreq2().bit()))
            .field("softbreq3", &format_args!("{}", self.softbreq3().bit()))
            .field("softbreq4", &format_args!("{}", self.softbreq4().bit()))
            .field("softbreq5", &format_args!("{}", self.softbreq5().bit()))
            .field("softbreq6", &format_args!("{}", self.softbreq6().bit()))
            .field("softbreq7", &format_args!("{}", self.softbreq7().bit()))
            .field("softbreq8", &format_args!("{}", self.softbreq8().bit()))
            .field("softbreq9", &format_args!("{}", self.softbreq9().bit()))
            .field("softbreq10", &format_args!("{}", self.softbreq10().bit()))
            .field("softbreq11", &format_args!("{}", self.softbreq11().bit()))
            .field("softbreq12", &format_args!("{}", self.softbreq12().bit()))
            .field("softbreq13", &format_args!("{}", self.softbreq13().bit()))
            .field("softbreq14", &format_args!("{}", self.softbreq14().bit()))
            .field("softbreq15", &format_args!("{}", self.softbreq15().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<SOFTBREQ_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    #[must_use]
    pub fn softbreq0(&mut self) -> SOFTBREQ0_W<SOFTBREQ_SPEC, 0> {
        SOFTBREQ0_W::new(self)
    }
    #[doc = "Bit 1 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    #[must_use]
    pub fn softbreq1(&mut self) -> SOFTBREQ1_W<SOFTBREQ_SPEC, 1> {
        SOFTBREQ1_W::new(self)
    }
    #[doc = "Bit 2 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    #[must_use]
    pub fn softbreq2(&mut self) -> SOFTBREQ2_W<SOFTBREQ_SPEC, 2> {
        SOFTBREQ2_W::new(self)
    }
    #[doc = "Bit 3 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    #[must_use]
    pub fn softbreq3(&mut self) -> SOFTBREQ3_W<SOFTBREQ_SPEC, 3> {
        SOFTBREQ3_W::new(self)
    }
    #[doc = "Bit 4 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    #[must_use]
    pub fn softbreq4(&mut self) -> SOFTBREQ4_W<SOFTBREQ_SPEC, 4> {
        SOFTBREQ4_W::new(self)
    }
    #[doc = "Bit 5 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    #[must_use]
    pub fn softbreq5(&mut self) -> SOFTBREQ5_W<SOFTBREQ_SPEC, 5> {
        SOFTBREQ5_W::new(self)
    }
    #[doc = "Bit 6 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    #[must_use]
    pub fn softbreq6(&mut self) -> SOFTBREQ6_W<SOFTBREQ_SPEC, 6> {
        SOFTBREQ6_W::new(self)
    }
    #[doc = "Bit 7 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    #[must_use]
    pub fn softbreq7(&mut self) -> SOFTBREQ7_W<SOFTBREQ_SPEC, 7> {
        SOFTBREQ7_W::new(self)
    }
    #[doc = "Bit 8 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    #[must_use]
    pub fn softbreq8(&mut self) -> SOFTBREQ8_W<SOFTBREQ_SPEC, 8> {
        SOFTBREQ8_W::new(self)
    }
    #[doc = "Bit 9 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    #[must_use]
    pub fn softbreq9(&mut self) -> SOFTBREQ9_W<SOFTBREQ_SPEC, 9> {
        SOFTBREQ9_W::new(self)
    }
    #[doc = "Bit 10 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    #[must_use]
    pub fn softbreq10(&mut self) -> SOFTBREQ10_W<SOFTBREQ_SPEC, 10> {
        SOFTBREQ10_W::new(self)
    }
    #[doc = "Bit 11 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    #[must_use]
    pub fn softbreq11(&mut self) -> SOFTBREQ11_W<SOFTBREQ_SPEC, 11> {
        SOFTBREQ11_W::new(self)
    }
    #[doc = "Bit 12 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    #[must_use]
    pub fn softbreq12(&mut self) -> SOFTBREQ12_W<SOFTBREQ_SPEC, 12> {
        SOFTBREQ12_W::new(self)
    }
    #[doc = "Bit 13 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    #[must_use]
    pub fn softbreq13(&mut self) -> SOFTBREQ13_W<SOFTBREQ_SPEC, 13> {
        SOFTBREQ13_W::new(self)
    }
    #[doc = "Bit 14 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    #[must_use]
    pub fn softbreq14(&mut self) -> SOFTBREQ14_W<SOFTBREQ_SPEC, 14> {
        SOFTBREQ14_W::new(self)
    }
    #[doc = "Bit 15 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    #[must_use]
    pub fn softbreq15(&mut self) -> SOFTBREQ15_W<SOFTBREQ_SPEC, 15> {
        SOFTBREQ15_W::new(self)
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
#[doc = "DMA Software Burst Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softbreq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softbreq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOFTBREQ_SPEC;
impl crate::RegisterSpec for SOFTBREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`softbreq::R`](R) reader structure"]
impl crate::Readable for SOFTBREQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`softbreq::W`](W) writer structure"]
impl crate::Writable for SOFTBREQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOFTBREQ to value 0"]
impl crate::Resettable for SOFTBREQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
