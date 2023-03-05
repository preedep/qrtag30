#![allow(dead_code)]

use std::convert::TryFrom;

use crate::emvo_qrcode::*;
use crate::emvo_types::*;

pub const BAHT: CurrencyCode = "764";
pub const THAI: CountryCode = "TH";
pub const LANG_TH: LanguageCode = "TH";

pub const ID_PROMPT_PAY_CREDIT_TRANSFER_AID: TagID = "00";
pub const ID_PROMPT_PAY_CREDIT_TRANSFER_MOBILE_NUMBER: TagID = "01";
pub const ID_PROMPT_PAY_CREDIT_TRANSFER_NATIONAL_ID: TagID = "02";
pub const ID_PROMPT_PAY_CREDIT_TRANSFER_EWALLET_ID: TagID = "03";
pub const ID_PROMPT_PAY_CREDIT_TRANSFER_BANK_ACCOUNT: TagID = "04";
pub const ID_PROMPT_PAY_CREDIT_TRANSFER_OTA: TagID = "05";

pub const ID_PROMPT_PAY_BILL_PAYMENT_AID: TagID = "00";
pub const ID_PROMPT_PAY_BILL_PAYMENT_BILLER_ID: TagID = "01";
pub const ID_PROMPT_PAY_BILL_PAYMENT_REF1: TagID = "02";
pub const ID_PROMPT_PAY_BILL_PAYMENT_REF2: TagID = "03";

pub type PromptPayPresentedType = usize;
pub const MERCHANT_PRESENTED: PromptPayPresentedType = 0;
pub const CUSTOMER_PRESENTED: PromptPayPresentedType = 1;

const MAX_LENGTH_AID: usize = 16;
const MAX_LENGTH_MOBILE_NO: usize = 13;
const MAX_LENGTH_NATIONAL_ID: usize = 13;
const MAX_LENGTH_EWALLET_ID: usize = 15;
const MAX_LENGTH_BANK_ACCOUNT: usize = 43;
const MAX_LENGTH_OTA: usize = 10;
const MAX_LENGTH_BILLER_ID: usize = 15;
const MAX_LENGTH_REF1: usize = 20;
const MAX_LENGTH_REF2: usize = 20;
const MAX_LENGTH_API_ID: usize = 3;
const MAX_LENGTH_SERVICE_PROFIDER_ID: usize = 3;
const MAX_LENGTH_TRANSACTION_REF: usize = 25;
const MAX_LENGTH_ACQUIRER_ID: usize = 3;

const PROMPTPAY_PRESENTED_TYPE_ARRAY: [&'static str; 2] = ["A000000677010111", "A000000677010114"];

#[derive(Default)]
pub struct MerchantPromptPayCreditTransfer {
    aid: Option<EMVQRFieldDataObject>,
    mobile: Option<EMVQRFieldDataObject>,
    national_id: Option<EMVQRFieldDataObject>,
    e_wallet_id: Option<EMVQRFieldDataObject>,
    bank_account: Option<EMVQRFieldDataObject>,
    ota: Option<EMVQRFieldDataObject>,
}

impl EMVOData for MerchantPromptPayCreditTransfer {
    fn len(&self) -> usize {
        let data = self.value();
        EMVAlphanumbericSpecial::try_from(data).unwrap().len()
    }

    fn valid(&self) -> bool {
        let data = self.value();
        EMVAlphanumbericSpecial::try_from(data).unwrap().valid()
    }

    fn value(&self) -> Data {
        let mut builder = EMVQFieldDataObjectListBuilder::new();
        if self.aid.is_some() {
            let aid_ref = self.aid.as_ref().unwrap();
            builder.add_field_object(aid_ref);
        }
        if self.mobile.is_some() {
            builder.add_field_object(self.mobile.as_ref().unwrap());
        }
        if self.national_id.is_some() {
            builder.add_field_object(self.national_id.as_ref().unwrap());
        }
        if self.e_wallet_id.is_some() {
            builder.add_field_object(self.e_wallet_id.as_ref().unwrap());
        }
        if self.bank_account.is_some() {
            builder.add_field_object(self.bank_account.as_ref().unwrap());
        }
        if self.aid.is_some() {
            let data = self.aid.as_ref().unwrap().value().unwrap();
            if data.eq(&PROMPTPAY_PRESENTED_TYPE_ARRAY[CUSTOMER_PRESENTED].to_string()) {
                if self.ota.is_some() {
                    builder.add_field_object(self.ota.as_ref().unwrap());
                }
            }
        }
        builder.to_string().unwrap()
    }

    fn type_name(&self) -> &str {
        TYPE_NAME_ALPHA_NUMBERIC
    }
}

impl MerchantPromptPayCreditTransfer {
    pub fn set_promptpay_presented_type(
        &mut self,
        prompt_pay_presented_type: PromptPayPresentedType,
    ) {
        let data = &PROMPTPAY_PRESENTED_TYPE_ARRAY[prompt_pay_presented_type];
        self.aid = Some(EMVQRFieldDataObject::new(
            ID_PROMPT_PAY_CREDIT_TRANSFER_AID,
            Box::new(EMVAlphanumbericSpecial::try_from(data.clone().to_string()).unwrap()),
            MAX_LENGTH_AID,
        ));
    }
    pub fn set_mobile_number(&mut self, mobile_no: &Data) {
        self.mobile = Some(EMVQRFieldDataObject::new(
            ID_PROMPT_PAY_CREDIT_TRANSFER_MOBILE_NUMBER,
            Box::new(EMVNumeric::try_from(mobile_no.clone()).unwrap()),
            MAX_LENGTH_MOBILE_NO,
        ));
    }
    pub fn set_national_id(&mut self, national_id: &Data) {
        self.national_id = Some(EMVQRFieldDataObject::new(
            ID_PROMPT_PAY_CREDIT_TRANSFER_NATIONAL_ID,
            Box::new(EMVNumeric::try_from(national_id.clone()).unwrap()),
            MAX_LENGTH_NATIONAL_ID,
        ));
    }
    pub fn set_e_wallet_id(&mut self, e_wallet_id: &Data) {
        self.e_wallet_id = Some(EMVQRFieldDataObject::new(
            ID_PROMPT_PAY_CREDIT_TRANSFER_EWALLET_ID,
            Box::new(EMVNumeric::try_from(e_wallet_id.clone()).unwrap()),
            MAX_LENGTH_EWALLET_ID,
        ));
    }
    pub fn set_bank_account(&mut self, bank_account: &Data) {
        self.bank_account = Some(EMVQRFieldDataObject::new(
            ID_PROMPT_PAY_CREDIT_TRANSFER_BANK_ACCOUNT,
            Box::new(EMVNumeric::try_from(bank_account.clone()).unwrap()),
            MAX_LENGTH_BANK_ACCOUNT,
        ));
    }
    pub fn set_ota(&mut self, ota: &Data) {
        self.ota = Some(EMVQRFieldDataObject::new(
            ID_PROMPT_PAY_CREDIT_TRANSFER_OTA,
            Box::new(EMVNumeric::try_from(ota.clone()).unwrap()),
            MAX_LENGTH_OTA,
        ));
    }
}
