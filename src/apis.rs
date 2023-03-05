#![allow(dead_code)]

use actix_web::http::header::{ContentEncoding, ContentType};
use serde::{Deserialize, Serialize};
//use qrcode::QrCode;
//use image::{Luma, ImageBuffer};

use qrcode_generator::QrCodeEcc;

use base64::encode;
use std::env;
use std::fs;
use std::fs::{remove_file, File};
use std::io;
use std::io::BufReader;
use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::emvo_qrcode::*;
use crate::prompt_pay::{MerchantPromptPayCreditTransfer, BAHT, CUSTOMER_PRESENTED, THAI};
use actix_web::web::Json;
use actix_web::{post, HttpResponse, error, Responder, HttpRequest};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;

use derive_more::{Display, Error};

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateQrCodeRq {
    pub transaction_amount: f32,
    pub mobile_number: String,
    pub merchant_name: String
}

#[derive(Debug, Display, Error)]
pub enum PromptPayServiceError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,
}
impl error::ResponseError for PromptPayServiceError {
    fn status_code(&self) -> StatusCode {
        match *self {
            PromptPayServiceError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            PromptPayServiceError::BadClientData => StatusCode::BAD_REQUEST,
            PromptPayServiceError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}
pub struct QRCodeResponse {
    pub qrcode_base64 : String
}
impl Responder for QRCodeResponse {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok()
            .append_header(("Content-Transfer-Encoding", "base64"))
            .content_type("image/jpg")
            .body(self.qrcode_base64)
    }
}
impl QRCodeResponse {
    fn create_response(qrcode_generator : String) -> QRCodeResponse {
        return QRCodeResponse{
            qrcode_base64: qrcode_generator,
        }
    }
}


#[post("/promptpay/qrcode")]
pub async fn qr_code_tag30(req: Json<GenerateQrCodeRq>) -> Result<QRCodeResponse,PromptPayServiceError> {
    let mut emvo = EMVQR::default();
    let result = emvo.set_payload_format_indicator("02".to_string());
    return if result.is_ok() {
        emvo.set_point_types(STATIC_POINT)
            .expect("TODO: panic message");

        let mut merchant_prompt_pay = MerchantPromptPayCreditTransfer::default();

        //let mobile_number = req.mobile_number.as_ref().unwrap(); //String::from("0809729900");

        merchant_prompt_pay.set_promptpay_presented_type(CUSTOMER_PRESENTED);
        merchant_prompt_pay.set_mobile_number(&req.0.mobile_number);

        emvo.set_transaction_currency(BAHT);
        emvo.set_transaction_amount(req.0.transaction_amount.to_string());
        emvo.set_merchant_name(req.0.merchant_name);
        emvo.set_merchant_category_code("5311".to_string());
        emvo.set_merchant_account_information("29", Box::new(merchant_prompt_pay));
        emvo.set_merchant_city("Bangkok".to_string());
        emvo.set_postal_code("10240".to_string());
        emvo.set_country_code(THAI);

        let result = emvo.generate_pay_load();
        if result.is_ok() {
            let data = result.unwrap();
            let result: Vec<u8> =
                qrcode_generator::to_png_to_vec_from_str(data, QrCodeEcc::Low, 320).unwrap();
            let str_b64 = encode(result);
            Ok(QRCodeResponse::create_response(str_b64))
        } else {
            //HttpResponse::BadRequest().finish()
            Err(PromptPayServiceError::BadClientData)
        }
    } else {
        //HttpResponse::InternalServerError().finish()
        Err(PromptPayServiceError::InternalError)
    };
}
