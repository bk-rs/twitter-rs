use reqwest::{Error as ReqwestError, StatusCode};
use reqwest_oauth1::Error as ReqwestOauth1Error;
use serde_json::Error as SerdeJsonError;

use crate::objects::ResponseBodyErrJson;

//
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum EndpointRet<T> {
    Ok(T),
    Other((StatusCode, Result<ResponseBodyErrJson, Vec<u8>>)),
}

//
#[derive(Debug)]
pub enum EndpointError {
    ValidateFailed(String),
    RespondFailed(ReqwestOauth1Error),
    ReadResponseBodyFailed(ReqwestError),
    DeResponseBodyOkJsonFailed(SerdeJsonError),
    //
    DeV2ResponseBodyOkJsonFailed(SerdeJsonError),
    ConvertV2ResponseBodyOkJsonFailed(String),
    //
    #[cfg(feature = "with_tokio_fs")]
    GetFileInfoFailed(std::io::Error),
    #[cfg(feature = "with_tokio_fs")]
    OpenFileFailed(std::io::Error),
}
impl core::fmt::Display for EndpointError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for EndpointError {}
