use std::fmt::Display;

use regex::Regex;

use crate::{server::get_log_level, utils::logger::Logger};

const CONTENT_TYPES: &[(u16, &str)] = &[
    (0, "text/html; charset=utf-8"),
    (1, "text/plain; charset=utf-8"),
    (2, "text/css; charset=utf-8"),
    (3, "text/javascript; charset=utf-8"),
    (4, "image/jpeg"),
    (5, "image/png"),
    (6, "image/gif"),
    (7, "image/svg+xml"),
    (8, "image/webp"),
    (9, "image/bmp"),
    (10, "image/tiff"),
    (11, "application/json"),
    (12, "application/xml"),
    (13, "application/pdf"),
    (14, "application/zip"),
    (15, "application/octet-stream"),
    (16, "application/x-abiword"),
    (17, "image/apng"),
    (18, "application/x-freearc"),
    (19, "video/x-msvideo"),
    (20, "application/vnd.amazon.ebook"),
    (21, "application/octet-stream"),
    (22, "image/bmp"),
    (23, "application/x-bzip"),
    (24, "application/x-bzip2"),
    (25, "application/x-cdf"),
    (26, "application/x-csh"),
    (
        27,
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    ),
    (28, "application/vnd.ms-fontobject"),
    (29, "application/epub+zip"),
    (30, "application/gzip"),
    (31, "application/vnd.microsoft.icon"),
    (32, "application/java-archive"),
    (33, "application/json"),
    (34, "application/ld+json"),
    (35, "audio/midi"),
    (36, "audio/x-midi"),
    (37, "text/javascript"),
    (38, "audio/mpeg"),
    (39, "video/mp4"),
    (40, "video/mpeg"),
    (41, "application/vnd.apple.installer+xml"),
    (42, "application/vnd.oasis.opendocument.presentation"),
    (43, "application/vnd.oasis.opendocument.spreadsheet"),
    (44, "application/vnd.oasis.opendocument.text"),
    (45, "audio/ogg"),
    (46, "font/otf"),
    (47, "application/ogg"),
    (48, "audio/ogg"),
    (49, "application/x-httpd-php"),
    (50, "application/vnd.ms-powerpoint"),
    (
        51,
        "application/vnd.openxmlformats-officedocument.presentationml.presentation",
    ),
    (52, "application/vnd.rar"),
    (53, "application/rtf"),
    (54, "application/x-sh"),
    (55, "image/svg+xml"),
    (56, "application/x-tar"),
    (57, "image/tiff"),
    (58, "video/mp2t"),
    (59, "application/x-sh"),
    (60, "font/ttf"),
    (61, "text/plain"),
    (62, "application/vnd.visio"),
    (63, "audio/wav"),
    (64, "audio/webm"),
    (65, "video/webm"),
    (66, "image/webp"),
    (67, "font/woff"),
    (68, "font/woff2"),
    (69, "application/xhtml+xml"),
    (70, "application/vnd.ms-excel"),
    (
        71,
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    ),
    (72, "application/xml"),
    (73, "application/vnd.mozilla.xul+xml"),
    (74, "application/zip"),
    (75, "video/3gpp"),
    (76, "audio/3gpp"),
    (77, "video/3gpp2"),
    (78, "audio/3gpp2"),
    (79, "application/x-7z-compressed"),
];

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[allow(unused)]
/// HTTP content types
/// # Example
/// ``` rust
/// use http::http_content_types::ContentType;
/// let content_type = ContentType::TextHtmlCharsetUtf8;
/// assert_eq!(content_type, ContentType::TextHtmlCharsetUtf8);
/// ```
pub enum ContentType {
    TextHtmlCharsetUtf8 = 0,
    TextPlainCharsetUtf8 = 1,
    TextCssCharsetUtf8 = 2,
    TextJavascriptCharsetUtf8 = 3,
    ImageJpeg = 4,
    ImagePng = 5,
    ImageGif = 6,
    ImageSvgXml = 7,
    ImageWebp = 8,
    ImageBmp = 9,
    ImageTiff = 10,
    ApplicationJson = 11,
    ApplicationXml = 12,
    ApplicationPdf = 13,
    ApplicationZip = 14,
    ApplicationOctetStream = 15,
    ApplicationXAbiword = 16,
    ImageApng = 17,
    ApplicationXFreearc = 18,
    VideoXMsvideo = 19,
    ApplicationVndAmazonEbook = 20,
    ApplicationOctetStreamDuplicate = 21,
    ImageBmpDuplicate = 22,
    ApplicationXBzip = 23,
    ApplicationXBzip2 = 24,
    ApplicationXCdf = 25,
    ApplicationXCsh = 26,
    ApplicationVndOpenxmlformatsOfficedocumentWordprocessingmlDocument = 27,
    ApplicationVndMsFontobject = 28,
    ApplicationEpubZip = 29,
    ApplicationGzip = 30,
    ApplicationVndMicrosoftIcon = 31,
    ApplicationJavaArchive = 32,
    ApplicationJsonDuplicate = 33,
    ApplicationLdJson = 34,
    AudioMidi = 35,
    AudioXMidi = 36,
    TextJavascriptDuplicate = 37,
    AudioMpeg = 38,
    VideoMp4 = 39,
    VideoMpeg = 40,
    ApplicationVndAppleInstallerXml = 41,
    ApplicationVndOasisOpendocumentPresentation = 42,
    ApplicationVndOasisOpendocumentSpreadsheet = 43,
    ApplicationVndOasisOpendocumentText = 44,
    AudioOgg = 45,
    FontOtf = 46,
    ApplicationOgg = 47,
    AudioOggDuplicate = 48,
    ApplicationXHttpdPhp = 49,
    ApplicationVndMsPowerpoint = 50,
    ApplicationVndOpenxmlformatsOfficedocumentPresentationmlPresentation = 51,
    ApplicationVndRar = 52,
    ApplicationRtf = 53,
    ApplicationXSh = 54,
    ImageSvgXmlDuplicate = 55,
    ApplicationXTar = 56,
    ImageTiffDuplicate = 57,
    VideoMp2t = 58,
    ApplicationXShDuplicate = 59,
    FontTtf = 60,
    TextPlainDuplicate = 61,
    ApplicationVndVisio = 62,
    AudioWav = 63,
    AudioWebm = 64,
    VideoWebm = 65,
    ImageWebpDuplicate = 66,
    FontWoff = 67,
    FontWoff2 = 68,
    ApplicationXhtmlXml = 69,
    ApplicationVndMsExcel = 70,
    ApplicationVndOpenxmlformatsOfficedocumentSpreadsheetmlSheet = 71,
    ApplicationXmlDuplicate = 72,
    ApplicationVndMozillaXulXml = 73,
    ApplicationZipDuplicate = 74,
    Video3gpp = 75,
    Audio3gpp = 76,
    Video3gpp2 = 77,
    Audio3gpp2 = 78,
    ApplicationX7zCompressed = 79,
}

