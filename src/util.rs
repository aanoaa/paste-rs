use std::{
    fs,
    io::Result as IoResult,
    path::{Path, PathBuf},
    time::SystemTime,
};

pub fn expired_files(path: &Path, ttl: u64) -> IoResult<Vec<PathBuf>> {
    let now = SystemTime::now();
    let mut expired: Vec<PathBuf> = Vec::new();
    match fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<IoResult<Vec<PathBuf>>>()
    {
        Ok(paths) => {
            for path in &paths {
                match now.duration_since(path.metadata()?.created()?) {
                    Ok(dur) => {
                        if dur.as_secs() > ttl {
                            log::debug!("{:?} duration_since created: {}s", &path, dur.as_secs());
                            expired.push(path.clone());
                        }
                    }
                    Err(e) => {
                        log::error!("{e}");
                    }
                }
            }
            Ok(expired)
        }
        Err(e) => {
            log::error!("read_dir {:?} fail: {e}", path);
            Err(e)
        }
    }
}
