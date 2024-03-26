#[doc = "Register `CCONFIG%s` reader"]
pub type R = crate::R<CCONFIG_SPEC>;
#[doc = "Register `CCONFIG%s` writer"]
pub type W = crate::W<CCONFIG_SPEC>;
#[doc = "Field `E` reader - Channel enable. Reading this bit indicates whether a channel is currently enabled or disabled: 0 = channel disabled. 1 = channel enabled. The Channel Enable bit status can also be found by reading the DMACEnbldChns Register. A channel is enabled by setting this bit. A channel can be disabled by clearing the Enable bit. This causes the current AHB transfer (if one is in progress) to complete and the channel is then disabled. Any data in the FIFO of the relevant channel is lost. Restarting the channel by setting the Channel Enable bit has unpredictable effects, the channel must be fully re-initialized. The channel is also disabled, and Channel Enable bit cleared, when the last LLI is reached, the DMA transfer is completed, or if a channel error is encountered. If a channel must be disabled without losing data in the FIFO, the Halt bit must be set so that further DMA requests are ignored. The Active bit must then be polled until it reaches 0, indicating that there is no data left in the FIFO. Finally, the Channel Enable bit can be cleared."]
pub type E_R = crate::BitReader;
#[doc = "Field `E` writer - Channel enable. Reading this bit indicates whether a channel is currently enabled or disabled: 0 = channel disabled. 1 = channel enabled. The Channel Enable bit status can also be found by reading the DMACEnbldChns Register. A channel is enabled by setting this bit. A channel can be disabled by clearing the Enable bit. This causes the current AHB transfer (if one is in progress) to complete and the channel is then disabled. Any data in the FIFO of the relevant channel is lost. Restarting the channel by setting the Channel Enable bit has unpredictable effects, the channel must be fully re-initialized. The channel is also disabled, and Channel Enable bit cleared, when the last LLI is reached, the DMA transfer is completed, or if a channel error is encountered. If a channel must be disabled without losing data in the FIFO, the Halt bit must be set so that further DMA requests are ignored. The Active bit must then be polled until it reaches 0, indicating that there is no data left in the FIFO. Finally, the Channel Enable bit can be cleared."]
pub type E_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRCPERIPHERAL` reader - Source peripheral. This value selects the DMA source request peripheral. This field is ignored if the source of the transfer is from memory. See Table 672 for peripheral identification."]
pub type SRCPERIPHERAL_R = crate::FieldReader;
#[doc = "Field `SRCPERIPHERAL` writer - Source peripheral. This value selects the DMA source request peripheral. This field is ignored if the source of the transfer is from memory. See Table 672 for peripheral identification."]
pub type SRCPERIPHERAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `DESTPERIPHERAL` reader - Destination peripheral. This value selects the DMA destination request peripheral. This field is ignored if the destination of the transfer is to memory. See Table 672 for peripheral identification."]
pub type DESTPERIPHERAL_R = crate::FieldReader;
#[doc = "Field `DESTPERIPHERAL` writer - Destination peripheral. This value selects the DMA destination request peripheral. This field is ignored if the destination of the transfer is to memory. See Table 672 for peripheral identification."]
pub type DESTPERIPHERAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `TRANSFERTYPE` reader - This value indicates the type of transfer and specifies the flow controller. The transfer type can be memory-to-memory, memory-to-peripheral, peripheral-to-memory, or peripheral-to-peripheral. Flow can be controlled by the DMA controller, the source peripheral, or the destination peripheral. Refer to Table 694 for the encoding of this field."]
pub type TRANSFERTYPE_R = crate::FieldReader;
#[doc = "Field `TRANSFERTYPE` writer - This value indicates the type of transfer and specifies the flow controller. The transfer type can be memory-to-memory, memory-to-peripheral, peripheral-to-memory, or peripheral-to-peripheral. Flow can be controlled by the DMA controller, the source peripheral, or the destination peripheral. Refer to Table 694 for the encoding of this field."]
pub type TRANSFERTYPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `IE` reader - Interrupt error mask. When cleared, this bit masks out the error interrupt of the relevant channel."]
pub type IE_R = crate::BitReader;
#[doc = "Field `IE` writer - Interrupt error mask. When cleared, this bit masks out the error interrupt of the relevant channel."]
pub type IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ITC` reader - Terminal count interrupt mask. When cleared, this bit masks out the terminal count interrupt of the relevant channel."]
pub type ITC_R = crate::BitReader;
#[doc = "Field `ITC` writer - Terminal count interrupt mask. When cleared, this bit masks out the terminal count interrupt of the relevant channel."]
pub type ITC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `L` reader - Lock. When set, this bit enables locked transfers. This information is not used in the LPC178x/177x."]
pub type L_R = crate::BitReader;
#[doc = "Field `L` writer - Lock. When set, this bit enables locked transfers. This information is not used in the LPC178x/177x."]
pub type L_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `A` reader - Active: 0 = there is no data in the FIFO of the channel. 1 = the channel FIFO has data. This value can be used with the Halt and Channel Enable bits to cleanly disable a DMA channel. This is a read-only bit."]
pub type A_R = crate::BitReader;
#[doc = "Field `A` writer - Active: 0 = there is no data in the FIFO of the channel. 1 = the channel FIFO has data. This value can be used with the Halt and Channel Enable bits to cleanly disable a DMA channel. This is a read-only bit."]
pub type A_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `H` reader - Halt: 0 = enable DMA requests. 1 = ignore further source DMA requests. The contents of the channel FIFO are drained. This value can be used with the Active and Channel Enable bits to cleanly disable a DMA channel."]
pub type H_R = crate::BitReader;
#[doc = "Field `H` writer - Halt: 0 = enable DMA requests. 1 = ignore further source DMA requests. The contents of the channel FIFO are drained. This value can be used with the Active and Channel Enable bits to cleanly disable a DMA channel."]
pub type H_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Channel enable. Reading this bit indicates whether a channel is currently enabled or disabled: 0 = channel disabled. 1 = channel enabled. The Channel Enable bit status can also be found by reading the DMACEnbldChns Register. A channel is enabled by setting this bit. A channel can be disabled by clearing the Enable bit. This causes the current AHB transfer (if one is in progress) to complete and the channel is then disabled. Any data in the FIFO of the relevant channel is lost. Restarting the channel by setting the Channel Enable bit has unpredictable effects, the channel must be fully re-initialized. The channel is also disabled, and Channel Enable bit cleared, when the last LLI is reached, the DMA transfer is completed, or if a channel error is encountered. If a channel must be disabled without losing data in the FIFO, the Halt bit must be set so that further DMA requests are ignored. The Active bit must then be polled until it reaches 0, indicating that there is no data left in the FIFO. Finally, the Channel Enable bit can be cleared."]
    #[inline(always)]
    pub fn e(&self) -> E_R {
        E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Source peripheral. This value selects the DMA source request peripheral. This field is ignored if the source of the transfer is from memory. See Table 672 for peripheral identification."]
    #[inline(always)]
    pub fn srcperipheral(&self) -> SRCPERIPHERAL_R {
        SRCPERIPHERAL_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - Destination peripheral. This value selects the DMA destination request peripheral. This field is ignored if the destination of the transfer is to memory. See Table 672 for peripheral identification."]
    #[inline(always)]
    pub fn destperipheral(&self) -> DESTPERIPHERAL_R {
        DESTPERIPHERAL_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:13 - This value indicates the type of transfer and specifies the flow controller. The transfer type can be memory-to-memory, memory-to-peripheral, peripheral-to-memory, or peripheral-to-peripheral. Flow can be controlled by the DMA controller, the source peripheral, or the destination peripheral. Refer to Table 694 for the encoding of this field."]
    #[inline(always)]
    pub fn transfertype(&self) -> TRANSFERTYPE_R {
        TRANSFERTYPE_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - Interrupt error mask. When cleared, this bit masks out the error interrupt of the relevant channel."]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Terminal count interrupt mask. When cleared, this bit masks out the terminal count interrupt of the relevant channel."]
    #[inline(always)]
    pub fn itc(&self) -> ITC_R {
        ITC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Lock. When set, this bit enables locked transfers. This information is not used in the LPC178x/177x."]
    #[inline(always)]
    pub fn l(&self) -> L_R {
        L_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Active: 0 = there is no data in the FIFO of the channel. 1 = the channel FIFO has data. This value can be used with the Halt and Channel Enable bits to cleanly disable a DMA channel. This is a read-only bit."]
    #[inline(always)]
    pub fn a(&self) -> A_R {
        A_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Halt: 0 = enable DMA requests. 1 = ignore further source DMA requests. The contents of the channel FIFO are drained. This value can be used with the Active and Channel Enable bits to cleanly disable a DMA channel."]
    #[inline(always)]
    pub fn h(&self) -> H_R {
        H_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCONFIG")
            .field("e", &format_args!("{}", self.e().bit()))
            .field(
                "srcperipheral",
                &format_args!("{}", self.srcperipheral().bits()),
            )
            .field(
                "destperipheral",
                &format_args!("{}", self.destperipheral().bits()),
            )
            .field(
                "transfertype",
                &format_args!("{}", self.transfertype().bits()),
            )
            .field("ie", &format_args!("{}", self.ie().bit()))
            .field("itc", &format_args!("{}", self.itc().bit()))
            .field("l", &format_args!("{}", self.l().bit()))
            .field("a", &format_args!("{}", self.a().bit()))
            .field("h", &format_args!("{}", self.h().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CCONFIG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable. Reading this bit indicates whether a channel is currently enabled or disabled: 0 = channel disabled. 1 = channel enabled. The Channel Enable bit status can also be found by reading the DMACEnbldChns Register. A channel is enabled by setting this bit. A channel can be disabled by clearing the Enable bit. This causes the current AHB transfer (if one is in progress) to complete and the channel is then disabled. Any data in the FIFO of the relevant channel is lost. Restarting the channel by setting the Channel Enable bit has unpredictable effects, the channel must be fully re-initialized. The channel is also disabled, and Channel Enable bit cleared, when the last LLI is reached, the DMA transfer is completed, or if a channel error is encountered. If a channel must be disabled without losing data in the FIFO, the Halt bit must be set so that further DMA requests are ignored. The Active bit must then be polled until it reaches 0, indicating that there is no data left in the FIFO. Finally, the Channel Enable bit can be cleared."]
    #[inline(always)]
    #[must_use]
    pub fn e(&mut self) -> E_W<CCONFIG_SPEC, 0> {
        E_W::new(self)
    }
    #[doc = "Bits 1:5 - Source peripheral. This value selects the DMA source request peripheral. This field is ignored if the source of the transfer is from memory. See Table 672 for peripheral identification."]
    #[inline(always)]
    #[must_use]
    pub fn srcperipheral(&mut self) -> SRCPERIPHERAL_W<CCONFIG_SPEC, 1> {
        SRCPERIPHERAL_W::new(self)
    }
    #[doc = "Bits 6:10 - Destination peripheral. This value selects the DMA destination request peripheral. This field is ignored if the destination of the transfer is to memory. See Table 672 for peripheral identification."]
    #[inline(always)]
    #[must_use]
    pub fn destperipheral(&mut self) -> DESTPERIPHERAL_W<CCONFIG_SPEC, 6> {
        DESTPERIPHERAL_W::new(self)
    }
    #[doc = "Bits 11:13 - This value indicates the type of transfer and specifies the flow controller. The transfer type can be memory-to-memory, memory-to-peripheral, peripheral-to-memory, or peripheral-to-peripheral. Flow can be controlled by the DMA controller, the source peripheral, or the destination peripheral. Refer to Table 694 for the encoding of this field."]
    #[inline(always)]
    #[must_use]
    pub fn transfertype(&mut self) -> TRANSFERTYPE_W<CCONFIG_SPEC, 11> {
        TRANSFERTYPE_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt error mask. When cleared, this bit masks out the error interrupt of the relevant channel."]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<CCONFIG_SPEC, 14> {
        IE_W::new(self)
    }
    #[doc = "Bit 15 - Terminal count interrupt mask. When cleared, this bit masks out the terminal count interrupt of the relevant channel."]
    #[inline(always)]
    #[must_use]
    pub fn itc(&mut self) -> ITC_W<CCONFIG_SPEC, 15> {
        ITC_W::new(self)
    }
    #[doc = "Bit 16 - Lock. When set, this bit enables locked transfers. This information is not used in the LPC178x/177x."]
    #[inline(always)]
    #[must_use]
    pub fn l(&mut self) -> L_W<CCONFIG_SPEC, 16> {
        L_W::new(self)
    }
    #[doc = "Bit 17 - Active: 0 = there is no data in the FIFO of the channel. 1 = the channel FIFO has data. This value can be used with the Halt and Channel Enable bits to cleanly disable a DMA channel. This is a read-only bit."]
    #[inline(always)]
    #[must_use]
    pub fn a(&mut self) -> A_W<CCONFIG_SPEC, 17> {
        A_W::new(self)
    }
    #[doc = "Bit 18 - Halt: 0 = enable DMA requests. 1 = ignore further source DMA requests. The contents of the channel FIFO are drained. This value can be used with the Active and Channel Enable bits to cleanly disable a DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn h(&mut self) -> H_W<CCONFIG_SPEC, 18> {
        H_W::new(self)
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
#[doc = "DMA Channel 0 Configuration Register\\[1\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cconfig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cconfig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCONFIG_SPEC;
impl crate::RegisterSpec for CCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cconfig::R`](R) reader structure"]
impl crate::Readable for CCONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cconfig::W`](W) writer structure"]
impl crate::Writable for CCONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCONFIG%s to value 0"]
impl crate::Resettable for CCONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
