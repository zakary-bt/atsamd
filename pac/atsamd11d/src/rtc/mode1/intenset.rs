#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP0` reader - Compare 0 Interrupt Enable"]
pub struct CMP0_R(crate::FieldReader<bool, bool>);
impl CMP0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP0` writer - Compare 0 Interrupt Enable"]
pub struct CMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `CMP1` reader - Compare 1 Interrupt Enable"]
pub struct CMP1_R(crate::FieldReader<bool, bool>);
impl CMP1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP1` writer - Compare 1 Interrupt Enable"]
pub struct CMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SYNCRDY` reader - Synchronization Ready Interrupt Enable"]
pub struct SYNCRDY_R(crate::FieldReader<bool, bool>);
impl SYNCRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYNCRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCRDY` writer - Synchronization Ready Interrupt Enable"]
pub struct SYNCRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCRDY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `OVF` reader - Overflow Interrupt Enable"]
pub struct OVF_R(crate::FieldReader<bool, bool>);
impl OVF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF` writer - Overflow Interrupt Enable"]
pub struct OVF_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Compare 0 Interrupt Enable"]
    #[inline(always)]
    pub fn cmp0(&self) -> CMP0_R {
        CMP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Compare 1 Interrupt Enable"]
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    pub fn syncrdy(&self) -> SYNCRDY_R {
        SYNCRDY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare 0 Interrupt Enable"]
    #[inline(always)]
    pub fn cmp0(&mut self) -> CMP0_W {
        CMP0_W { w: self }
    }
    #[doc = "Bit 1 - Compare 1 Interrupt Enable"]
    #[inline(always)]
    pub fn cmp1(&mut self) -> CMP1_W {
        CMP1_W { w: self }
    }
    #[doc = "Bit 6 - Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    pub fn syncrdy(&mut self) -> SYNCRDY_W {
        SYNCRDY_W { w: self }
    }
    #[doc = "Bit 7 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ovf(&mut self) -> OVF_W {
        OVF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MODE1 Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
