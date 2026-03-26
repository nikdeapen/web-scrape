use crate::Error;
use clerr::{Code, Report};
use enc::base_64::Base64Encoder;
use enc::hex::HexEncoder;
use enc::StringEncoder;
use file_storage::{FilePath, FolderPath};
use web_url::WebUrl;

/// Responsible for caching web data.
#[derive(Clone, Debug)]
pub struct WebCache {
    local: Option<FolderPath>,
    remote: Option<FolderPath>,
    encoder: Base64Encoder,
    extension: String,
}

impl WebCache {
    //! Construction

    /// Creates a new web cache.
    pub fn new(local: Option<FolderPath>, remote: Option<FolderPath>) -> Self {
        Self {
            local,
            remote,
            encoder: Base64Encoder::url_safe_encoder(),
            extension: ".web-cache".to_string(),
        }
    }
}

impl WebCache {
    //! Read

    /// Reads the optional cached data for the `url`.
    pub fn read(&self, url: &WebUrl) -> Result<Option<Vec<u8>>, Error> {
        if let Some(local) = &self.local {
            let file: FilePath = self.file_for_root(url, local)?;
            if let Some(data) = file.read_as_vec_if_exists()? {
                return Ok(Some(data));
            }
        }

        if let Some(remote) = &self.remote {
            let file: FilePath = self.file_for_root(url, remote)?;
            if let Some(data) = file.read_as_vec_if_exists()? {
                return Ok(Some(data));
            }
        }

        Ok(None)
    }
}

impl WebCache {
    //! Write

    /// Overwrites the cached `data` for the `url`.
    pub fn write(&self, url: &WebUrl, data: &[u8]) -> Result<(), Error> {
        if let Some(local) = &self.local {
            self.write_to_root(url, data, local)?;
        }
        if let Some(remote) = &self.remote {
            self.write_to_root(url, data, remote)?;
        }
        Ok(())
    }

    /// Writes the `data` from the `url` to the `root` folder.
    fn write_to_root(&self, url: &WebUrl, data: &[u8], root: &FolderPath) -> Result<(), Error> {
        let file: FilePath = self.file_for_root(url, root)?;
        file.delete()?;
        Ok(file.write_data_if_not_exists(data).map(|_| ())?)
    }
}

impl WebCache {
    //! Files

    /// Gets the file for the `url` given the `root` folder.
    fn file_for_root(&self, url: &WebUrl, root: &FolderPath) -> Result<FilePath, Error> {
        let folder_char: char = self.folder_char(url.as_str());
        let base_64: String = self
            .encoder
            .encode_as_string(url.as_str().as_bytes())
            .map_err(|e| {
                Error::Other(Report::new(Code::error(
                    "WEB_CACHE_BASE_64",
                    format!("error converting the URL to base-64: {} -- {}", url, e),
                )))
            })?;
        let extra: usize = folder_char.len_utf8()
            + root.path().file_separator().len_utf8()
            + base_64.len()
            + self.extension.len();
        root.clone_with_extra_capacity(extra)
            .with_appended_char(folder_char)
            .make_folder()
            .with_appended(base_64.as_str())
            .make_file(self.extension.as_str())
            .map_err(|path| {
                Error::Other(Report::new(Code::error(
                    "WEB_CACHE_INVALID_EXTENSION",
                    format!("the file extension makes the path a folder: {}", path),
                )))
            })
    }

    /// Gets the folder char for the cache `key`. (a single lowercase hex char)
    fn folder_char(&self, key: &str) -> char {
        let mut fold: u8 = 0;
        for byte in key.as_bytes() {
            fold ^= byte;
        }
        let (_, hex) = HexEncoder::LOWER.encode_chars(fold);
        hex
    }
}
