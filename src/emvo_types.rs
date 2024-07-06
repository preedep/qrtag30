#![allow(dead_code)]
use std::convert::*;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

use regex::Regex;

pub const TYPE_NAME_NUMBERIC: &'static str = "numeric";
pub const TYPE_NAME_ALPHA_NUMBERIC: &'static str = "alpha_numeric";
pub const TYPE_NAME_STR: &'static str = "str";

pub type Data = String;

pub type TagID = &'static str;
pub type CurrencyCode = &'static str;
pub type CountryCode = &'static str;
pub type LanguageCode = &'static str;
pub type PointType = usize;

pub trait EMVOData {
    fn len(&self) -> usize;
    fn valid(&self) -> bool;
    fn value(&self) -> Data;
    fn type_name(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct EMVOError {
    details: String,
}
#[derive(Debug, Clone)]
pub struct EMVNumeric {
    d: Option<Data>,
}
#[derive(Debug, Clone)]
pub struct EMVAlphanumbericSpecial {
    d: Option<Data>,
}
#[derive(Debug, Clone)]
pub struct EMVString {
    d: Option<Data>,
}

//Error
impl EMVOError {
    pub fn new(msg: &str) -> EMVOError {
        EMVOError {
            details: msg.to_string(),
        }
    }
}
impl fmt::Display for EMVOError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}
impl Error for EMVOError {
    fn description(&self) -> &str {
        &self.details
    }
}
////
impl EMVOData for EMVNumeric {
    fn len(&self) -> usize {
        self.d.clone().unwrap().len()
    }
    fn valid(&self) -> bool {
        let data = self.value();
        let re = Regex::new(r"(^[0-9]*$)").unwrap();
        let caps = re.captures_iter(data.as_str());
        caps.count() > 0
    }
    fn value(&self) -> Data {
        self.d.clone().unwrap()
    }

    fn type_name(&self) -> &str {
        TYPE_NAME_NUMBERIC
    }
}
impl TryFrom<Data> for EMVNumeric {
    type Error = EMVOError;
    fn try_from(value: Data) -> Result<Self, Self::Error> {
        let numeric = EMVNumeric { d: Some(value) };
        if numeric.valid() {
            Ok(numeric)
        } else {
            Err(EMVOError::new("Numeric invalid"))
        }
    }
}
/////
impl EMVOData for EMVAlphanumbericSpecial {
    fn len(&self) -> usize {
        self.d.clone().unwrap().len()
    }
    fn valid(&self) -> bool {
        let data = self.value();
        let re = Regex::new(r"(^[0-9a-zA-Z\s.!?\\-]*$)").unwrap();
        let caps = re.captures_iter(data.as_str());
        caps.count() > 0
    }
    fn value(&self) -> Data {
        self.d.clone().unwrap()
    }
    fn type_name(&self) -> &str {
        TYPE_NAME_ALPHA_NUMBERIC
    }
}
impl TryFrom<Data> for EMVAlphanumbericSpecial {
    type Error = EMVOError;
    fn try_from(value: Data) -> Result<Self, Self::Error> {
        let alpha_numeric_special = EMVAlphanumbericSpecial { d: Some(value) };
        if alpha_numeric_special.valid() {
            Ok(alpha_numeric_special)
        } else {
            Err(EMVOError::new("Alpha Numeric Invalid"))
        }
    }
}
////
impl EMVOData for EMVString {
    fn len(&self) -> usize {
        self.d.clone().unwrap().chars().count()
    }
    fn valid(&self) -> bool {
        std::str::from_utf8(self.value().as_bytes()).is_ok()
    }
    fn value(&self) -> Data {
        self.d.clone().unwrap()
    }
    fn type_name(&self) -> &str {
        TYPE_NAME_STR
    }
}
impl TryFrom<Data> for EMVString {
    type Error = EMVOError;

    fn try_from(value: Data) -> Result<Self, Self::Error> {
        let emv_str = EMVString { d: Some(value) };
        if emv_str.valid() {
            Ok(emv_str)
        } else {
            Err(EMVOError::new("String  Invalid"))
        }
    }
}

pub trait TagIDData {
    fn is_between(&self, start: TagID, end: TagID) -> bool;
}
impl TagIDData for TagID {
    fn is_between(&self, start: TagID, end: TagID) -> bool {
        let current_tag: usize = self.to_string().parse().unwrap();
        let start_tag: usize = start.to_string().parse().unwrap();
        let end_tag: usize = end.to_string().parse().unwrap();
        return (current_tag >= start_tag) && (current_tag <= end_tag);
    }
}
