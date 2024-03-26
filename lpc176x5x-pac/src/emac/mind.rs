#[doc = "Register `MIND` reader"]
pub type R = crate::R<MIND_SPEC>;
#[doc = "Field `BUSY` reader - When 1 is returned - indicates MII Mgmt is currently performing an MII Mgmt Read or Write cycle."]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `SCANNING` reader - When 1 is returned - indicates a scan operation (continuous MII Mgmt Read cycles) is in progress."]
pub type SCANNING_R = crate::BitReader;
#[doc = "Field `NOTVALID` reader - When 1 is returned - indicates MII Mgmt Read cycle has not completed and the Read Data is not yet valid."]
pub type NOTVALID_R = crate::BitReader;
#[doc = "Field `MIILINKFAIL` reader - When 1 is returned - indicates that an MII Mgmt link fail has occurred."]
pub type MIILINKFAIL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - When 1 is returned - indicates MII Mgmt is currently performing an MII Mgmt Read or Write cycle."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1 is returned - indicates a scan operation (continuous MII Mgmt Read cycles) is in progress."]
    #[inline(always)]
    pub fn scanning(&self) -> SCANNING_R {
        SCANNING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When 1 is returned - indicates MII Mgmt Read cycle has not completed and the Read Data is not yet valid."]
    #[inline(always)]
    pub fn notvalid(&self) -> NOTVALID_R {
        NOTVALID_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When 1 is returned - indicates that an MII Mgmt link fail has occurred."]
    #[inline(always)]
    pub fn miilinkfail(&self) -> MIILINKFAIL_R {
        MIILINKFAIL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MIND")
            .field("busy", &format_args!("{}", self.busy().bit()))
            .field("scanning", &format_args!("{}", self.scanning().bit()))
            .field("notvalid", &format_args!("{}", self.notvalid().bit()))
            .field("miilinkfail", &format_args!("{}", self.miilinkfail().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<MIND_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "MII Mgmt Indicators register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mind::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIND_SPEC;
impl crate::RegisterSpec for MIND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mind::R`](R) reader structure"]
impl crate::Readable for MIND_SPEC {}
#[doc = "`reset()` method sets MIND to value 0"]
impl crate::Resettable for MIND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
