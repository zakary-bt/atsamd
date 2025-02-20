#[doc = "Register `FCSE` reader"]
pub struct R(crate::R<FCSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FCKR` reader - Frame Check Sequence Errors"]
pub struct FCKR_R(crate::FieldReader<u16, u16>);
impl FCKR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FCKR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCKR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9 - Frame Check Sequence Errors"]
    #[inline(always)]
    pub fn fckr(&self) -> FCKR_R {
        FCKR_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Frame Check Sequence Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcse](index.html) module"]
pub struct FCSE_SPEC;
impl crate::RegisterSpec for FCSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcse::R](R) reader structure"]
impl crate::Readable for FCSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FCSE to value 0"]
impl crate::Resettable for FCSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
