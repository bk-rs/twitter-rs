use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

//
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
pub enum MediaCategory {
    #[serde(rename = "tweet_image")]
    TweetImage,
    #[serde(rename = "tweet_video")]
    TweetVideo,
    #[serde(rename = "tweet_gif")]
    TweetGif,
    #[serde(rename = "dm_image")]
    DmImage,
    #[serde(rename = "dm_video")]
    DmVideo,
    #[serde(rename = "dm_gif")]
    DmGif,
    #[serde(rename = "subtitles")]
    Subtitles,
    #[serde(other)]
    Other(String),
}

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MediaImage {
    pub image_type: String,
    pub w: usize,
    pub h: usize,
}

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MediaVideo {
    pub video_type: String,
}

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MediaProcessingInfo {
    pub state: MediaProcessingInfoState,
    pub check_after_secs: Option<usize>,
    pub progress_percent: Option<usize>,
    pub error: Option<MediaProcessingInfoError>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaProcessingInfoState {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "succeeded")]
    Succeeded,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MediaProcessingInfoError {
    pub code: usize,
    pub name: String,
    pub message: String,
}
