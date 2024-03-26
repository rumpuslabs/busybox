#[doc = "Register `IMSC` reader"]
pub type R = crate::R<IMSC_SPEC>;
#[doc = "Register `IMSC` writer"]
pub type W = crate::W<IMSC_SPEC>;
#[doc = "Field `RORIM` reader - Software should set this bit to enable interrupt when a Receive Overrun occurs, that is, when the Rx FIFO is full and another frame is completely received. The ARM spec implies that the preceding frame data is overwritten by the new frame data when this occurs."]
pub type RORIM_R = crate::BitReader;
#[doc = "Field `RORIM` writer - Software should set this bit to enable interrupt when a Receive Overrun occurs, that is, when the Rx FIFO is full and another frame is completely received. The ARM spec implies that the preceding frame data is overwritten by the new frame data when this occurs."]
pub type RORIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTIM` reader - Software should set this bit to enable interrupt when a Receive Time-out condition occurs. A Receive Time-out occurs when the Rx FIFO is not empty, and no has not been read for a time-out period. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR X \\[SCR+1\\])."]
pub type RTIM_R = crate::BitReader;
#[doc = "Field `RTIM` writer - Software should set this bit to enable interrupt when a Receive Time-out condition occurs. A Receive Time-out occurs when the Rx FIFO is not empty, and no has not been read for a time-out period. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR X \\[SCR+1\\])."]
pub type RTIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXIM` reader - Software should set this bit to enable interrupt when the Rx FIFO is at least half full."]
pub type RXIM_R = crate::BitReader;
#[doc = "Field `RXIM` writer - Software should set this bit to enable interrupt when the Rx FIFO is at least half full."]
pub type RXIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXIM` reader - Software should set this bit to enable interrupt when the Tx FIFO is at least half empty."]
pub type TXIM_R = crate::BitReader;
#[doc = "Field `TXIM` writer - Software should set this bit to enable interrupt when the Tx FIFO is at least half empty."]
pub type TXIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Software should set this bit to enable interrupt when a Receive Overrun occurs, that is, when the Rx FIFO is full and another frame is completely received. The ARM spec implies that the preceding frame data is overwritten by the new frame data when this occurs."]
    #[inline(always)]
    pub fn rorim(&self) -> RORIM_R {
        RORIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software should set this bit to enable interrupt when a Receive Time-out condition occurs. A Receive Time-out occurs when the Rx FIFO is not empty, and no has not been read for a time-out period. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR X \\[SCR+1\\])."]
    #[inline(always)]
    pub fn rtim(&self) -> RTIM_R {
        RTIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software should set this bit to enable interrupt when the Rx FIFO is at least half full."]
    #[inline(always)]
    pub fn rxim(&self) -> RXIM_R {
        RXIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software should set this bit to enable interrupt when the Tx FIFO is at least half empty."]
    #[inline(always)]
    pub fn txim(&self) -> TXIM_R {
        TXIM_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMSC")
            .field("rorim", &format_args!("{}", self.rorim().bit()))
            .field("rtim", &format_args!("{}", self.rtim().bit()))
            .field("rxim", &format_args!("{}", self.rxim().bit()))
            .field("txim", &format_args!("{}", self.txim().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IMSC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Software should set this bit to enable interrupt when a Receive Overrun occurs, that is, when the Rx FIFO is full and another frame is completely received. The ARM spec implies that the preceding frame data is overwritten by the new frame data when this occurs."]
    #[inline(always)]
    #[must_use]
    pub fn rorim(&mut self) -> RORIM_W<IMSC_SPEC, 0> {
        RORIM_W::new(self)
    }
    #[doc = "Bit 1 - Software should set this bit to enable interrupt when a Receive Time-out condition occurs. A Receive Time-out occurs when the Rx FIFO is not empty, and no has not been read for a time-out period. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR X \\[SCR+1\\])."]
    #[inline(always)]
    #[must_use]
    pub fn rtim(&mut self) -> RTIM_W<IMSC_SPEC, 1> {
        RTIM_W::new(self)
    }
    #[doc = "Bit 2 - Software should set this bit to enable interrupt when the Rx FIFO is at least half full."]
    #[inline(always)]
    #[must_use]
    pub fn rxim(&mut self) -> RXIM_W<IMSC_SPEC, 2> {
        RXIM_W::new(self)
    }
    #[doc = "Bit 3 - Software should set this bit to enable interrupt when the Tx FIFO is at least half empty."]
    #[inline(always)]
    #[must_use]
    pub fn txim(&mut self) -> TXIM_W<IMSC_SPEC, 3> {
        TXIM_W::new(self)
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
#[doc = "Interrupt Mask Set and Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imsc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imsc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMSC_SPEC;
impl crate::RegisterSpec for IMSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imsc::R`](R) reader structure"]
impl crate::Readable for IMSC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`imsc::W`](W) writer structure"]
impl crate::Writable for IMSC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMSC to value 0"]
impl crate::Resettable for IMSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
