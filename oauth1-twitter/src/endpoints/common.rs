use http_api_client_endpoint::{
    http::{Error as HttpError, StatusCode},
    Body,
};
use reqwest::Error as ReqwestError;
use reqwest_oauth1::SignerError as ReqwestOauth1SignerError;
use serde_json::Error as SerdeJsonError;
use serde_urlencoded::de::Error as SerdeUrlencodedDeError;

use crate::objects::response_body_fail::ResponseBodyFail;

//
#[derive(Debug, Clone)]
pub enum EndpointRet<T> {
    Ok(T),
    Other((StatusCode, Result<ResponseBodyFail, Body>)),
}

//
#[derive(Debug)]
pub enum EndpointError {
    MakeReqwestRequestBuilderFailed(ReqwestOauth1SignerError),
    MakeReqwestRequestFailed(ReqwestError),
    MakeRequestFailed(HttpError),
    DeResponseBodyOkFailed(SerdeUrlencodedDeError),
    DeResponseBodyOkJsonFailed(SerdeJsonError),
    Other(Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl core::fmt::Display for EndpointError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for EndpointError {}
