#[doc = "Register `DAT` reader"]
pub type R = crate::R<DAT_SPEC>;
#[doc = "Register `DAT` writer"]
pub type W = crate::W<DAT_SPEC>;
#[doc = "Field `Data` reader - This register holds data values that have been received or are to be transmitted."]
pub type DATA_R = crate::FieldReader;
#[doc = "Field `Data` writer - This register holds data values that have been received or are to be transmitted."]
pub type DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - This register holds data values that have been received or are to be transmitted."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAT")
            .field("data", &format_args!("{}", self.data().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DAT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register holds data values that have been received or are to be transmitted."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<DAT_SPEC, 0> {
        DATA_W::new(self)
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
#[doc = "I2C Data Register. During master or slave transmit mode, data to be transmitted is written to this register. During master or slave receive mode, data that has been received may be read from this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAT_SPEC;
impl crate::RegisterSpec for DAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dat::R`](R) reader structure"]
impl crate::Readable for DAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dat::W`](W) writer structure"]
impl crate::Writable for DAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAT to value 0"]
impl crate::Resettable for DAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