/// Get the content type from the code
/// # Arguments
/// * `value` - The content type code : ContentType
/// # Returns
/// * `String` - The content type : String
/// # Example
/// ``` rust
/// use http::http_content_types::get_type;
/// let content_type = get_type(ContentType::TextHtmlCharsetUtf8);
/// assert_eq!(content_type, "text/html; charset=utf-8");
/// ```
pub fn get_type(value: ContentType) -> String {
    CONTENT_TYPES
        .binary_search_by(|(code, _)| code.cmp(&(value as u16)))
        .map(|idx| CONTENT_TYPES[idx].1)
        .unwrap()
        .to_string()
}

impl ContentType {
    /// Parse the file name to get the content type
    /// # Arguments
    /// * `value` - The file name : &str
    /// # Returns
    /// * `ContentType` - The content type : ContentType
    pub fn parse_file_name(value: &str) -> ContentType {
        let logger: Logger = Logger {
            c_name: "http_content_types",
            level: get_log_level(),
        };
        logger.debug(&["parse_file_name", value]);
        let regex = Regex::new(r"(\..*)$").unwrap();
        let f_type = regex.captures(value).unwrap().get(1).unwrap().as_str();

        match f_type {
            ".html" => ContentType::TextHtmlCharsetUtf8,
            ".txt" => ContentType::TextPlainCharsetUtf8,
            ".css" => ContentType::TextCssCharsetUtf8,
            ".js" => ContentType::TextJavascriptCharsetUtf8,
            ".jpeg" | ".jpg" => ContentType::ImageJpeg,
            ".png" => ContentType::ImagePng,
            ".gif" => ContentType::ImageGif,
            ".svg" => ContentType::ImageSvgXml,
            ".webp" => ContentType::ImageWebp,
            ".bmp" => ContentType::ImageBmp,
            ".tiff" => ContentType::ImageTiff,
            ".json" => ContentType::ApplicationJson,
            ".xml" => ContentType::ApplicationXml,
            ".pdf" => ContentType::ApplicationPdf,
            ".zip" => ContentType::ApplicationZip,
            ".bin" => ContentType::ApplicationOctetStream,
            ".abw" => ContentType::ApplicationXAbiword,
            ".apng" => ContentType::ImageApng,
            ".arc" => ContentType::ApplicationXFreearc,
            ".avi" => ContentType::VideoXMsvideo,
            ".azw" => ContentType::ApplicationVndAmazonEbook,
            ".bz" => ContentType::ApplicationXBzip,
            ".bz2" => ContentType::ApplicationXBzip2,
            ".cda" => ContentType::ApplicationXCdf,
            ".csh" => ContentType::ApplicationXCsh,
            ".docx" => {
                ContentType::ApplicationVndOpenxmlformatsOfficedocumentWordprocessingmlDocument
            }
            ".eot" => ContentType::ApplicationVndMsFontobject,
            ".epub" => ContentType::ApplicationEpubZip,
            ".gz" => ContentType::ApplicationGzip,
            ".ico" => ContentType::ApplicationVndMicrosoftIcon,
            ".jar" => ContentType::ApplicationJavaArchive,
            ".midi" => ContentType::AudioMidi,
            ".mjs" => ContentType::TextJavascriptDuplicate,
            ".mp3" => ContentType::AudioMpeg,
            ".mp4" => ContentType::VideoMp4,
            ".mpeg" => ContentType::VideoMpeg,
            ".mpkg" => ContentType::ApplicationVndAppleInstallerXml,
            ".odp" => ContentType::ApplicationVndOasisOpendocumentPresentation,
            ".ods" => ContentType::ApplicationVndOasisOpendocumentSpreadsheet,
            ".odt" => ContentType::ApplicationVndOasisOpendocumentText,
            ".oga" => ContentType::AudioOgg,
            ".otf" => ContentType::FontOtf,
            ".ogx" => ContentType::ApplicationOgg,
            ".php" => ContentType::ApplicationXHttpdPhp,
            ".pptx" => {
                ContentType::ApplicationVndOpenxmlformatsOfficedocumentPresentationmlPresentation
            }
            ".rar" => ContentType::ApplicationVndRar,
            ".rtf" => ContentType::ApplicationRtf,
            ".sh" => ContentType::ApplicationXSh,
            ".tar" => ContentType::ApplicationXTar,
            ".tif" => ContentType::ImageTiffDuplicate,
            ".ts" => ContentType::VideoMp2t,
            ".ttf" => ContentType::FontTtf,
            ".wav" => ContentType::AudioWav,
            ".weba" => ContentType::AudioWebm,
            ".webm" => ContentType::VideoWebm,
            ".woff" => ContentType::FontWoff,
            ".woff2" => ContentType::FontWoff2,
            ".xhtml" => ContentType::ApplicationXhtmlXml,
            ".xls" => ContentType::ApplicationVndMsExcel,
            ".xlsx" => ContentType::ApplicationVndOpenxmlformatsOfficedocumentSpreadsheetmlSheet,
            ".xul" => ContentType::ApplicationVndMozillaXulXml,
            ".3gp" => ContentType::Video3gpp,
            ".3g2" => ContentType::Video3gpp2,
            ".7z" => ContentType::ApplicationX7zCompressed,
            _ => ContentType::TextPlainCharsetUtf8,
        }
    }
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", get_type(*self))
    }
}
