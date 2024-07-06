#![allow(dead_code)]
use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::*;
use std::vec::Vec;

use crc::{Algorithm, Crc};

use crate::emvo_types::*;

pub const ID_PAYLOAD_FORMAT_INDICATOR: TagID = "00"; // (M) Payload Format Indicator
pub const ID_POINT_OF_INITIATION_METHOD: TagID = "01"; // (O) Point of Initiation Method
pub const ID_MERCHANT_ACCOUNT_INFORMATION_RANGE_START: TagID = "02"; // (M) 2-51 Merchant Account Information
pub const ID_MERCHANT_ACCOUNT_INFORMATION_RANGE_END: TagID = "51"; // (M) 2-51 Merchant Account Information
pub const ID_MERCHANT_CATEGORY_CODE: TagID = "52"; // (M) Merchant Category Code
pub const ID_TRANSACTION_CURRENCY: TagID = "53"; // (M) Transaction Currency
pub const ID_TRANSACTION_AMOUNT: TagID = "54"; // (C) Transaction Amount
pub const ID_TIP_OR_CONVENIENCE_INDICATOR: TagID = "55"; // (O) Tip or Convenience Indicator
pub const ID_VALUE_OF_CONVENIENCE_FEE_FIXED: TagID = "56"; // (C) Value of Convenience Fee Fixed
pub const ID_VALUE_OF_CONVENIENCE_FEE_PERCENTAGE: TagID = "57"; // (C) Value of Convenience Fee Percentage
pub const ID_COUNTRY_CODE: TagID = "58"; // (M) Country Code
pub const ID_MERCHANT_NAME: TagID = "59"; // (M) Merchant Name
pub const ID_MERCHANT_CITY: TagID = "60"; // (M) Merchant City
pub const ID_POSTAL_CODE: TagID = "61"; // (O) Postal Code
pub const ID_ADDITIONAL_DATA_FIELD_TEMPLATE: TagID = "62"; // (O) Additional Data Field Template
pub const ID_CRC: TagID = "63"; // (M) CRC
pub const ID_MERCHANT_INFORMATION_LANGUAGE_TEMPLATE: TagID = "64"; // (O) Merchant Information— Language Template
pub const ID_RFU_FOR_EMVCO_RANGE_START: TagID = "65"; // (O) 65-79 RFU for EMVCo
pub const ID_RFU_FOR_EMVCO_RANGE_END: TagID = "79"; // (O) 65-79 RFU for EMVCo
pub const ID_UNRESERVED_TEMPLATES_RANGE_START: TagID = "80"; // (O) 80-99 Unreserved Templates
pub const ID_UNRESERVED_TEMPLATES_RANGE_END: TagID = "99"; // (O) 80-99 Unreserved Templates

// Data Object ID Allocation in Merchant Account Information Template ...
pub const MERCHANT_ACCOUNT_INFORMATION_ID_GLOBALLY_UNIQUE_IDENTIFIER: TagID = "00";
pub const MERCHANT_ACCOUNT_INFORMATION_ID_PAYMENT_NETWORK_SPECIFIC_START: TagID = "01"; // (O) 03-99 RFU for EMVCo
pub const MERCHANT_ACCOUNT_INFORMATION_ID_PAYMENT_NETWORK_SPECIFIC_END: TagID = "99"; // (O) 03-99 RFU for EMVCo

pub const ADDITIONAL_ID_BILL_NUMBER: TagID = "01"; // (O) Bill Number
pub const ADDITIONAL_ID_MOBILE_NUMBER: TagID = "02"; // (O) Mobile Number
pub const ADDITIONAL_ID_STORE_LABEL: TagID = "03"; // (O) Store Label
pub const ADDITIONAL_ID_LOYALTY_NUMBER: TagID = "04"; // (O) Loyalty Number
pub const ADDITIONAL_ID_REFERENCE_LABEL: TagID = "05"; // (O) Reference Label
pub const ADDITIONAL_ID_CUSTOMER_LABEL: TagID = "06"; // (O) Customer Label
pub const ADDITIONAL_ID_TERMINAL_LABEL: TagID = "07"; // (O) Terminal Label
pub const ADDITIONAL_ID_PURPOSE_TRANSACTION: TagID = "08"; // (O) Purpose Transaction
pub const ADDITIONAL_ID_ADDITIONAL_CONSUMER_DATA_REQUEST: TagID = "09"; // (O) Additional Consumer Data Request
pub const ADDITIONAL_ID_RFUFOR_EMVCO_RANGE_START: TagID = "10"; // (O) RFU for EMVCo
pub const ADDITIONAL_ID_RFUFOR_EMVCO_RANGE_END: TagID = "49"; // (O) RFU for EMVCo
pub const ADDITIONAL_ID_PAYMENT_SYSTEM_SPECIFIC_TEMPLATES_RANGE_START: TagID = "50"; // (O) Payment System Specific Templates
pub const ADDITIONAL_ID_PAYMENT_SYSTEM_SPECIFIC_TEMPLATES_RANGE_END: TagID = "99"; // (O) Payment System Specific Templates

