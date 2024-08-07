#![allow(dead_code)]

use actix_web::{error, HttpRequest, HttpResponse, post, Responder};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use base64::Engine;
use base64::engine::general_purpose;
use derive_more::{Display, Error};
use log::{error, info};
use qrcode_generator::QrCodeEcc;
use serde::{Deserialize, Serialize};

use crate::emvo_qrcode::*;
use crate::prompt_pay::{BAHT, CUSTOMER_PRESENTED, MerchantPromptPayCreditTransfer, THAI};


//use qrcode::QrCode;
//use image::{Luma, ImageBuffer};


#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateQrCodeRq {
    pub transaction_amount: f32,
    pub mobile_number: String,
    pub merchant_name: String,
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
    pub qrcode_base64: String,
}
impl Responder for QRCodeResponse {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok()
            .append_header(("Content-Transfer-Encoding", "base64"))
            .content_type("image/jpg")
            .body(self.qrcode_base64)
    }
}
impl QRCodeResponse {
    fn create_response(qrcode_generator: String) -> QRCodeResponse {
        return QRCodeResponse {
            qrcode_base64: qrcode_generator,
        };
    }
}

#[post("/promptpay/qrcode")]
pub async fn qr_code_tag30(
    req: Json<GenerateQrCodeRq>,
) -> Result<QRCodeResponse, PromptPayServiceError> {
    let mut emvo = EMVQR::default();
    let result = emvo.set_payload_format_indicator("02".to_string());

    return if result.is_ok() {
        emvo.set_point_types(STATIC_POINT)
            .expect("TODO: panic message");

        let mut merchant_prompt_pay = MerchantPromptPayCreditTransfer::default();
        //let mobile_number = req.mobile_number.as_ref().unwrap(); //String::from("0809729900");

        merchant_prompt_pay.set_promptpay_presented_type(CUSTOMER_PRESENTED);
        merchant_prompt_pay.set_mobile_number(&req.0.mobile_number);
        emvo.set_point_types(STATIC_POINT).expect("Point Type Error");
        emvo.set_transaction_currency(BAHT);
        emvo.set_transaction_amount(req.0.transaction_amount.to_string());
        emvo.set_merchant_name(req.0.merchant_name);
        emvo.set_merchant_category_code("5311".to_string());
        emvo.set_merchant_account_information("29", Box::new(merchant_prompt_pay));
        emvo.set_merchant_city("Bangkok".to_string());
        emvo.set_postal_code("10240".to_string());
        emvo.set_country_code(THAI);

        // info!("Payload: {:?}", emvo.generate_pay_load());

        let result = emvo.generate_pay_load();

        if let Ok(result) = result {
            let data = result;

            let result: Vec<u8> =
                qrcode_generator::to_png_to_vec_from_str(data, QrCodeEcc::Low, 320).unwrap();
            let str_b64 = general_purpose::STANDARD.encode(&result);
            info!("QRCode: {}", str_b64);


            Ok(QRCodeResponse::create_response(str_b64))
        } else {
            //HttpResponse::BadRequest().finish()
            error!("Bad request");
            Err(PromptPayServiceError::BadClientData)
        }
    } else {
        //HttpResponse::InternalServerError().finish()
        Err(PromptPayServiceError::InternalError)
    };
}
