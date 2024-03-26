#[doc = "Register `CMR` writer"]
pub type W = crate::W<CMR_SPEC>;
#[doc = "Transmission Request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TR_AW {
    #[doc = "0: Absent.No transmission request."]
    ABSENT = 0,
    #[doc = "1: Present. The message, previously written to the CANxTFI, CANxTID, and optionally the CANxTDA and CANxTDB registers, is queued for transmission from the selected Transmit Buffer. If at two or all three of STB1, STB2 and STB3 bits are selected when TR=1 is written, Transmit Buffer will be selected based on the chosen priority scheme (for details see Section 21.5.3 Transmit Buffers (TXB))"]
    PRESENT = 1,
}
impl From<TR_AW> for bool {
    #[inline(always)]
    fn from(variant: TR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TR` writer - Transmission Request."]
pub type TR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TR_AW>;
impl<'a, REG, const O: u8> TR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Absent.No transmission request."]
    #[inline(always)]
    pub fn absent(self) -> &'a mut crate::W<REG> {
        self.variant(TR_AW::ABSENT)
    }
    #[doc = "Present. The message, previously written to the CANxTFI, CANxTID, and optionally the CANxTDA and CANxTDB registers, is queued for transmission from the selected Transmit Buffer. If at two or all three of STB1, STB2 and STB3 bits are selected when TR=1 is written, Transmit Buffer will be selected based on the chosen priority scheme (for details see Section 21.5.3 Transmit Buffers (TXB))"]
    #[inline(always)]
    pub fn present(self) -> &'a mut crate::W<REG> {
        self.variant(TR_AW::PRESENT)
    }
}
#[doc = "Abort Transmission.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AT_AW {
    #[doc = "0: No action. Do not abort the transmission."]
    NO_ACTION = 0,
    #[doc = "1: Present. if not already in progress, a pending Transmission Request for the selected Transmit Buffer is cancelled."]
    PRESENT = 1,
}
impl From<AT_AW> for bool {
    #[inline(always)]
    fn from(variant: AT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AT` writer - Abort Transmission."]
pub type AT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AT_AW>;
impl<'a, REG, const O: u8> AT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action. Do not abort the transmission."]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(AT_AW::NO_ACTION)
    }
    #[doc = "Present. if not already in progress, a pending Transmission Request for the selected Transmit Buffer is cancelled."]
    #[inline(always)]
    pub fn present(self) -> &'a mut crate::W<REG> {
        self.variant(AT_AW::PRESENT)
    }
}
#[doc = "Release Receive Buffer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRB_AW {
    #[doc = "0: No action. Do not release the receive buffer."]
    NO_ACTION = 0,
    #[doc = "1: Released. The information in the Receive Buffer (consisting of CANxRFS, CANxRID, and if applicable the CANxRDA and CANxRDB registers) is released, and becomes eligible for replacement by the next received frame. If the next received frame is not available, writing this command clears the RBS bit in the Status Register(s)."]
    RELEASED = 1,
}
impl From<RRB_AW> for bool {
    #[inline(always)]
    fn from(variant: RRB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRB` writer - Release Receive Buffer."]
pub type RRB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RRB_AW>;
impl<'a, REG, const O: u8> RRB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action. Do not release the receive buffer."]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(RRB_AW::NO_ACTION)
    }
    #[doc = "Released. The information in the Receive Buffer (consisting of CANxRFS, CANxRID, and if applicable the CANxRDA and CANxRDB registers) is released, and becomes eligible for replacement by the next received frame. If the next received frame is not available, writing this command clears the RBS bit in the Status Register(s)."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(RRB_AW::RELEASED)
    }
}
#[doc = "Clear Data Overrun.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDO_AW {
    #[doc = "0: No action. Do not clear the data overrun bit."]
    NO_ACTION = 0,
    #[doc = "1: Clear. The Data Overrun bit in Status Register(s) is cleared."]
    CLEAR = 1,
}
impl From<CDO_AW> for bool {
    #[inline(always)]
    fn from(variant: CDO_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDO` writer - Clear Data Overrun."]
pub type CDO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CDO_AW>;
impl<'a, REG, const O: u8> CDO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action. Do not clear the data overrun bit."]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(CDO_AW::NO_ACTION)
    }
    #[doc = "Clear. The Data Overrun bit in Status Register(s) is cleared."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CDO_AW::CLEAR)
    }
}
#[doc = "Self Reception Request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRR_AW {
    #[doc = "0: Absent. No self reception request."]
    ABSENT = 0,
    #[doc = "1: Present. The message, previously written to the CANxTFS, CANxTID, and optionally the CANxTDA and CANxTDB registers, is queued for transmission from the selected Transmit Buffer and received simultaneously. This differs from the TR bit above in that the receiver is not disabled during the transmission, so that it receives the message if its Identifier is recognized by the Acceptance Filter."]
    PRESENT = 1,
}
impl From<SRR_AW> for bool {
    #[inline(always)]
    fn from(variant: SRR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRR` writer - Self Reception Request."]
pub type SRR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SRR_AW>;
impl<'a, REG, const O: u8> SRR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Absent. No self reception request."]
    #[inline(always)]
    pub fn absent(self) -> &'a mut crate::W<REG> {
        self.variant(SRR_AW::ABSENT)
    }
    #[doc = "Present. The message, previously written to the CANxTFS, CANxTID, and optionally the CANxTDA and CANxTDB registers, is queued for transmission from the selected Transmit Buffer and received simultaneously. This differs from the TR bit above in that the receiver is not disabled during the transmission, so that it receives the message if its Identifier is recognized by the Acceptance Filter."]
    #[inline(always)]
    pub fn present(self) -> &'a mut crate::W<REG> {
        self.variant(SRR_AW::PRESENT)
    }
}
#[doc = "Select Tx Buffer 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STB1_AW {
    #[doc = "0: Not selected. Tx Buffer 1 is not selected for transmission."]
    NOT_SELECTED = 0,
    #[doc = "1: Selected. Tx Buffer 1 is selected for transmission."]
    SELECTED = 1,
}
impl From<STB1_AW> for bool {
    #[inline(always)]
    fn from(variant: STB1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STB1` writer - Select Tx Buffer 1."]
pub type STB1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STB1_AW>;
impl<'a, REG, const O: u8> STB1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected. Tx Buffer 1 is not selected for transmission."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(STB1_AW::NOT_SELECTED)
    }
    #[doc = "Selected. Tx Buffer 1 is selected for transmission."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(STB1_AW::SELECTED)
    }
}
#[doc = "Select Tx Buffer 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STB2_AW {
    #[doc = "0: Not selected. Tx Buffer 2 is not selected for transmission."]
    NOT_SELECTED = 0,
    #[doc = "1: Selected. Tx Buffer 2 is selected for transmission."]
    SELECTED = 1,
}
impl From<STB2_AW> for bool {
    #[inline(always)]
    fn from(variant: STB2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STB2` writer - Select Tx Buffer 2."]
pub type STB2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STB2_AW>;
impl<'a, REG, const O: u8> STB2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected. Tx Buffer 2 is not selected for transmission."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(STB2_AW::NOT_SELECTED)
    }
    #[doc = "Selected. Tx Buffer 2 is selected for transmission."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(STB2_AW::SELECTED)
    }
}
#[doc = "Select Tx Buffer 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STB3_AW {
    #[doc = "0: Not selected. Tx Buffer 3 is not selected for transmission."]
    NOT_SELECTED = 0,
    #[doc = "1: Selected. Tx Buffer 3 is selected for transmission."]
    SELECTED = 1,
}
impl From<STB3_AW> for bool {
    #[inline(always)]
    fn from(variant: STB3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STB3` writer - Select Tx Buffer 3."]
pub type STB3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, STB3_AW>;
impl<'a, REG, const O: u8> STB3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected. Tx Buffer 3 is not selected for transmission."]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(STB3_AW::NOT_SELECTED)
    }
    #[doc = "Selected. Tx Buffer 3 is selected for transmission."]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(STB3_AW::SELECTED)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CMR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Transmission Request."]
    #[inline(always)]
    #[must_use]
    pub fn tr(&mut self) -> TR_W<CMR_SPEC, 0> {
        TR_W::new(self)
    }
    #[doc = "Bit 1 - Abort Transmission."]
    #[inline(always)]
    #[must_use]
    pub fn at(&mut self) -> AT_W<CMR_SPEC, 1> {
        AT_W::new(self)
    }
    #[doc = "Bit 2 - Release Receive Buffer."]
    #[inline(always)]
    #[must_use]
    pub fn rrb(&mut self) -> RRB_W<CMR_SPEC, 2> {
        RRB_W::new(self)
    }
    #[doc = "Bit 3 - Clear Data Overrun."]
    #[inline(always)]
    #[must_use]
    pub fn cdo(&mut self) -> CDO_W<CMR_SPEC, 3> {
        CDO_W::new(self)
    }
    #[doc = "Bit 4 - Self Reception Request."]
    #[inline(always)]
    #[must_use]
    pub fn srr(&mut self) -> SRR_W<CMR_SPEC, 4> {
        SRR_W::new(self)
    }
    #[doc = "Bit 5 - Select Tx Buffer 1."]
    #[inline(always)]
    #[must_use]
    pub fn stb1(&mut self) -> STB1_W<CMR_SPEC, 5> {
        STB1_W::new(self)
    }
    #[doc = "Bit 6 - Select Tx Buffer 2."]
    #[inline(always)]
    #[must_use]
    pub fn stb2(&mut self) -> STB2_W<CMR_SPEC, 6> {
        STB2_W::new(self)
    }
    #[doc = "Bit 7 - Select Tx Buffer 3."]
    #[inline(always)]
    #[must_use]
    pub fn stb3(&mut self) -> STB3_W<CMR_SPEC, 7> {
        STB3_W::new(self)
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
#[doc = "Command bits that affect the state of the CAN Controller\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMR_SPEC;
impl crate::RegisterSpec for CMR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmr::W`](W) writer structure"]
impl crate::Writable for CMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMR to value 0"]
impl crate::Resettable for CMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
