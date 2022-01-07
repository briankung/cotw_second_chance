use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

pub fn twir_editions() -> impl Iterator<Item = PathBuf> {
    let dirs = fs::read_dir("this-week-in-rust/content/").expect("the directory");
    let paths = dirs.filter_map(Result::ok).map(|e| e.path());
    paths.filter_map(|path| {
        let file_name = path.file_name().and_then(OsStr::to_str);
        let extension = path.extension().and_then(OsStr::to_str);
        let mut result = None;

        if let (Some(fname), Some(ext)) = (file_name, extension) {
            if fname.contains("this-week") && (ext == "md" || ext == "markdown") {
                result = Some(path);
            };
        };

        result
    })
}
