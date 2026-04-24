use std::{ffi::OsStr, path::Path};

use axum::{body::Body, response::{IntoResponse, Response}};
use reqwest::{StatusCode, header::CONTENT_TYPE};

use crate::mainapi::SharedEngineAPI;

#[derive(Debug, Clone)]
pub enum FileContents {
    HTML(String),
    CSS(String),
    JS(String),
    JSON(String),
    Plain(String),
    Bytes(Vec<u8>),
    Text(String),
    PNG(Vec<u8>),
    JPEG(Vec<u8>),
    GIF(Vec<u8>),
    MP3(Vec<u8>),
    OGG(Vec<u8>),
    MP4(Vec<u8>),
    ICO(Vec<u8>),
    Unknown,
    Invalid(String)
}

impl FileContents {
    pub fn read<P: AsRef<Path>>(path: P, engine_api: SharedEngineAPI) -> Self {
        let path_ref: &Path = path.as_ref();

        let opt: Option<&OsStr> = path_ref.extension();

        if let Option::None = opt {
            return Self::Unknown;
        }

        let osstr: &OsStr = opt.unwrap();

        let opt: Option<&str> = osstr.to_str();

        if let Option::None = opt {
            return Self::Unknown;
        }

        let ext: &str = opt.unwrap();

        return match ext {
            "html" => Self::read_as_html(path, engine_api),
            "css" => Self::read_as_css(path, engine_api),
            "js" => Self::read_as_js(path, engine_api),
            "txt" => Self::read_as_plain(path, engine_api),
            "png" => Self::read_as_png(path, engine_api),
            "jpeg" => Self::read_as_jpeg(path, engine_api),
            "gif" => Self::read_as_gif(path, engine_api),
            "mp3" => Self::read_as_mp3(path, engine_api),
            "ogg" => Self::read_as_ogg(path, engine_api),
            "mp4" => Self::read_as_mp4(path, engine_api),
            "json" => Self::read_as_json(path, engine_api),
            "ico" => Self::read_as_ico(path, engine_api),
            _ => Self::Unknown
        };
    }

    fn read_as_png<P: AsRef<Path>>(path: P, engine_api: SharedEngineAPI) -> Self {
        return Self::read_as_bytes(path, engine_api).map_if_bytes(|bytes| Self::PNG(bytes));
    }

    fn read_as_ico<P: AsRef<Path>>(path: P, engine_api: SharedEngineAPI) -> Self {
        return Self::read_as_bytes(path, engine_api).map_if_bytes(|bytes| Self::ICO(bytes));
    }

    fn read_as_jpeg<P: AsRef<Path>>(path: P, engine_api: SharedEngineAPI) -> Self {
        return Self::read_as_bytes(path, engine_api).map_if_bytes(|bytes| Self::JPEG(bytes));
    }

    fn read_as_gif<P: AsRef<Path>>(path: P, engine_api: SharedEngineAPI) -> Self {
        return Self::read_as_bytes(path, engine_api).map_if_bytes(|bytes| Self::GIF(bytes));
    }

    fn read_as_mp3<P: AsRef<Path>>(path: P, engine_api: SharedEngineAPI) -> Self {
        return Self::read_as_bytes(path, engine_api).map_if_bytes(|bytes| Self::MP3(bytes));
    }

    fn read_as_ogg<P: AsRef<Path>>(path: P, engine_api: SharedEngineAPI) -> Self {
        return Self::read_as_bytes(path, engine_api).map_if_bytes(|bytes| Self::OGG(bytes));
    }

    fn read_as_mp4<P: AsRef<Path>>(path: P, engine_api: SharedEngineAPI) -> Self {
        return Self::read_as_bytes(path, engine_api).map_if_bytes(|bytes| Self::MP4(bytes));
    }

    fn read_as_html<P: AsRef<Path>>(path: P, engine_api: SharedEngineAPI) -> Self {
        return Self::read_as_text(path, engine_api).map_if_text(|text| Self::HTML(text));
    }

    fn read_as_json<P: AsRef<Path>>(path: P, engine_api: SharedEngineAPI) -> Self {
        return Self::read_as_text(path, engine_api).map_if_text(|text| Self::JSON(text));
    }

    fn read_as_css<P: AsRef<Path>>(path: P, engine_api: SharedEngineAPI) -> Self {
        return Self::read_as_text(path, engine_api).map_if_text(|text| Self::CSS(text));
    }

