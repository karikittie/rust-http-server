use std::path::Path;
use std::ffi::OsStr;

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum CONTENT_TYPE {
    TEXT_HTML,
    TEXT_CSS,
    TEXT_JS,
    TEXT_SVG_XML,
    MULTIPART_FORM,
    IMAGE_PNG,
    IMAGE_JPG,
    IMAGE_BMP,
    APPLICATION_JSON,
    APPLICATION_XML,
}

impl CONTENT_TYPE {
    /// This transforms the enum into the actual string that is represented in the
    /// HTTP response string.
    pub fn stringify(&self) -> String {
        match self {
            &CONTENT_TYPE::TEXT_HTML => String::from("text/html"),
            &CONTENT_TYPE::TEXT_CSS => String::from("text/css"),
            &CONTENT_TYPE::TEXT_JS => String::from("text/javascript"),
            &CONTENT_TYPE::TEXT_SVG_XML => String::from("text/svg+xml"),
            &CONTENT_TYPE::MULTIPART_FORM => String::from("multipart/form"),
            &CONTENT_TYPE::IMAGE_JPG => String::from("image/jpg"),
            &CONTENT_TYPE::IMAGE_PNG => String::from("image/png"),
            &CONTENT_TYPE::IMAGE_BMP => String::from("image/bmp"),
            &CONTENT_TYPE::APPLICATION_JSON => String::from("application/json"),
            &CONTENT_TYPE::APPLICATION_XML => String::from("application/xml"),
        }
    }
}

/// Returns the content type enum that is associated with a certain
/// filename by parsing out the extension.
pub fn get_content_type(filename: &String) -> CONTENT_TYPE {
    let ext = Path::new(filename)
        .extension()
        .and_then(OsStr::to_str);
    match ext {
        Some(ext) => get_file_type_from_extension(ext),
        None => CONTENT_TYPE::TEXT_HTML
    }
}

// This function has all the mappings to the supported content types.
// New content types need to be added here as well to be supported.
fn get_file_type_from_extension(ext: &str) -> CONTENT_TYPE {
    match ext {
        "jpg" => CONTENT_TYPE::IMAGE_JPG,
        "jpeg" => CONTENT_TYPE::IMAGE_JPG,
        "png" => CONTENT_TYPE::IMAGE_PNG,
        "bmp" => CONTENT_TYPE::IMAGE_BMP,
        "css" => CONTENT_TYPE::TEXT_CSS,
        "js" => CONTENT_TYPE::TEXT_JS,
        "svg" => CONTENT_TYPE::TEXT_SVG_XML,
        "json" => CONTENT_TYPE::APPLICATION_JSON,
        "xml" => CONTENT_TYPE::APPLICATION_XML,
        _ => CONTENT_TYPE::TEXT_HTML,
    }
}
