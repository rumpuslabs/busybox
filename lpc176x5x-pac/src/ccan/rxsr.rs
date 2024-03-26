#[doc = "Register `RXSR` reader"]
pub type R = crate::R<RXSR_SPEC>;
#[doc = "Field `RS1` reader - When 1, CAN1 is receiving a message (same as RS in CAN1GSR)."]
pub type RS1_R = crate::BitReader;
#[doc = "Field `RS2` reader - When 1, CAN2 is receiving a message (same as RS in CAN2GSR)."]
pub type RS2_R = crate::BitReader;
#[doc = "Field `RB1` reader - When 1, a received message is available in the CAN1 controller (same as RBS in CAN1GSR)."]
pub type RB1_R = crate::BitReader;
#[doc = "Field `RB2` reader - When 1, a received message is available in the CAN2 controller (same as RBS in CAN2GSR)."]
pub type RB2_R = crate::BitReader;
#[doc = "Field `DOS1` reader - When 1, a message was lost because the preceding message to CAN1 controller was not read out quickly enough (same as DOS in CAN1GSR)."]
pub type DOS1_R = crate::BitReader;
#[doc = "Field `DOS2` reader - When 1, a message was lost because the preceding message to CAN2 controller was not read out quickly enough (same as DOS in CAN2GSR)."]
pub type DOS2_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - When 1, CAN1 is receiving a message (same as RS in CAN1GSR)."]
    #[inline(always)]
    pub fn rs1(&self) -> RS1_R {
        RS1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, CAN2 is receiving a message (same as RS in CAN2GSR)."]
    #[inline(always)]
    pub fn rs2(&self) -> RS2_R {
        RS2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - When 1, a received message is available in the CAN1 controller (same as RBS in CAN1GSR)."]
    #[inline(always)]
    pub fn rb1(&self) -> RB1_R {
        RB1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When 1, a received message is available in the CAN2 controller (same as RBS in CAN2GSR)."]
    #[inline(always)]
    pub fn rb2(&self) -> RB2_R {
        RB2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - When 1, a message was lost because the preceding message to CAN1 controller was not read out quickly enough (same as DOS in CAN1GSR)."]
    #[inline(always)]
    pub fn dos1(&self) -> DOS1_R {
        DOS1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When 1, a message was lost because the preceding message to CAN2 controller was not read out quickly enough (same as DOS in CAN2GSR)."]
    #[inline(always)]
    pub fn dos2(&self) -> DOS2_R {
        DOS2_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXSR")
            .field("rs1", &format_args!("{}", self.rs1().bit()))
            .field("rs2", &format_args!("{}", self.rs2().bit()))
            .field("rb1", &format_args!("{}", self.rb1().bit()))
            .field("rb2", &format_args!("{}", self.rb2().bit()))
            .field("dos1", &format_args!("{}", self.dos1().bit()))
            .field("dos2", &format_args!("{}", self.dos2().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RXSR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "CAN Central Receive Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXSR_SPEC;
impl crate::RegisterSpec for RXSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxsr::R`](R) reader structure"]
impl crate::Readable for RXSR_SPEC {}
#[doc = "`reset()` method sets RXSR to value 0"]
impl crate::Resettable for RXSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
