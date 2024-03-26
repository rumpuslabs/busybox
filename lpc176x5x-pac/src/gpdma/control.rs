#[doc = "Register `CONTROL%s` reader"]
pub type R = crate::R<CONTROL_SPEC>;
#[doc = "Register `CONTROL%s` writer"]
pub type W = crate::W<CONTROL_SPEC>;
#[doc = "Field `TRANSFERSIZE` reader - Transfer size. This field sets the size of the transfer when the DMA controller is the flow controller, in which case the value must be set before the channel is enabled. Transfer size is updated as data transfers are completed. A read from this field indicates the number of transfers completed on the destination bus. Reading the register when the channel is active does not give useful information because by the time that the software has processed the value read, the channel might have progressed. It is intended to be used only when a channel is enabled and then disabled. The transfer size value is not used if a peripheral is the flow controller."]
pub type TRANSFERSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `TRANSFERSIZE` writer - Transfer size. This field sets the size of the transfer when the DMA controller is the flow controller, in which case the value must be set before the channel is enabled. Transfer size is updated as data transfers are completed. A read from this field indicates the number of transfers completed on the destination bus. Reading the register when the channel is active does not give useful information because by the time that the software has processed the value read, the channel might have progressed. It is intended to be used only when a channel is enabled and then disabled. The transfer size value is not used if a peripheral is the flow controller."]
pub type TRANSFERSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `SBSIZE` reader - Source burst size. Indicates the number of transfers that make up a source burst. This value must be set to the burst size of the source peripheral, or if the source is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACBREQ signal goes active in the source peripheral. 000 - 1 001 - 4 010 - 8 011 - 16 100 - 32 101 - 64 110 - 128 111 - 256"]
pub type SBSIZE_R = crate::FieldReader;
#[doc = "Field `SBSIZE` writer - Source burst size. Indicates the number of transfers that make up a source burst. This value must be set to the burst size of the source peripheral, or if the source is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACBREQ signal goes active in the source peripheral. 000 - 1 001 - 4 010 - 8 011 - 16 100 - 32 101 - 64 110 - 128 111 - 256"]
pub type SBSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DBSIZE` reader - Destination burst size. Indicates the number of transfers that make up a destination burst transfer request. This value must be set to the burst size of the destination peripheral or, if the destination is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACBREQ signal goes active in the destination peripheral. 000 - 1 001 - 4 010 - 8 011 - 16 100 - 32 101 - 64 110 - 128 111 - 256"]
pub type DBSIZE_R = crate::FieldReader;
#[doc = "Field `DBSIZE` writer - Destination burst size. Indicates the number of transfers that make up a destination burst transfer request. This value must be set to the burst size of the destination peripheral or, if the destination is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACBREQ signal goes active in the destination peripheral. 000 - 1 001 - 4 010 - 8 011 - 16 100 - 32 101 - 64 110 - 128 111 - 256"]
pub type DBSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SWIDTH` reader - Source transfer width. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data as required. 000 - Byte (8-bit) 001 - Halfword (16-bit) 010 - Word (32-bit) 011 to 111 - Reserved"]
pub type SWIDTH_R = crate::FieldReader;
#[doc = "Field `SWIDTH` writer - Source transfer width. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data as required. 000 - Byte (8-bit) 001 - Halfword (16-bit) 010 - Word (32-bit) 011 to 111 - Reserved"]
pub type SWIDTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DWIDTH` reader - Destination transfer width. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data as required. 000 - Byte (8-bit) 001 - Halfword (16-bit) 010 - Word (32-bit) 011 to 111 - Reserved"]
pub type DWIDTH_R = crate::FieldReader;
#[doc = "Field `DWIDTH` writer - Destination transfer width. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data as required. 000 - Byte (8-bit) 001 - Halfword (16-bit) 010 - Word (32-bit) 011 to 111 - Reserved"]
pub type DWIDTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SI` reader - Source increment: 0 - the source address is not incremented after each transfer. 1 - the source address is incremented after each transfer."]
pub type SI_R = crate::BitReader;
#[doc = "Field `SI` writer - Source increment: 0 - the source address is not incremented after each transfer. 1 - the source address is incremented after each transfer."]
pub type SI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DI` reader - Destination increment: 0 - the destination address is not incremented after each transfer. 1 - the destination address is incremented after each transfer."]
pub type DI_R = crate::BitReader;
#[doc = "Field `DI` writer - Destination increment: 0 - the destination address is not incremented after each transfer. 1 - the destination address is incremented after each transfer."]
pub type DI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PROT1` reader - This is provided to the peripheral during a DMA bus access and indicates that the access is in user mode or privileged mode. This information is not used in the LPC178x/177x. 0 - access is in user mode. 1 - access is in privileged mode."]
pub type PROT1_R = crate::BitReader;
#[doc = "Field `PROT1` writer - This is provided to the peripheral during a DMA bus access and indicates that the access is in user mode or privileged mode. This information is not used in the LPC178x/177x. 0 - access is in user mode. 1 - access is in privileged mode."]
pub type PROT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PROT2` reader - This is provided to the peripheral during a DMA bus access and indicates to the peripheral that the access is bufferable or not bufferable. This information is not used in the LPC178x/177x. 0 - access is not bufferable. 1 - access is bufferable."]
pub type PROT2_R = crate::BitReader;
#[doc = "Field `PROT2` writer - This is provided to the peripheral during a DMA bus access and indicates to the peripheral that the access is bufferable or not bufferable. This information is not used in the LPC178x/177x. 0 - access is not bufferable. 1 - access is bufferable."]
pub type PROT2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PROT3` reader - This is provided to the peripheral during a DMA bus access and indicates to the peripheral that the access is cacheable or not cacheable. This information is not used in the LPC178x/177x. 0 - access is not cacheable. 1 - access is cacheable."]
pub type PROT3_R = crate::BitReader;
#[doc = "Field `PROT3` writer - This is provided to the peripheral during a DMA bus access and indicates to the peripheral that the access is cacheable or not cacheable. This information is not used in the LPC178x/177x. 0 - access is not cacheable. 1 - access is cacheable."]
pub type PROT3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I` reader - Terminal count interrupt enable bit. 0 - the terminal count interrupt is disabled. 1 - the terminal count interrupt is enabled."]
pub type I_R = crate::BitReader;
#[doc = "Field `I` writer - Terminal count interrupt enable bit. 0 - the terminal count interrupt is disabled. 1 - the terminal count interrupt is enabled."]
pub type I_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:11 - Transfer size. This field sets the size of the transfer when the DMA controller is the flow controller, in which case the value must be set before the channel is enabled. Transfer size is updated as data transfers are completed. A read from this field indicates the number of transfers completed on the destination bus. Reading the register when the channel is active does not give useful information because by the time that the software has processed the value read, the channel might have progressed. It is intended to be used only when a channel is enabled and then disabled. The transfer size value is not used if a peripheral is the flow controller."]
    #[inline(always)]
    pub fn transfersize(&self) -> TRANSFERSIZE_R {
        TRANSFERSIZE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14 - Source burst size. Indicates the number of transfers that make up a source burst. This value must be set to the burst size of the source peripheral, or if the source is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACBREQ signal goes active in the source peripheral. 000 - 1 001 - 4 010 - 8 011 - 16 100 - 32 101 - 64 110 - 128 111 - 256"]
    #[inline(always)]
    pub fn sbsize(&self) -> SBSIZE_R {
        SBSIZE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Destination burst size. Indicates the number of transfers that make up a destination burst transfer request. This value must be set to the burst size of the destination peripheral or, if the destination is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACBREQ signal goes active in the destination peripheral. 000 - 1 001 - 4 010 - 8 011 - 16 100 - 32 101 - 64 110 - 128 111 - 256"]
    #[inline(always)]
    pub fn dbsize(&self) -> DBSIZE_R {
        DBSIZE_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Source transfer width. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data as required. 000 - Byte (8-bit) 001 - Halfword (16-bit) 010 - Word (32-bit) 011 to 111 - Reserved"]
    #[inline(always)]
    pub fn swidth(&self) -> SWIDTH_R {
        SWIDTH_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Destination transfer width. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data as required. 000 - Byte (8-bit) 001 - Halfword (16-bit) 010 - Word (32-bit) 011 to 111 - Reserved"]
    #[inline(always)]
    pub fn dwidth(&self) -> DWIDTH_R {
        DWIDTH_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 26 - Source increment: 0 - the source address is not incremented after each transfer. 1 - the source address is incremented after each transfer."]
    #[inline(always)]
    pub fn si(&self) -> SI_R {
        SI_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Destination increment: 0 - the destination address is not incremented after each transfer. 1 - the destination address is incremented after each transfer."]
    #[inline(always)]
    pub fn di(&self) -> DI_R {
        DI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - This is provided to the peripheral during a DMA bus access and indicates that the access is in user mode or privileged mode. This information is not used in the LPC178x/177x. 0 - access is in user mode. 1 - access is in privileged mode."]
    #[inline(always)]
    pub fn prot1(&self) -> PROT1_R {
        PROT1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This is provided to the peripheral during a DMA bus access and indicates to the peripheral that the access is bufferable or not bufferable. This information is not used in the LPC178x/177x. 0 - access is not bufferable. 1 - access is bufferable."]
    #[inline(always)]
    pub fn prot2(&self) -> PROT2_R {
        PROT2_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This is provided to the peripheral during a DMA bus access and indicates to the peripheral that the access is cacheable or not cacheable. This information is not used in the LPC178x/177x. 0 - access is not cacheable. 1 - access is cacheable."]
    #[inline(always)]
    pub fn prot3(&self) -> PROT3_R {
        PROT3_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Terminal count interrupt enable bit. 0 - the terminal count interrupt is disabled. 1 - the terminal count interrupt is enabled."]
    #[inline(always)]
    pub fn i(&self) -> I_R {
        I_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONTROL")
            .field(
                "transfersize",
                &format_args!("{}", self.transfersize().bits()),
            )
            .field("sbsize", &format_args!("{}", self.sbsize().bits()))
            .field("dbsize", &format_args!("{}", self.dbsize().bits()))
            .field("swidth", &format_args!("{}", self.swidth().bits()))
            .field("dwidth", &format_args!("{}", self.dwidth().bits()))
            .field("si", &format_args!("{}", self.si().bit()))
            .field("di", &format_args!("{}", self.di().bit()))
            .field("prot1", &format_args!("{}", self.prot1().bit()))
            .field("prot2", &format_args!("{}", self.prot2().bit()))
            .field("prot3", &format_args!("{}", self.prot3().bit()))
            .field("i", &format_args!("{}", self.i().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CONTROL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:11 - Transfer size. This field sets the size of the transfer when the DMA controller is the flow controller, in which case the value must be set before the channel is enabled. Transfer size is updated as data transfers are completed. A read from this field indicates the number of transfers completed on the destination bus. Reading the register when the channel is active does not give useful information because by the time that the software has processed the value read, the channel might have progressed. It is intended to be used only when a channel is enabled and then disabled. The transfer size value is not used if a peripheral is the flow controller."]
    #[inline(always)]
    #[must_use]
    pub fn transfersize(&mut self) -> TRANSFERSIZE_W<CONTROL_SPEC, 0> {
        TRANSFERSIZE_W::new(self)
    }
    #[doc = "Bits 12:14 - Source burst size. Indicates the number of transfers that make up a source burst. This value must be set to the burst size of the source peripheral, or if the source is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACBREQ signal goes active in the source peripheral. 000 - 1 001 - 4 010 - 8 011 - 16 100 - 32 101 - 64 110 - 128 111 - 256"]
    #[inline(always)]
    #[must_use]
    pub fn sbsize(&mut self) -> SBSIZE_W<CONTROL_SPEC, 12> {
        SBSIZE_W::new(self)
    }
    #[doc = "Bits 15:17 - Destination burst size. Indicates the number of transfers that make up a destination burst transfer request. This value must be set to the burst size of the destination peripheral or, if the destination is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACBREQ signal goes active in the destination peripheral. 000 - 1 001 - 4 010 - 8 011 - 16 100 - 32 101 - 64 110 - 128 111 - 256"]
    #[inline(always)]
    #[must_use]
    pub fn dbsize(&mut self) -> DBSIZE_W<CONTROL_SPEC, 15> {
        DBSIZE_W::new(self)
    }
    #[doc = "Bits 18:20 - Source transfer width. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data as required. 000 - Byte (8-bit) 001 - Halfword (16-bit) 010 - Word (32-bit) 011 to 111 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn swidth(&mut self) -> SWIDTH_W<CONTROL_SPEC, 18> {
        SWIDTH_W::new(self)
    }
    #[doc = "Bits 21:23 - Destination transfer width. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data as required. 000 - Byte (8-bit) 001 - Halfword (16-bit) 010 - Word (32-bit) 011 to 111 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn dwidth(&mut self) -> DWIDTH_W<CONTROL_SPEC, 21> {
        DWIDTH_W::new(self)
    }
    #[doc = "Bit 26 - Source increment: 0 - the source address is not incremented after each transfer. 1 - the source address is incremented after each transfer."]
    #[inline(always)]
    #[must_use]
    pub fn si(&mut self) -> SI_W<CONTROL_SPEC, 26> {
        SI_W::new(self)
    }
    #[doc = "Bit 27 - Destination increment: 0 - the destination address is not incremented after each transfer. 1 - the destination address is incremented after each transfer."]
    #[inline(always)]
    #[must_use]
    pub fn di(&mut self) -> DI_W<CONTROL_SPEC, 27> {
        DI_W::new(self)
    }
    #[doc = "Bit 28 - This is provided to the peripheral during a DMA bus access and indicates that the access is in user mode or privileged mode. This information is not used in the LPC178x/177x. 0 - access is in user mode. 1 - access is in privileged mode."]
    #[inline(always)]
    #[must_use]
    pub fn prot1(&mut self) -> PROT1_W<CONTROL_SPEC, 28> {
        PROT1_W::new(self)
    }
    #[doc = "Bit 29 - This is provided to the peripheral during a DMA bus access and indicates to the peripheral that the access is bufferable or not bufferable. This information is not used in the LPC178x/177x. 0 - access is not bufferable. 1 - access is bufferable."]
    #[inline(always)]
    #[must_use]
    pub fn prot2(&mut self) -> PROT2_W<CONTROL_SPEC, 29> {
        PROT2_W::new(self)
    }
    #[doc = "Bit 30 - This is provided to the peripheral during a DMA bus access and indicates to the peripheral that the access is cacheable or not cacheable. This information is not used in the LPC178x/177x. 0 - access is not cacheable. 1 - access is cacheable."]
    #[inline(always)]
    #[must_use]
    pub fn prot3(&mut self) -> PROT3_W<CONTROL_SPEC, 30> {
        PROT3_W::new(self)
    }
    #[doc = "Bit 31 - Terminal count interrupt enable bit. 0 - the terminal count interrupt is disabled. 1 - the terminal count interrupt is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn i(&mut self) -> I_W<CONTROL_SPEC, 31> {
        I_W::new(self)
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
#[doc = "DMA Channel 0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONTROL%s to value 0"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
