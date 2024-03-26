#[doc = "Register `TXDESCRIPTOR` reader"]
pub type R = crate::R<TXDESCRIPTOR_SPEC>;
#[doc = "Register `TXDESCRIPTOR` writer"]
pub type W = crate::W<TXDESCRIPTOR_SPEC>;
#[doc = "Field `TXD` reader - TxDescriptor. MSBs of transmit descriptor base address."]
pub type TXD_R = crate::FieldReader<u32>;
#[doc = "Field `TXD` writer - TxDescriptor. MSBs of transmit descriptor base address."]
pub type TXD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 30, O, u32>;
impl R {
    #[doc = "Bits 2:31 - TxDescriptor. MSBs of transmit descriptor base address."]
    #[inline(always)]
    pub fn txd(&self) -> TXD_R {
        TXD_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXDESCRIPTOR")
            .field("txd", &format_args!("{}", self.txd().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TXDESCRIPTOR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 2:31 - TxDescriptor. MSBs of transmit descriptor base address."]
    #[inline(always)]
    #[must_use]
    pub fn txd(&mut self) -> TXD_W<TXDESCRIPTOR_SPEC, 2> {
        TXD_W::new(self)
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
#[doc = "Transmit descriptor base address register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdescriptor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdescriptor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDESCRIPTOR_SPEC;
impl crate::RegisterSpec for TXDESCRIPTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdescriptor::R`](R) reader structure"]
impl crate::Readable for TXDESCRIPTOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txdescriptor::W`](W) writer structure"]
impl crate::Writable for TXDESCRIPTOR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXDESCRIPTOR to value 0"]
impl crate::Resettable for TXDESCRIPTOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
