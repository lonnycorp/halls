use std::fs;
use std::io::Read;
use url::Url;

pub struct FetchedData {
    data: Vec<u8>,
}

impl FetchedData {
    pub fn new(url: &Url) -> Result<Self, FetchError> {
        match url.scheme() {
            "http" | "https" => {
                let response = ureq::get(url.as_str())
                    .call()
                    .map_err(|_| FetchError::HTTP)?;
                let mut data = Vec::new();
                response
                    .into_reader()
                    .read_to_end(&mut data)
                    .map_err(|_| FetchError::IO)?;
                return Ok(Self { data });
            }
            "file" => {
                let path = url.to_file_path().map_err(|_| FetchError::InvalidURL)?;
                let data = fs::read(&path).map_err(|_| FetchError::IO)?;
                return Ok(Self { data });
            }
            _ => return Err(FetchError::InvalidURL),
        }
    }

    pub fn data(&self) -> &[u8] {
        return &self.data;
    }
}

#[derive(Debug, Clone)]
pub enum FetchError {
    IO,
    HTTP,
    InvalidURL,
}
