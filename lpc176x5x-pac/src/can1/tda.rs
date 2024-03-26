#[doc = "Register `TDA%s` reader"]
pub type R = crate::R<TDA_SPEC>;
#[doc = "Register `TDA%s` writer"]
pub type W = crate::W<TDA_SPEC>;
#[doc = "Field `DATA1` reader - Data 1. If RTR = 0 and DLC >= 0001 in the corresponding CANxTFI, this byte is sent as the first Data byte of the next transmit message."]
pub type DATA1_R = crate::FieldReader;
#[doc = "Field `DATA1` writer - Data 1. If RTR = 0 and DLC >= 0001 in the corresponding CANxTFI, this byte is sent as the first Data byte of the next transmit message."]
pub type DATA1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DATA2` reader - Data 2. If RTR = 0 and DLC >= 0010 in the corresponding CANxTFI, this byte is sent as the 2nd Data byte of the next transmit message."]
pub type DATA2_R = crate::FieldReader;
#[doc = "Field `DATA2` writer - Data 2. If RTR = 0 and DLC >= 0010 in the corresponding CANxTFI, this byte is sent as the 2nd Data byte of the next transmit message."]
pub type DATA2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DATA3` reader - Data 3. If RTR = 0 and DLC >= 0011 in the corresponding CANxTFI, this byte is sent as the 3rd Data byte of the next transmit message."]
pub type DATA3_R = crate::FieldReader;
#[doc = "Field `DATA3` writer - Data 3. If RTR = 0 and DLC >= 0011 in the corresponding CANxTFI, this byte is sent as the 3rd Data byte of the next transmit message."]
pub type DATA3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DATA4` reader - Data 4. If RTR = 0 and DLC >= 0100 in the corresponding CANxTFI, this byte is sent as the 4th Data byte of the next transmit message."]
pub type DATA4_R = crate::FieldReader;
#[doc = "Field `DATA4` writer - Data 4. If RTR = 0 and DLC >= 0100 in the corresponding CANxTFI, this byte is sent as the 4th Data byte of the next transmit message."]
pub type DATA4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data 1. If RTR = 0 and DLC >= 0001 in the corresponding CANxTFI, this byte is sent as the first Data byte of the next transmit message."]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data 2. If RTR = 0 and DLC >= 0010 in the corresponding CANxTFI, this byte is sent as the 2nd Data byte of the next transmit message."]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data 3. If RTR = 0 and DLC >= 0011 in the corresponding CANxTFI, this byte is sent as the 3rd Data byte of the next transmit message."]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data 4. If RTR = 0 and DLC >= 0100 in the corresponding CANxTFI, this byte is sent as the 4th Data byte of the next transmit message."]
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TDA")
            .field("data1", &format_args!("{}", self.data1().bits()))
            .field("data2", &format_args!("{}", self.data2().bits()))
            .field("data3", &format_args!("{}", self.data3().bits()))
            .field("data4", &format_args!("{}", self.data4().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TDA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 1. If RTR = 0 and DLC >= 0001 in the corresponding CANxTFI, this byte is sent as the first Data byte of the next transmit message."]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> DATA1_W<TDA_SPEC, 0> {
        DATA1_W::new(self)
    }
    #[doc = "Bits 8:15 - Data 2. If RTR = 0 and DLC >= 0010 in the corresponding CANxTFI, this byte is sent as the 2nd Data byte of the next transmit message."]
    #[inline(always)]
    #[must_use]
    pub fn data2(&mut self) -> DATA2_W<TDA_SPEC, 8> {
        DATA2_W::new(self)
    }
    #[doc = "Bits 16:23 - Data 3. If RTR = 0 and DLC >= 0011 in the corresponding CANxTFI, this byte is sent as the 3rd Data byte of the next transmit message."]
    #[inline(always)]
    #[must_use]
    pub fn data3(&mut self) -> DATA3_W<TDA_SPEC, 16> {
        DATA3_W::new(self)
    }
    #[doc = "Bits 24:31 - Data 4. If RTR = 0 and DLC >= 0100 in the corresponding CANxTFI, this byte is sent as the 4th Data byte of the next transmit message."]
    #[inline(always)]
    #[must_use]
    pub fn data4(&mut self) -> DATA4_W<TDA_SPEC, 24> {
        DATA4_W::new(self)
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
#[doc = "Transmit data bytes 1-4 (Tx Buffer)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tda::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tda::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TDA_SPEC;
impl crate::RegisterSpec for TDA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tda::R`](R) reader structure"]
impl crate::Readable for TDA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tda::W`](W) writer structure"]
impl crate::Writable for TDA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TDA%s to value 0"]
impl crate::Resettable for TDA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
