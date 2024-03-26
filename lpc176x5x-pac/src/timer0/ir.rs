#[doc = "Register `IR` reader"]
pub type R = crate::R<IR_SPEC>;
#[doc = "Register `IR` writer"]
pub type W = crate::W<IR_SPEC>;
#[doc = "Field `MR0INT` reader - Interrupt flag for match channel 0."]
pub type MR0INT_R = crate::BitReader;
#[doc = "Field `MR0INT` writer - Interrupt flag for match channel 0."]
pub type MR0INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MR1INT` reader - Interrupt flag for match channel 1."]
pub type MR1INT_R = crate::BitReader;
#[doc = "Field `MR1INT` writer - Interrupt flag for match channel 1."]
pub type MR1INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MR2INT` reader - Interrupt flag for match channel 2."]
pub type MR2INT_R = crate::BitReader;
#[doc = "Field `MR2INT` writer - Interrupt flag for match channel 2."]
pub type MR2INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MR3INT` reader - Interrupt flag for match channel 3."]
pub type MR3INT_R = crate::BitReader;
#[doc = "Field `MR3INT` writer - Interrupt flag for match channel 3."]
pub type MR3INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CR0INT` reader - Interrupt flag for capture channel 0 event."]
pub type CR0INT_R = crate::BitReader;
#[doc = "Field `CR0INT` writer - Interrupt flag for capture channel 0 event."]
pub type CR0INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CR1INT` reader - Interrupt flag for capture channel 1 event."]
pub type CR1INT_R = crate::BitReader;
#[doc = "Field `CR1INT` writer - Interrupt flag for capture channel 1 event."]
pub type CR1INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Interrupt flag for match channel 0."]
    #[inline(always)]
    pub fn mr0int(&self) -> MR0INT_R {
        MR0INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt flag for match channel 1."]
    #[inline(always)]
    pub fn mr1int(&self) -> MR1INT_R {
        MR1INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt flag for match channel 2."]
    #[inline(always)]
    pub fn mr2int(&self) -> MR2INT_R {
        MR2INT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt flag for match channel 3."]
    #[inline(always)]
    pub fn mr3int(&self) -> MR3INT_R {
        MR3INT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt flag for capture channel 0 event."]
    #[inline(always)]
    pub fn cr0int(&self) -> CR0INT_R {
        CR0INT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt flag for capture channel 1 event."]
    #[inline(always)]
    pub fn cr1int(&self) -> CR1INT_R {
        CR1INT_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IR")
            .field("mr0int", &format_args!("{}", self.mr0int().bit()))
            .field("mr1int", &format_args!("{}", self.mr1int().bit()))
            .field("mr2int", &format_args!("{}", self.mr2int().bit()))
            .field("mr3int", &format_args!("{}", self.mr3int().bit()))
            .field("cr0int", &format_args!("{}", self.cr0int().bit()))
            .field("cr1int", &format_args!("{}", self.cr1int().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt flag for match channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn mr0int(&mut self) -> MR0INT_W<IR_SPEC, 0> {
        MR0INT_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt flag for match channel 1."]
    #[inline(always)]
    #[must_use]
    pub fn mr1int(&mut self) -> MR1INT_W<IR_SPEC, 1> {
        MR1INT_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt flag for match channel 2."]
    #[inline(always)]
    #[must_use]
    pub fn mr2int(&mut self) -> MR2INT_W<IR_SPEC, 2> {
        MR2INT_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt flag for match channel 3."]
    #[inline(always)]
    #[must_use]
    pub fn mr3int(&mut self) -> MR3INT_W<IR_SPEC, 3> {
        MR3INT_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt flag for capture channel 0 event."]
    #[inline(always)]
    #[must_use]
    pub fn cr0int(&mut self) -> CR0INT_W<IR_SPEC, 4> {
        CR0INT_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt flag for capture channel 1 event."]
    #[inline(always)]
    #[must_use]
    pub fn cr1int(&mut self) -> CR1INT_W<IR_SPEC, 5> {
        CR1INT_W::new(self)
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
#[doc = "Interrupt Register. The IR can be written to clear interrupts. The IR can be read to identify which of eight possible interrupt sources are pending.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IR_SPEC;
impl crate::RegisterSpec for IR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir::R`](R) reader structure"]
impl crate::Readable for IR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ir::W`](W) writer structure"]
impl crate::Writable for IR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IR to value 0"]
impl crate::Resettable for IR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
