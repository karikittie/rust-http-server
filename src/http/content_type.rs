use std::path::Path;
use std::ffi::OsStr;

#[derive(Copy, Clone, Debug, Eq)]
pub enum ContentType {
    TextHtml,
    TextCss,
    TextJs,
    TextSvgXml,
    MultipartForm,
    ImagePng,
    ImageJpg,
    ImageBmp,
    ImageGif,
    ApplicationJson,
    ApplicationXml,
}

impl ContentType {
    /// This transforms the enum into the actual string that is represented in the
    /// HTTP response string.
    pub fn stringify(&self) -> String {
        match self {
            &ContentType::TextHtml => String::from("text/html"),
            &ContentType::TextCss => String::from("text/css"),
            &ContentType::TextJs => String::from("text/javascript"),
            &ContentType::TextSvgXml => String::from("text/svg+xml"),
            &ContentType::MultipartForm => String::from("multipart/form"),
            &ContentType::ImageJpg => String::from("image/jpg"),
            &ContentType::ImagePng => String::from("image/png"),
            &ContentType::ImageBmp => String::from("image/bmp"),
            &ContentType::ImageGif => String::from("image/gif"),
            &ContentType::ApplicationJson => String::from("application/json"),
            &ContentType::ApplicationXml => String::from("application/xml"),
        }
    }
}

impl PartialEq for ContentType {
        fn eq(&self, other: &ContentType) -> bool {
        self == other
    }
}

/// Returns the content type enum that is associated with a certain
/// filename by parsing out the extension.
pub fn get_content_type(filename: &String) -> ContentType {
    let ext = Path::new(filename)
        .extension()
        .and_then(OsStr::to_str);
    match ext {
        Some(ext) => get_file_type_from_extension(ext),
        None => ContentType::TextHtml
    }
}

// This function has all the mappings to the supported content types.
// New content types need to be added here as well to be supported.
fn get_file_type_from_extension(ext: &str) -> ContentType {
    match ext {
        "jpg" => ContentType::ImageJpg,
        "jpeg" => ContentType::ImageJpg,
        "png" => ContentType::ImagePng,
        "gif" => ContentType::ImageGif,
        "bmp" => ContentType::ImageBmp,
        "css" => ContentType::TextCss,
        "js" => ContentType::TextJs,
        "svg" => ContentType::TextSvgXml,
        "json" => ContentType::ApplicationJson,
        "xml" => ContentType::ApplicationXml,
        _ => ContentType::TextHtml,
    }
}
