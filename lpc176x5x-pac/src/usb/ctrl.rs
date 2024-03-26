#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `RD_EN` reader - Read mode control. Enables reading data from the OUT endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBRxData register. This bit is cleared by hardware when the last word of the current packet is read from USBRxData."]
pub type RD_EN_R = crate::BitReader<RD_EN_A>;
#[doc = "Read mode control. Enables reading data from the OUT endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBRxData register. This bit is cleared by hardware when the last word of the current packet is read from USBRxData.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RD_EN_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<RD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RD_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RD_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RD_EN_A {
        match self.bits {
            false => RD_EN_A::DISABLED,
            true => RD_EN_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RD_EN_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RD_EN_A::ENABLED
    }
}
#[doc = "Field `RD_EN` writer - Read mode control. Enables reading data from the OUT endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBRxData register. This bit is cleared by hardware when the last word of the current packet is read from USBRxData."]
pub type RD_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RD_EN_A>;
impl<'a, REG, const O: u8> RD_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RD_EN_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RD_EN_A::ENABLED)
    }
}
#[doc = "Field `WR_EN` reader - Write mode control. Enables writing data to the IN endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBTxData register. This bit is cleared by hardware when the number of bytes in USBTxLen have been sent."]
pub type WR_EN_R = crate::BitReader<WR_EN_A>;
#[doc = "Write mode control. Enables writing data to the IN endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBTxData register. This bit is cleared by hardware when the number of bytes in USBTxLen have been sent.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WR_EN_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<WR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WR_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl WR_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WR_EN_A {
        match self.bits {
            false => WR_EN_A::DISABLED,
            true => WR_EN_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WR_EN_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WR_EN_A::ENABLED
    }
}
#[doc = "Field `WR_EN` writer - Write mode control. Enables writing data to the IN endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBTxData register. This bit is cleared by hardware when the number of bytes in USBTxLen have been sent."]
pub type WR_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WR_EN_A>;
impl<'a, REG, const O: u8> WR_EN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WR_EN_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WR_EN_A::ENABLED)
    }
}
#[doc = "Field `LOG_ENDPOINT` reader - Logical Endpoint number."]
pub type LOG_ENDPOINT_R = crate::FieldReader;
#[doc = "Field `LOG_ENDPOINT` writer - Logical Endpoint number."]
pub type LOG_ENDPOINT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - Read mode control. Enables reading data from the OUT endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBRxData register. This bit is cleared by hardware when the last word of the current packet is read from USBRxData."]
    #[inline(always)]
    pub fn rd_en(&self) -> RD_EN_R {
        RD_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write mode control. Enables writing data to the IN endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBTxData register. This bit is cleared by hardware when the number of bytes in USBTxLen have been sent."]
    #[inline(always)]
    pub fn wr_en(&self) -> WR_EN_R {
        WR_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - Logical Endpoint number."]
    #[inline(always)]
    pub fn log_endpoint(&self) -> LOG_ENDPOINT_R {
        LOG_ENDPOINT_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("rd_en", &format_args!("{}", self.rd_en().bit()))
            .field("wr_en", &format_args!("{}", self.wr_en().bit()))
            .field(
                "log_endpoint",
                &format_args!("{}", self.log_endpoint().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Read mode control. Enables reading data from the OUT endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBRxData register. This bit is cleared by hardware when the last word of the current packet is read from USBRxData."]
    #[inline(always)]
    #[must_use]
    pub fn rd_en(&mut self) -> RD_EN_W<CTRL_SPEC, 0> {
        RD_EN_W::new(self)
    }
    #[doc = "Bit 1 - Write mode control. Enables writing data to the IN endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBTxData register. This bit is cleared by hardware when the number of bytes in USBTxLen have been sent."]
    #[inline(always)]
    #[must_use]
    pub fn wr_en(&mut self) -> WR_EN_W<CTRL_SPEC, 1> {
        WR_EN_W::new(self)
    }
    #[doc = "Bits 2:5 - Logical Endpoint number."]
    #[inline(always)]
    #[must_use]
    pub fn log_endpoint(&mut self) -> LOG_ENDPOINT_W<CTRL_SPEC, 2> {
        LOG_ENDPOINT_W::new(self)
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
#[doc = "USB Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
