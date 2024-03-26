#[doc = "Register `CLR2` writer"]
pub type W = crate::W<CLR2_SPEC>;
#[doc = "Field `P2_00CI` writer - Clear GPIO port Interrupts for P2\\[0\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_00CI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P2_01CI` writer - Clear GPIO port Interrupts for P2\\[1\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_01CI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P2_02CI` writer - Clear GPIO port Interrupts for P2\\[2\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_02CI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P2_03CI` writer - Clear GPIO port Interrupts for P2\\[3\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_03CI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P2_04CI` writer - Clear GPIO port Interrupts for P2\\[4\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_04CI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P2_05CI` writer - Clear GPIO port Interrupts for P2\\[5\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_05CI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P2_06CI` writer - Clear GPIO port Interrupts for P2\\[6\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_06CI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P2_07CI` writer - Clear GPIO port Interrupts for P2\\[7\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_07CI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P2_08CI` writer - Clear GPIO port Interrupts for P2\\[8\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_08CI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P2_09CI` writer - Clear GPIO port Interrupts for P2\\[9\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_09CI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P2_10CI` writer - Clear GPIO port Interrupts for P2\\[10\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_10CI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P2_11CI` writer - Clear GPIO port Interrupts for P2\\[11\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_11CI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P2_12CI` writer - Clear GPIO port Interrupts for P2\\[12\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_12CI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `P2_13CI` writer - Clear GPIO port Interrupts for P2\\[13\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
pub type P2_13CI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CLR2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clear GPIO port Interrupts for P2\\[0\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_00ci(&mut self) -> P2_00CI_W<CLR2_SPEC, 0> {
        P2_00CI_W::new(self)
    }
    #[doc = "Bit 1 - Clear GPIO port Interrupts for P2\\[1\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_01ci(&mut self) -> P2_01CI_W<CLR2_SPEC, 1> {
        P2_01CI_W::new(self)
    }
    #[doc = "Bit 2 - Clear GPIO port Interrupts for P2\\[2\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_02ci(&mut self) -> P2_02CI_W<CLR2_SPEC, 2> {
        P2_02CI_W::new(self)
    }
    #[doc = "Bit 3 - Clear GPIO port Interrupts for P2\\[3\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_03ci(&mut self) -> P2_03CI_W<CLR2_SPEC, 3> {
        P2_03CI_W::new(self)
    }
    #[doc = "Bit 4 - Clear GPIO port Interrupts for P2\\[4\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_04ci(&mut self) -> P2_04CI_W<CLR2_SPEC, 4> {
        P2_04CI_W::new(self)
    }
    #[doc = "Bit 5 - Clear GPIO port Interrupts for P2\\[5\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_05ci(&mut self) -> P2_05CI_W<CLR2_SPEC, 5> {
        P2_05CI_W::new(self)
    }
    #[doc = "Bit 6 - Clear GPIO port Interrupts for P2\\[6\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_06ci(&mut self) -> P2_06CI_W<CLR2_SPEC, 6> {
        P2_06CI_W::new(self)
    }
    #[doc = "Bit 7 - Clear GPIO port Interrupts for P2\\[7\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_07ci(&mut self) -> P2_07CI_W<CLR2_SPEC, 7> {
        P2_07CI_W::new(self)
    }
    #[doc = "Bit 8 - Clear GPIO port Interrupts for P2\\[8\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_08ci(&mut self) -> P2_08CI_W<CLR2_SPEC, 8> {
        P2_08CI_W::new(self)
    }
    #[doc = "Bit 9 - Clear GPIO port Interrupts for P2\\[9\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_09ci(&mut self) -> P2_09CI_W<CLR2_SPEC, 9> {
        P2_09CI_W::new(self)
    }
    #[doc = "Bit 10 - Clear GPIO port Interrupts for P2\\[10\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_10ci(&mut self) -> P2_10CI_W<CLR2_SPEC, 10> {
        P2_10CI_W::new(self)
    }
    #[doc = "Bit 11 - Clear GPIO port Interrupts for P2\\[11\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_11ci(&mut self) -> P2_11CI_W<CLR2_SPEC, 11> {
        P2_11CI_W::new(self)
    }
    #[doc = "Bit 12 - Clear GPIO port Interrupts for P2\\[12\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_12ci(&mut self) -> P2_12CI_W<CLR2_SPEC, 12> {
        P2_12CI_W::new(self)
    }
    #[doc = "Bit 13 - Clear GPIO port Interrupts for P2\\[13\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    #[must_use]
    pub fn p2_13ci(&mut self) -> P2_13CI_W<CLR2_SPEC, 13> {
        P2_13CI_W::new(self)
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
#[doc = "GPIO Interrupt Clear.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLR2_SPEC;
impl crate::RegisterSpec for CLR2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clr2::W`](W) writer structure"]
impl crate::Writable for CLR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLR2 to value 0"]
impl crate::Resettable for CLR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
