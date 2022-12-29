pub mod models;

use crate::models::PartialRangeIter;
use reqwest::{
    header::{CONTENT_LENGTH, RANGE},
    StatusCode,
};
use std::{fs::File, io::Read, str::FromStr};

fn main() {
    println!("Starting");

    get_something().unwrap();

    println!("Done");
}

fn get_something() -> Result<(), reqwest::Error> {
    let url = "https://dados.cvm.gov.br/dados/FI/DOC/INF_DIARIO/DADOS/inf_diario_fi_202212.zip";

    let client = reqwest::blocking::Client::new();
    let head_response = client.head(url).send().unwrap();

    let length = head_response
        .headers()
        .get(CONTENT_LENGTH)
        .ok_or("Response doesn't include the content length")
        .unwrap();

    let length = u64::from_str(length.to_str().unwrap())
        .map_err(|_| "invalid Content-Length header")
        .unwrap();

    println!("Lenght is: {}", length);

    let mut output_file = File::create("info_cad_fi.zip").unwrap();

    println!("Starting download...");

    const CHUNK_SIZE: u32 = 500000;

    for range in PartialRangeIter::new(0, length - 1, CHUNK_SIZE).unwrap() {
        println!("range {:?}", range);
        let mut response = client.get(url).header(RANGE, range).send().unwrap();

        let status = response.status();
        if !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT) {
            return Ok(());
        }

        let mut response_bytes: Vec<u8> = Vec::new();
        let bytes_size = response.read_to_end(&mut response_bytes).unwrap();

        println!("Bytes size is: {:?}", bytes_size);

        std::io::copy(&mut response_bytes.as_slice(), &mut output_file).unwrap();
    }

    Ok(())
}
