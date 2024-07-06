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
        TYPE_NAME_ALPHA_NUMERIC
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
            Box::new(EMVAlphanumbericSpecial::try_from(data.to_string()).unwrap()),
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
mod test {
    use base64::Engine;
    use base64::engine::general_purpose;
    use qrcode_generator::QrCodeEcc;
    use super::*;

    #[test]
    fn test_merchant_prompt_pay_credit_transfer() {
        let mut emvo = EMVQR::default();
        let result = emvo.set_payload_format_indicator("02".to_string()).expect("Error");
        let mut merchant_prompt_pay = MerchantPromptPayCreditTransfer::default();
        //let mobile_number = req.mobile_number.as_ref().unwrap(); //String::from("0809729900");
        merchant_prompt_pay.set_promptpay_presented_type(CUSTOMER_PRESENTED);
        merchant_prompt_pay.set_mobile_number(&"0809729900".to_string());

        emvo.set_transaction_currency(BAHT);
        emvo.set_transaction_amount("50".to_string());
        emvo.set_merchant_name("test".to_string());
        emvo.set_merchant_category_code("5311".to_string());
        emvo.set_merchant_account_information("29", Box::new(merchant_prompt_pay));
        emvo.set_merchant_city("Bangkok".to_string());
        emvo.set_postal_code("10240".to_string());
        emvo.set_country_code(THAI);

        let result = emvo.generate_pay_load().expect("Error");
        let result: Vec<u8> =
            qrcode_generator::to_png_to_vec_from_str(result, QrCodeEcc::Low, 320).unwrap();
        let str_b64 = general_purpose::STANDARD.encode(&result);
        let expected_qr_code = "iVBORw0KGgoAAAANSUhEUgAAAUAAAAFACAAAAADo+/p2AAAEA0lEQVR42u3WS3bbMBBFQe1/084458jQbZB2IKV6xg+AftUc8PGlLtUDAUCAAAEqgAABAlQAAQIEqAACfGPAx7Seb/vXs9VVX746fbznbWkBAgQIECBAgKcBfr2ue5DuYQkz7aqjNwECBAgQIECAhwKO44UkIXN/ZXd896QFCBAgQIAAAX4CYF8X/L9y7WQGCBAgQIAAAQL8vr8g14eyqsCyagkgQIAAAQIE+LGAqxofflE8jKEvv5gWIECAAAECBHguYK8V57tcjdMCBAgQIECAAA8EvFhPz7k13niX0OCVwAABAgQIECDAUwD7v9Bjs8J5PezunvdMAyBAgAABAgR4IGDoL9xc5erH7s5m3PVql++eAQQIECBAgABPAxxXOHW1bhVvtaDDj+fd3gQIECBAgAABng0YWEKuwNnDdqsVS58iQIAAAQIECPB4wNBfr9D7TtNxs1+ZBkCAAAECBAjwGMDe+0WP0PTFEa367OtePAMIECBAgAABHgPYkca5QqDV1W5nY+PRFwUQIECAAAECPBTwos7qqsfr1EHgHlWAAAECBAgQ4IGAK7LQyi5LOG+XepWvdwYQIECAAAECPB6wt/KPWMIJPUrv7LtnAAECBAgQIMB3Agw6Y6Tdm91xjNT2BAgQIECAAAEeAzjWCdR9NuPMu/Pu7b5qDiBAgAABAgR4GGC/Ch5j3F25Pu9wbOMECBAgQIAAAR4DGLrtEXrm8aQuDqzP9MUzgAABAgQIEOChgOMk47Dj6Pcs78YvTgcIECBAgAABHgN4j8fqzacnjHf5tQIIECBAgAABvidgVw2Hr7Ze/VT2eOMf1Yv5AAIECBAgQIAnAfaw4wirjn5iz90voy8HCBAgQIAAAR4IuJuk+/fedwV6E320AAECBAgQIMBzAcNeF9tc6fQIT5u4p2uAAAECBAgQ4HsCdoFVhG7VM68266rjKF/rAggQIECAAAEeAzhmGVOHm6GJ3T572rAcIECAAAECBPgugG3L+Owe6u4/HlGIAhAgQIAAAQI8FzDUY1EXhzJ+M1z1rgECBAgQIECAbw24Wj0+daUT4FdNhD1369X3AxAgQIAAAQI8BXC3VgI/yDLeM3wSo2cAAQIECBAgwGMAH5v/Sf3qIm5ot78Srl60BBAgQIAAAQI8DbD3Hp6tGgvndepws3fdHAECBAgQIECAhwKO/6F2c43D9nb7K+PRAgQIECBAgAA/D3B1eHhzl2VMvbscIECAAAECBPixgGPVVZthXV9+0REgQIAAAQIE+C6AvdtgvBvhnpthijvjAwgQIECAAAGeBtgrrNuNF4Y5biJsNloHECBAgAABAjwGUM0KIECAAAEqgAABAlQAAQIEqAACBPj/1B/AwBs9kdYQtQAAAABJRU5ErkJggg==";
        assert_eq!(str_b64, expected_qr_code);
    }
}
