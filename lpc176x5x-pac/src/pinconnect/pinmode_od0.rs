#[doc = "Register `PINMODE_OD0` reader"]
pub type R = crate::R<PINMODE_OD0_SPEC>;
#[doc = "Register `PINMODE_OD0` writer"]
pub type W = crate::W<PINMODE_OD0_SPEC>;
#[doc = "Field `P0_00OD` reader - Port 0 pin 0 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub type P0_00OD_R = crate::BitReader<PINMODE_OD_A>;
#[doc = "Port 0 pin 0 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINMODE_OD_A {
    #[doc = "0: Normal. Pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. Pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<PINMODE_OD_A> for bool {
    #[inline(always)]
    fn from(variant: PINMODE_OD_A) -> Self {
        variant as u8 != 0
    }
}
impl P0_00OD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PINMODE_OD_A {
        match self.bits {
            false => PINMODE_OD_A::NORMAL,
            true => PINMODE_OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Normal. Pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PINMODE_OD_A::NORMAL
    }
    #[doc = "Open-drain. Pin is in the open drain mode."]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == PINMODE_OD_A::OPEN_DRAIN
    }
}
#[doc = "Field `P0_00OD` writer - Port 0 pin 0 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub type P0_00OD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PINMODE_OD_A>;
impl<'a, REG, const O: u8> P0_00OD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. Pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PINMODE_OD_A::NORMAL)
    }
    #[doc = "Open-drain. Pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(PINMODE_OD_A::OPEN_DRAIN)
    }
}
#[doc = "Field `P0_01OD` reader - Port 0 pin 1 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub use P0_00OD_R as P0_01OD_R;
#[doc = "Field `P0_02OD` reader - Port 0 pin 2 open drain mode control"]
pub use P0_00OD_R as P0_02OD_R;
#[doc = "Field `P0_03OD` reader - Port 0 pin 3 open drain mode control"]
pub use P0_00OD_R as P0_03OD_R;
#[doc = "Field `P0_04OD` reader - Port 0 pin 4 open drain mode control"]
pub use P0_00OD_R as P0_04OD_R;
#[doc = "Field `P0_05OD` reader - Port 0 pin 5 open drain mode control"]
pub use P0_00OD_R as P0_05OD_R;
#[doc = "Field `P0_06OD` reader - Port 0 pin 6 open drain mode control"]
pub use P0_00OD_R as P0_06OD_R;
#[doc = "Field `P0_07OD` reader - Port 0 pin 7 open drain mode control"]
pub use P0_00OD_R as P0_07OD_R;
#[doc = "Field `P0_08OD` reader - Port 0 pin 8 open drain mode control"]
pub use P0_00OD_R as P0_08OD_R;
#[doc = "Field `P0_09OD` reader - Port 0 pin 9 open drain mode control"]
pub use P0_00OD_R as P0_09OD_R;
#[doc = "Field `P0_10OD` reader - Port 0 pin 10 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub use P0_00OD_R as P0_10OD_R;
#[doc = "Field `P0_11OD` reader - Port 0 pin 11 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub use P0_00OD_R as P0_11OD_R;
#[doc = "Field `P0_15OD` reader - Port 0 pin 15 open drain mode control"]
pub use P0_00OD_R as P0_15OD_R;
#[doc = "Field `P0_16OD` reader - Port 0 pin 16 open drain mode control"]
pub use P0_00OD_R as P0_16OD_R;
#[doc = "Field `P0_17OD` reader - Port 0 pin 17 open drain mode control"]
pub use P0_00OD_R as P0_17OD_R;
#[doc = "Field `P0_18OD` reader - Port 0 pin 18 open drain mode control"]
pub use P0_00OD_R as P0_18OD_R;
#[doc = "Field `P0_19OD` reader - Port 0 pin 19 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub use P0_00OD_R as P0_19OD_R;
#[doc = "Field `P0_20OD` reader - Port 0 pin 20open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub use P0_00OD_R as P0_20OD_R;
#[doc = "Field `P0_21OD` reader - Port 0 pin 21 open drain mode control"]
pub use P0_00OD_R as P0_21OD_R;
#[doc = "Field `P0_22OD` reader - Port 0 pin 22 open drain mode control"]
pub use P0_00OD_R as P0_22OD_R;
#[doc = "Field `P0_23OD` reader - Port 0 pin 23 open drain mode control"]
pub use P0_00OD_R as P0_23OD_R;
#[doc = "Field `P0_24OD` reader - Port 0 pin 24open drain mode control"]
pub use P0_00OD_R as P0_24OD_R;
#[doc = "Field `P0_25OD` reader - Port 0 pin 25 open drain mode control"]
pub use P0_00OD_R as P0_25OD_R;
#[doc = "Field `P0_26OD` reader - Port 0 pin 26 open drain mode control"]
pub use P0_00OD_R as P0_26OD_R;
#[doc = "Field `P0_29OD` reader - Port 0 pin 29 open drain mode control"]
pub use P0_00OD_R as P0_29OD_R;
#[doc = "Field `P0_30OD` reader - Port 0 pin 30 open drain mode control"]
pub use P0_00OD_R as P0_30OD_R;
#[doc = "Field `P0_01OD` writer - Port 0 pin 1 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub use P0_00OD_W as P0_01OD_W;
#[doc = "Field `P0_02OD` writer - Port 0 pin 2 open drain mode control"]
pub use P0_00OD_W as P0_02OD_W;
#[doc = "Field `P0_03OD` writer - Port 0 pin 3 open drain mode control"]
pub use P0_00OD_W as P0_03OD_W;
#[doc = "Field `P0_04OD` writer - Port 0 pin 4 open drain mode control"]
pub use P0_00OD_W as P0_04OD_W;
#[doc = "Field `P0_05OD` writer - Port 0 pin 5 open drain mode control"]
pub use P0_00OD_W as P0_05OD_W;
#[doc = "Field `P0_06OD` writer - Port 0 pin 6 open drain mode control"]
pub use P0_00OD_W as P0_06OD_W;
#[doc = "Field `P0_07OD` writer - Port 0 pin 7 open drain mode control"]
pub use P0_00OD_W as P0_07OD_W;
#[doc = "Field `P0_08OD` writer - Port 0 pin 8 open drain mode control"]
pub use P0_00OD_W as P0_08OD_W;
#[doc = "Field `P0_09OD` writer - Port 0 pin 9 open drain mode control"]
pub use P0_00OD_W as P0_09OD_W;
#[doc = "Field `P0_10OD` writer - Port 0 pin 10 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub use P0_00OD_W as P0_10OD_W;
#[doc = "Field `P0_11OD` writer - Port 0 pin 11 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub use P0_00OD_W as P0_11OD_W;
#[doc = "Field `P0_15OD` writer - Port 0 pin 15 open drain mode control"]
pub use P0_00OD_W as P0_15OD_W;
#[doc = "Field `P0_16OD` writer - Port 0 pin 16 open drain mode control"]
pub use P0_00OD_W as P0_16OD_W;
#[doc = "Field `P0_17OD` writer - Port 0 pin 17 open drain mode control"]
pub use P0_00OD_W as P0_17OD_W;
#[doc = "Field `P0_18OD` writer - Port 0 pin 18 open drain mode control"]
pub use P0_00OD_W as P0_18OD_W;
#[doc = "Field `P0_19OD` writer - Port 0 pin 19 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub use P0_00OD_W as P0_19OD_W;
#[doc = "Field `P0_20OD` writer - Port 0 pin 20open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
pub use P0_00OD_W as P0_20OD_W;
#[doc = "Field `P0_21OD` writer - Port 0 pin 21 open drain mode control"]
pub use P0_00OD_W as P0_21OD_W;
#[doc = "Field `P0_22OD` writer - Port 0 pin 22 open drain mode control"]
pub use P0_00OD_W as P0_22OD_W;
#[doc = "Field `P0_23OD` writer - Port 0 pin 23 open drain mode control"]
pub use P0_00OD_W as P0_23OD_W;
#[doc = "Field `P0_24OD` writer - Port 0 pin 24open drain mode control"]
pub use P0_00OD_W as P0_24OD_W;
#[doc = "Field `P0_25OD` writer - Port 0 pin 25 open drain mode control"]
pub use P0_00OD_W as P0_25OD_W;
#[doc = "Field `P0_26OD` writer - Port 0 pin 26 open drain mode control"]
pub use P0_00OD_W as P0_26OD_W;
#[doc = "Field `P0_29OD` writer - Port 0 pin 29 open drain mode control"]
pub use P0_00OD_W as P0_29OD_W;
#[doc = "Field `P0_30OD` writer - Port 0 pin 30 open drain mode control"]
pub use P0_00OD_W as P0_30OD_W;
impl R {
    #[doc = "Bit 0 - Port 0 pin 0 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_00od(&self) -> P0_00OD_R {
        P0_00OD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port 0 pin 1 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_01od(&self) -> P0_01OD_R {
        P0_01OD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port 0 pin 2 open drain mode control"]
    #[inline(always)]
    pub fn p0_02od(&self) -> P0_02OD_R {
        P0_02OD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port 0 pin 3 open drain mode control"]
    #[inline(always)]
    pub fn p0_03od(&self) -> P0_03OD_R {
        P0_03OD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port 0 pin 4 open drain mode control"]
    #[inline(always)]
    pub fn p0_04od(&self) -> P0_04OD_R {
        P0_04OD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port 0 pin 5 open drain mode control"]
    #[inline(always)]
    pub fn p0_05od(&self) -> P0_05OD_R {
        P0_05OD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port 0 pin 6 open drain mode control"]
    #[inline(always)]
    pub fn p0_06od(&self) -> P0_06OD_R {
        P0_06OD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port 0 pin 7 open drain mode control"]
    #[inline(always)]
    pub fn p0_07od(&self) -> P0_07OD_R {
        P0_07OD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port 0 pin 8 open drain mode control"]
    #[inline(always)]
    pub fn p0_08od(&self) -> P0_08OD_R {
        P0_08OD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port 0 pin 9 open drain mode control"]
    #[inline(always)]
    pub fn p0_09od(&self) -> P0_09OD_R {
        P0_09OD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port 0 pin 10 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_10od(&self) -> P0_10OD_R {
        P0_10OD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port 0 pin 11 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_11od(&self) -> P0_11OD_R {
        P0_11OD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Port 0 pin 15 open drain mode control"]
    #[inline(always)]
    pub fn p0_15od(&self) -> P0_15OD_R {
        P0_15OD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Port 0 pin 16 open drain mode control"]
    #[inline(always)]
    pub fn p0_16od(&self) -> P0_16OD_R {
        P0_16OD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Port 0 pin 17 open drain mode control"]
    #[inline(always)]
    pub fn p0_17od(&self) -> P0_17OD_R {
        P0_17OD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Port 0 pin 18 open drain mode control"]
    #[inline(always)]
    pub fn p0_18od(&self) -> P0_18OD_R {
        P0_18OD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Port 0 pin 19 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_19od(&self) -> P0_19OD_R {
        P0_19OD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Port 0 pin 20open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_20od(&self) -> P0_20OD_R {
        P0_20OD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Port 0 pin 21 open drain mode control"]
    #[inline(always)]
    pub fn p0_21od(&self) -> P0_21OD_R {
        P0_21OD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Port 0 pin 22 open drain mode control"]
    #[inline(always)]
    pub fn p0_22od(&self) -> P0_22OD_R {
        P0_22OD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Port 0 pin 23 open drain mode control"]
    #[inline(always)]
    pub fn p0_23od(&self) -> P0_23OD_R {
        P0_23OD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Port 0 pin 24open drain mode control"]
    #[inline(always)]
    pub fn p0_24od(&self) -> P0_24OD_R {
        P0_24OD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Port 0 pin 25 open drain mode control"]
    #[inline(always)]
    pub fn p0_25od(&self) -> P0_25OD_R {
        P0_25OD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Port 0 pin 26 open drain mode control"]
    #[inline(always)]
    pub fn p0_26od(&self) -> P0_26OD_R {
        P0_26OD_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - Port 0 pin 29 open drain mode control"]
    #[inline(always)]
    pub fn p0_29od(&self) -> P0_29OD_R {
        P0_29OD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Port 0 pin 30 open drain mode control"]
    #[inline(always)]
    pub fn p0_30od(&self) -> P0_30OD_R {
        P0_30OD_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PINMODE_OD0")
            .field("p0_00od", &format_args!("{}", self.p0_00od().bit()))
            .field("p0_01od", &format_args!("{}", self.p0_01od().bit()))
            .field("p0_02od", &format_args!("{}", self.p0_02od().bit()))
            .field("p0_03od", &format_args!("{}", self.p0_03od().bit()))
            .field("p0_04od", &format_args!("{}", self.p0_04od().bit()))
            .field("p0_05od", &format_args!("{}", self.p0_05od().bit()))
            .field("p0_06od", &format_args!("{}", self.p0_06od().bit()))
            .field("p0_07od", &format_args!("{}", self.p0_07od().bit()))
            .field("p0_08od", &format_args!("{}", self.p0_08od().bit()))
            .field("p0_09od", &format_args!("{}", self.p0_09od().bit()))
            .field("p0_10od", &format_args!("{}", self.p0_10od().bit()))
            .field("p0_11od", &format_args!("{}", self.p0_11od().bit()))
            .field("p0_15od", &format_args!("{}", self.p0_15od().bit()))
            .field("p0_16od", &format_args!("{}", self.p0_16od().bit()))
            .field("p0_17od", &format_args!("{}", self.p0_17od().bit()))
            .field("p0_18od", &format_args!("{}", self.p0_18od().bit()))
            .field("p0_19od", &format_args!("{}", self.p0_19od().bit()))
            .field("p0_20od", &format_args!("{}", self.p0_20od().bit()))
            .field("p0_21od", &format_args!("{}", self.p0_21od().bit()))
            .field("p0_22od", &format_args!("{}", self.p0_22od().bit()))
            .field("p0_23od", &format_args!("{}", self.p0_23od().bit()))
            .field("p0_24od", &format_args!("{}", self.p0_24od().bit()))
            .field("p0_25od", &format_args!("{}", self.p0_25od().bit()))
            .field("p0_26od", &format_args!("{}", self.p0_26od().bit()))
            .field("p0_29od", &format_args!("{}", self.p0_29od().bit()))
            .field("p0_30od", &format_args!("{}", self.p0_30od().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PINMODE_OD0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Port 0 pin 0 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    #[must_use]
    pub fn p0_00od(&mut self) -> P0_00OD_W<PINMODE_OD0_SPEC, 0> {
        P0_00OD_W::new(self)
    }
    #[doc = "Bit 1 - Port 0 pin 1 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    #[must_use]
    pub fn p0_01od(&mut self) -> P0_01OD_W<PINMODE_OD0_SPEC, 1> {
        P0_01OD_W::new(self)
    }
    #[doc = "Bit 2 - Port 0 pin 2 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_02od(&mut self) -> P0_02OD_W<PINMODE_OD0_SPEC, 2> {
        P0_02OD_W::new(self)
    }
    #[doc = "Bit 3 - Port 0 pin 3 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_03od(&mut self) -> P0_03OD_W<PINMODE_OD0_SPEC, 3> {
        P0_03OD_W::new(self)
    }
    #[doc = "Bit 4 - Port 0 pin 4 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_04od(&mut self) -> P0_04OD_W<PINMODE_OD0_SPEC, 4> {
        P0_04OD_W::new(self)
    }
    #[doc = "Bit 5 - Port 0 pin 5 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_05od(&mut self) -> P0_05OD_W<PINMODE_OD0_SPEC, 5> {
        P0_05OD_W::new(self)
    }
    #[doc = "Bit 6 - Port 0 pin 6 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_06od(&mut self) -> P0_06OD_W<PINMODE_OD0_SPEC, 6> {
        P0_06OD_W::new(self)
    }
    #[doc = "Bit 7 - Port 0 pin 7 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_07od(&mut self) -> P0_07OD_W<PINMODE_OD0_SPEC, 7> {
        P0_07OD_W::new(self)
    }
    #[doc = "Bit 8 - Port 0 pin 8 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_08od(&mut self) -> P0_08OD_W<PINMODE_OD0_SPEC, 8> {
        P0_08OD_W::new(self)
    }
    #[doc = "Bit 9 - Port 0 pin 9 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_09od(&mut self) -> P0_09OD_W<PINMODE_OD0_SPEC, 9> {
        P0_09OD_W::new(self)
    }
    #[doc = "Bit 10 - Port 0 pin 10 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    #[must_use]
    pub fn p0_10od(&mut self) -> P0_10OD_W<PINMODE_OD0_SPEC, 10> {
        P0_10OD_W::new(self)
    }
    #[doc = "Bit 11 - Port 0 pin 11 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    #[must_use]
    pub fn p0_11od(&mut self) -> P0_11OD_W<PINMODE_OD0_SPEC, 11> {
        P0_11OD_W::new(self)
    }
    #[doc = "Bit 15 - Port 0 pin 15 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_15od(&mut self) -> P0_15OD_W<PINMODE_OD0_SPEC, 15> {
        P0_15OD_W::new(self)
    }
    #[doc = "Bit 16 - Port 0 pin 16 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_16od(&mut self) -> P0_16OD_W<PINMODE_OD0_SPEC, 16> {
        P0_16OD_W::new(self)
    }
    #[doc = "Bit 17 - Port 0 pin 17 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_17od(&mut self) -> P0_17OD_W<PINMODE_OD0_SPEC, 17> {
        P0_17OD_W::new(self)
    }
    #[doc = "Bit 18 - Port 0 pin 18 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_18od(&mut self) -> P0_18OD_W<PINMODE_OD0_SPEC, 18> {
        P0_18OD_W::new(self)
    }
    #[doc = "Bit 19 - Port 0 pin 19 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    #[must_use]
    pub fn p0_19od(&mut self) -> P0_19OD_W<PINMODE_OD0_SPEC, 19> {
        P0_19OD_W::new(self)
    }
    #[doc = "Bit 20 - Port 0 pin 20open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    #[must_use]
    pub fn p0_20od(&mut self) -> P0_20OD_W<PINMODE_OD0_SPEC, 20> {
        P0_20OD_W::new(self)
    }
    #[doc = "Bit 21 - Port 0 pin 21 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_21od(&mut self) -> P0_21OD_W<PINMODE_OD0_SPEC, 21> {
        P0_21OD_W::new(self)
    }
    #[doc = "Bit 22 - Port 0 pin 22 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_22od(&mut self) -> P0_22OD_W<PINMODE_OD0_SPEC, 22> {
        P0_22OD_W::new(self)
    }
    #[doc = "Bit 23 - Port 0 pin 23 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_23od(&mut self) -> P0_23OD_W<PINMODE_OD0_SPEC, 23> {
        P0_23OD_W::new(self)
    }
    #[doc = "Bit 24 - Port 0 pin 24open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_24od(&mut self) -> P0_24OD_W<PINMODE_OD0_SPEC, 24> {
        P0_24OD_W::new(self)
    }
    #[doc = "Bit 25 - Port 0 pin 25 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_25od(&mut self) -> P0_25OD_W<PINMODE_OD0_SPEC, 25> {
        P0_25OD_W::new(self)
    }
    #[doc = "Bit 26 - Port 0 pin 26 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_26od(&mut self) -> P0_26OD_W<PINMODE_OD0_SPEC, 26> {
        P0_26OD_W::new(self)
    }
    #[doc = "Bit 29 - Port 0 pin 29 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_29od(&mut self) -> P0_29OD_W<PINMODE_OD0_SPEC, 29> {
        P0_29OD_W::new(self)
    }
    #[doc = "Bit 30 - Port 0 pin 30 open drain mode control"]
    #[inline(always)]
    #[must_use]
    pub fn p0_30od(&mut self) -> P0_30OD_W<PINMODE_OD0_SPEC, 30> {
        P0_30OD_W::new(self)
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
#[doc = "Open drain mode control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinmode_od0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinmode_od0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINMODE_OD0_SPEC;
impl crate::RegisterSpec for PINMODE_OD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode_od0::R`](R) reader structure"]
impl crate::Readable for PINMODE_OD0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pinmode_od0::W`](W) writer structure"]
impl crate::Writable for PINMODE_OD0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINMODE_OD0 to value 0"]
impl crate::Resettable for PINMODE_OD0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
