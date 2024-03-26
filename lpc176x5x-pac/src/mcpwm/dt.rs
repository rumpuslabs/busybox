#[doc = "Register `DT` reader"]
pub type R = crate::R<DT_SPEC>;
#[doc = "Register `DT` writer"]
pub type W = crate::W<DT_SPEC>;
#[doc = "Field `DT0` reader - Dead time for channel 0.\\[1\\]"]
pub type DT0_R = crate::FieldReader<u16>;
#[doc = "Field `DT0` writer - Dead time for channel 0.\\[1\\]"]
pub type DT0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `DT1` reader - Dead time for channel 1.\\[2\\]"]
pub type DT1_R = crate::FieldReader<u16>;
#[doc = "Field `DT1` writer - Dead time for channel 1.\\[2\\]"]
pub type DT1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `DT2` reader - Dead time for channel 2.\\[2\\]"]
pub type DT2_R = crate::FieldReader<u16>;
#[doc = "Field `DT2` writer - Dead time for channel 2.\\[2\\]"]
pub type DT2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 0:9 - Dead time for channel 0.\\[1\\]"]
    #[inline(always)]
    pub fn dt0(&self) -> DT0_R {
        DT0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Dead time for channel 1.\\[2\\]"]
    #[inline(always)]
    pub fn dt1(&self) -> DT1_R {
        DT1_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - Dead time for channel 2.\\[2\\]"]
    #[inline(always)]
    pub fn dt2(&self) -> DT2_R {
        DT2_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT")
            .field("dt0", &format_args!("{}", self.dt0().bits()))
            .field("dt1", &format_args!("{}", self.dt1().bits()))
            .field("dt2", &format_args!("{}", self.dt2().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9 - Dead time for channel 0.\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dt0(&mut self) -> DT0_W<DT_SPEC, 0> {
        DT0_W::new(self)
    }
    #[doc = "Bits 10:19 - Dead time for channel 1.\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dt1(&mut self) -> DT1_W<DT_SPEC, 10> {
        DT1_W::new(self)
    }
    #[doc = "Bits 20:29 - Dead time for channel 2.\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dt2(&mut self) -> DT2_W<DT_SPEC, 20> {
        DT2_W::new(self)
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
#[doc = "Dead time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT_SPEC;
impl crate::RegisterSpec for DT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt::R`](R) reader structure"]
impl crate::Readable for DT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt::W`](W) writer structure"]
impl crate::Writable for DT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT to value 0x3fff_ffff"]
impl crate::Resettable for DT_SPEC {
    const RESET_VALUE: Self::Ux = 0x3fff_ffff;
}
