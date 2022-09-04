use super::super::*;
use test_log::test;

#[test]
fn blp2_attack() {
    let blp_bytes = include_bytes!("../../../../assets/blp2/Attack.blp");
    let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
    let header = BlpHeader {
        version: BlpVersion::Blp2,
        content: BlpContentTag::Direct,
        flags: BlpFlags::Blp2 {
            compression: Compression::Raw1,
            alpha_bits: 1,
            alpha_type: 8,
            has_mipmaps: 1,
        },
        width: 32,
        height: 32,
        mipmap_locator: MipmapLocator::Internal {
            offsets: [1172, 2324, 2612, 2684, 2702, 2707, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
            sizes: [1152, 288, 72, 18, 5, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        },
    };
    assert_eq!(parsed.header, header);
}

#[test]
fn blp2_ui_paid_character_customization() {
    let blp_bytes = include_bytes!("../../../../assets/blp2/UI-PAIDCHARACTERCUSTOMIZATION-BUTTON.BLP");
    let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
    let header = BlpHeader {
        version: BlpVersion::Blp2,
        content: BlpContentTag::Direct,
        flags: BlpFlags::Blp2 {
            compression: Compression::Raw3,
            alpha_bits: 8,
            alpha_type: 2,
            has_mipmaps: 1,
        },
        width: 128,
        height: 128,
        mipmap_locator: MipmapLocator::Internal {
            offsets: [1172, 66708, 83092, 87188, 88212, 88468, 88532, 88548, 0, 0, 0, 0, 0, 0, 0, 0], 
            sizes: [65536, 16384, 4096, 1024, 256, 64, 16, 4, 0, 0, 0, 0, 0, 0, 0, 0]
        },
    };
    assert_eq!(parsed.header, header);
}

#[test]
fn blp2_sun_glare() {
    let blp_bytes = include_bytes!("../../../../assets/blp2/SunGlare.blp");
    let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
    let header = BlpHeader {
        version: BlpVersion::Blp2,
        content: BlpContentTag::Direct,
        flags: BlpFlags::Blp2 {
            compression: Compression::Raw3,
            alpha_bits: 136,
            alpha_type: 2,
            has_mipmaps: 0,
        },
        width: 256,
        height: 256,
        mipmap_locator: MipmapLocator::Internal {
            offsets: [1172, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
            sizes: [262144, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        },
    };
    assert_eq!(parsed.header, header);
}

#[test]
fn blp2_oilslickenv_a() {
    let blp_bytes = include_bytes!("../../../../assets/blp2/oilslickenvA.blp");
    let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
    let header = BlpHeader {
        version: BlpVersion::Blp2,
        content: BlpContentTag::Direct,
        flags: BlpFlags::Blp2 {
            compression: Compression::Raw3,
            alpha_bits: 1,
            alpha_type: 2,
            has_mipmaps: 1,
        },
        width: 768,
        height: 128,
        mipmap_locator: MipmapLocator::Internal {
            offsets: [1172, 394388, 492692, 517268, 523412, 524948, 525332, 525428, 0, 0, 0, 0, 0, 0, 0, 0], 
            sizes: [393216, 98304, 24576, 6144, 1536, 384, 96, 24, 0, 0, 0, 0, 0, 0, 0, 0]
        },
    };
    assert_eq!(parsed.header, header);
}