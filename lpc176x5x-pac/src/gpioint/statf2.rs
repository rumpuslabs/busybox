#[doc = "Register `STATF2` reader"]
pub type R = crate::R<STATF2_SPEC>;
#[doc = "Field `P2_00FEI` reader - Status of Falling Edge Interrupt for P2\\[0\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_00FEI_R = crate::BitReader;
#[doc = "Field `P2_01FEI` reader - Status of Falling Edge Interrupt for P2\\[1\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_01FEI_R = crate::BitReader;
#[doc = "Field `P2_02FEI` reader - Status of Falling Edge Interrupt for P2\\[2\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_02FEI_R = crate::BitReader;
#[doc = "Field `P2_03FEI` reader - Status of Falling Edge Interrupt for P2\\[3\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_03FEI_R = crate::BitReader;
#[doc = "Field `P2_04FEI` reader - Status of Falling Edge Interrupt for P2\\[4\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_04FEI_R = crate::BitReader;
#[doc = "Field `P2_05FEI` reader - Status of Falling Edge Interrupt for P2\\[5\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_05FEI_R = crate::BitReader;
#[doc = "Field `P2_06FEI` reader - Status of Falling Edge Interrupt for P2\\[6\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_06FEI_R = crate::BitReader;
#[doc = "Field `P2_07FEI` reader - Status of Falling Edge Interrupt for P2\\[7\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_07FEI_R = crate::BitReader;
#[doc = "Field `P2_08FEI` reader - Status of Falling Edge Interrupt for P2\\[8\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_08FEI_R = crate::BitReader;
#[doc = "Field `P2_09FEI` reader - Status of Falling Edge Interrupt for P2\\[9\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_09FEI_R = crate::BitReader;
#[doc = "Field `P2_10FEI` reader - Status of Falling Edge Interrupt for P2\\[10\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_10FEI_R = crate::BitReader;
#[doc = "Field `P2_11FEI` reader - Status of Falling Edge Interrupt for P2\\[11\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_11FEI_R = crate::BitReader;
#[doc = "Field `P2_12FEI` reader - Status of Falling Edge Interrupt for P2\\[12\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_12FEI_R = crate::BitReader;
#[doc = "Field `P2_13FEI` reader - Status of Falling Edge Interrupt for P2\\[13\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub type P2_13FEI_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Status of Falling Edge Interrupt for P2\\[0\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_00fei(&self) -> P2_00FEI_R {
        P2_00FEI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status of Falling Edge Interrupt for P2\\[1\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_01fei(&self) -> P2_01FEI_R {
        P2_01FEI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status of Falling Edge Interrupt for P2\\[2\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_02fei(&self) -> P2_02FEI_R {
        P2_02FEI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status of Falling Edge Interrupt for P2\\[3\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_03fei(&self) -> P2_03FEI_R {
        P2_03FEI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Status of Falling Edge Interrupt for P2\\[4\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_04fei(&self) -> P2_04FEI_R {
        P2_04FEI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status of Falling Edge Interrupt for P2\\[5\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_05fei(&self) -> P2_05FEI_R {
        P2_05FEI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Status of Falling Edge Interrupt for P2\\[6\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_06fei(&self) -> P2_06FEI_R {
        P2_06FEI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Status of Falling Edge Interrupt for P2\\[7\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_07fei(&self) -> P2_07FEI_R {
        P2_07FEI_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Status of Falling Edge Interrupt for P2\\[8\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_08fei(&self) -> P2_08FEI_R {
        P2_08FEI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Status of Falling Edge Interrupt for P2\\[9\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_09fei(&self) -> P2_09FEI_R {
        P2_09FEI_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Status of Falling Edge Interrupt for P2\\[10\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_10fei(&self) -> P2_10FEI_R {
        P2_10FEI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Status of Falling Edge Interrupt for P2\\[11\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_11fei(&self) -> P2_11FEI_R {
        P2_11FEI_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Status of Falling Edge Interrupt for P2\\[12\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_12fei(&self) -> P2_12FEI_R {
        P2_12FEI_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Status of Falling Edge Interrupt for P2\\[13\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_13fei(&self) -> P2_13FEI_R {
        P2_13FEI_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATF2")
            .field("p2_00fei", &format_args!("{}", self.p2_00fei().bit()))
            .field("p2_01fei", &format_args!("{}", self.p2_01fei().bit()))
            .field("p2_02fei", &format_args!("{}", self.p2_02fei().bit()))
            .field("p2_03fei", &format_args!("{}", self.p2_03fei().bit()))
            .field("p2_04fei", &format_args!("{}", self.p2_04fei().bit()))
            .field("p2_05fei", &format_args!("{}", self.p2_05fei().bit()))
            .field("p2_06fei", &format_args!("{}", self.p2_06fei().bit()))
            .field("p2_07fei", &format_args!("{}", self.p2_07fei().bit()))
            .field("p2_08fei", &format_args!("{}", self.p2_08fei().bit()))
            .field("p2_09fei", &format_args!("{}", self.p2_09fei().bit()))
            .field("p2_10fei", &format_args!("{}", self.p2_10fei().bit()))
            .field("p2_11fei", &format_args!("{}", self.p2_11fei().bit()))
            .field("p2_12fei", &format_args!("{}", self.p2_12fei().bit()))
            .field("p2_13fei", &format_args!("{}", self.p2_13fei().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<STATF2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "GPIO Interrupt Status for Falling edge for Port 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statf2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATF2_SPEC;
impl crate::RegisterSpec for STATF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statf2::R`](R) reader structure"]
impl crate::Readable for STATF2_SPEC {}
#[doc = "`reset()` method sets STATF2 to value 0"]
impl crate::Resettable for STATF2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
