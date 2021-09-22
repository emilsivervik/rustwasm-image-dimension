use image::{GenericImageView};
use worker::*;
use serde_json::json;
use lazy_static::lazy_static;
use regex::Regex;
use cfg_if::cfg_if;

lazy_static! {
    static ref IMAGE_REGEX: Regex = Regex::new("image/").unwrap();
}

cfg_if! {
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

#[event(fetch)]
pub async fn main(mut req: Request, _env: Env) -> Result<Response> {
    set_panic_hook();
    console_log!(
        "{} {}, located at: {:?}, within: {}",
        req.method().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );

    if !matches!(req.method(), Method::Post) {
        return Response::error("Method Not Allowed", 405);
    }

    if let Ok(content_type) = req.headers().get("Content-Type") {
        if let Some(content_type) = content_type {
            if !IMAGE_REGEX.is_match(&content_type) {
                return Response::error("Media Not Allowed", 415);
            }
        } else {
            return Response::error("Bad Request", 400)
        }
    } else { 
        return Response::error("Bad Request", 400)
    }

    match req.bytes().await {
        Ok(buffer) => {
            match image::load_from_memory(&buffer) {
                Ok(img) => {
                    let (x, y) = img.dimensions();
                    let val = json!({
                        "x": x, 
                        "y": y
                    });
                    Response::from_json(&val)
                }
                Err(err) => {
                    Response::error("Bad Request", 400)
                }
            }
        },
        Err(err) => {
            Response::error("Bad Request", 400)
        }
    }
}