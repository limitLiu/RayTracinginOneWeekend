pub mod algorithm;
use algorithm::generator::{generate_bmp, generate_raw_data};

#[allow(non_camel_case_types)]
#[swift_bridge::bridge]
mod ffi {
  extern "Rust" {
    #[swift_bridge(swift_name = "generateBMP")]
    fn generate_bmp(width: usize, height: usize) -> Vec<u8>;

    #[swift_bridge(swift_name = "generateRawData")]
    fn generate_raw_data(width: usize, height: usize) -> Vec<u8>;
  }
}
