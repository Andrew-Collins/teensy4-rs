#[doc = "Reader of register XBAR1_IN17_SELECT_INPUT"]
pub type R = crate::R<u32, super::XBAR1_IN17_SELECT_INPUT>;
#[doc = "Writer for register XBAR1_IN17_SELECT_INPUT"]
pub type W = crate::W<u32, super::XBAR1_IN17_SELECT_INPUT>;
#[doc = "Register XBAR1_IN17_SELECT_INPUT `reset()`'s with value 0"]
impl crate::ResetValue for super::XBAR1_IN17_SELECT_INPUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selecting Pads Involved in Daisy Chain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DAISY_A {
    #[doc = "0: Selecting Pad: GPIO_EMC_08 for Mode: ALT3"]
    GPIO_EMC_08_ALT3 = 0,
    #[doc = "1: Selecting Pad: GPIO_AD_B0_03 for Mode: ALT1"]
    GPIO_AD_B0_03_ALT1 = 1,
    #[doc = "2: Selecting Pad: GPIO_AD_B0_05 for Mode: ALT6"]
    GPIO_AD_B0_05_ALT6 = 2,
    #[doc = "3: Selecting Pad: GPIO_B1_03 for Mode: ALT1"]
    GPIO_B1_03_ALT1 = 3,
}
impl From<DAISY_A> for u8 {
    #[inline(always)]
    fn from(variant: DAISY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DAISY`"]
pub type DAISY_R = crate::R<u8, DAISY_A>;
impl DAISY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAISY_A {
        match self.bits {
            0 => DAISY_A::GPIO_EMC_08_ALT3,
            1 => DAISY_A::GPIO_AD_B0_03_ALT1,
            2 => DAISY_A::GPIO_AD_B0_05_ALT6,
            3 => DAISY_A::GPIO_B1_03_ALT1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_EMC_08_ALT3`"]
    #[inline(always)]
    pub fn is_gpio_emc_08_alt3(&self) -> bool {
        *self == DAISY_A::GPIO_EMC_08_ALT3
    }
    #[doc = "Checks if the value of the field is `GPIO_AD_B0_03_ALT1`"]
    #[inline(always)]
    pub fn is_gpio_ad_b0_03_alt1(&self) -> bool {
        *self == DAISY_A::GPIO_AD_B0_03_ALT1
    }
    #[doc = "Checks if the value of the field is `GPIO_AD_B0_05_ALT6`"]
    #[inline(always)]
    pub fn is_gpio_ad_b0_05_alt6(&self) -> bool {
        *self == DAISY_A::GPIO_AD_B0_05_ALT6
    }
    #[doc = "Checks if the value of the field is `GPIO_B1_03_ALT1`"]
    #[inline(always)]
    pub fn is_gpio_b1_03_alt1(&self) -> bool {
        *self == DAISY_A::GPIO_B1_03_ALT1
    }
}
#[doc = "Write proxy for field `DAISY`"]
pub struct DAISY_W<'a> {
    w: &'a mut W,
}
impl<'a> DAISY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAISY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Selecting Pad: GPIO_EMC_08 for Mode: ALT3"]
    #[inline(always)]
    pub fn gpio_emc_08_alt3(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_EMC_08_ALT3)
    }
    #[doc = "Selecting Pad: GPIO_AD_B0_03 for Mode: ALT1"]
    #[inline(always)]
    pub fn gpio_ad_b0_03_alt1(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_AD_B0_03_ALT1)
    }
    #[doc = "Selecting Pad: GPIO_AD_B0_05 for Mode: ALT6"]
    #[inline(always)]
    pub fn gpio_ad_b0_05_alt6(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_AD_B0_05_ALT6)
    }
    #[doc = "Selecting Pad: GPIO_B1_03 for Mode: ALT1"]
    #[inline(always)]
    pub fn gpio_b1_03_alt1(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_B1_03_ALT1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub fn daisy(&self) -> DAISY_R {
        DAISY_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub fn daisy(&mut self) -> DAISY_W {
        DAISY_W { w: self }
    }
}
