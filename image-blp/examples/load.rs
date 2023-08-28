use ::image::DynamicImage;
use image_blp::{
    convert::blp_to_image,
    parser::{load_blp, load_blp_from_buf},
};

fn main() {
    let blp_filename = std::env::args().nth(1).unwrap_or("test.blp".to_owned());
    let output_filename = std::env::args().nth(2).unwrap_or("output.png".to_owned());
    let blp_file = load_blp(blp_filename).expect("loaded blp");
    let image: DynamicImage = blp_to_image(&blp_file, 0).expect("converted");
    image.save(output_filename).expect("saved");
}

#[test]
fn test_from_buf() {
    let blp_filename = std::env::args().nth(1).unwrap_or("test.blp".to_owned());
    let output_filename = std::env::args().nth(2).unwrap_or("output.png".to_owned());
    // load bytes
    let blp_bytes = std::fs::read(blp_filename).expect("loaded blp");
    let blp_file = load_blp_from_buf(blp_bytes).expect("loaded blp");
    let image: DynamicImage = blp_to_image(&blp_file, 0).expect("converted");
    image.save(output_filename).expect("saved");
}
