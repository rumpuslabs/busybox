#[doc = "Register `RXBITRATE` reader"]
pub type R = crate::R<RXBITRATE_SPEC>;
#[doc = "Register `RXBITRATE` writer"]
pub type W = crate::W<RXBITRATE_SPEC>;
#[doc = "Field `RX_BITRATE` reader - I2S receive bit rate. This value plus one is used to divide RX_MCLK to produce the receive bit clock."]
pub type RX_BITRATE_R = crate::FieldReader;
#[doc = "Field `RX_BITRATE` writer - I2S receive bit rate. This value plus one is used to divide RX_MCLK to produce the receive bit clock."]
pub type RX_BITRATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - I2S receive bit rate. This value plus one is used to divide RX_MCLK to produce the receive bit clock."]
    #[inline(always)]
    pub fn rx_bitrate(&self) -> RX_BITRATE_R {
        RX_BITRATE_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXBITRATE")
            .field("rx_bitrate", &format_args!("{}", self.rx_bitrate().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RXBITRATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - I2S receive bit rate. This value plus one is used to divide RX_MCLK to produce the receive bit clock."]
    #[inline(always)]
    #[must_use]
    pub fn rx_bitrate(&mut self) -> RX_BITRATE_W<RXBITRATE_SPEC, 0> {
        RX_BITRATE_W::new(self)
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
#[doc = "I2S Receive bit rate divider. This register determines the I2S receive bit rate by specifying the value to divide RX_MCLK by in order to produce the receive bit clock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxbitrate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxbitrate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXBITRATE_SPEC;
impl crate::RegisterSpec for RXBITRATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxbitrate::R`](R) reader structure"]
impl crate::Readable for RXBITRATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxbitrate::W`](W) writer structure"]
impl crate::Writable for RXBITRATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXBITRATE to value 0"]
impl crate::Resettable for RXBITRATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
