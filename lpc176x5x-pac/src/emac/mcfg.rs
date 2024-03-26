#[doc = "Register `MCFG` reader"]
pub type R = crate::R<MCFG_SPEC>;
#[doc = "Register `MCFG` writer"]
pub type W = crate::W<MCFG_SPEC>;
#[doc = "Field `SCANINC` reader - SCAN INCREMENT. Set this bit to cause the MII Management hardware to perform read cycles across a range of PHYs. When set, the MII Management hardware will perform read cycles from address 1 through the value set in PHY ADDRESS\\[4:0\\]. Clear this bit to allow continuous reads of the same PHY."]
pub type SCANINC_R = crate::BitReader;
#[doc = "Field `SCANINC` writer - SCAN INCREMENT. Set this bit to cause the MII Management hardware to perform read cycles across a range of PHYs. When set, the MII Management hardware will perform read cycles from address 1 through the value set in PHY ADDRESS\\[4:0\\]. Clear this bit to allow continuous reads of the same PHY."]
pub type SCANINC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SUPPPREAMBLE` reader - SUPPRESS PREAMBLE. Set this bit to cause the MII Management hardware to perform read/write cycles without the 32-bit preamble field. Clear this bit to cause normal cycles to be performed. Some PHYs support suppressed preamble."]
pub type SUPPPREAMBLE_R = crate::BitReader;
#[doc = "Field `SUPPPREAMBLE` writer - SUPPRESS PREAMBLE. Set this bit to cause the MII Management hardware to perform read/write cycles without the 32-bit preamble field. Clear this bit to cause normal cycles to be performed. Some PHYs support suppressed preamble."]
pub type SUPPPREAMBLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLOCKSEL` reader - CLOCK SELECT. This field is used by the clock divide logic in creating the MII Management Clock (MDC) which IEEE 802.3u defines to be no faster than 2.5 MHz. Some PHYs support clock rates up to 12.5 MHz, however. The AHB bus clock (HCLK) is divided by the specified amount. Refer to Table 160 below for the definition of values for this field."]
pub type CLOCKSEL_R = crate::FieldReader;
#[doc = "Field `CLOCKSEL` writer - CLOCK SELECT. This field is used by the clock divide logic in creating the MII Management Clock (MDC) which IEEE 802.3u defines to be no faster than 2.5 MHz. Some PHYs support clock rates up to 12.5 MHz, however. The AHB bus clock (HCLK) is divided by the specified amount. Refer to Table 160 below for the definition of values for this field."]
pub type CLOCKSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `RESETMIIMGMT` reader - RESET MII MGMT. This bit resets the MII Management hardware."]
pub type RESETMIIMGMT_R = crate::BitReader;
#[doc = "Field `RESETMIIMGMT` writer - RESET MII MGMT. This bit resets the MII Management hardware."]
pub type RESETMIIMGMT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SCAN INCREMENT. Set this bit to cause the MII Management hardware to perform read cycles across a range of PHYs. When set, the MII Management hardware will perform read cycles from address 1 through the value set in PHY ADDRESS\\[4:0\\]. Clear this bit to allow continuous reads of the same PHY."]
    #[inline(always)]
    pub fn scaninc(&self) -> SCANINC_R {
        SCANINC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SUPPRESS PREAMBLE. Set this bit to cause the MII Management hardware to perform read/write cycles without the 32-bit preamble field. Clear this bit to cause normal cycles to be performed. Some PHYs support suppressed preamble."]
    #[inline(always)]
    pub fn supppreamble(&self) -> SUPPPREAMBLE_R {
        SUPPPREAMBLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - CLOCK SELECT. This field is used by the clock divide logic in creating the MII Management Clock (MDC) which IEEE 802.3u defines to be no faster than 2.5 MHz. Some PHYs support clock rates up to 12.5 MHz, however. The AHB bus clock (HCLK) is divided by the specified amount. Refer to Table 160 below for the definition of values for this field."]
    #[inline(always)]
    pub fn clocksel(&self) -> CLOCKSEL_R {
        CLOCKSEL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - RESET MII MGMT. This bit resets the MII Management hardware."]
    #[inline(always)]
    pub fn resetmiimgmt(&self) -> RESETMIIMGMT_R {
        RESETMIIMGMT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCFG")
            .field("scaninc", &format_args!("{}", self.scaninc().bit()))
            .field(
                "supppreamble",
                &format_args!("{}", self.supppreamble().bit()),
            )
            .field("clocksel", &format_args!("{}", self.clocksel().bits()))
            .field(
                "resetmiimgmt",
                &format_args!("{}", self.resetmiimgmt().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<MCFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - SCAN INCREMENT. Set this bit to cause the MII Management hardware to perform read cycles across a range of PHYs. When set, the MII Management hardware will perform read cycles from address 1 through the value set in PHY ADDRESS\\[4:0\\]. Clear this bit to allow continuous reads of the same PHY."]
    #[inline(always)]
    #[must_use]
    pub fn scaninc(&mut self) -> SCANINC_W<MCFG_SPEC, 0> {
        SCANINC_W::new(self)
    }
    #[doc = "Bit 1 - SUPPRESS PREAMBLE. Set this bit to cause the MII Management hardware to perform read/write cycles without the 32-bit preamble field. Clear this bit to cause normal cycles to be performed. Some PHYs support suppressed preamble."]
    #[inline(always)]
    #[must_use]
    pub fn supppreamble(&mut self) -> SUPPPREAMBLE_W<MCFG_SPEC, 1> {
        SUPPPREAMBLE_W::new(self)
    }
    #[doc = "Bits 2:5 - CLOCK SELECT. This field is used by the clock divide logic in creating the MII Management Clock (MDC) which IEEE 802.3u defines to be no faster than 2.5 MHz. Some PHYs support clock rates up to 12.5 MHz, however. The AHB bus clock (HCLK) is divided by the specified amount. Refer to Table 160 below for the definition of values for this field."]
    #[inline(always)]
    #[must_use]
    pub fn clocksel(&mut self) -> CLOCKSEL_W<MCFG_SPEC, 2> {
        CLOCKSEL_W::new(self)
    }
    #[doc = "Bit 15 - RESET MII MGMT. This bit resets the MII Management hardware."]
    #[inline(always)]
    #[must_use]
    pub fn resetmiimgmt(&mut self) -> RESETMIIMGMT_W<MCFG_SPEC, 15> {
        RESETMIIMGMT_W::new(self)
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
#[doc = "MII Mgmt Configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCFG_SPEC;
impl crate::RegisterSpec for MCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcfg::R`](R) reader structure"]
impl crate::Readable for MCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcfg::W`](W) writer structure"]
impl crate::Writable for MCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCFG to value 0"]
impl crate::Resettable for MCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
