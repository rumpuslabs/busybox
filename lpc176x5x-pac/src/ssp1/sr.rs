#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Field `TFE` reader - Transmit FIFO Empty. This bit is 1 is the Transmit FIFO is empty, 0 if not."]
pub type TFE_R = crate::BitReader;
#[doc = "Field `TNF` reader - Transmit FIFO Not Full. This bit is 0 if the Tx FIFO is full, 1 if not."]
pub type TNF_R = crate::BitReader;
#[doc = "Field `RNE` reader - Receive FIFO Not Empty. This bit is 0 if the Receive FIFO is empty, 1 if not."]
pub type RNE_R = crate::BitReader;
#[doc = "Field `RFF` reader - Receive FIFO Full. This bit is 1 if the Receive FIFO is full, 0 if not."]
pub type RFF_R = crate::BitReader;
#[doc = "Field `BSY` reader - Busy. This bit is 0 if the SSPn controller is idle, or 1 if it is currently sending/receiving a frame and/or the Tx FIFO is not empty."]
pub type BSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit FIFO Empty. This bit is 1 is the Transmit FIFO is empty, 0 if not."]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Not Full. This bit is 0 if the Tx FIFO is full, 1 if not."]
    #[inline(always)]
    pub fn tnf(&self) -> TNF_R {
        TNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO Not Empty. This bit is 0 if the Receive FIFO is empty, 1 if not."]
    #[inline(always)]
    pub fn rne(&self) -> RNE_R {
        RNE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Full. This bit is 1 if the Receive FIFO is full, 0 if not."]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Busy. This bit is 0 if the SSPn controller is idle, or 1 if it is currently sending/receiving a frame and/or the Tx FIFO is not empty."]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("tfe", &format_args!("{}", self.tfe().bit()))
            .field("tnf", &format_args!("{}", self.tnf().bit()))
            .field("rne", &format_args!("{}", self.rne().bit()))
            .field("rff", &format_args!("{}", self.rff().bit()))
            .field("bsy", &format_args!("{}", self.bsy().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<SR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`reset()` method sets SR to value 0x03"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
