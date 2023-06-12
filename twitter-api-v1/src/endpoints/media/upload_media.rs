use reqwest::{
    multipart::{Form, Part},
    Body, Client, StatusCode,
};
use reqwest_oauth1::OAuthClientProvider as _;
use serde::{Deserialize, Serialize};
use serde_json::Map;

use crate::{
    endpoints::common::{EndpointError, EndpointRet},
    objects::{
        media::{MediaCategory, MediaImage, MediaProcessingInfo, MediaVideo},
        ResponseBodyErrJson,
    },
    secrets::TokenSecrets,
};

//
pub const UPLOAD_URL: &str = "https://upload.twitter.com/1.1/media/upload.json";

pub const SEGMENT_SIZE: usize = 1024 * 1024 * 5;
pub const SEGMENT_INDEX_MIN: usize = 0;
pub const SEGMENT_INDEX_MAX: usize = 999;

//
//
//
pub async fn upload_image<T>(
    secrets: &TokenSecrets,
    client: Client,
    media_category: MediaCategory,
    stream: T,
    stream_length: Option<u64>,
    file_name: Option<String>,
) -> Result<EndpointRet<UploadResponseBodyOkJson>, EndpointError>
where
    T: Into<Body>,
{
    match media_category {
        MediaCategory::TweetImage | MediaCategory::DmImage => {}
        _ => {
            return Err(EndpointError::ValidateFailed(
                "media_category invalid".into(),
            ))
        }
    }

    let mut query = Map::new();
    query.insert("media_category".into(), media_category.to_string().into());

    //
    let part = if let Some(stream_length) = stream_length {
        Part::stream_with_length(stream, stream_length)
    } else {
        Part::stream(stream)
    };

    let part = if let Some(file_name) = file_name {
        part.file_name(file_name)
    } else {
        part
    };

    let form = Form::new().part("media", part).percent_encode_noop();

    //
    let response = client
        .oauth1(secrets.secrets())
        .post(UPLOAD_URL)
        .query(&query)
        .multipart(form)
        .send()
        .await
        .map_err(EndpointError::RespondFailed)?;

    //
    let response_status = response.status();
    let response_body = response
        .bytes()
        .await
        .map_err(EndpointError::ReadResponseBodyFailed)?;
    let response_body = response_body.as_ref();

    match response_status {
        StatusCode::OK => Ok(EndpointRet::Ok(
            serde_json::from_slice(response_body)
                .map_err(EndpointError::DeResponseBodyOkJsonFailed)?,
        )),
        status => match serde_json::from_slice::<ResponseBodyErrJson>(response_body) {
            Ok(err_json) => Ok(EndpointRet::Other((status, Ok(err_json)))),
            Err(_) => Ok(EndpointRet::Other((status, Err(response_body.to_owned())))),
        },
    }
}

#[cfg(feature = "with_tokio")]
pub async fn upload_image_from_reader_stream<S>(
    secrets: &TokenSecrets,
    client: Client,
    media_category: MediaCategory,
    stream: S,
    stream_length: Option<u64>,
    file_name: Option<String>,
) -> Result<EndpointRet<UploadResponseBodyOkJson>, EndpointError>
where
    S: tokio::io::AsyncRead + Send + Sync + 'static,
{
    use tokio_util::io::ReaderStream;

    upload_image(
        secrets,
        client,
        media_category,
        Body::wrap_stream(ReaderStream::new(stream)),
        stream_length,
        file_name,
    )
    .await
}

#[cfg(feature = "with_tokio_fs")]
pub async fn upload_image_from_file(
    secrets: &TokenSecrets,
    client: Client,
    media_category: MediaCategory,
    file_path: &std::path::PathBuf,
) -> Result<EndpointRet<UploadResponseBodyOkJson>, EndpointError> {
    use tokio::fs::File;

    let crate::tokio_fs_util::Info {
        file_size,
        file_name,
    } = crate::tokio_fs_util::info(file_path)
        .await
        .map_err(EndpointError::GetFileInfoFailed)?;

    let file = File::open(&file_path)
        .await
        .map_err(EndpointError::OpenFileFailed)?;

    upload_image_from_reader_stream(
        secrets,
        client,
        media_category,
        file,
        Some(file_size),
        file_name,
    )
    .await
}