// Data Objects for Merchant Information—Language Template (ID "64")

pub const MERCHANT_INFORMATION_ID_LANGUAGE_PREFERENCE: TagID = "00"; // (M) Language Preference
pub const MERCHANT_INFORMATION_ID_MERCHANT_NAME: TagID = "01"; // (M) Merchant Name
pub const MERCHANT_INFORMATION_ID_MERCHANT_CITY: TagID = "02"; // (O) Merchant City
pub const MERCHANT_INFORMATION_ID_RFUFOR_EMVCO_RANGE_START: TagID = "03"; // (O) 03-99 RFU for EMVCo
pub const MERCHANT_INFORMATION_ID_RFUFOR_EMVCO_RANGE_END: TagID = "99"; // (O) 03-99 RFU for EMVCo

// Data Object ID Allocation in Merchant Account Information Template ...

pub const UNRESERVED_TEMPLATE_ID_GLOBALLY_UNIQUE_IDENTIFIER: TagID = "00";
pub const UNRESERVED_TEMPLATE_ID_CONTEXT_SPECIFIC_DATA_START: TagID = "01"; // (O) 03-99 RFU for EMVCo
pub const UNRESERVED_TEMPLATE_ID_CONTEXT_SPECIFIC_DATA_END: TagID = "99"; // (O) 03-99 RFU for EMVCo

pub const STATIC_POINT: PointType = 0;
pub const DYNAMIC_POINT: PointType = 1;

//#[derive(Debug,Clone)]
pub struct EMVQRFieldDataObject {
    tag_id: TagID,
    data: Box<dyn EMVOData>,
    length: usize,
}
pub struct EMVQFieldDataObjectListBuilder<'a> {
    fields: Vec<&'a EMVQRFieldDataObject>,
}
/*
pub struct AdditionalDataFieldTemplate {
    bill_number: Option<EMVQRFieldDataObject>,
    mobile_number: Option<EMVQRFieldDataObject>,
    storage_label: Option<EMVQRFieldDataObject>,
    loyalty: Option<EMVQRFieldDataObject>,
    reference_label: Option<EMVQRFieldDataObject>,
    customer_label: Option<EMVQRFieldDataObject>,
    terminal_label: Option<EMVQRFieldDataObject>,
    purpose_of_transaction: Option<EMVQRFieldDataObject>,
    additional_consumer_data: Option<EMVQRFieldDataObject>,
    rfu_emvcos: Option<Vec<EMVQRFieldDataObject>>,
    payment_system_templates: Option<Vec<EMVQRFieldDataObject>>,
}
*/

//#[derive(Debug, Clone)]
#[derive(Default)]
pub struct EMVQR {
    payload_format_indicator: Option<EMVQRFieldDataObject>,
    point_of_initiation_method: Option<EMVQRFieldDataObject>,
    merchant_account_information: Option<HashMap<TagID, Box<dyn EMVOData>>>,
    //merchant_account_information_summary: Option<EMVQRFieldDataObject>,
    merchant_category_code: Option<EMVQRFieldDataObject>,
    transaction_currency: Option<EMVQRFieldDataObject>,
    transaction_amount: Option<EMVQRFieldDataObject>,
    tip_or_convenience_indicator: Option<EMVQRFieldDataObject>,
    value_of_convenience_fee_fixed: Option<EMVQRFieldDataObject>,
    value_of_convenience_fee_percentage: Option<EMVQRFieldDataObject>,
    country_code: Option<EMVQRFieldDataObject>,
    merchant_name: Option<EMVQRFieldDataObject>,
    merchant_city: Option<EMVQRFieldDataObject>,
    postal_code: Option<EMVQRFieldDataObject>,
    additional_data_field_template: Option<Box<dyn EMVOData>>,
    crc: Option<EMVQRFieldDataObject>,
    merchant_information_language_template: Option<Box<dyn EMVOData>>,
    rfu_for_emvcos: Option<Vec<EMVQRFieldDataObject>>,
}
//////////

