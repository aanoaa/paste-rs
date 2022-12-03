use std::{
    fs::{self, File},
    io::{Result as IoResult, Write},
    path::PathBuf,
};

use rand::{distributions::Alphanumeric, Rng};

use crate::config::Config;

#[derive(Debug, Default)]
pub struct Paste {
    /// data to store
    pub data: Vec<u8>,

    /// paste-rs app configuration
    pub config: Config,
}

impl Paste {
    /// returns data `mime_type` and `extension` as tuple
    pub fn mime_type(&self) -> (&str, &str) {
        const DEFAULT_MIME_TYPE: &str = "text/plain";
        const DEFAULT_EXTENSION: &str = "txt";
        match infer::get(&self.data) {
            Some(kind) => (kind.mime_type(), kind.extension()),
            None => (DEFAULT_MIME_TYPE, DEFAULT_EXTENSION),
        }
    }

    /// returns 3 length alpha-numeric randomized string
    pub fn random_file_name(&self) -> String {
        rand::thread_rng()
            .sample_iter(Alphanumeric)
            .take(3)
            .map(char::from)
            .collect::<String>()
    }

    // TODO: stdin for '-'
    pub fn save_as(&self) -> IoResult<String> {
        let mut path = PathBuf::from(&self.config.upload_path);
        if path.is_dir() && !path.exists() {
            fs::create_dir_all(&path)?;
            log::trace!("{:?} directory created", path);
        }
        path.push(self.random_file_name());
        path.set_extension(self.mime_type().1);
        let mut buf = File::create(&path)?;
        buf.write_all(&self.data)?;
        log::trace!("{:?} create and wrote file successfully", path);
        Ok(String::from(path.as_os_str().to_str().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mime_type() {
        let data = "hello world".as_bytes().to_vec();
        let config = Config::default();
        let paste = Paste {
            data,
            config: config.clone(),
        };
        let (mime_type, extension) = paste.mime_type();
        assert_eq!(mime_type, "text/plain");
        assert_eq!(extension, "txt");

        let data = r#"<!DOCTYPE html>
<html lang="ko">
<head>
<meta charset="UTF-8"/>
<title>title</title>
<body>hello world</body>
</html>"#
            .as_bytes()
            .to_vec();
        let paste = Paste { data, config };
        let (mime_type, extension) = paste.mime_type();
        assert_eq!(mime_type, "text/html");
        assert_eq!(extension, "html");
    }

    #[test]
    fn test_random_file_name() {
        let paste = Paste::default();
        let s = paste.random_file_name();
        assert_eq!(s.len(), 3);
    }

    #[test]
    fn test_save_as() -> IoResult<()> {
        let data = "hello world".as_bytes().to_vec();
        let config = Config::from(&PathBuf::from("config.toml")).expect("config read fail");
        let paste = Paste { data, config };
        let file_name = paste.save_as()?;

        // cleanup
        fs::remove_file(PathBuf::from(file_name))?;

        Ok(())
    }
}
