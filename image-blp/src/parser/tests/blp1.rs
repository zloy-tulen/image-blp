use super::super::*;
use test_log::test;

#[test]
fn blp1_simplest_direct_alpha() {
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
        mipmap_locator: MipmapLocator::Internal {
            offsets: [1180; 16],
            sizes: [8; 16],
        },
    };
    assert_eq!(parsed.header, header);
}

#[test]
fn blp1_simplest_direct_noalpha() {
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
        mipmap_locator: MipmapLocator::Internal {
            offsets: [1180; 16],
            sizes: [4; 16],
        },
    };
    assert_eq!(parsed.header, header);
}

#[test]
fn blp1_simplest_blp() {
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
        mipmap_locator: MipmapLocator::Internal {
            offsets: [160; 16],
            sizes: [482; 16],
        },
    };
    assert_eq!(parsed.header, header);
}

#[test]
fn blp1_rect_direct() {
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
        mipmap_locator: MipmapLocator::Internal {
            offsets: [1180; 16],
            sizes: [12; 16],
        },
    };
    assert_eq!(parsed.header, header);
}

#[test]
fn blp1_rect_jpg_no_alpha() {
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
        mipmap_locator: MipmapLocator::Internal {
            offsets: [160; 16],
            sizes: [448; 16],
        },
    };
    assert_eq!(parsed.header, header);
}
