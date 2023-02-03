use std::{
    fs::{self, File},
    io::{Result as IoResult, Write},
    path::PathBuf,
};

use rand::{distributions::Alphanumeric, Rng};
use regex::Regex;

use crate::{DEFAULT_EXTENSION, DEFAULT_MIME_TYPE};

#[derive(Debug, Default)]
pub struct Paste {
    /// data to store
    pub data: Vec<u8>,
}

impl Paste {
    /// returns data `mime_type` and `extension` as tuple
    pub fn mime_type(data: &[u8]) -> (&'static str, &'static str) {
        match infer::get(data) {
            Some(kind) => {
                let mime_type = kind.mime_type();
                let mime_type = match mime_type {
                    "text/plain" => DEFAULT_MIME_TYPE,
                    _ => mime_type,
                };
                (mime_type, kind.extension())
            }
            None => (DEFAULT_MIME_TYPE, DEFAULT_EXTENSION),
        }
    }

    /// returns 3 length alpha-numeric randomized string
    pub fn random_file_name(base: Option<&str>) -> String {
        loop {
            let file_name = rand::thread_rng()
                .sample_iter(Alphanumeric)
                .take(3)
                .map(char::from)
                .collect::<String>();

            let path = if let Some(base) = base {
                PathBuf::from(format!("{}/{}", base, &file_name))
            } else {
                PathBuf::from(&file_name)
            };

            if !path.exists() {
                return String::from(path.to_str().unwrap());
            }
        }
    }

    // TODO: stdin for '-'
    pub fn save_to(&self, path: &mut PathBuf) -> IoResult<PathBuf> {
        if path.is_dir() && !path.exists() {
            fs::create_dir_all(&path)?;
            log::trace!("{:?} directory created", path);
        }
        path.push(Paste::random_file_name(path.to_str()));
        let (mime_type, _) = Paste::mime_type(&self.data); // text/plain, txt
        let re = Regex::new(r"^text").unwrap();
        if !re.is_match(mime_type) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "file type not permitted",
            ));
        }
        let mut buf = File::create(&path)?;
        buf.write_all(&self.data)?;
        log::trace!("{:?} create and wrote file successfully", path);
        Ok(path.to_path_buf())
    }

    pub fn append(chunk: &mut Vec<u8>, path: &PathBuf) -> IoResult<usize> {
        let (mime_type, _) = Paste::mime_type(chunk);
        let re = Regex::new(r"^text").unwrap();
        if !re.is_match(mime_type) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "file type not permitted",
            ));
        }

        let mut buf = File::options().create(true).append(true).open(path)?;
        let size = chunk.len();
        buf.write_all(chunk)?;
        Ok(size)
    }

    pub fn from(path: &PathBuf) -> IoResult<Self> {
        let data = fs::read(path)?;
        Ok(Paste { data })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mime_type() {
        let data = "hello world".as_bytes().to_vec();
        let paste = Paste { data };
        let (mime_type, extension) = Paste::mime_type(&paste.data);
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
        let paste = Paste { data };
        let (mime_type, extension) = Paste::mime_type(&paste.data);
        assert_eq!(mime_type, "text/html");
        assert_eq!(extension, "html");
    }

    #[test]
    fn test_random_file_name() {
        let s = Paste::random_file_name(Some("upload"));
        assert_eq!(s.len(), 3);
    }

    #[test]
    fn test_save_as() -> IoResult<()> {
        let data = "hello world".as_bytes().to_vec();
        let paste = Paste { data };
        let filename = paste.save_to(&mut PathBuf::from("./upload"))?;

        // cleanup
        fs::remove_file(filename)?;

        Ok(())
    }
}