impl EMVQRFieldDataObject {
    pub fn new(tag_id: TagID, data: Box<dyn EMVOData>, length: usize) -> Self {
        EMVQRFieldDataObject {
            tag_id,
            data,
            length,
        }
    }

    pub fn value(&self) -> Result<String, EMVOError> {
        return if !self.data.valid() {
            let msg = format!("Tag Id {} Data Invalid", self.tag_id);
            Err(EMVOError::new(msg.as_str()))
        } else {
            if !self.data.value().is_empty() {
                if self.data.len() > self.length {
                    let msg = format!("Tag Id {} Data Length Invalid", self.tag_id);
                    return Err(EMVOError::new(msg.as_str()));
                }
                let data_type_name = self.data.type_name();
                let result = match data_type_name {
                    TYPE_NAME_NUMERIC => {
                        //numberic
                        format!(
                            "{}{:0>2}{}",
                            self.tag_id,
                            self.length,
                            format!("{:0>width$}", self.data.value(), width = self.length).as_str()
                        )
                    }
                    _ => {
                        //default
                        format!(
                            "{}{:0>2}{}",
                            self.tag_id,
                            self.data.len(),
                            self.data.value()
                        )
                    }
                };
                Ok(String::from(result))
            } else {
                let msg = format!("Tag Id {} Data Is Empty", self.tag_id);
                Err(EMVOError::new(msg.as_str()))
            }
        };
    }
}
impl<'a> EMVQFieldDataObjectListBuilder<'a> {
    pub fn new() -> Self {
        let fields: Vec<&EMVQRFieldDataObject> = Vec::new();
        EMVQFieldDataObjectListBuilder { fields }
    }

    /*
    pub fn add_field(&mut self, tag_id: TagID, data: Box<dyn EMVOData>, length: usize) {
        let field = EMVQRFieldDataObject::new(tag_id, data, length);
        self.add_field_object(&field);
    }*/

    pub fn add_field_object(&mut self, field_object: &'a EMVQRFieldDataObject) {
        self.fields.push(field_object);
    }
    pub fn to_string(&self) -> Result<String, EMVOError> {
        let mut data: String = String::from("");
        for (_pos, item) in self.fields.iter().enumerate() {
            let result = item.value();
            match result {
                Ok(d) => {
                    if d.len() > 0 {
                        data.push_str(d.as_str());
                    }
                }
                Err(er) => {
                    return Err(er);
                }
            }
        }
        Ok(data)
    }
}
/*
impl AdditionalDataFieldTemplate {
    pub fn new() -> Self {
        AdditionalDataFieldTemplate {
            bill_number: None,
            mobile_number: None,
            storage_label: None,
            loyalty: None,
            reference_label: None,
            customer_label: None,
            terminal_label: None,
            purpose_of_transaction: None,
            additional_consumer_data: None,
            rfu_emvcos: None,
            payment_system_templates: None,
        }
    }
}
*/

