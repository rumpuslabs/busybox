#[doc = "Register `CLRT` reader"]
pub type R = crate::R<CLRT_SPEC>;
#[doc = "Register `CLRT` writer"]
pub type W = crate::W<CLRT_SPEC>;
#[doc = "Field `RETRANSMAX` reader - RETRANSMISSION MAXIMUM.This is a programmable field specifying the number of retransmission attempts following a collision before aborting the packet due to excessive collisions. The Standard specifies the attemptLimit to be 0xF (15d). See IEEE 802.3/4.2.3.2.5."]
pub type RETRANSMAX_R = crate::FieldReader;
#[doc = "Field `RETRANSMAX` writer - RETRANSMISSION MAXIMUM.This is a programmable field specifying the number of retransmission attempts following a collision before aborting the packet due to excessive collisions. The Standard specifies the attemptLimit to be 0xF (15d). See IEEE 802.3/4.2.3.2.5."]
pub type RETRANSMAX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `COLLWIN` reader - COLLISION WINDOW. This is a programmable field representing the slot time or collision window during which collisions occur in properly configured networks. The default value of 0x37 (55d) represents a 56 byte window following the preamble and SFD."]
pub type COLLWIN_R = crate::FieldReader;
#[doc = "Field `COLLWIN` writer - COLLISION WINDOW. This is a programmable field representing the slot time or collision window during which collisions occur in properly configured networks. The default value of 0x37 (55d) represents a 56 byte window following the preamble and SFD."]
pub type COLLWIN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:3 - RETRANSMISSION MAXIMUM.This is a programmable field specifying the number of retransmission attempts following a collision before aborting the packet due to excessive collisions. The Standard specifies the attemptLimit to be 0xF (15d). See IEEE 802.3/4.2.3.2.5."]
    #[inline(always)]
    pub fn retransmax(&self) -> RETRANSMAX_R {
        RETRANSMAX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - COLLISION WINDOW. This is a programmable field representing the slot time or collision window during which collisions occur in properly configured networks. The default value of 0x37 (55d) represents a 56 byte window following the preamble and SFD."]
    #[inline(always)]
    pub fn collwin(&self) -> COLLWIN_R {
        COLLWIN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLRT")
            .field("retransmax", &format_args!("{}", self.retransmax().bits()))
            .field("collwin", &format_args!("{}", self.collwin().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CLRT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - RETRANSMISSION MAXIMUM.This is a programmable field specifying the number of retransmission attempts following a collision before aborting the packet due to excessive collisions. The Standard specifies the attemptLimit to be 0xF (15d). See IEEE 802.3/4.2.3.2.5."]
    #[inline(always)]
    #[must_use]
    pub fn retransmax(&mut self) -> RETRANSMAX_W<CLRT_SPEC, 0> {
        RETRANSMAX_W::new(self)
    }
    #[doc = "Bits 8:13 - COLLISION WINDOW. This is a programmable field representing the slot time or collision window during which collisions occur in properly configured networks. The default value of 0x37 (55d) represents a 56 byte window following the preamble and SFD."]
    #[inline(always)]
    #[must_use]
    pub fn collwin(&mut self) -> COLLWIN_W<CLRT_SPEC, 8> {
        COLLWIN_W::new(self)
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
#[doc = "Collision window / Retry register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clrt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clrt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLRT_SPEC;
impl crate::RegisterSpec for CLRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clrt::R`](R) reader structure"]
impl crate::Readable for CLRT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clrt::W`](W) writer structure"]
impl crate::Writable for CLRT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLRT to value 0x370f"]
impl crate::Resettable for CLRT_SPEC {
    const RESET_VALUE: Self::Ux = 0x370f;
}
