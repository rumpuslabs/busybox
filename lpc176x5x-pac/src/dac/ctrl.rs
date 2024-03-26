#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `INT_DMA_REQ` reader - DMA interrupt request"]
pub type INT_DMA_REQ_R = crate::BitReader<INT_DMA_REQ_A>;
#[doc = "DMA interrupt request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_DMA_REQ_A {
    #[doc = "0: Clear on any write to the DACR register."]
    NOT_PENDING = 0,
    #[doc = "1: Set by hardware when the timer times out."]
    PENDING = 1,
}
impl From<INT_DMA_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: INT_DMA_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_DMA_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT_DMA_REQ_A {
        match self.bits {
            false => INT_DMA_REQ_A::NOT_PENDING,
            true => INT_DMA_REQ_A::PENDING,
        }
    }
    #[doc = "Clear on any write to the DACR register."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == INT_DMA_REQ_A::NOT_PENDING
    }
    #[doc = "Set by hardware when the timer times out."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == INT_DMA_REQ_A::PENDING
    }
}
#[doc = "Field `INT_DMA_REQ` writer - DMA interrupt request"]
pub type INT_DMA_REQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INT_DMA_REQ_A>;
impl<'a, REG, const O: u8> INT_DMA_REQ_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear on any write to the DACR register."]
    #[inline(always)]
    pub fn not_pending(self) -> &'a mut crate::W<REG> {
        self.variant(INT_DMA_REQ_A::NOT_PENDING)
    }
    #[doc = "Set by hardware when the timer times out."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(INT_DMA_REQ_A::PENDING)
    }
}
#[doc = "Field `DBLBUF_ENA` reader - Double buffering"]
pub type DBLBUF_ENA_R = crate::BitReader<DBLBUF_ENA_A>;
#[doc = "Double buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBLBUF_ENA_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable. When this bit and the CNT_ENA bit are both set, the double-buffering feature in the DACR register will be enabled. Writes to the DACR register are written to a pre-buffer and then transferred to the DACR on the next time-out of the counter."]
    ENABLE = 1,
}
impl From<DBLBUF_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: DBLBUF_ENA_A) -> Self {
        variant as u8 != 0
    }
}
impl DBLBUF_ENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBLBUF_ENA_A {
        match self.bits {
            false => DBLBUF_ENA_A::DISABLE,
            true => DBLBUF_ENA_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DBLBUF_ENA_A::DISABLE
    }
    #[doc = "Enable. When this bit and the CNT_ENA bit are both set, the double-buffering feature in the DACR register will be enabled. Writes to the DACR register are written to a pre-buffer and then transferred to the DACR on the next time-out of the counter."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DBLBUF_ENA_A::ENABLE
    }
}
#[doc = "Field `DBLBUF_ENA` writer - Double buffering"]
pub type DBLBUF_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DBLBUF_ENA_A>;
impl<'a, REG, const O: u8> DBLBUF_ENA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DBLBUF_ENA_A::DISABLE)
    }
    #[doc = "Enable. When this bit and the CNT_ENA bit are both set, the double-buffering feature in the DACR register will be enabled. Writes to the DACR register are written to a pre-buffer and then transferred to the DACR on the next time-out of the counter."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DBLBUF_ENA_A::ENABLE)
    }
}
#[doc = "Field `CNT_ENA` reader - Time-out counter operation"]
pub type CNT_ENA_R = crate::BitReader<CNT_ENA_A>;
#[doc = "Time-out counter operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNT_ENA_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<CNT_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: CNT_ENA_A) -> Self {
        variant as u8 != 0
    }
}
impl CNT_ENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CNT_ENA_A {
        match self.bits {
            false => CNT_ENA_A::DISABLE,
            true => CNT_ENA_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CNT_ENA_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CNT_ENA_A::ENABLE
    }
}
#[doc = "Field `CNT_ENA` writer - Time-out counter operation"]
pub type CNT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CNT_ENA_A>;
impl<'a, REG, const O: u8> CNT_ENA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CNT_ENA_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CNT_ENA_A::ENABLE)
    }
}
#[doc = "Field `DMA_ENA` reader - DMA access"]
pub type DMA_ENA_R = crate::BitReader<DMA_ENA_A>;
#[doc = "DMA access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_ENA_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable. DMA Burst Request Input 7 is enabled for the DAC (see Table 672)."]
    ENABLE_DMA_BURST_RE = 1,
}
impl From<DMA_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_ENA_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_ENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA_ENA_A {
        match self.bits {
            false => DMA_ENA_A::DISABLE,
            true => DMA_ENA_A::ENABLE_DMA_BURST_RE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMA_ENA_A::DISABLE
    }
    #[doc = "Enable. DMA Burst Request Input 7 is enabled for the DAC (see Table 672)."]
    #[inline(always)]
    pub fn is_enable_dma_burst_re(&self) -> bool {
        *self == DMA_ENA_A::ENABLE_DMA_BURST_RE
    }
}
#[doc = "Field `DMA_ENA` writer - DMA access"]
pub type DMA_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DMA_ENA_A>;
impl<'a, REG, const O: u8> DMA_ENA_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_ENA_A::DISABLE)
    }
    #[doc = "Enable. DMA Burst Request Input 7 is enabled for the DAC (see Table 672)."]
    #[inline(always)]
    pub fn enable_dma_burst_re(self) -> &'a mut crate::W<REG> {
        self.variant(DMA_ENA_A::ENABLE_DMA_BURST_RE)
    }
}
impl R {
    #[doc = "Bit 0 - DMA interrupt request"]
    #[inline(always)]
    pub fn int_dma_req(&self) -> INT_DMA_REQ_R {
        INT_DMA_REQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Double buffering"]
    #[inline(always)]
    pub fn dblbuf_ena(&self) -> DBLBUF_ENA_R {
        DBLBUF_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Time-out counter operation"]
    #[inline(always)]
    pub fn cnt_ena(&self) -> CNT_ENA_R {
        CNT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA access"]
    #[inline(always)]
    pub fn dma_ena(&self) -> DMA_ENA_R {
        DMA_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("int_dma_req", &format_args!("{}", self.int_dma_req().bit()))
            .field("dblbuf_ena", &format_args!("{}", self.dblbuf_ena().bit()))
            .field("cnt_ena", &format_args!("{}", self.cnt_ena().bit()))
            .field("dma_ena", &format_args!("{}", self.dma_ena().bit()))
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
    #[doc = "Bit 0 - DMA interrupt request"]
    #[inline(always)]
    #[must_use]
    pub fn int_dma_req(&mut self) -> INT_DMA_REQ_W<CTRL_SPEC, 0> {
        INT_DMA_REQ_W::new(self)
    }
    #[doc = "Bit 1 - Double buffering"]
    #[inline(always)]
    #[must_use]
    pub fn dblbuf_ena(&mut self) -> DBLBUF_ENA_W<CTRL_SPEC, 1> {
        DBLBUF_ENA_W::new(self)
    }
    #[doc = "Bit 2 - Time-out counter operation"]
    #[inline(always)]
    #[must_use]
    pub fn cnt_ena(&mut self) -> CNT_ENA_W<CTRL_SPEC, 2> {
        CNT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - DMA access"]
    #[inline(always)]
    #[must_use]
    pub fn dma_ena(&mut self) -> DMA_ENA_W<CTRL_SPEC, 3> {
        DMA_ENA_W::new(self)
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
#[doc = "DAC Control register. This register controls DMA and timer operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
