use bytes::Bytes;
use reqwest::StatusCode;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum CustomError {
    HttpRequest(u16),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::HttpRequest(errcode) => write!(f, "Http request error with code {errcode}"),
        }
    }
}

impl Error for CustomError {}

#[derive(Debug)]
struct BinaryData {
    data: Bytes,
    last_bytes_index: u16,
}

impl Iterator for BinaryData {
    type Item = Bytes;

    fn next(&mut self) -> Option<Self::Item> {
        let until = if self.last_bytes_index + 5 < self.data.len() as u16 {
            self.last_bytes_index + 5
        } else {
            self.last_bytes_index + (self.data.len() as u16 - self.last_bytes_index)
        };

        if until >= self.data.len() as u16 {
            self.last_bytes_index = self.data.len() as u16;
            return None;
        }

        let data = self
            .data
            .slice(self.last_bytes_index as usize..until as usize);
        self.last_bytes_index = until;
        Some(data)
    }
}

fn download_binary(url: &String) -> Result<Bytes, Box<dyn Error>> {
    let body = reqwest::blocking::get(url)?;
    match body.status() {
        StatusCode::OK => {
            let result = body.bytes()?;
            Ok(result)
        }
        s => Err(Box::new(CustomError::HttpRequest(s.as_u16()))),
    }
}

fn main() {
    let url: String = "http://localhost:7777/bin/tes.txt".to_string();
    // let packet_size: u8 = 5;
    let result = download_binary(&url);
    let mut mydata = BinaryData {
        data: Bytes::new(),
        last_bytes_index: 0,
    };

    match result {
        Ok(data) => mydata.data = Bytes::from(data),
        Err(err) => {
            println!("Error: {err}");
            std::process::exit(1);
        }
    };

    for val in mydata {
        println!("{val:?}");
    }

    // println!("{:?}", data);
}