//
//
//
pub async fn upload_init(
    secrets: &TokenSecrets,
    client: Client,
    total_bytes: usize,
    media_type: impl AsRef<str>,
    media_category: MediaCategory,
) -> Result<EndpointRet<UploadResponseBodyOkJson>, EndpointError> {
    //
    let mut form = Map::new();
    form.insert("command".into(), "INIT".into());
    form.insert("total_bytes".into(), total_bytes.into());
    form.insert("media_type".into(), media_type.as_ref().into());
    form.insert("media_category".into(), media_category.to_string().into());

    //
    let response = client
        .oauth1(secrets.secrets())
        .post(UPLOAD_URL)
        .form(&form)
        .send()
        .await
        .map_err(EndpointError::RespondFailed)?;

    //
    let response_status = response.status();
    let response_body = response
        .bytes()
        .await
        .map_err(EndpointError::ReadResponseBodyFailed)?;
    let response_body = response_body.as_ref();

    match response_status {
        // 202
        StatusCode::ACCEPTED => Ok(EndpointRet::Ok(
            serde_json::from_slice(response_body)
                .map_err(EndpointError::DeResponseBodyOkJsonFailed)?,
        )),
        status => match serde_json::from_slice::<ResponseBodyErrJson>(response_body) {
            Ok(err_json) => Ok(EndpointRet::Other((status, Ok(err_json)))),
            Err(_) => Ok(EndpointRet::Other((status, Err(response_body.to_owned())))),
        },
    }
}

//
//
//
pub async fn upload_append<T>(
    secrets: &TokenSecrets,
    client: Client,
    media_id: u64,
    segment_index: usize,
    stream: T,
    stream_length: Option<u64>,
) -> Result<EndpointRet<()>, EndpointError>
where
    T: Into<Body>,
{
    if segment_index > SEGMENT_INDEX_MAX {
        return Err(EndpointError::ValidateFailed(
            "segment_index invalid".into(),
        ));
    }

    //
    let mut query = Map::new();
    query.insert("command".into(), "APPEND".into());
    query.insert("media_id".into(), media_id.into());
    query.insert("segment_index".into(), segment_index.into());

    //
    let part = if let Some(stream_length) = stream_length {
        Part::stream_with_length(stream, stream_length)
    } else {
        Part::stream(stream)
    };

    let form = Form::new().part("media", part).percent_encode_noop();

    //
    let response = client
        .oauth1(secrets.secrets())
        .post(UPLOAD_URL)
        .query(&query)
        .multipart(form)
        .send()
        .await
        .map_err(EndpointError::RespondFailed)?;

    //
    let response_status = response.status();

    match response_status {
        StatusCode::NO_CONTENT => Ok(EndpointRet::Ok(())),
        status => {
            let response_body = response
                .bytes()
                .await
                .map_err(EndpointError::ReadResponseBodyFailed)?;
            let response_body = response_body.as_ref();

            match serde_json::from_slice::<ResponseBodyErrJson>(response_body) {
                Ok(err_json) => Ok(EndpointRet::Other((status, Ok(err_json)))),
                Err(_) => Ok(EndpointRet::Other((status, Err(response_body.to_owned())))),
            }
        }
    }
}

#[cfg(feature = "with_tokio")]
pub async fn upload_append_from_reader_stream<S>(
    secrets: &TokenSecrets,
    client: Client,
    media_id: u64,
    segment_index: usize,
    stream: S,
    stream_length: Option<u64>,
) -> Result<EndpointRet<()>, EndpointError>
where
    S: tokio::io::AsyncRead + Send + Sync + 'static,
{
    use tokio_util::io::ReaderStream;

    upload_append(
        secrets,
        client,
        media_id,
        segment_index,
        Body::wrap_stream(ReaderStream::new(stream)),
        stream_length,
    )
    .await
}

