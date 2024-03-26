#[doc = "Register `MAXF` reader"]
pub type R = crate::R<MAXF_SPEC>;
#[doc = "Register `MAXF` writer"]
pub type W = crate::W<MAXF_SPEC>;
#[doc = "Field `MAXFLEN` reader - MAXIMUM FRAME LENGTH. This field resets to the value 0x0600, which represents a maximum receive frame of 1536 octets. An untagged maximum size Ethernet frame is 1518 octets. A tagged frame adds four octets for a total of 1522 octets. If a shorter maximum length restriction is desired, program this 16-bit field."]
pub type MAXFLEN_R = crate::FieldReader<u16>;
#[doc = "Field `MAXFLEN` writer - MAXIMUM FRAME LENGTH. This field resets to the value 0x0600, which represents a maximum receive frame of 1536 octets. An untagged maximum size Ethernet frame is 1518 octets. A tagged frame adds four octets for a total of 1522 octets. If a shorter maximum length restriction is desired, program this 16-bit field."]
pub type MAXFLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - MAXIMUM FRAME LENGTH. This field resets to the value 0x0600, which represents a maximum receive frame of 1536 octets. An untagged maximum size Ethernet frame is 1518 octets. A tagged frame adds four octets for a total of 1522 octets. If a shorter maximum length restriction is desired, program this 16-bit field."]
    #[inline(always)]
    pub fn maxflen(&self) -> MAXFLEN_R {
        MAXFLEN_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAXF")
            .field("maxflen", &format_args!("{}", self.maxflen().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<MAXF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAXIMUM FRAME LENGTH. This field resets to the value 0x0600, which represents a maximum receive frame of 1536 octets. An untagged maximum size Ethernet frame is 1518 octets. A tagged frame adds four octets for a total of 1522 octets. If a shorter maximum length restriction is desired, program this 16-bit field."]
    #[inline(always)]
    #[must_use]
    pub fn maxflen(&mut self) -> MAXFLEN_W<MAXF_SPEC, 0> {
        MAXFLEN_W::new(self)
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
#[doc = "Maximum Frame register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maxf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maxf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAXF_SPEC;
impl crate::RegisterSpec for MAXF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maxf::R`](R) reader structure"]
impl crate::Readable for MAXF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maxf::W`](W) writer structure"]
impl crate::Writable for MAXF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAXF to value 0x0600"]
impl crate::Resettable for MAXF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0600;
}
