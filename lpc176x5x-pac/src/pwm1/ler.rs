#[doc = "Register `LER` reader"]
pub type R = crate::R<LER_SPEC>;
#[doc = "Register `LER` writer"]
pub type W = crate::W<LER_SPEC>;
#[doc = "Field `MAT0LATCHEN` reader - Enable PWM Match 0 Latch. PWM MR0 register update control. Writing a one to this bit allows the last value written to the PWM Match Register 0 to be become effective when the timer is next reset by a PWM Match event. See Section 27.6.7."]
pub type MAT0LATCHEN_R = crate::BitReader;
#[doc = "Field `MAT0LATCHEN` writer - Enable PWM Match 0 Latch. PWM MR0 register update control. Writing a one to this bit allows the last value written to the PWM Match Register 0 to be become effective when the timer is next reset by a PWM Match event. See Section 27.6.7."]
pub type MAT0LATCHEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MAT1LATCHEN` reader - Enable PWM Match 1 Latch. PWM MR1 register update control. See bit 0 for details."]
pub type MAT1LATCHEN_R = crate::BitReader;
#[doc = "Field `MAT1LATCHEN` writer - Enable PWM Match 1 Latch. PWM MR1 register update control. See bit 0 for details."]
pub type MAT1LATCHEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MAT2LATCHEN` reader - Enable PWM Match 2 Latch. PWM MR2 register update control. See bit 0 for details."]
pub type MAT2LATCHEN_R = crate::BitReader;
#[doc = "Field `MAT2LATCHEN` writer - Enable PWM Match 2 Latch. PWM MR2 register update control. See bit 0 for details."]
pub type MAT2LATCHEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MAT3LATCHEN` reader - Enable PWM Match 3 Latch. PWM MR3 register update control. See bit 0 for details."]
pub type MAT3LATCHEN_R = crate::BitReader;
#[doc = "Field `MAT3LATCHEN` writer - Enable PWM Match 3 Latch. PWM MR3 register update control. See bit 0 for details."]
pub type MAT3LATCHEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MAT4LATCHEN` reader - Enable PWM Match 4 Latch. PWM MR4 register update control. See bit 0 for details."]
pub type MAT4LATCHEN_R = crate::BitReader;
#[doc = "Field `MAT4LATCHEN` writer - Enable PWM Match 4 Latch. PWM MR4 register update control. See bit 0 for details."]
pub type MAT4LATCHEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MAT5LATCHEN` reader - Enable PWM Match 5 Latch. PWM MR5 register update control. See bit 0 for details."]
pub type MAT5LATCHEN_R = crate::BitReader;
#[doc = "Field `MAT5LATCHEN` writer - Enable PWM Match 5 Latch. PWM MR5 register update control. See bit 0 for details."]
pub type MAT5LATCHEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MAT6LATCHEN` reader - Enable PWM Match 6 Latch. PWM MR6 register update control. See bit 0 for details."]
pub type MAT6LATCHEN_R = crate::BitReader;
#[doc = "Field `MAT6LATCHEN` writer - Enable PWM Match 6 Latch. PWM MR6 register update control. See bit 0 for details."]
pub type MAT6LATCHEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Enable PWM Match 0 Latch. PWM MR0 register update control. Writing a one to this bit allows the last value written to the PWM Match Register 0 to be become effective when the timer is next reset by a PWM Match event. See Section 27.6.7."]
    #[inline(always)]
    pub fn mat0latchen(&self) -> MAT0LATCHEN_R {
        MAT0LATCHEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable PWM Match 1 Latch. PWM MR1 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat1latchen(&self) -> MAT1LATCHEN_R {
        MAT1LATCHEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable PWM Match 2 Latch. PWM MR2 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat2latchen(&self) -> MAT2LATCHEN_R {
        MAT2LATCHEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable PWM Match 3 Latch. PWM MR3 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat3latchen(&self) -> MAT3LATCHEN_R {
        MAT3LATCHEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable PWM Match 4 Latch. PWM MR4 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat4latchen(&self) -> MAT4LATCHEN_R {
        MAT4LATCHEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable PWM Match 5 Latch. PWM MR5 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat5latchen(&self) -> MAT5LATCHEN_R {
        MAT5LATCHEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable PWM Match 6 Latch. PWM MR6 register update control. See bit 0 for details."]
    #[inline(always)]
    pub fn mat6latchen(&self) -> MAT6LATCHEN_R {
        MAT6LATCHEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LER")
            .field("mat0latchen", &format_args!("{}", self.mat0latchen().bit()))
            .field("mat1latchen", &format_args!("{}", self.mat1latchen().bit()))
            .field("mat2latchen", &format_args!("{}", self.mat2latchen().bit()))
            .field("mat3latchen", &format_args!("{}", self.mat3latchen().bit()))
            .field("mat4latchen", &format_args!("{}", self.mat4latchen().bit()))
            .field("mat5latchen", &format_args!("{}", self.mat5latchen().bit()))
            .field("mat6latchen", &format_args!("{}", self.mat6latchen().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<LER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Enable PWM Match 0 Latch. PWM MR0 register update control. Writing a one to this bit allows the last value written to the PWM Match Register 0 to be become effective when the timer is next reset by a PWM Match event. See Section 27.6.7."]
    #[inline(always)]
    #[must_use]
    pub fn mat0latchen(&mut self) -> MAT0LATCHEN_W<LER_SPEC, 0> {
        MAT0LATCHEN_W::new(self)
    }
    #[doc = "Bit 1 - Enable PWM Match 1 Latch. PWM MR1 register update control. See bit 0 for details."]
    #[inline(always)]
    #[must_use]
    pub fn mat1latchen(&mut self) -> MAT1LATCHEN_W<LER_SPEC, 1> {
        MAT1LATCHEN_W::new(self)
    }
    #[doc = "Bit 2 - Enable PWM Match 2 Latch. PWM MR2 register update control. See bit 0 for details."]
    #[inline(always)]
    #[must_use]
    pub fn mat2latchen(&mut self) -> MAT2LATCHEN_W<LER_SPEC, 2> {
        MAT2LATCHEN_W::new(self)
    }
    #[doc = "Bit 3 - Enable PWM Match 3 Latch. PWM MR3 register update control. See bit 0 for details."]
    #[inline(always)]
    #[must_use]
    pub fn mat3latchen(&mut self) -> MAT3LATCHEN_W<LER_SPEC, 3> {
        MAT3LATCHEN_W::new(self)
    }
    #[doc = "Bit 4 - Enable PWM Match 4 Latch. PWM MR4 register update control. See bit 0 for details."]
    #[inline(always)]
    #[must_use]
    pub fn mat4latchen(&mut self) -> MAT4LATCHEN_W<LER_SPEC, 4> {
        MAT4LATCHEN_W::new(self)
    }
    #[doc = "Bit 5 - Enable PWM Match 5 Latch. PWM MR5 register update control. See bit 0 for details."]
    #[inline(always)]
    #[must_use]
    pub fn mat5latchen(&mut self) -> MAT5LATCHEN_W<LER_SPEC, 5> {
        MAT5LATCHEN_W::new(self)
    }
    #[doc = "Bit 6 - Enable PWM Match 6 Latch. PWM MR6 register update control. See bit 0 for details."]
    #[inline(always)]
    #[must_use]
    pub fn mat6latchen(&mut self) -> MAT6LATCHEN_W<LER_SPEC, 6> {
        MAT6LATCHEN_W::new(self)
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
#[doc = "Load Enable Register. Enables use of updated PWM match values.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ler::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ler::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LER_SPEC;
impl crate::RegisterSpec for LER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ler::R`](R) reader structure"]
impl crate::Readable for LER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ler::W`](W) writer structure"]
impl crate::Writable for LER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LER to value 0"]
impl crate::Resettable for LER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
