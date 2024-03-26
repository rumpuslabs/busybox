#[doc = "Register `USBCLKCFG` reader"]
pub type R = crate::R<USBCLKCFG_SPEC>;
#[doc = "Register `USBCLKCFG` writer"]
pub type W = crate::W<USBCLKCFG_SPEC>;
#[doc = "Field `USBSEL` reader - Selects the divide value for creating the USB clock from the PLL0 output. Only the values shown below can produce even number multiples of 48 MHz from the PLL0 output. Warning: Improper setting of this value will result in incorrect operation of the USB interface. 5 = PLL0 output is divided by 6. PLL0 output must be 288 MHz. 7 = PLL0 output is divided by 8. PLL0 output must be 384 MHz. 9 = PLL0 output is divided by 10. PLL0 output must be 480 MHz."]
pub type USBSEL_R = crate::FieldReader;
#[doc = "Field `USBSEL` writer - Selects the divide value for creating the USB clock from the PLL0 output. Only the values shown below can produce even number multiples of 48 MHz from the PLL0 output. Warning: Improper setting of this value will result in incorrect operation of the USB interface. 5 = PLL0 output is divided by 6. PLL0 output must be 288 MHz. 7 = PLL0 output is divided by 8. PLL0 output must be 384 MHz. 9 = PLL0 output is divided by 10. PLL0 output must be 480 MHz."]
pub type USBSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Selects the divide value for creating the USB clock from the PLL0 output. Only the values shown below can produce even number multiples of 48 MHz from the PLL0 output. Warning: Improper setting of this value will result in incorrect operation of the USB interface. 5 = PLL0 output is divided by 6. PLL0 output must be 288 MHz. 7 = PLL0 output is divided by 8. PLL0 output must be 384 MHz. 9 = PLL0 output is divided by 10. PLL0 output must be 480 MHz."]
    #[inline(always)]
    pub fn usbsel(&self) -> USBSEL_R {
        USBSEL_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBCLKCFG")
            .field("usbsel", &format_args!("{}", self.usbsel().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<USBCLKCFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects the divide value for creating the USB clock from the PLL0 output. Only the values shown below can produce even number multiples of 48 MHz from the PLL0 output. Warning: Improper setting of this value will result in incorrect operation of the USB interface. 5 = PLL0 output is divided by 6. PLL0 output must be 288 MHz. 7 = PLL0 output is divided by 8. PLL0 output must be 384 MHz. 9 = PLL0 output is divided by 10. PLL0 output must be 480 MHz."]
    #[inline(always)]
    #[must_use]
    pub fn usbsel(&mut self) -> USBSEL_W<USBCLKCFG_SPEC, 0> {
        USBSEL_W::new(self)
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
#[doc = "USB Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbclkcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbclkcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBCLKCFG_SPEC;
impl crate::RegisterSpec for USBCLKCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbclkcfg::R`](R) reader structure"]
impl crate::Readable for USBCLKCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbclkcfg::W`](W) writer structure"]
impl crate::Writable for USBCLKCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBCLKCFG to value 0"]
impl crate::Resettable for USBCLKCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
