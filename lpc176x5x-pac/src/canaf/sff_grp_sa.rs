#[doc = "Register `SFF_GRP_SA` reader"]
pub type R = crate::R<SFF_GRP_SA_SPEC>;
#[doc = "Register `SFF_GRP_SA` writer"]
pub type W = crate::W<SFF_GRP_SA_SPEC>;
#[doc = "Field `SFF_GRP_SA` reader - The start address of the table of grouped Standard Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the EFF_sa register described below. The largest value that should be written to this register is 0x800, when only the Standard Individual table is used, and the last word (address 0x7FC) in AF Lookup Table RAM is used. For compatibility with possible future devices, please write zeroes in bits 31:12 and 1:0 of this register."]
pub type SFF_GRP_SA_R = crate::FieldReader<u16>;
#[doc = "Field `SFF_GRP_SA` writer - The start address of the table of grouped Standard Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the EFF_sa register described below. The largest value that should be written to this register is 0x800, when only the Standard Individual table is used, and the last word (address 0x7FC) in AF Lookup Table RAM is used. For compatibility with possible future devices, please write zeroes in bits 31:12 and 1:0 of this register."]
pub type SFF_GRP_SA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 2:11 - The start address of the table of grouped Standard Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the EFF_sa register described below. The largest value that should be written to this register is 0x800, when only the Standard Individual table is used, and the last word (address 0x7FC) in AF Lookup Table RAM is used. For compatibility with possible future devices, please write zeroes in bits 31:12 and 1:0 of this register."]
    #[inline(always)]
    pub fn sff_grp_sa(&self) -> SFF_GRP_SA_R {
        SFF_GRP_SA_R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFF_GRP_SA")
            .field("sff_grp_sa", &format_args!("{}", self.sff_grp_sa().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<SFF_GRP_SA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 2:11 - The start address of the table of grouped Standard Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the EFF_sa register described below. The largest value that should be written to this register is 0x800, when only the Standard Individual table is used, and the last word (address 0x7FC) in AF Lookup Table RAM is used. For compatibility with possible future devices, please write zeroes in bits 31:12 and 1:0 of this register."]
    #[inline(always)]
    #[must_use]
    pub fn sff_grp_sa(&mut self) -> SFF_GRP_SA_W<SFF_GRP_SA_SPEC, 2> {
        SFF_GRP_SA_W::new(self)
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
#[doc = "Standard Frame Group Start Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sff_grp_sa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sff_grp_sa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SFF_GRP_SA_SPEC;
impl crate::RegisterSpec for SFF_GRP_SA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sff_grp_sa::R`](R) reader structure"]
impl crate::Readable for SFF_GRP_SA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sff_grp_sa::W`](W) writer structure"]
impl crate::Writable for SFF_GRP_SA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFF_GRP_SA to value 0"]
impl crate::Resettable for SFF_GRP_SA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