    fn read_as_plain<P: AsRef<Path>>(path: P, engine_api: SharedEngineAPI) -> Self {
        return Self::read_as_text(path, engine_api).map_if_text(|text| Self::Plain(text));
    }

    fn read_as_js<P: AsRef<Path>>(path: P, engine_api: SharedEngineAPI) -> Self {
        return Self::read_as_text(path, engine_api).map_if_text(|text| Self::JS(text));
    }

    fn read_as_bytes<P: AsRef<Path>>(path: P, engine_api: SharedEngineAPI) -> Self {
        let res = engine_api.lock();

        if let Result::Err(err) = res {
            return Self::Invalid(err.to_string());
        }

        let mut engine_api = res.unwrap();

        let res = engine_api.read(path.as_ref());

        if let Result::Err(err) = res {
            return Self::Invalid(err.to_string());
        }

        let data = res.ok().unwrap();

        return Self::Bytes(data);
    }

    fn read_as_text<P: AsRef<Path>>(path: P, engine_api: SharedEngineAPI) -> Self {
        let res = engine_api.lock();

        if let Result::Err(err) = res {
            return Self::Invalid(err.to_string());
        }

        let mut engine_api = res.unwrap();

        let res = engine_api.read_to_string(path.as_ref());

        if let Result::Err(err) = res {
            return Self::Invalid(err.to_string());
        }

        let contents = res.ok().unwrap();

        return Self::Text(contents);
    }
}

impl FileContents {
    pub fn map_if_text(self, callback: impl FnOnce(String) -> Self) -> Self {
        return match self {
            Self::Text(text) => callback(text),
            _ => self
        };
    }

    pub fn map_if_bytes(self, callback: impl FnOnce(Vec<u8>) -> Self) -> Self {
        return match self {
            Self::Bytes(bytes) => callback(bytes),
            _ => self
        };
    }
}

impl IntoResponse for FileContents {
    fn into_response(self) -> Response {
        return match self {
            Self::Invalid(err) => (StatusCode::NOT_FOUND, err).into_response(),
            Self::HTML(html) => {
                Response::builder()
                    .header(CONTENT_TYPE, "text/html")
                    .body(Body::from(html))
                    .unwrap()
            },
            Self::CSS(css) => {
                Response::builder()
                    .header(CONTENT_TYPE, "text/css")
                    .body(Body::from(css))
                    .unwrap()
            },
            Self::JS(js) => {
                Response::builder()
                    .header(CONTENT_TYPE, "text/javascript")
                    .body(Body::from(js))
                    .unwrap()
            },
            Self::JSON(json) => {
                Response::builder()
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(json))
                    .unwrap()
            },
            Self::Plain(plaintext) => {
                Response::builder()
                    .header(CONTENT_TYPE, "text/plain")
                    .body(Body::from(plaintext))
                    .unwrap()
            },
            Self::PNG(png) => {
                Response::builder()
                    .header(CONTENT_TYPE, "image/png")
                    .body(Body::from(png))
                    .unwrap()
            },
            Self::ICO(ico) => {
                Response::builder()
                    .header(CONTENT_TYPE, "image/vnd.microsoft.icon")
                    .body(Body::from(ico))
                    .unwrap()
            },
            Self::JPEG(jpeg) => {
                Response::builder()
                    .header(CONTENT_TYPE, "image/jpeg")
                    .body(Body::from(jpeg))
                    .unwrap()
            },
            Self::GIF(gif) => {
                Response::builder()
                    .header(CONTENT_TYPE, "image/gif")
                    .body(Body::from(gif))
                    .unwrap()
            },
            Self::MP4(mp4) => {
                Response::builder()
                    .header(CONTENT_TYPE, "video/mp4")
                    .body(Body::from(mp4))
                    .unwrap()
            },
            Self::MP3(mp3) => {
                Response::builder()
                    .header(CONTENT_TYPE, "audio/mp3")
                    .body(Body::from(mp3))
                    .unwrap()
            },
            Self::OGG(ogg) => {
                Response::builder()
                    .header(CONTENT_TYPE, "audio/ogg")
                    .body(Body::from(ogg))
                    .unwrap()
            },
            _ => (StatusCode::BAD_REQUEST, "Unknown file type!").into_response(),
        }
    }
}
