#[doc = "Register `DR` reader"]
pub type R = crate::R<DR_SPEC>;
#[doc = "Register `DR` writer"]
pub type W = crate::W<DR_SPEC>;
#[doc = "Field `DATALOW` reader - SPI Bi-directional data port."]
pub type DATALOW_R = crate::FieldReader;
#[doc = "Field `DATALOW` writer - SPI Bi-directional data port."]
pub type DATALOW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DATAHIGH` reader - If bit 2 of the SPCR is 1 and bits 11:8 are other than 1000, some or all of these bits contain the additional transmit and receive bits. When less than 16 bits are selected, the more significant among these bits read as zeroes."]
pub type DATAHIGH_R = crate::FieldReader;
#[doc = "Field `DATAHIGH` writer - If bit 2 of the SPCR is 1 and bits 11:8 are other than 1000, some or all of these bits contain the additional transmit and receive bits. When less than 16 bits are selected, the more significant among these bits read as zeroes."]
pub type DATAHIGH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - SPI Bi-directional data port."]
    #[inline(always)]
    pub fn datalow(&self) -> DATALOW_R {
        DATALOW_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - If bit 2 of the SPCR is 1 and bits 11:8 are other than 1000, some or all of these bits contain the additional transmit and receive bits. When less than 16 bits are selected, the more significant among these bits read as zeroes."]
    #[inline(always)]
    pub fn datahigh(&self) -> DATAHIGH_R {
        DATAHIGH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI Bi-directional data port."]
    #[inline(always)]
    #[must_use]
    pub fn datalow(&mut self) -> DATALOW_W<DR_SPEC, 0> {
        DATALOW_W::new(self)
    }
    #[doc = "Bits 8:15 - If bit 2 of the SPCR is 1 and bits 11:8 are other than 1000, some or all of these bits contain the additional transmit and receive bits. When less than 16 bits are selected, the more significant among these bits read as zeroes."]
    #[inline(always)]
    #[must_use]
    pub fn datahigh(&mut self) -> DATAHIGH_W<DR_SPEC, 8> {
        DATAHIGH_W::new(self)
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
#[doc = "SPI Data Register. This bi-directional register provides the transmit and receive data for the SPI. Transmit data is provided to the SPI0 by writing to this register. Data received by the SPI0 can be read from this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`](R). WARN: The register is **modified** in some way after a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dr::W`](W) writer structure"]
impl crate::Writable for DR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