#[cfg(feature = "with_tokio_fs")]
pub async fn upload_append_from_file(
    secrets: &TokenSecrets,
    client: Client,
    media_id: u64,
    segment_index: usize,
    file_path: &std::path::PathBuf,
    file_index: core::ops::Range<usize>,
) -> Result<EndpointRet<()>, EndpointError> {
    use tokio::{
        fs::File,
        io::{AsyncReadExt as _, AsyncSeekExt as _, SeekFrom},
    };

    let file_index_start = file_index.start;
    let file_index_end = file_index.end;

    let file_take_size = file_index_end - file_index_start;

    let mut file = File::open(&file_path)
        .await
        .map_err(EndpointError::OpenFileFailed)?;
    file.seek(SeekFrom::Start(file_index_start as u64))
        .await
        .map_err(EndpointError::OpenFileFailed)?;
    let file = file.take(file_take_size as u64);

    upload_append_from_reader_stream(
        secrets,
        client.to_owned(),
        media_id,
        segment_index,
        file,
        Some(file_take_size as u64),
    )
    .await
}

#[cfg(feature = "with_tokio_fs")]
pub async fn upload_append_all_from_file(
    secrets: &TokenSecrets,
    client: Client,
    media_id: u64,
    file_path: &std::path::PathBuf,
) -> Result<Result<(), EndpointRet<()>>, EndpointError> {
    let crate::tokio_fs_util::Info {
        file_size,
        file_name: _,
    } = crate::tokio_fs_util::info(file_path)
        .await
        .map_err(EndpointError::GetFileInfoFailed)?;

    for segment_index in SEGMENT_INDEX_MIN..=SEGMENT_INDEX_MAX {
        let file_index_start = segment_index * SEGMENT_SIZE;
        let file_index_end = core::cmp::min(file_index_start + SEGMENT_SIZE, file_size as usize);

        let ret = upload_append_from_file(
            secrets,
            client.to_owned(),
            media_id,
            segment_index,
            file_path,
            file_index_start..file_index_end,
        )
        .await?;
        match ret {
            EndpointRet::Ok(_) => {}
            x => return Ok(Err(x)),
        };

        if file_index_start + SEGMENT_SIZE >= file_size as usize {
            break;
        }
    }

    Ok(Ok(()))
}

//
//
//
pub async fn upload_finalize(
    secrets: &TokenSecrets,
    client: Client,
    media_id: u64,
) -> Result<EndpointRet<UploadResponseBodyOkJson>, EndpointError> {
    //
    let mut form = Map::new();
    form.insert("command".into(), "FINALIZE".into());
    form.insert("media_id".into(), media_id.into());

    //
    let response = client
        .oauth1(secrets.secrets())
        .post(UPLOAD_URL)
        .form(&form)
        .send()
        .await
        .map_err(EndpointError::RespondFailed)?;

    //
    let response_status = response.status();
    let response_body = response
        .bytes()
        .await
        .map_err(EndpointError::ReadResponseBodyFailed)?;
    let response_body = response_body.as_ref();

    match response_status {
        // 200
        StatusCode::OK => Ok(EndpointRet::Ok(
            serde_json::from_slice(response_body)
                .map_err(EndpointError::DeResponseBodyOkJsonFailed)?,
        )),
        status => match serde_json::from_slice::<ResponseBodyErrJson>(response_body) {
            Ok(err_json) => Ok(EndpointRet::Other((status, Ok(err_json)))),
            Err(_) => Ok(EndpointRet::Other((status, Err(response_body.to_owned())))),
        },
    }
}

//
//
//
pub async fn get_upload_status(
    secrets: &TokenSecrets,
    client: Client,
    media_id: u64,
) -> Result<EndpointRet<UploadResponseBodyOkJson>, EndpointError> {
    //
    let mut query = Map::new();
    query.insert("command".into(), "STATUS".into());
    query.insert("media_id".into(), media_id.into());

    //
    let response = client
        .oauth1(secrets.secrets())
        .get(UPLOAD_URL)
        .query(&query)
        .send()
        .await
        .map_err(EndpointError::RespondFailed)?;

    //
    let response_status = response.status();
    let response_body = response
        .bytes()
        .await
        .map_err(EndpointError::ReadResponseBodyFailed)?;
    let response_body = response_body.as_ref();

    match response_status {
        // 200
        StatusCode::OK => Ok(EndpointRet::Ok(
            serde_json::from_slice(response_body)
                .map_err(EndpointError::DeResponseBodyOkJsonFailed)?,
        )),
        status => match serde_json::from_slice::<ResponseBodyErrJson>(response_body) {
            Ok(err_json) => Ok(EndpointRet::Other((status, Ok(err_json)))),
            Err(_) => Ok(EndpointRet::Other((status, Err(response_body.to_owned())))),
        },
    }
}

