#[doc = "Register `STATR2` reader"]
pub type R = crate::R<STATR2_SPEC>;
#[doc = "Field `P2_00REI` reader - Status of Rising Edge Interrupt for P2\\[0\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_00REI_R = crate::BitReader;
#[doc = "Field `P2_01REI` reader - Status of Rising Edge Interrupt for P2\\[1\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_01REI_R = crate::BitReader;
#[doc = "Field `P2_02REI` reader - Status of Rising Edge Interrupt for P2\\[2\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_02REI_R = crate::BitReader;
#[doc = "Field `P2_03REI` reader - Status of Rising Edge Interrupt for P2\\[3\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_03REI_R = crate::BitReader;
#[doc = "Field `P2_04REI` reader - Status of Rising Edge Interrupt for P2\\[4\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_04REI_R = crate::BitReader;
#[doc = "Field `P2_05REI` reader - Status of Rising Edge Interrupt for P2\\[5\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_05REI_R = crate::BitReader;
#[doc = "Field `P2_06REI` reader - Status of Rising Edge Interrupt for P2\\[6\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_06REI_R = crate::BitReader;
#[doc = "Field `P2_07REI` reader - Status of Rising Edge Interrupt for P2\\[7\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_07REI_R = crate::BitReader;
#[doc = "Field `P2_08REI` reader - Status of Rising Edge Interrupt for P2\\[8\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_08REI_R = crate::BitReader;
#[doc = "Field `P2_09REI` reader - Status of Rising Edge Interrupt for P2\\[9\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_09REI_R = crate::BitReader;
#[doc = "Field `P2_10REI` reader - Status of Rising Edge Interrupt for P2\\[10\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_10REI_R = crate::BitReader;
#[doc = "Field `P2_11REI` reader - Status of Rising Edge Interrupt for P2\\[11\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_11REI_R = crate::BitReader;
#[doc = "Field `P2_12REI` reader - Status of Rising Edge Interrupt for P2\\[12\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_12REI_R = crate::BitReader;
#[doc = "Field `P2_13REI` reader - Status of Rising Edge Interrupt for P2\\[13\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub type P2_13REI_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Status of Rising Edge Interrupt for P2\\[0\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_00rei(&self) -> P2_00REI_R {
        P2_00REI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status of Rising Edge Interrupt for P2\\[1\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_01rei(&self) -> P2_01REI_R {
        P2_01REI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status of Rising Edge Interrupt for P2\\[2\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_02rei(&self) -> P2_02REI_R {
        P2_02REI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status of Rising Edge Interrupt for P2\\[3\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_03rei(&self) -> P2_03REI_R {
        P2_03REI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Status of Rising Edge Interrupt for P2\\[4\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_04rei(&self) -> P2_04REI_R {
        P2_04REI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status of Rising Edge Interrupt for P2\\[5\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_05rei(&self) -> P2_05REI_R {
        P2_05REI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Status of Rising Edge Interrupt for P2\\[6\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_06rei(&self) -> P2_06REI_R {
        P2_06REI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Status of Rising Edge Interrupt for P2\\[7\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_07rei(&self) -> P2_07REI_R {
        P2_07REI_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Status of Rising Edge Interrupt for P2\\[8\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_08rei(&self) -> P2_08REI_R {
        P2_08REI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Status of Rising Edge Interrupt for P2\\[9\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_09rei(&self) -> P2_09REI_R {
        P2_09REI_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Status of Rising Edge Interrupt for P2\\[10\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_10rei(&self) -> P2_10REI_R {
        P2_10REI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Status of Rising Edge Interrupt for P2\\[11\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_11rei(&self) -> P2_11REI_R {
        P2_11REI_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Status of Rising Edge Interrupt for P2\\[12\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_12rei(&self) -> P2_12REI_R {
        P2_12REI_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Status of Rising Edge Interrupt for P2\\[13\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_13rei(&self) -> P2_13REI_R {
        P2_13REI_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATR2")
            .field("p2_00rei", &format_args!("{}", self.p2_00rei().bit()))
            .field("p2_01rei", &format_args!("{}", self.p2_01rei().bit()))
            .field("p2_02rei", &format_args!("{}", self.p2_02rei().bit()))
            .field("p2_03rei", &format_args!("{}", self.p2_03rei().bit()))
            .field("p2_04rei", &format_args!("{}", self.p2_04rei().bit()))
            .field("p2_05rei", &format_args!("{}", self.p2_05rei().bit()))
            .field("p2_06rei", &format_args!("{}", self.p2_06rei().bit()))
            .field("p2_07rei", &format_args!("{}", self.p2_07rei().bit()))
            .field("p2_08rei", &format_args!("{}", self.p2_08rei().bit()))
            .field("p2_09rei", &format_args!("{}", self.p2_09rei().bit()))
            .field("p2_10rei", &format_args!("{}", self.p2_10rei().bit()))
            .field("p2_11rei", &format_args!("{}", self.p2_11rei().bit()))
            .field("p2_12rei", &format_args!("{}", self.p2_12rei().bit()))
            .field("p2_13rei", &format_args!("{}", self.p2_13rei().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<STATR2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "GPIO Interrupt Status for Rising edge for Port 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATR2_SPEC;
impl crate::RegisterSpec for STATR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statr2::R`](R) reader structure"]
impl crate::Readable for STATR2_SPEC {}
#[doc = "`reset()` method sets STATR2 to value 0"]
impl crate::Resettable for STATR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