impl EMVQR {
    pub fn set_payload_format_indicator(&mut self, data: Data) -> Result<(), EMVOError> {
        if data.is_empty() {
            return Err(EMVOError::new("Data is empty"));
        }
        if data.eq("01") {
            return Err(EMVOError::new("Data Invalid"));
        }
        let numeric = EMVNumeric::try_from(data).unwrap();
        let box_numeric = Box::new(numeric);
        let field = EMVQRFieldDataObject::new(ID_PAYLOAD_FORMAT_INDICATOR, box_numeric, 2);
        //self.add_payload_format_indicator(field);
        self.payload_format_indicator = Some(field);
        Ok(())
    }
    /*
    fn add_payload_format_indicator(&mut self,field:EMVQRFieldDataObject) {
        self.payload_format_indicator = Some(field);
    }*/
    pub fn set_point_types(&mut self, point_type: PointType) -> Result<(), EMVOError> {
        self.point_of_initiation_method = Some(EMVQRFieldDataObject::new(
            ID_POINT_OF_INITIATION_METHOD,
            Box::new(self.get_point_types(point_type)),
            2,
        ));
        Ok(())
    }
    pub fn get_point_types(&self, point_type: PointType) -> EMVNumeric {
        if point_type == STATIC_POINT {
            EMVNumeric::try_from(String::from("11")).unwrap()
        } else {
            EMVNumeric::try_from(String::from("12")).unwrap()
        }
    }
    pub fn set_merchant_account_information(&mut self, tag_id: TagID, data: Box<dyn EMVOData>) {
        if tag_id.is_between(
            ID_MERCHANT_ACCOUNT_INFORMATION_RANGE_START,
            ID_MERCHANT_ACCOUNT_INFORMATION_RANGE_END,
        ) {
            if self.merchant_account_information.is_none() {
                self.merchant_account_information = Some(HashMap::new());
            }
            self.merchant_account_information
                .as_mut()
                .unwrap()
                .insert(tag_id, data);
        }
    }
    pub fn set_merchant_category_code(&mut self, merchant_category_code: Data) {
        self.merchant_category_code = Some(EMVQRFieldDataObject::new(
            ID_MERCHANT_CATEGORY_CODE,
            Box::new(EMVNumeric::try_from(merchant_category_code).unwrap()),
            4,
        ));
    }
    pub fn set_transaction_currency(&mut self, transaction_currency: CurrencyCode) {
        self.transaction_currency = Some(EMVQRFieldDataObject::new(
            ID_TRANSACTION_CURRENCY,
            Box::new(EMVNumeric::try_from(transaction_currency.to_string()).unwrap()),
            3,
        ));
    }
    pub fn set_transaction_amount(&mut self, transaction_amount: Data) {
        self.transaction_amount = Some(EMVQRFieldDataObject::new(
            ID_TRANSACTION_AMOUNT,
            Box::new(EMVAlphanumbericSpecial::try_from(transaction_amount).unwrap()),
            14,
        ));
    }
    pub fn set_tip_convenience_indicator(&mut self, tip: Data) {
        self.transaction_amount = Some(EMVQRFieldDataObject::new(
            ID_TIP_OR_CONVENIENCE_INDICATOR,
            Box::new(EMVNumeric::try_from(tip).unwrap()),
            2,
        ));
    }
    pub fn set_value_of_convenience_fee_fixed(&mut self, value_of_convenience_fee_fixed: Data) {
        self.value_of_convenience_fee_fixed = Some(EMVQRFieldDataObject::new(
            ID_VALUE_OF_CONVENIENCE_FEE_FIXED,
            Box::new(EMVAlphanumbericSpecial::try_from(value_of_convenience_fee_fixed).unwrap()),
            13,
        ));
    }
    pub fn set_value_of_convenience_fee_percentage(
        &mut self,
        value_of_convenience_fee_percentage: Data,
    ) {
        self.value_of_convenience_fee_percentage = Some(EMVQRFieldDataObject::new(
            ID_VALUE_OF_CONVENIENCE_FEE_PERCENTAGE,
            Box::new(
                EMVAlphanumbericSpecial::try_from(value_of_convenience_fee_percentage).unwrap(),
            ),
            5,
        ));
    }
    pub fn set_country_code(&mut self, country_code: CountryCode) {
        self.country_code = Some(EMVQRFieldDataObject::new(
            ID_COUNTRY_CODE,
            Box::new(EMVAlphanumbericSpecial::try_from(country_code.to_string()).unwrap()),
            2,
        ))
    }
    pub fn set_merchant_name(&mut self, merchant_name: Data) {
        self.merchant_name = Some(EMVQRFieldDataObject::new(
            ID_MERCHANT_NAME,
            Box::new(EMVAlphanumbericSpecial::try_from(merchant_name).unwrap()),
            25,
        ))
    }
    pub fn set_merchant_city(&mut self, merchant_city: Data) {
        self.merchant_city = Some(EMVQRFieldDataObject::new(
            ID_MERCHANT_CITY,
            Box::new(EMVAlphanumbericSpecial::try_from(merchant_city).unwrap()),
            15,
        ))
    }
    pub fn set_postal_code(&mut self, postal_code: Data) {
        self.postal_code = Some(EMVQRFieldDataObject::new(
            ID_POSTAL_CODE,
            Box::new(EMVAlphanumbericSpecial::try_from(postal_code).unwrap()),
            10,
        ))
    }
    pub fn set_rfu(&mut self, tag_id: TagID, rfu_for_emvco: Data) {
        if tag_id.is_between(ID_RFU_FOR_EMVCO_RANGE_START, ID_RFU_FOR_EMVCO_RANGE_END) {
            if self.rfu_for_emvcos.is_none() {
                let rfu_for_emvcos: Vec<EMVQRFieldDataObject> = Vec::new();
                self.rfu_for_emvcos = Some(rfu_for_emvcos);
            }
            let item = EMVQRFieldDataObject::new(
                tag_id,
                Box::new(EMVString::try_from(rfu_for_emvco).unwrap()),
                99,
            );
            self.rfu_for_emvcos.as_mut().unwrap().push(item);
        }
    }
    pub fn generate_pay_load(&mut self) -> Result<String, EMVOError> {
        let mut builder = EMVQFieldDataObjectListBuilder::new();
        let mut merchant_list: Vec<EMVQRFieldDataObject> = vec![];
        if self.payload_format_indicator.is_some() {
            builder.add_field_object(self.payload_format_indicator.as_ref().unwrap());
        }
        if self.point_of_initiation_method.is_some() {
            builder.add_field_object(self.point_of_initiation_method.as_ref().unwrap());
        }
        if self.merchant_account_information.is_some() {
            for (k, v) in self.merchant_account_information.as_ref().unwrap() {
                //let item = EMVQRFieldDataObject::new(k,v,99);
                let data = v.value();
                merchant_list.push(EMVQRFieldDataObject::new(
                    k,
                    Box::new(EMVAlphanumbericSpecial::try_from(data).unwrap()),
                    99,
                ));
            }
            for (index, _) in merchant_list.iter().enumerate() {
                builder.add_field_object(merchant_list.get(index).unwrap());
            }
        }
        if self.merchant_category_code.is_some() {
            builder.add_field_object(self.merchant_category_code.as_ref().unwrap());
        }
        if self.transaction_currency.is_some() {
            builder.add_field_object(self.transaction_currency.as_ref().unwrap());
        }
        if self.transaction_amount.is_some() {
            builder.add_field_object(self.transaction_amount.as_ref().unwrap());
        }
        if self.tip_or_convenience_indicator.is_some() {
            builder.add_field_object(self.tip_or_convenience_indicator.as_ref().unwrap());
        }
        if self.value_of_convenience_fee_fixed.is_some() {
            builder.add_field_object(self.value_of_convenience_fee_fixed.as_ref().unwrap());
        }
        if self.value_of_convenience_fee_percentage.is_some() {
            builder.add_field_object(self.value_of_convenience_fee_percentage.as_ref().unwrap());
        }
        if self.country_code.is_some() {
            builder.add_field_object(self.country_code.as_ref().unwrap());
        }
        if self.merchant_name.is_some() {
            builder.add_field_object(self.merchant_name.as_ref().unwrap());
        }
        if self.merchant_city.is_some() {
            builder.add_field_object(self.merchant_city.as_ref().unwrap());
        }
        if self.postal_code.is_some() {
            builder.add_field_object(self.postal_code.as_ref().unwrap());
        }
        let result = builder.to_string();
        let mut crc_value = String::from(result.unwrap().clone());
        crc_value.push_str(ID_CRC);
        crc_value.push_str("04");

        // use custom algorithm
        const CUSTOM_ALG: Algorithm<u16> = Algorithm {
            width: 16,
            poly: 0x1021,
            init: 0xffff,
            refin: false,
            refout: false,
            xorout: 0x0000,
            check: 0x29B1,
            residue: 0x0000,
        };

        let crc = Crc::<u16>::new(&CUSTOM_ALG);
        let mut digest = crc.digest();
        digest.update(crc_value.into_bytes().as_slice());
        let crc_value_string = format!("{:04x}", digest.finalize());
        println!("crc value {}", crc_value_string);

        self.crc = Some(EMVQRFieldDataObject::new(
            ID_CRC,
            Box::new(EMVAlphanumbericSpecial::try_from(crc_value_string.to_uppercase()).unwrap()),
            4,
        ));
        builder.add_field_object(self.crc.as_ref().unwrap());
        builder.to_string()
    }
}