//
//
//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UploadResponseBodyOkJson {
    pub media_id: u64,
    pub media_id_string: String,
    #[serde(default)]
    pub media_key: String,
    pub size: Option<usize>,
    pub expires_after_secs: Option<usize>,
    pub image: Option<MediaImage>,
    pub video: Option<MediaVideo>,
    pub processing_info: Option<MediaProcessingInfo>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::objects::media::MediaProcessingInfoState;

    #[test]
    fn de_upload_response_body_ok_json() {
        //
        match serde_json::from_str::<UploadResponseBodyOkJson>(include_str!(
            "../../../tests/response_body_json_files/media__upload_image__ok.json"
        )) {
            Ok(ok_json) => {
                assert!(ok_json.image.is_some());
                assert!(ok_json.processing_info.is_none());
            }
            Err(err) => panic!("{err}"),
        }

        //
        match serde_json::from_str::<UploadResponseBodyOkJson>(include_str!(
            "../../../tests/response_body_json_files/media__upload_init__ok.json"
        )) {
            Ok(ok_json) => {
                assert!(ok_json.video.is_none());
                assert!(ok_json.processing_info.is_none());
            }
            Err(err) => panic!("{err}"),
        }

        //
        match serde_json::from_str::<UploadResponseBodyOkJson>(include_str!(
            "../../../tests/response_body_json_files/media__upload_finalize__ok_async.json"
        )) {
            Ok(ok_json) => {
                assert!(ok_json.video.is_none());
                assert!(ok_json.processing_info.is_some());
                assert_eq!(
                    ok_json.processing_info.as_ref().unwrap().state,
                    MediaProcessingInfoState::Pending
                );
            }
            Err(err) => panic!("{err}"),
        }

        match serde_json::from_str::<UploadResponseBodyOkJson>(include_str!(
            "../../../tests/response_body_json_files/media__upload_finalize__ok_sync.json"
        )) {
            Ok(ok_json) => {
                assert!(ok_json.video.is_some());
                assert!(ok_json.processing_info.is_none());
            }
            Err(err) => panic!("{err}"),
        }

        //
        match serde_json::from_str::<UploadResponseBodyOkJson>(include_str!(
            "../../../tests/response_body_json_files/media__get_upload_status__ok_failed.json"
        )) {
            Ok(ok_json) => {
                assert!(ok_json.video.is_none());
                assert!(ok_json.processing_info.is_some());
                assert_eq!(
                    ok_json.processing_info.as_ref().unwrap().state,
                    MediaProcessingInfoState::Failed
                );
                assert!(ok_json.processing_info.unwrap().error.is_some());
            }
            Err(err) => panic!("{err}"),
        }

        match serde_json::from_str::<UploadResponseBodyOkJson>(include_str!(
            "../../../tests/response_body_json_files/media__get_upload_status__ok_in_progress.json"
        )) {
            Ok(ok_json) => {
                assert!(ok_json.video.is_none());
                assert!(ok_json.processing_info.is_some());
                assert_eq!(
                    ok_json.processing_info.as_ref().unwrap().state,
                    MediaProcessingInfoState::InProgress
                );
                assert!(ok_json.processing_info.unwrap().error.is_none());
            }
            Err(err) => panic!("{err}"),
        }

        match serde_json::from_str::<UploadResponseBodyOkJson>(include_str!(
            "../../../tests/response_body_json_files/media__get_upload_status__ok_succeeded.json"
        )) {
            Ok(ok_json) => {
                assert!(ok_json.video.is_some());
                assert!(ok_json.processing_info.is_some());
                assert_eq!(
                    ok_json.processing_info.as_ref().unwrap().state,
                    MediaProcessingInfoState::Succeeded
                );
                assert!(ok_json.processing_info.unwrap().error.is_none());
            }
            Err(err) => panic!("{err}"),
        }
    }
}
