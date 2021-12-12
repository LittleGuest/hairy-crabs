//! 媒体类型工具类

pub const IMAGE_PNG: &str = "image/png";

pub const IMAGE_JPG: &str = "image/jpg";

pub const IMAGE_JPEG: &str = "image/jpeg";

pub const IMAGE_BMP: &str = "image/bmp";

pub const IMAGE_GIF: &str = "image/gif";

pub const IMAGE_EXTENSION: [&str] = ["bmp", "gif", "jpg", "jpeg", "png"];

pub const FLASH_EXTENSION: [&str] = ["swf", "flv"];

pub const MEDIA_EXTENSION: [&str] = [
    "swf", "flv", "mp3", "wav", "wma", "wmv", "mid", "avi", "mpg", "asf", "rm", "rmvb",
];

pub const VIDEO_EXTENSION: [&str] = ["mp4", "avi", "rmvb"];

pub const DEFAULT_ALLOWED_EXTENSION: [&str] = [
    // 图片
    "bmp", "gif", "jpg", "jpeg", "png", // word excel powerpoint
    "doc", "docx", "xls", "xlsx", "ppt", "pptx", "html", "htm", "txt", // 压缩文件
    "rar", "zip", "gz", "bz2", // 视频格式
    "mp4", "avi", "rmvb", // pdf
    "pdf",
];

/// 获取图片扩展名
pub const fn extension(prefix: &str) -> &str {
    match prefix {
        IMAGE_PNG => "png",
        IMAGE_JPG => "jpg",
        IMAGE_JPEG => "jpeg",
        IMAGE_BMP => "bmp",
        IMAGE_GIF => "gif",
        _ => "",
    }
}
