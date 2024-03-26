#[doc = "Register `MCMD` reader"]
pub type R = crate::R<MCMD_SPEC>;
#[doc = "Register `MCMD` writer"]
pub type W = crate::W<MCMD_SPEC>;
#[doc = "Field `READ` reader - This bit causes the MII Management hardware to perform a single Read cycle. The Read data is returned in Register MRDD (MII Mgmt Read Data)."]
pub type READ_R = crate::BitReader;
#[doc = "Field `READ` writer - This bit causes the MII Management hardware to perform a single Read cycle. The Read data is returned in Register MRDD (MII Mgmt Read Data)."]
pub type READ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCAN` reader - This bit causes the MII Management hardware to perform Read cycles continuously. This is useful for monitoring Link Fail for example."]
pub type SCAN_R = crate::BitReader;
#[doc = "Field `SCAN` writer - This bit causes the MII Management hardware to perform Read cycles continuously. This is useful for monitoring Link Fail for example."]
pub type SCAN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - This bit causes the MII Management hardware to perform a single Read cycle. The Read data is returned in Register MRDD (MII Mgmt Read Data)."]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit causes the MII Management hardware to perform Read cycles continuously. This is useful for monitoring Link Fail for example."]
    #[inline(always)]
    pub fn scan(&self) -> SCAN_R {
        SCAN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCMD")
            .field("read", &format_args!("{}", self.read().bit()))
            .field("scan", &format_args!("{}", self.scan().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<MCMD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - This bit causes the MII Management hardware to perform a single Read cycle. The Read data is returned in Register MRDD (MII Mgmt Read Data)."]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> READ_W<MCMD_SPEC, 0> {
        READ_W::new(self)
    }
    #[doc = "Bit 1 - This bit causes the MII Management hardware to perform Read cycles continuously. This is useful for monitoring Link Fail for example."]
    #[inline(always)]
    #[must_use]
    pub fn scan(&mut self) -> SCAN_W<MCMD_SPEC, 1> {
        SCAN_W::new(self)
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
#[doc = "MII Mgmt Command register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCMD_SPEC;
impl crate::RegisterSpec for MCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcmd::R`](R) reader structure"]
impl crate::Readable for MCMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcmd::W`](W) writer structure"]
impl crate::Writable for MCMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCMD to value 0"]
impl crate::Resettable for MCMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
