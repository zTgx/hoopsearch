use actix_web::{web, HttpResponse, Result};
use crate::http::response::ResponseBody;
use crate::http::error::ResponseError;
use crate::http::data::Data;
use serde::{Deserialize, Serialize};

extern crate image;
extern crate img_hash;
use img_hash::{HasherConfig, HashAlg};
use image::{ImageFormat, GenericImageView};
use std::io::Write;

extern crate curl;

use curl::easy::{Easy2, Handler, WriteError};

#[derive(Debug, Deserialize, Serialize)]
pub struct Input {
    image_path: String,
}

pub async fn search(_data: web::Data<Data>, body: web::Json<Input>) -> Result<HttpResponse, ResponseError> {
    let image_path = body.into_inner().image_path;
    println!("image path: {}", image_path);

    let url = image_path;

    struct Collector(Vec<u8>);
    impl Handler for Collector {
        fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
            self.0.extend_from_slice(data);
            Ok(data.len())
        }
    }
    
    let mut easy = Easy2::new(Collector(Vec::new()));
    easy.get(true).unwrap();
    easy.url(&url).unwrap();
    easy.perform().unwrap();
    
    // assert_eq!(easy.response_code().unwrap(), 200);
    let contents = easy.get_ref().0.as_slice();
    println!("{:?}", contents);

    let image = image::load_from_memory(contents).unwrap();
    println!("image: {:?}", image.dimensions());

    let hasher = HasherConfig::new().to_hasher();
    let hash   = hasher.hash_image(&image);    
    println!("Image hash: {}", hash.to_base64());
    
    // println!("Hamming Distance: {}", hash1.dist(&hash2));

    Err(ResponseError::bad_request("没有搜索到相关的图片"))
}