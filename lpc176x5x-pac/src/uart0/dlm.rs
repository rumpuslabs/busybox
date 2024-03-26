#[doc = "Register `DLM` reader"]
pub type R = crate::R<DLM_SPEC>;
#[doc = "Register `DLM` writer"]
pub type W = crate::W<DLM_SPEC>;
#[doc = "Field `DLMSB` reader - The UARTn Divisor Latch MSB Register, along with the U0DLL register, determines the baud rate of the UARTn."]
pub type DLMSB_R = crate::FieldReader;
#[doc = "Field `DLMSB` writer - The UARTn Divisor Latch MSB Register, along with the U0DLL register, determines the baud rate of the UARTn."]
pub type DLMSB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - The UARTn Divisor Latch MSB Register, along with the U0DLL register, determines the baud rate of the UARTn."]
    #[inline(always)]
    pub fn dlmsb(&self) -> DLMSB_R {
        DLMSB_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLM")
            .field("dlmsb", &format_args!("{}", self.dlmsb().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DLM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - The UARTn Divisor Latch MSB Register, along with the U0DLL register, determines the baud rate of the UARTn."]
    #[inline(always)]
    #[must_use]
    pub fn dlmsb(&mut self) -> DLMSB_W<DLM_SPEC, 0> {
        DLMSB_W::new(self)
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
#[doc = "Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider (DLAB =1).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLM_SPEC;
impl crate::RegisterSpec for DLM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dlm::R`](R) reader structure"]
impl crate::Readable for DLM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dlm::W`](W) writer structure"]
impl crate::Writable for DLM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DLM to value 0"]
impl crate::Resettable for DLM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
