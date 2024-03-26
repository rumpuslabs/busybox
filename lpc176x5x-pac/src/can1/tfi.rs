#[doc = "Register `TFI%s` reader"]
pub type R = crate::R<TFI_SPEC>;
#[doc = "Register `TFI%s` writer"]
pub type W = crate::W<TFI_SPEC>;
#[doc = "Field `PRIO` reader - If the TPM (Transmit Priority Mode) bit in the CANxMOD register is set to 1, enabled Tx Buffers contend for the right to send their messages based on this field. The buffer with the lowest TX Priority value wins the prioritization and is sent first."]
pub type PRIO_R = crate::FieldReader;
#[doc = "Field `PRIO` writer - If the TPM (Transmit Priority Mode) bit in the CANxMOD register is set to 1, enabled Tx Buffers contend for the right to send their messages based on this field. The buffer with the lowest TX Priority value wins the prioritization and is sent first."]
pub type PRIO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DLC` reader - Data Length Code. This value is sent in the DLC field of the next transmit message. In addition, if RTR = 0, this value controls the number of Data bytes sent in the next transmit message, from the CANxTDA and CANxTDB registers: 0000-0111 = 0-7 bytes 1xxx = 8 bytes"]
pub type DLC_R = crate::FieldReader;
#[doc = "Field `DLC` writer - Data Length Code. This value is sent in the DLC field of the next transmit message. In addition, if RTR = 0, this value controls the number of Data bytes sent in the next transmit message, from the CANxTDA and CANxTDB registers: 0000-0111 = 0-7 bytes 1xxx = 8 bytes"]
pub type DLC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `RTR` reader - This value is sent in the RTR bit of the next transmit message. If this bit is 0, the number of data bytes called out by the DLC field are sent from the CANxTDA and CANxTDB registers. If this bit is 1, a Remote Frame is sent, containing a request for that number of bytes."]
pub type RTR_R = crate::BitReader;
#[doc = "Field `RTR` writer - This value is sent in the RTR bit of the next transmit message. If this bit is 0, the number of data bytes called out by the DLC field are sent from the CANxTDA and CANxTDB registers. If this bit is 1, a Remote Frame is sent, containing a request for that number of bytes."]
pub type RTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FF` reader - If this bit is 0, the next transmit message will be sent with an 11-bit Identifier (standard frame format), while if it's 1, the message will be sent with a 29-bit Identifier (extended frame format)."]
pub type FF_R = crate::BitReader;
#[doc = "Field `FF` writer - If this bit is 0, the next transmit message will be sent with an 11-bit Identifier (standard frame format), while if it's 1, the message will be sent with a 29-bit Identifier (extended frame format)."]
pub type FF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7 - If the TPM (Transmit Priority Mode) bit in the CANxMOD register is set to 1, enabled Tx Buffers contend for the right to send their messages based on this field. The buffer with the lowest TX Priority value wins the prioritization and is sent first."]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Data Length Code. This value is sent in the DLC field of the next transmit message. In addition, if RTR = 0, this value controls the number of Data bytes sent in the next transmit message, from the CANxTDA and CANxTDB registers: 0000-0111 = 0-7 bytes 1xxx = 8 bytes"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - This value is sent in the RTR bit of the next transmit message. If this bit is 0, the number of data bytes called out by the DLC field are sent from the CANxTDA and CANxTDB registers. If this bit is 1, a Remote Frame is sent, containing a request for that number of bytes."]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - If this bit is 0, the next transmit message will be sent with an 11-bit Identifier (standard frame format), while if it's 1, the message will be sent with a 29-bit Identifier (extended frame format)."]
    #[inline(always)]
    pub fn ff(&self) -> FF_R {
        FF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TFI")
            .field("prio", &format_args!("{}", self.prio().bits()))
            .field("dlc", &format_args!("{}", self.dlc().bits()))
            .field("rtr", &format_args!("{}", self.rtr().bit()))
            .field("ff", &format_args!("{}", self.ff().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TFI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - If the TPM (Transmit Priority Mode) bit in the CANxMOD register is set to 1, enabled Tx Buffers contend for the right to send their messages based on this field. The buffer with the lowest TX Priority value wins the prioritization and is sent first."]
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PRIO_W<TFI_SPEC, 0> {
        PRIO_W::new(self)
    }
    #[doc = "Bits 16:19 - Data Length Code. This value is sent in the DLC field of the next transmit message. In addition, if RTR = 0, this value controls the number of Data bytes sent in the next transmit message, from the CANxTDA and CANxTDB registers: 0000-0111 = 0-7 bytes 1xxx = 8 bytes"]
    #[inline(always)]
    #[must_use]
    pub fn dlc(&mut self) -> DLC_W<TFI_SPEC, 16> {
        DLC_W::new(self)
    }
    #[doc = "Bit 30 - This value is sent in the RTR bit of the next transmit message. If this bit is 0, the number of data bytes called out by the DLC field are sent from the CANxTDA and CANxTDB registers. If this bit is 1, a Remote Frame is sent, containing a request for that number of bytes."]
    #[inline(always)]
    #[must_use]
    pub fn rtr(&mut self) -> RTR_W<TFI_SPEC, 30> {
        RTR_W::new(self)
    }
    #[doc = "Bit 31 - If this bit is 0, the next transmit message will be sent with an 11-bit Identifier (standard frame format), while if it's 1, the message will be sent with a 29-bit Identifier (extended frame format)."]
    #[inline(always)]
    #[must_use]
    pub fn ff(&mut self) -> FF_W<TFI_SPEC, 31> {
        FF_W::new(self)
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
#[doc = "Transmit frame info (Tx Buffer )\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tfi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TFI_SPEC;
impl crate::RegisterSpec for TFI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tfi::R`](R) reader structure"]
impl crate::Readable for TFI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tfi::W`](W) writer structure"]
impl crate::Writable for TFI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TFI%s to value 0"]
impl crate::Resettable for TFI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
