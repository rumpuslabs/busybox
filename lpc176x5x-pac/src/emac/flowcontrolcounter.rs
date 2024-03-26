#[doc = "Register `FLOWCONTROLCOUNTER` reader"]
pub type R = crate::R<FLOWCONTROLCOUNTER_SPEC>;
#[doc = "Register `FLOWCONTROLCOUNTER` writer"]
pub type W = crate::W<FLOWCONTROLCOUNTER_SPEC>;
#[doc = "Field `MC` reader - MirrorCounter. In full duplex mode the MirrorCounter specifies the number of cycles before re-issuing the Pause control frame."]
pub type MC_R = crate::FieldReader<u16>;
#[doc = "Field `MC` writer - MirrorCounter. In full duplex mode the MirrorCounter specifies the number of cycles before re-issuing the Pause control frame."]
pub type MC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `PT` reader - PauseTimer. In full-duplex mode the PauseTimer specifies the value that is inserted into the pause timer field of a pause flow control frame. In half duplex mode the PauseTimer specifies the number of backpressure cycles."]
pub type PT_R = crate::FieldReader<u16>;
#[doc = "Field `PT` writer - PauseTimer. In full-duplex mode the PauseTimer specifies the value that is inserted into the pause timer field of a pause flow control frame. In half duplex mode the PauseTimer specifies the number of backpressure cycles."]
pub type PT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - MirrorCounter. In full duplex mode the MirrorCounter specifies the number of cycles before re-issuing the Pause control frame."]
    #[inline(always)]
    pub fn mc(&self) -> MC_R {
        MC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - PauseTimer. In full-duplex mode the PauseTimer specifies the value that is inserted into the pause timer field of a pause flow control frame. In half duplex mode the PauseTimer specifies the number of backpressure cycles."]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLOWCONTROLCOUNTER")
            .field("mc", &format_args!("{}", self.mc().bits()))
            .field("pt", &format_args!("{}", self.pt().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<FLOWCONTROLCOUNTER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - MirrorCounter. In full duplex mode the MirrorCounter specifies the number of cycles before re-issuing the Pause control frame."]
    #[inline(always)]
    #[must_use]
    pub fn mc(&mut self) -> MC_W<FLOWCONTROLCOUNTER_SPEC, 0> {
        MC_W::new(self)
    }
    #[doc = "Bits 16:31 - PauseTimer. In full-duplex mode the PauseTimer specifies the value that is inserted into the pause timer field of a pause flow control frame. In half duplex mode the PauseTimer specifies the number of backpressure cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PT_W<FLOWCONTROLCOUNTER_SPEC, 16> {
        PT_W::new(self)
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
#[doc = "Flow control counter register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flowcontrolcounter::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flowcontrolcounter::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLOWCONTROLCOUNTER_SPEC;
impl crate::RegisterSpec for FLOWCONTROLCOUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flowcontrolcounter::R`](R) reader structure"]
impl crate::Readable for FLOWCONTROLCOUNTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flowcontrolcounter::W`](W) writer structure"]
impl crate::Writable for FLOWCONTROLCOUNTER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLOWCONTROLCOUNTER to value 0"]
impl crate::Resettable for FLOWCONTROLCOUNTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
