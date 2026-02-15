use std::fs;
use std::io::Read;
use url::Url;

#[derive(Debug)]
pub enum FetchError {
    HTTP,
    IO,
    InvalidScheme,
}

pub fn fetch(url: &Url) -> Result<Vec<u8>, FetchError> {
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
            return Ok(data);
        }
        "file" => {
            let path = url.to_file_path().map_err(|_| FetchError::IO)?;
            let data = fs::read(&path).map_err(|_| FetchError::IO)?;
            return Ok(data);
        }
        _ => {
            return Err(FetchError::InvalidScheme);
        }
    };
}
