use std::hash::{DefaultHasher, Hash, Hasher};

use enc::base_64::Base64Encoder;
use enc::hex::HexEncoder;
use enc::StringEncoder;
use file_storage::{FilePath, FolderPath};
use reqwest::Method;
use web_url::WebUrl;

use crate::Error;

/// Responsible for caching data from the web.
#[derive(Clone, Debug)]
pub struct WebCache {
    root: FolderPath,
    base_64_encoder: Base64Encoder,
}

impl From<FolderPath> for WebCache {
    fn from(root: FolderPath) -> Self {
        Self {
            root,
            base_64_encoder: Base64Encoder::new(b'-', b'_', None).unwrap(),
        }
    }
}

impl WebCache {
    //! Read

    /// Reads the optional cached data.
    pub fn read(&self, method: Method, url: &WebUrl) -> Result<Option<Vec<u8>>, Error> {
        let file: FilePath = self.file(method, url)?;
        Ok(file.read_as_vec_if_exists()?)
    }
}

impl WebCache {
    //! Write

    /// Overwrites the cached `data`.
    pub fn write(&self, method: Method, url: &WebUrl, data: &[u8]) -> Result<(), Error> {
        let file: FilePath = self.file(method, url)?;
        file.delete()?;
        Ok(file.write_data_if_not_exists(data).map(|_| ())?)
    }
}

impl WebCache {
    //! Clear

    /// Clears the cached data.
    pub fn clear(&self, method: Method, url: &WebUrl) -> Result<(), Error> {
        let file: FilePath = self.file(method, url)?;
        Ok(file.delete()?)
    }
}

impl WebCache {
    //! Files

    /// Gets the folder char for the cache `key`. (a single lowercase hex char)
    fn folder_char(&self, key: &str) -> char {
        let mut hasher: DefaultHasher = DefaultHasher::default();
        key.hash(&mut hasher);
        let hash: u64 = hasher.finish();
        let hash: u64 = (hash >> 32) ^ hash;
        let hash: u64 = (hash >> 16) ^ hash;
        let hash: u64 = (hash >> 8) ^ hash;
        let hash: u64 = (hash >> 4) ^ hash;
        let hash: u8 = hash as u8;
        let (_, hex) = HexEncoder::LOWER.encode_chars(hash);
        hex
    }

    /// Gets the cache file for the `method` and `url`.
    fn file(&self, method: Method, url: &WebUrl) -> Result<FilePath, Error> {
        let key: String = format!("{} {}", method, url);
        let folder_char: char = self.folder_char(key.as_str());
        let base_64: String = self
            .base_64_encoder
            .encode_as_string(key.as_bytes())
            .map_err(|e| Error::Other(format!("error encoding URL: {}", e)))?;
        let extension: &str = ".web-cache";
        let extra: usize = folder_char.len_utf8()
            + self.root.path().file_separator().len_utf8()
            + base_64.len()
            + extension.len();
        self.root
            .clone_with_extra_capacity(extra)
            .with_appended_char(folder_char)
            .make_folder()
            .with_appended(base_64.as_str())
            .make_file(extension)
            .ok_or_else(|| Error::Other("the letter 'e' is a file-separator".to_string()))
    }
}
