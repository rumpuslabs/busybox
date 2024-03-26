#[doc = "Register `CONF` reader"]
pub type R = crate::R<CONF_SPEC>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<CONF_SPEC>;
#[doc = "Field `DIRINV` reader - Direction invert. When 1, complements the DIR bit."]
pub type DIRINV_R = crate::BitReader;
#[doc = "Field `DIRINV` writer - Direction invert. When 1, complements the DIR bit."]
pub type DIRINV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SIGMODE` reader - Signal Mode. When 0, PhA and PhB function as quadrature encoder inputs. When 1, PhA functions as the direction signal and PhB functions as the clock signal."]
pub type SIGMODE_R = crate::BitReader;
#[doc = "Field `SIGMODE` writer - Signal Mode. When 0, PhA and PhB function as quadrature encoder inputs. When 1, PhA functions as the direction signal and PhB functions as the clock signal."]
pub type SIGMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAPMODE` reader - Capture Mode. When 0, only PhA edges are counted (2X). When 1, BOTH PhA and PhB edges are counted (4X), increasing resolution but decreasing range."]
pub type CAPMODE_R = crate::BitReader;
#[doc = "Field `CAPMODE` writer - Capture Mode. When 0, only PhA edges are counted (2X). When 1, BOTH PhA and PhB edges are counted (4X), increasing resolution but decreasing range."]
pub type CAPMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INVINX` reader - Invert Index. When 1, inverts the sense of the index input."]
pub type INVINX_R = crate::BitReader;
#[doc = "Field `INVINX` writer - Invert Index. When 1, inverts the sense of the index input."]
pub type INVINX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRESPI` reader - Continuously reset the position counter on index. When 1, resets the position counter to all zeros whenever an index pulse occurs after the next position increase (recalibration)."]
pub type CRESPI_R = crate::BitReader;
#[doc = "Field `CRESPI` writer - Continuously reset the position counter on index. When 1, resets the position counter to all zeros whenever an index pulse occurs after the next position increase (recalibration)."]
pub type CRESPI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INXGATE` reader - Index gating configuration: When INXGATE\\[16\\]
= 1, pass the index when PHA = 1 and PHB = 0, otherwise block index. When INXGATE\\[17\\]
= 1, pass the index when PHA = 1 and PHB = 1, otherwise block index. When INXGATE\\[18\\]
= 1, pass the index when PHA = 0 and PHB = 1, otherwise block index. When INXGATE\\[19\\]
= 1, pass the index when PHA = 0 and PHB = 0, otherwise block index."]
pub type INXGATE_R = crate::FieldReader;
#[doc = "Field `INXGATE` writer - Index gating configuration: When INXGATE\\[16\\]
= 1, pass the index when PHA = 1 and PHB = 0, otherwise block index. When INXGATE\\[17\\]
= 1, pass the index when PHA = 1 and PHB = 1, otherwise block index. When INXGATE\\[18\\]
= 1, pass the index when PHA = 0 and PHB = 1, otherwise block index. When INXGATE\\[19\\]
= 1, pass the index when PHA = 0 and PHB = 0, otherwise block index."]
pub type INXGATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - Direction invert. When 1, complements the DIR bit."]
    #[inline(always)]
    pub fn dirinv(&self) -> DIRINV_R {
        DIRINV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Signal Mode. When 0, PhA and PhB function as quadrature encoder inputs. When 1, PhA functions as the direction signal and PhB functions as the clock signal."]
    #[inline(always)]
    pub fn sigmode(&self) -> SIGMODE_R {
        SIGMODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture Mode. When 0, only PhA edges are counted (2X). When 1, BOTH PhA and PhB edges are counted (4X), increasing resolution but decreasing range."]
    #[inline(always)]
    pub fn capmode(&self) -> CAPMODE_R {
        CAPMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Invert Index. When 1, inverts the sense of the index input."]
    #[inline(always)]
    pub fn invinx(&self) -> INVINX_R {
        INVINX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Continuously reset the position counter on index. When 1, resets the position counter to all zeros whenever an index pulse occurs after the next position increase (recalibration)."]
    #[inline(always)]
    pub fn crespi(&self) -> CRESPI_R {
        CRESPI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Index gating configuration: When INXGATE\\[16\\]
= 1, pass the index when PHA = 1 and PHB = 0, otherwise block index. When INXGATE\\[17\\]
= 1, pass the index when PHA = 1 and PHB = 1, otherwise block index. When INXGATE\\[18\\]
= 1, pass the index when PHA = 0 and PHB = 1, otherwise block index. When INXGATE\\[19\\]
= 1, pass the index when PHA = 0 and PHB = 0, otherwise block index."]
    #[inline(always)]
    pub fn inxgate(&self) -> INXGATE_R {
        INXGATE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field("dirinv", &format_args!("{}", self.dirinv().bit()))
            .field("sigmode", &format_args!("{}", self.sigmode().bit()))
            .field("capmode", &format_args!("{}", self.capmode().bit()))
            .field("invinx", &format_args!("{}", self.invinx().bit()))
            .field("crespi", &format_args!("{}", self.crespi().bit()))
            .field("inxgate", &format_args!("{}", self.inxgate().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Direction invert. When 1, complements the DIR bit."]
    #[inline(always)]
    #[must_use]
    pub fn dirinv(&mut self) -> DIRINV_W<CONF_SPEC, 0> {
        DIRINV_W::new(self)
    }
    #[doc = "Bit 1 - Signal Mode. When 0, PhA and PhB function as quadrature encoder inputs. When 1, PhA functions as the direction signal and PhB functions as the clock signal."]
    #[inline(always)]
    #[must_use]
    pub fn sigmode(&mut self) -> SIGMODE_W<CONF_SPEC, 1> {
        SIGMODE_W::new(self)
    }
    #[doc = "Bit 2 - Capture Mode. When 0, only PhA edges are counted (2X). When 1, BOTH PhA and PhB edges are counted (4X), increasing resolution but decreasing range."]
    #[inline(always)]
    #[must_use]
    pub fn capmode(&mut self) -> CAPMODE_W<CONF_SPEC, 2> {
        CAPMODE_W::new(self)
    }
    #[doc = "Bit 3 - Invert Index. When 1, inverts the sense of the index input."]
    #[inline(always)]
    #[must_use]
    pub fn invinx(&mut self) -> INVINX_W<CONF_SPEC, 3> {
        INVINX_W::new(self)
    }
    #[doc = "Bit 4 - Continuously reset the position counter on index. When 1, resets the position counter to all zeros whenever an index pulse occurs after the next position increase (recalibration)."]
    #[inline(always)]
    #[must_use]
    pub fn crespi(&mut self) -> CRESPI_W<CONF_SPEC, 4> {
        CRESPI_W::new(self)
    }
    #[doc = "Bits 16:19 - Index gating configuration: When INXGATE\\[16\\]
= 1, pass the index when PHA = 1 and PHB = 0, otherwise block index. When INXGATE\\[17\\]
= 1, pass the index when PHA = 1 and PHB = 1, otherwise block index. When INXGATE\\[18\\]
= 1, pass the index when PHA = 0 and PHB = 1, otherwise block index. When INXGATE\\[19\\]
= 1, pass the index when PHA = 0 and PHB = 0, otherwise block index."]
    #[inline(always)]
    #[must_use]
    pub fn inxgate(&mut self) -> INXGATE_W<CONF_SPEC, 16> {
        INXGATE_W::new(self)
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
#[doc = "Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf::W`](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF to value 0"]
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
