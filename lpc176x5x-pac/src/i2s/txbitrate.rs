#[doc = "Register `TXBITRATE` reader"]
pub type R = crate::R<TXBITRATE_SPEC>;
#[doc = "Register `TXBITRATE` writer"]
pub type W = crate::W<TXBITRATE_SPEC>;
#[doc = "Field `TX_BITRATE` reader - I2S transmit bit rate. This value plus one is used to divide TX_MCLK to produce the transmit bit clock."]
pub type TX_BITRATE_R = crate::FieldReader;
#[doc = "Field `TX_BITRATE` writer - I2S transmit bit rate. This value plus one is used to divide TX_MCLK to produce the transmit bit clock."]
pub type TX_BITRATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - I2S transmit bit rate. This value plus one is used to divide TX_MCLK to produce the transmit bit clock."]
    #[inline(always)]
    pub fn tx_bitrate(&self) -> TX_BITRATE_R {
        TX_BITRATE_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBITRATE")
            .field("tx_bitrate", &format_args!("{}", self.tx_bitrate().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TXBITRATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - I2S transmit bit rate. This value plus one is used to divide TX_MCLK to produce the transmit bit clock."]
    #[inline(always)]
    #[must_use]
    pub fn tx_bitrate(&mut self) -> TX_BITRATE_W<TXBITRATE_SPEC, 0> {
        TX_BITRATE_W::new(self)
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
#[doc = "I2S Transmit bit rate divider. This register determines the I2S transmit bit rate by specifying the value to divide TX_MCLK by in order to produce the transmit bit clock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbitrate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbitrate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBITRATE_SPEC;
impl crate::RegisterSpec for TXBITRATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbitrate::R`](R) reader structure"]
impl crate::Readable for TXBITRATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txbitrate::W`](W) writer structure"]
impl crate::Writable for TXBITRATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXBITRATE to value 0"]
impl crate::Resettable for TXBITRATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
