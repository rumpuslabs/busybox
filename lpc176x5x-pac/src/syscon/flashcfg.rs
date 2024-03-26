#[doc = "Register `FLASHCFG` reader"]
pub type R = crate::R<FLASHCFG_SPEC>;
#[doc = "Register `FLASHCFG` writer"]
pub type W = crate::W<FLASHCFG_SPEC>;
#[doc = "Field `FLASHTIM` reader - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. Other values are reserved."]
pub type FLASHTIM_R = crate::FieldReader<FLASHTIM_A>;
#[doc = "Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. Other values are reserved.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLASHTIM_A {
    #[doc = "0: Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock."]
    _1CLK = 0,
    #[doc = "1: Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock."]
    _2CLK = 1,
    #[doc = "2: Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock."]
    _3CLK = 2,
    #[doc = "3: Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock."]
    _4CLK = 3,
    #[doc = "4: Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock. Use for up to 120 Mhz for LPC1759 and LPC1769 only."]
    _5CLK = 4,
    #[doc = "5: Flash accesses use 6 CPU clocks. This safe setting will work under any conditions."]
    _6CLK = 5,
}
impl From<FLASHTIM_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASHTIM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FLASHTIM_A {
    type Ux = u8;
}
impl FLASHTIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FLASHTIM_A> {
        match self.bits {
            0 => Some(FLASHTIM_A::_1CLK),
            1 => Some(FLASHTIM_A::_2CLK),
            2 => Some(FLASHTIM_A::_3CLK),
            3 => Some(FLASHTIM_A::_4CLK),
            4 => Some(FLASHTIM_A::_5CLK),
            5 => Some(FLASHTIM_A::_6CLK),
            _ => None,
        }
    }
    #[doc = "Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock."]
    #[inline(always)]
    pub fn is_1clk(&self) -> bool {
        *self == FLASHTIM_A::_1CLK
    }
    #[doc = "Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock."]
    #[inline(always)]
    pub fn is_2clk(&self) -> bool {
        *self == FLASHTIM_A::_2CLK
    }
    #[doc = "Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock."]
    #[inline(always)]
    pub fn is_3clk(&self) -> bool {
        *self == FLASHTIM_A::_3CLK
    }
    #[doc = "Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock."]
    #[inline(always)]
    pub fn is_4clk(&self) -> bool {
        *self == FLASHTIM_A::_4CLK
    }
    #[doc = "Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock. Use for up to 120 Mhz for LPC1759 and LPC1769 only."]
    #[inline(always)]
    pub fn is_5clk(&self) -> bool {
        *self == FLASHTIM_A::_5CLK
    }
    #[doc = "Flash accesses use 6 CPU clocks. This safe setting will work under any conditions."]
    #[inline(always)]
    pub fn is_6clk(&self) -> bool {
        *self == FLASHTIM_A::_6CLK
    }
}
#[doc = "Field `FLASHTIM` writer - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. Other values are reserved."]
pub type FLASHTIM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, FLASHTIM_A>;
impl<'a, REG, const O: u8> FLASHTIM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock."]
    #[inline(always)]
    pub fn _1clk(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHTIM_A::_1CLK)
    }
    #[doc = "Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock."]
    #[inline(always)]
    pub fn _2clk(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHTIM_A::_2CLK)
    }
    #[doc = "Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock."]
    #[inline(always)]
    pub fn _3clk(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHTIM_A::_3CLK)
    }
    #[doc = "Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock."]
    #[inline(always)]
    pub fn _4clk(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHTIM_A::_4CLK)
    }
    #[doc = "Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock. Use for up to 120 Mhz for LPC1759 and LPC1769 only."]
    #[inline(always)]
    pub fn _5clk(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHTIM_A::_5CLK)
    }
    #[doc = "Flash accesses use 6 CPU clocks. This safe setting will work under any conditions."]
    #[inline(always)]
    pub fn _6clk(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHTIM_A::_6CLK)
    }
}
impl R {
    #[doc = "Bits 12:15 - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. Other values are reserved."]
    #[inline(always)]
    pub fn flashtim(&self) -> FLASHTIM_R {
        FLASHTIM_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASHCFG")
            .field("flashtim", &format_args!("{}", self.flashtim().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<FLASHCFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 12:15 - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. Other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn flashtim(&mut self) -> FLASHTIM_W<FLASHCFG_SPEC, 12> {
        FLASHTIM_W::new(self)
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
#[doc = "Flash Accelerator Configuration Register. Controls flash access timing.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flashcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flashcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASHCFG_SPEC;
impl crate::RegisterSpec for FLASHCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashcfg::R`](R) reader structure"]
impl crate::Readable for FLASHCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flashcfg::W`](W) writer structure"]
impl crate::Writable for FLASHCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASHCFG to value 0x303a"]
impl crate::Resettable for FLASHCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x303a;
}
