#[doc = "Register `DLL` reader"]
pub type R = crate::R<DLL_SPEC>;
#[doc = "Register `DLL` writer"]
pub type W = crate::W<DLL_SPEC>;
#[doc = "Field `DLLSB` reader - The UARTn Divisor Latch LSB Register, along with the UnDLM register, determines the baud rate of the UARTn."]
pub type DLLSB_R = crate::FieldReader;
#[doc = "Field `DLLSB` writer - The UARTn Divisor Latch LSB Register, along with the UnDLM register, determines the baud rate of the UARTn."]
pub type DLLSB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - The UARTn Divisor Latch LSB Register, along with the UnDLM register, determines the baud rate of the UARTn."]
    #[inline(always)]
    pub fn dllsb(&self) -> DLLSB_R {
        DLLSB_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLL")
            .field("dllsb", &format_args!("{}", self.dllsb().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DLL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - The UARTn Divisor Latch LSB Register, along with the UnDLM register, determines the baud rate of the UARTn."]
    #[inline(always)]
    #[must_use]
    pub fn dllsb(&mut self) -> DLLSB_W<DLL_SPEC, 0> {
        DLLSB_W::new(self)
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
#[doc = "Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider (DLAB =1).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLL_SPEC;
impl crate::RegisterSpec for DLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dll::R`](R) reader structure"]
impl crate::Readable for DLL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dll::W`](W) writer structure"]
impl crate::Writable for DLL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DLL to value 0x01"]
impl crate::Resettable for DLL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
