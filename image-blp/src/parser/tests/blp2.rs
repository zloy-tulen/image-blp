use super::super::*;
use test_log::test;

#[test]
fn blp2_loading_bar() {
    let blp_bytes = include_bytes!("../../../../assets/blp2/UI-LoadingBar-Spark.blp");
    let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
    let header = BlpHeader {
        version: BlpVersion::Blp2,
        content: BlpContentTag::Jpeg,
        flags: BlpFlags::Old {
            alpha_bits: 0,
            extra: 5,
            has_mipmaps: 5,
        },
        width: 2,
        height: 3,
        mipmap_locator: MipmapLocator::External,
    };
    assert_eq!(parsed.header, header);
}
