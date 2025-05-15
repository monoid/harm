use std::io;
use std::path::{Path, PathBuf};

use crate::{
    AARCHMRS_2025_03_FILE, AARCHMRS_2025_03_MD5, AARCHMRS_2025_03_SIZE, AARCHMRS_2025_03_URL,
};
use md5::{Digest, Md5};

#[derive(thiserror::Error, Debug)]
pub enum DownloadError {
    #[error("HTTP error: {0}")]
    Http(#[from] ureq::Error),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

pub(crate) fn ensure_archive(cache_dir: &Path) -> Result<PathBuf, DownloadError> {
    let archive_file = cache_dir.join(AARCHMRS_2025_03_FILE);
    if !is_valid_archive(&archive_file) {
        println!("cargo::warning=Downloading an archive file...");
        download_archive(&archive_file)?;
        if !is_valid_archive(&archive_file) {
            return Err(
                io::Error::new(io::ErrorKind::Other, "Failed to download a valid data").into(),
            );
        }
    } else {
        println!("cargo::warning=The cached file is valid, using it...");
    }
    Ok(archive_file)
}

fn download_archive(archive_file: &Path) -> Result<(), DownloadError> {
    let _ = std::fs::remove_file(archive_file);
    let _ = std::fs::remove_dir(archive_file);

    let mut req = ureq::get(AARCHMRS_2025_03_URL).call()?;
    let mut body_reader = req.body_mut().as_reader();
    let mut out_file = std::fs::File::create(archive_file)?;

    std::io::copy(&mut body_reader, &mut out_file)?;
    Ok(())
}

fn is_valid_archive(archive_file: &Path) -> bool {
    if !std::fs::exists(archive_file).unwrap_or(false) {
        return false;
    }
    match std::fs::File::open(archive_file) {
        Ok(mut file) => {
            match file.metadata() {
                Ok(metadata) => {
                    if metadata.len() != AARCHMRS_2025_03_SIZE {
                        return false;
                    }
                    if !metadata.is_file() {
                        return false;
                    }
                }
                Err(_) => return false,
            }

            let mut hasher = Md5::new();

            if std::io::copy(&mut file, &mut hasher).is_err() {
                return false;
            };

            let md5 = hasher.finalize();
            md5[..] == AARCHMRS_2025_03_MD5
        }
        Err(_err) => false,
    }
}
