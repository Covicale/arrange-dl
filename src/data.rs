use std::collections::HashSet;

let DOCUMENTOS = HashSet::from(
  [".doc", ".docx", ".pdf", ".xls", ".xlsx", ".ppt", ".pptx", ".txt", ".rtf", ".odt", ".pages", ".wpd", ".md"]
);

static MEDIA: HashSet<&str> = HashSet::from(
  [".jpg", ".jpeg", ".png", ".gif", ".bmp", ".tiff", ".ico", ".svg", ".heif", ".webp", ".mp4", ".mkv", ".mov", ".avi", ".wmv", ".flv", ".m4v", ".webm", ".3gp", ".ogg", ".mpg", ".mpeg"]
);

static COMPRESS: HashSet<&str> = HashSet::from(
  [".zip", ".rar", ".7z", ".tar", ".gz", ".bz2", ".tgz", ".tbz", ".z", ".cab", ".lzh", ".lzma", ".xz", ".sit", ".dmg", ".wim", ".esd", ".rpm", ".deb"]
);