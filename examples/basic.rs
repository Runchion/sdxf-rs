extern crate sdxf;

use sdxf::*;

fn main() {
    let sdxf = Sdxf::new();
    sdxf.create_structured(3301)
        .create(3302, "first chunk")
        .create(3303, "second chunk")
        .create_structured(3304)
        .create(3305, "chunk in a structure")
        .create(3306, "next chunk in a structure")
        .leave()
        .create(3307, "third chunk")
        .leave();
}