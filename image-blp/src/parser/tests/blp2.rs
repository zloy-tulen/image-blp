use super::super::*;
use crate::encode::encode_blp;
use test_log::test;

fn blp2_test(name: &str, blp_bytes: &[u8], header: &BlpHeader) {
    let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
    assert_eq!(&parsed.header, header);
    let expected_mipmaps = header
        .internal_mipmaps()
        .map(|(offsets, _)| offsets.iter().filter(|a| **a > 0).count())
        .unwrap_or(0);
    assert_eq!(
        parsed.get_image_count(),
        expected_mipmaps
    );
    let encoded = encode_blp(&parsed).expect("encoded blp");
    assert_eq!(encoded, blp_bytes);
    // Test File API
    {
        let dir = tempfile::tempdir().expect("temporary directory");
        let blp_name = format!("{}.blp", name);
        let blp_path = dir.path().join(Path::new(&blp_name));
        std::fs::write(&blp_path, blp_bytes).expect("write");

        let loaded = load_blp(&blp_path).expect("loaded");
        assert_eq!(loaded, parsed);
    }
}

#[test]
fn test_attack() {
    let blp_bytes = include_bytes!("../../../../assets/blp2/Attack.blp");
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
            offsets: [
                1172, 2324, 2612, 2684, 2702, 2707, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            sizes: [1152, 288, 72, 18, 5, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        },
    };
    blp2_test("Attack", blp_bytes, &header);
}

#[test]
fn test_ui_paid_character_customization() {
    let blp_bytes =
        include_bytes!("../../../../assets/blp2/UI-PAIDCHARACTERCUSTOMIZATION-BUTTON.BLP");
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
            offsets: [
                1172, 66708, 83092, 87188, 88212, 88468, 88532, 88548, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            sizes: [
                65536, 16384, 4096, 1024, 256, 64, 16, 4, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        },
    };
    blp2_test("UI-PAIDCHARACTERCUSTOMIZATION-BUTTON", blp_bytes, &header);
}

#[test]
fn test_sun_glare() {
    let blp_bytes = include_bytes!("../../../../assets/blp2/SunGlare.blp");
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
            sizes: [262144, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        },
    };
    blp2_test("SunGlare", blp_bytes, &header);
}

#[test]
fn test_oilslickenv_a() {
    let blp_bytes = include_bytes!("../../../../assets/blp2/oilslickenvA.blp");
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
            offsets: [
                1172, 394388, 492692, 517268, 523412, 524948, 525332, 525428, 0, 0, 0, 0, 0, 0, 0,
                0,
            ],
            sizes: [
                393216, 98304, 24576, 6144, 1536, 384, 96, 24, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        },
    };
    blp2_test("oilslickenvA", blp_bytes, &header);
}

