pub mod algorithm;
use algorithm::generator::generate_bmp;

#[allow(non_camel_case_types)]
#[swift_bridge::bridge]
mod ffi {
  extern "Rust" {
    #[swift_bridge(swift_name = "generateBMP")]
    fn generate_bmp(width: usize, height: usize) -> Vec<u8>;
  }
}
