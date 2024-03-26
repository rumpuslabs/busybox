#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `P0INT` reader - Port 0 GPIO interrupt pending."]
pub type P0INT_R = crate::BitReader<P0INT_A>;
#[doc = "Port 0 GPIO interrupt pending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0INT_A {
    #[doc = "0: No pending interrupts on Port 0."]
    NOT_PENDING = 0,
    #[doc = "1: At least one pending interrupt on Port 0."]
    PENDING = 1,
}
impl From<P0INT_A> for bool {
    #[inline(always)]
    fn from(variant: P0INT_A) -> Self {
        variant as u8 != 0
    }
}
impl P0INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0INT_A {
        match self.bits {
            false => P0INT_A::NOT_PENDING,
            true => P0INT_A::PENDING,
        }
    }
    #[doc = "No pending interrupts on Port 0."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == P0INT_A::NOT_PENDING
    }
    #[doc = "At least one pending interrupt on Port 0."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == P0INT_A::PENDING
    }
}
#[doc = "Field `P2INT` reader - Port 2 GPIO interrupt pending."]
pub type P2INT_R = crate::BitReader<P2INT_A>;
#[doc = "Port 2 GPIO interrupt pending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2INT_A {
    #[doc = "0: No pending interrupts on Port 2."]
    NOT_PENDING = 0,
    #[doc = "1: At least one pending interrupt on Port 2."]
    PENDING = 1,
}
impl From<P2INT_A> for bool {
    #[inline(always)]
    fn from(variant: P2INT_A) -> Self {
        variant as u8 != 0
    }
}
impl P2INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2INT_A {
        match self.bits {
            false => P2INT_A::NOT_PENDING,
            true => P2INT_A::PENDING,
        }
    }
    #[doc = "No pending interrupts on Port 2."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == P2INT_A::NOT_PENDING
    }
    #[doc = "At least one pending interrupt on Port 2."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == P2INT_A::PENDING
    }
}
impl R {
    #[doc = "Bit 0 - Port 0 GPIO interrupt pending."]
    #[inline(always)]
    pub fn p0int(&self) -> P0INT_R {
        P0INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Port 2 GPIO interrupt pending."]
    #[inline(always)]
    pub fn p2int(&self) -> P2INT_R {
        P2INT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("p0int", &format_args!("{}", self.p0int().bit()))
            .field("p2int", &format_args!("{}", self.p2int().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "GPIO overall Interrupt Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
