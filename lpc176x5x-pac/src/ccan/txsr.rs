#[doc = "Register `TXSR` reader"]
pub type R = crate::R<TXSR_SPEC>;
#[doc = "Field `TS1` reader - When 1, the CAN controller 1 is sending a message (same as TS in the CAN1GSR)."]
pub type TS1_R = crate::BitReader;
#[doc = "Field `TS2` reader - When 1, the CAN controller 2 is sending a message (same as TS in the CAN2GSR)"]
pub type TS2_R = crate::BitReader;
#[doc = "Field `TBS1` reader - When 1, all 3 Tx Buffers of the CAN1 controller are available to the CPU (same as TBS in CAN1GSR)."]
pub type TBS1_R = crate::BitReader;
#[doc = "Field `TBS2` reader - When 1, all 3 Tx Buffers of the CAN2 controller are available to the CPU (same as TBS in CAN2GSR)."]
pub type TBS2_R = crate::BitReader;
#[doc = "Field `TCS1` reader - When 1, all requested transmissions have been completed successfully by the CAN1 controller (same as TCS in CAN1GSR)."]
pub type TCS1_R = crate::BitReader;
#[doc = "Field `TCS2` reader - When 1, all requested transmissions have been completed successfully by the CAN2 controller (same as TCS in CAN2GSR)."]
pub type TCS2_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - When 1, the CAN controller 1 is sending a message (same as TS in the CAN1GSR)."]
    #[inline(always)]
    pub fn ts1(&self) -> TS1_R {
        TS1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, the CAN controller 2 is sending a message (same as TS in the CAN2GSR)"]
    #[inline(always)]
    pub fn ts2(&self) -> TS2_R {
        TS2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - When 1, all 3 Tx Buffers of the CAN1 controller are available to the CPU (same as TBS in CAN1GSR)."]
    #[inline(always)]
    pub fn tbs1(&self) -> TBS1_R {
        TBS1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When 1, all 3 Tx Buffers of the CAN2 controller are available to the CPU (same as TBS in CAN2GSR)."]
    #[inline(always)]
    pub fn tbs2(&self) -> TBS2_R {
        TBS2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - When 1, all requested transmissions have been completed successfully by the CAN1 controller (same as TCS in CAN1GSR)."]
    #[inline(always)]
    pub fn tcs1(&self) -> TCS1_R {
        TCS1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When 1, all requested transmissions have been completed successfully by the CAN2 controller (same as TCS in CAN2GSR)."]
    #[inline(always)]
    pub fn tcs2(&self) -> TCS2_R {
        TCS2_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXSR")
            .field("ts1", &format_args!("{}", self.ts1().bit()))
            .field("ts2", &format_args!("{}", self.ts2().bit()))
            .field("tbs1", &format_args!("{}", self.tbs1().bit()))
            .field("tbs2", &format_args!("{}", self.tbs2().bit()))
            .field("tcs1", &format_args!("{}", self.tcs1().bit()))
            .field("tcs2", &format_args!("{}", self.tcs2().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TXSR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "CAN Central Transmit Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXSR_SPEC;
impl crate::RegisterSpec for TXSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txsr::R`](R) reader structure"]
impl crate::Readable for TXSR_SPEC {}
#[doc = "`reset()` method sets TXSR to value 0x0003_0300"]
impl crate::Resettable for TXSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0300;
}
