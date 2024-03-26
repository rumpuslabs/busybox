#[doc = "Register `PC` reader"]
pub type R = crate::R<PC_SPEC>;
#[doc = "Register `PC` writer"]
pub type W = crate::W<PC_SPEC>;
#[doc = "Field `PC` reader - Prescale counter value."]
pub type PC_R = crate::FieldReader<u32>;
#[doc = "Field `PC` writer - Prescale counter value."]
pub type PC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Prescale counter value."]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PC")
            .field("pc", &format_args!("{}", self.pc().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Prescale counter value."]
    #[inline(always)]
    #[must_use]
    pub fn pc(&mut self) -> PC_W<PC_SPEC, 0> {
        PC_W::new(self)
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
#[doc = "Prescale Counter. The 32 bit PC is a counter which is incremented to the value stored in PR. When the value in PR is reached, the TC is incremented and the PC is cleared. The PC is observable and controllable through the bus interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PC_SPEC;
impl crate::RegisterSpec for PC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc::R`](R) reader structure"]
impl crate::Readable for PC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pc::W`](W) writer structure"]
impl crate::Writable for PC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PC to value 0"]
impl crate::Resettable for PC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
