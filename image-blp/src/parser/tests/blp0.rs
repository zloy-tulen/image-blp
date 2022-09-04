use super::super::*;
use test_log::test;

#[test]
fn blp0_test() {
    let blp_bytes = include_bytes!("../../../../assets/blp0/WyvernRider.blp");
    let blp_mipmaps = vec![
        include_bytes!("../../../../assets/blp0/WyvernRider.b00").to_vec(),
        include_bytes!("../../../../assets/blp0/WyvernRider.b01").to_vec(),
        include_bytes!("../../../../assets/blp0/WyvernRider.b02").to_vec(),
        include_bytes!("../../../../assets/blp0/WyvernRider.b03").to_vec(),
        include_bytes!("../../../../assets/blp0/WyvernRider.b04").to_vec(),
        include_bytes!("../../../../assets/blp0/WyvernRider.b05").to_vec(),
        include_bytes!("../../../../assets/blp0/WyvernRider.b06").to_vec(),
        include_bytes!("../../../../assets/blp0/WyvernRider.b07").to_vec(),
        include_bytes!("../../../../assets/blp0/WyvernRider.b08").to_vec(),
        include_bytes!("../../../../assets/blp0/WyvernRider.b09").to_vec(),
    ];
    let (_, parsed) = parse_blp_with_externals(blp_bytes, |i| {
        if (i as usize) < blp_mipmaps.len() {
            Ok(Some(&blp_mipmaps[i as usize]))
        } else {
            Ok(None)
        }
    })
    .expect("successfull parsing");
    let header = BlpHeader {
        version: BlpVersion::Blp0,
        content: BlpContentTag::Jpeg,
        flags: BlpFlags::Old {
            alpha_bits: 8,
            extra: 4,
            has_mipmaps: 1,
        },
        width: 512,
        height: 256,
        mipmap_locator: MipmapLocator::External,
    };
    assert_eq!(parsed.header, header);
    assert_eq!(parsed.get_content_jpeg().expect("jpeg").images.len(), 10);
}