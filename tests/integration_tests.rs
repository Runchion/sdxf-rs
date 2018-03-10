extern crate sdxf;

use sdxf::Sdxf;
use sdxf::Value;

#[test]
fn init() {
    // These bytes define a structure with a single chunk with ID 0,
    // containing just the numerical value 123456789
    let bytes: &[u8] = &[0, 0, 96, 0, 0, 4, 7, 91, 205, 21];
    let mut sdxf = Sdxf::new();
    
    let result = sdxf.enter();
}