#[test]
fn test_taurenfemaileskin00_001_extra() {
    let blp_bytes = include_bytes!("../../../../assets/blp2/TAURENFEMALESKIN00_01_EXTRA.BLP");
    let header = BlpHeader {
        version: BlpVersion::Blp2,
        content: BlpContentTag::Direct,
        flags: BlpFlags::Blp2 {
            compression: Compression::Raw1,
            alpha_bits: 4,
            alpha_type: 8,
            has_mipmaps: 1,
        },
        width: 128,
        height: 128,
        mipmap_locator: MipmapLocator::Internal {
            offsets: [
                1172, 25748, 31892, 33428, 33812, 33908, 33932, 33938, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            sizes: [24576, 6144, 1536, 384, 96, 24, 6, 2, 0, 0, 0, 0, 0, 0, 0, 0],
        },
    };
    blp2_test("TAURENFEMALESKIN00_01_EXTRA", blp_bytes, &header);
}

#[test]
fn test_buy() {
    let blp_bytes = include_bytes!("../../../../assets/blp2/Buy.blp");
    let header = BlpHeader {
        version: BlpVersion::Blp2,
        content: BlpContentTag::Direct,
        flags: BlpFlags::Blp2 {
            compression: Compression::Raw1,
            alpha_bits: 8,
            alpha_type: 8,
            has_mipmaps: 1,
        },
        width: 32,
        height: 32,
        mipmap_locator: MipmapLocator::Internal {
            offsets: [
                1172, 3220, 3732, 3860, 3892, 3900, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            sizes: [2048, 512, 128, 32, 8, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        },
    };
    blp2_test("Buy", blp_bytes, &header);
}

#[test]
fn test_trade_alchemy() {
    let blp_bytes = include_bytes!("../../../../assets/blp2/Trade_Alchemy.blp");
    let header = BlpHeader {
        version: BlpVersion::Blp2,
        content: BlpContentTag::Direct,
        flags: BlpFlags::Blp2 {
            compression: Compression::Dxtc,
            alpha_bits: 0,
            alpha_type: 0,
            has_mipmaps: 17, // why 17 >_<?
        },
        width: 64,
        height: 64,
        mipmap_locator: MipmapLocator::Internal {
            offsets: [
                1172, 3220, 3732, 3860, 3892, 3900, 3908, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            sizes: [2048, 512, 128, 32, 8, 8, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        },
    };
    blp2_test("Trade_Alchemy", blp_bytes, &header);
}

#[test]
fn test_buyout_icon() {
    let blp_bytes = include_bytes!("../../../../assets/blp2/BuyoutIcon.blp");
    let header = BlpHeader {
        version: BlpVersion::Blp2,
        content: BlpContentTag::Direct,
        flags: BlpFlags::Blp2 {
            compression: Compression::Dxtc,
            alpha_bits: 1,
            alpha_type: 0,
            has_mipmaps: 1,
        },
        width: 16,
        height: 16,
        mipmap_locator: MipmapLocator::Internal {
            offsets: [
                1172, 1300, 1332, 1340, 1348, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            sizes: [128, 32, 8, 8, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        },
    };
    blp2_test("BuyoutIcon", blp_bytes, &header);
}

#[test]
fn test_inv_fishingpole_02() {
    let blp_bytes = include_bytes!("../../../../assets/blp2/INV_Fishingpole_02.blp");
    let header = BlpHeader {
        version: BlpVersion::Blp2,
        content: BlpContentTag::Direct,
        flags: BlpFlags::Blp2 {
            compression: Compression::Dxtc,
            alpha_bits: 8,
            alpha_type: 1,
            has_mipmaps: 17,
        },
        width: 64,
        height: 64,
        mipmap_locator: MipmapLocator::Internal {
            offsets: [
                1172, 5268, 6292, 6548, 6612, 6628, 6644, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            sizes: [4096, 1024, 256, 64, 16, 16, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        },
    };
    blp2_test("INV_Fishingpole_02", blp_bytes, &header);
}

#[test]
fn test_ability_rogue_shadowstep() {
    let blp_bytes = include_bytes!("../../../../assets/blp2/Ability_Rogue_Shadowstep.blp");
    let header = BlpHeader {
        version: BlpVersion::Blp2,
        content: BlpContentTag::Direct,
        flags: BlpFlags::Blp2 {
            compression: Compression::Dxtc,
            alpha_bits: 8,
            alpha_type: 7,
            has_mipmaps: 17,
        },
        width: 64,
        height: 64,
        mipmap_locator: MipmapLocator::Internal {
            offsets: [
                1172, 5268, 6292, 6548, 6612, 6628, 6644, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            sizes: [4096, 1024, 256, 64, 16, 16, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        },
    };
    blp2_test("Ability_Rogue_Shadowstep", blp_bytes, &header);
}

#[test]
fn test_hell_fire_sky_nebula_03() {
    let blp_bytes = include_bytes!("../../../../assets/blp2/HellFireSkyNebula03.blp");
    let header = BlpHeader {
        version: BlpVersion::Blp2,
        content: BlpContentTag::Direct,
        flags: BlpFlags::Blp2 {
            compression: Compression::Dxtc,
            alpha_bits: 0,
            alpha_type: 7,
            has_mipmaps: 1,
        },
        width: 512,
        height: 256,
        mipmap_locator: MipmapLocator::Internal {
            offsets: [
                1172, 132244, 165012, 173204, 175252, 175764, 175892, 175924, 175940, 175956, 0, 0,
                0, 0, 0, 0,
            ],
            sizes: [
                131072, 32768, 8192, 2048, 512, 128, 32, 16, 16, 16, 0, 0, 0, 0, 0, 0,
            ],
        },
    };
    blp2_test("HellFireSkyNebula03", blp_bytes, &header);
}
