use super::super::*;
use test_log::test;

#[test]
fn simplest_direct_blp_alpha() {
    let blp_bytes = include_bytes!("../../../../assets/blp1/simple_with_alpha.blp");
    let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
    let header = BlpHeader {
        version: BlpVersion::Blp1,
        content: BlpContentTag::Direct,
        flags: BlpFlags::Old {
            alpha_bits: 8,
            extra: 3,
            has_mipmaps: 5,
        },
        width: 2,
        height: 2,
    };
    assert_eq!(parsed.header, header);
}

#[test]
fn simplest_direct_blp_noalpha() {
    let blp_bytes = include_bytes!("../../../../assets/blp1/simple_without_alpha.blp");
    let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
    let header = BlpHeader {
        version: BlpVersion::Blp1,
        content: BlpContentTag::Direct,
        flags: BlpFlags::Old {
            alpha_bits: 0,
            extra: 5,
            has_mipmaps: 5,
        },
        width: 2,
        height: 2,
    };
    assert_eq!(parsed.header, header);
}

#[test]
fn simplest_jpg_blp() {
    let blp_bytes = include_bytes!("../../../../assets/blp1/simple_jpg.blp");
    let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
    let header = BlpHeader {
        version: BlpVersion::Blp1,
        content: BlpContentTag::Jpeg,
        flags: BlpFlags::Old {
            alpha_bits: 8,
            extra: 3,
            has_mipmaps: 5,
        },
        width: 2,
        height: 2,
    };
    assert_eq!(parsed.header, header);
}

#[test]
fn rect_direct_blp() {
    let blp_bytes = include_bytes!("../../../../assets/blp1/rect_with_alpha.blp");
    let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
    let header = BlpHeader {
        version: BlpVersion::Blp1,
        content: BlpContentTag::Direct,
        flags: BlpFlags::Old {
            alpha_bits: 8,
            extra: 3,
            has_mipmaps: 5,
        },
        width: 2,
        height: 3,
    };
    assert_eq!(parsed.header, header);
}

#[test]
fn rect_jpg_no_alpha_blp() {
    let blp_bytes = include_bytes!("../../../../assets/blp1/rect_without_alpha.blp");
    let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
    let header = BlpHeader {
        version: BlpVersion::Blp1,
        content: BlpContentTag::Jpeg,
        flags: BlpFlags::Old {
            alpha_bits: 0,
            extra: 5,
            has_mipmaps: 5,
        },
        width: 2,
        height: 3,
    };
    assert_eq!(parsed.header, header);
}
