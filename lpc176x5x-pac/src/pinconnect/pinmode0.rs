#[doc = "Register `PINMODE0` reader"]
pub type R = crate::R<PINMODE0_SPEC>;
#[doc = "Register `PINMODE0` writer"]
pub type W = crate::W<PINMODE0_SPEC>;
#[doc = "Field `P0_00MODE` reader - Port 0 pin 0 on-chip pull-up/down resistor control."]
pub type P0_00MODE_R = crate::FieldReader<PINMODE_A>;
#[doc = "Port 0 pin 0 on-chip pull-up/down resistor control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PINMODE_A {
    #[doc = "0: Pull-up. Pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. Pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. Pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. Pin has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<PINMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PINMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PINMODE_A {
    type Ux = u8;
}
impl P0_00MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PINMODE_A {
        match self.bits {
            0 => PINMODE_A::PULL_UP,
            1 => PINMODE_A::REPEATER,
            2 => PINMODE_A::DISABLED,
            3 => PINMODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Pull-up. Pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PINMODE_A::PULL_UP
    }
    #[doc = "Repeater. Pin has repeater mode enabled."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == PINMODE_A::REPEATER
    }
    #[doc = "Disabled. Pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINMODE_A::DISABLED
    }
    #[doc = "Pull-down. Pin has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PINMODE_A::PULL_DOWN
    }
}
#[doc = "Field `P0_00MODE` writer - Port 0 pin 0 on-chip pull-up/down resistor control."]
pub type P0_00MODE_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PINMODE_A>;
impl<'a, REG, const O: u8> P0_00MODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pull-up. Pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PINMODE_A::PULL_UP)
    }
    #[doc = "Repeater. Pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(PINMODE_A::REPEATER)
    }
    #[doc = "Disabled. Pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PINMODE_A::DISABLED)
    }
    #[doc = "Pull-down. Pin has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PINMODE_A::PULL_DOWN)
    }
}
#[doc = "Field `P0_01MODE` reader - Port 0 pin 1 control."]
pub use P0_00MODE_R as P0_01MODE_R;
#[doc = "Field `P0_02MODE` reader - Port 0 pin 2 control."]
pub use P0_00MODE_R as P0_02MODE_R;
#[doc = "Field `P0_03MODE` reader - Port 0 pin 3 control."]
pub use P0_00MODE_R as P0_03MODE_R;
#[doc = "Field `P0_04MODE` reader - Port 0 pin 4 control."]
pub use P0_00MODE_R as P0_04MODE_R;
#[doc = "Field `P0_05MODE` reader - Port 0 pin 5 control."]
pub use P0_00MODE_R as P0_05MODE_R;
#[doc = "Field `P0_06MODE` reader - Port 0 pin 6 control."]
pub use P0_00MODE_R as P0_06MODE_R;
#[doc = "Field `P0_07MODE` reader - Port 0 pin 7 control."]
pub use P0_00MODE_R as P0_07MODE_R;
#[doc = "Field `P0_08MODE` reader - Port 0 pin 8 control."]
pub use P0_00MODE_R as P0_08MODE_R;
#[doc = "Field `P0_09MODE` reader - Port 0 pin 9 control."]
pub use P0_00MODE_R as P0_09MODE_R;
#[doc = "Field `P0_10MODE` reader - Port 0 pin 10 control."]
pub use P0_00MODE_R as P0_10MODE_R;
#[doc = "Field `P0_11MODE` reader - Port 0 pin 11 control."]
pub use P0_00MODE_R as P0_11MODE_R;
#[doc = "Field `P0_15MODE` reader - Port 0 pin 15 control."]
pub use P0_00MODE_R as P0_15MODE_R;
#[doc = "Field `P0_01MODE` writer - Port 0 pin 1 control."]
pub use P0_00MODE_W as P0_01MODE_W;
#[doc = "Field `P0_02MODE` writer - Port 0 pin 2 control."]
pub use P0_00MODE_W as P0_02MODE_W;
#[doc = "Field `P0_03MODE` writer - Port 0 pin 3 control."]
pub use P0_00MODE_W as P0_03MODE_W;
#[doc = "Field `P0_04MODE` writer - Port 0 pin 4 control."]
pub use P0_00MODE_W as P0_04MODE_W;
#[doc = "Field `P0_05MODE` writer - Port 0 pin 5 control."]
pub use P0_00MODE_W as P0_05MODE_W;
#[doc = "Field `P0_06MODE` writer - Port 0 pin 6 control."]
pub use P0_00MODE_W as P0_06MODE_W;
#[doc = "Field `P0_07MODE` writer - Port 0 pin 7 control."]
pub use P0_00MODE_W as P0_07MODE_W;
#[doc = "Field `P0_08MODE` writer - Port 0 pin 8 control."]
pub use P0_00MODE_W as P0_08MODE_W;
#[doc = "Field `P0_09MODE` writer - Port 0 pin 9 control."]
pub use P0_00MODE_W as P0_09MODE_W;
#[doc = "Field `P0_10MODE` writer - Port 0 pin 10 control."]
pub use P0_00MODE_W as P0_10MODE_W;
#[doc = "Field `P0_11MODE` writer - Port 0 pin 11 control."]
pub use P0_00MODE_W as P0_11MODE_W;
#[doc = "Field `P0_15MODE` writer - Port 0 pin 15 control."]
pub use P0_00MODE_W as P0_15MODE_W;
impl R {
    #[doc = "Bits 0:1 - Port 0 pin 0 on-chip pull-up/down resistor control."]
    #[inline(always)]
    pub fn p0_00mode(&self) -> P0_00MODE_R {
        P0_00MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port 0 pin 1 control."]
    #[inline(always)]
    pub fn p0_01mode(&self) -> P0_01MODE_R {
        P0_01MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port 0 pin 2 control."]
    #[inline(always)]
    pub fn p0_02mode(&self) -> P0_02MODE_R {
        P0_02MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port 0 pin 3 control."]
    #[inline(always)]
    pub fn p0_03mode(&self) -> P0_03MODE_R {
        P0_03MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port 0 pin 4 control."]
    #[inline(always)]
    pub fn p0_04mode(&self) -> P0_04MODE_R {
        P0_04MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port 0 pin 5 control."]
    #[inline(always)]
    pub fn p0_05mode(&self) -> P0_05MODE_R {
        P0_05MODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port 0 pin 6 control."]
    #[inline(always)]
    pub fn p0_06mode(&self) -> P0_06MODE_R {
        P0_06MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port 0 pin 7 control."]
    #[inline(always)]
    pub fn p0_07mode(&self) -> P0_07MODE_R {
        P0_07MODE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port 0 pin 8 control."]
    #[inline(always)]
    pub fn p0_08mode(&self) -> P0_08MODE_R {
        P0_08MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port 0 pin 9 control."]
    #[inline(always)]
    pub fn p0_09mode(&self) -> P0_09MODE_R {
        P0_09MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port 0 pin 10 control."]
    #[inline(always)]
    pub fn p0_10mode(&self) -> P0_10MODE_R {
        P0_10MODE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port 0 pin 11 control."]
    #[inline(always)]
    pub fn p0_11mode(&self) -> P0_11MODE_R {
        P0_11MODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port 0 pin 15 control."]
    #[inline(always)]
    pub fn p0_15mode(&self) -> P0_15MODE_R {
        P0_15MODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PINMODE0")
            .field("p0_00mode", &format_args!("{}", self.p0_00mode().bits()))
            .field("p0_01mode", &format_args!("{}", self.p0_01mode().bits()))
            .field("p0_02mode", &format_args!("{}", self.p0_02mode().bits()))
            .field("p0_03mode", &format_args!("{}", self.p0_03mode().bits()))
            .field("p0_04mode", &format_args!("{}", self.p0_04mode().bits()))
            .field("p0_05mode", &format_args!("{}", self.p0_05mode().bits()))
            .field("p0_06mode", &format_args!("{}", self.p0_06mode().bits()))
            .field("p0_07mode", &format_args!("{}", self.p0_07mode().bits()))
            .field("p0_08mode", &format_args!("{}", self.p0_08mode().bits()))
            .field("p0_09mode", &format_args!("{}", self.p0_09mode().bits()))
            .field("p0_10mode", &format_args!("{}", self.p0_10mode().bits()))
            .field("p0_11mode", &format_args!("{}", self.p0_11mode().bits()))
            .field("p0_15mode", &format_args!("{}", self.p0_15mode().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PINMODE0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port 0 pin 0 on-chip pull-up/down resistor control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_00mode(&mut self) -> P0_00MODE_W<PINMODE0_SPEC, 0> {
        P0_00MODE_W::new(self)
    }
    #[doc = "Bits 2:3 - Port 0 pin 1 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_01mode(&mut self) -> P0_01MODE_W<PINMODE0_SPEC, 2> {
        P0_01MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - Port 0 pin 2 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_02mode(&mut self) -> P0_02MODE_W<PINMODE0_SPEC, 4> {
        P0_02MODE_W::new(self)
    }
    #[doc = "Bits 6:7 - Port 0 pin 3 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_03mode(&mut self) -> P0_03MODE_W<PINMODE0_SPEC, 6> {
        P0_03MODE_W::new(self)
    }
    #[doc = "Bits 8:9 - Port 0 pin 4 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_04mode(&mut self) -> P0_04MODE_W<PINMODE0_SPEC, 8> {
        P0_04MODE_W::new(self)
    }
    #[doc = "Bits 10:11 - Port 0 pin 5 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_05mode(&mut self) -> P0_05MODE_W<PINMODE0_SPEC, 10> {
        P0_05MODE_W::new(self)
    }
    #[doc = "Bits 12:13 - Port 0 pin 6 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_06mode(&mut self) -> P0_06MODE_W<PINMODE0_SPEC, 12> {
        P0_06MODE_W::new(self)
    }
    #[doc = "Bits 14:15 - Port 0 pin 7 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_07mode(&mut self) -> P0_07MODE_W<PINMODE0_SPEC, 14> {
        P0_07MODE_W::new(self)
    }
    #[doc = "Bits 16:17 - Port 0 pin 8 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_08mode(&mut self) -> P0_08MODE_W<PINMODE0_SPEC, 16> {
        P0_08MODE_W::new(self)
    }
    #[doc = "Bits 18:19 - Port 0 pin 9 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_09mode(&mut self) -> P0_09MODE_W<PINMODE0_SPEC, 18> {
        P0_09MODE_W::new(self)
    }
    #[doc = "Bits 20:21 - Port 0 pin 10 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_10mode(&mut self) -> P0_10MODE_W<PINMODE0_SPEC, 20> {
        P0_10MODE_W::new(self)
    }
    #[doc = "Bits 22:23 - Port 0 pin 11 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_11mode(&mut self) -> P0_11MODE_W<PINMODE0_SPEC, 22> {
        P0_11MODE_W::new(self)
    }
    #[doc = "Bits 30:31 - Port 0 pin 15 control."]
    #[inline(always)]
    #[must_use]
    pub fn p0_15mode(&mut self) -> P0_15MODE_W<PINMODE0_SPEC, 30> {
        P0_15MODE_W::new(self)
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
#[doc = "Pin mode select register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinmode0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinmode0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINMODE0_SPEC;
impl crate::RegisterSpec for PINMODE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinmode0::R`](R) reader structure"]
impl crate::Readable for PINMODE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pinmode0::W`](W) writer structure"]
impl crate::Writable for PINMODE0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINMODE0 to value 0"]
impl crate::Resettable for PINMODE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
