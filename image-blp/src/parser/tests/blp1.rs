use super::super::*;
use crate::encode::encode_blp;
use test_log::test;

fn blp1_test(name: &str, blp_bytes: &[u8], header: &BlpHeader) {
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
fn test_cliff_brush01_blp_compression_uncompressed() {
    let blp_bytes =
        include_bytes!("../../../../assets/blp1/CliffBrush01-BlpCompressionUncompressed.blp");
    let header = BlpHeader {
        version: BlpVersion::Blp1,
        content: BlpContentTag::Direct,
        flags: BlpFlags::Old {
            alpha_bits: 8,
            extra: 4,
            has_mipmaps: 0,
        },
        width: 32,
        height: 32,
        mipmap_locator: MipmapLocator::Internal {
            offsets: [1180, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            sizes: [2048, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        },
    };
    blp1_test(
        "CliffBrush01-BlpCompressionUncompressed",
        blp_bytes,
        &header,
    );
}

#[test]
fn test_day_indicator_texture3() {
    let blp_bytes = include_bytes!("../../../../assets/blp1/day-indicator-texture3.blp");
    let header = BlpHeader {
        version: BlpVersion::Blp1,
        content: BlpContentTag::Direct,
        flags: BlpFlags::Old {
            alpha_bits: 8,
            extra: 5,
            has_mipmaps: 1,
        },
        width: 512,
        height: 256,
        mipmap_locator: MipmapLocator::Internal {
            offsets: [
                1180, 263324, 328860, 345244, 349340, 350364, 350620, 350684, 350700, 350704, 0, 0,
                0, 0, 0, 0,
            ],
            sizes: [
                262144, 65536, 16384, 4096, 1024, 256, 64, 16, 4, 2, 0, 0, 0, 0, 0, 0,
            ],
        },
    };
    blp1_test("day-indicator-texture3", blp_bytes, &header);
}

#[test]
fn test_editor_gem_deact() {
    let blp_bytes = include_bytes!("../../../../assets/blp1/EditorGemDeact.blp");
    let header = BlpHeader {
        version: BlpVersion::Blp1,
        content: BlpContentTag::Direct,
        flags: BlpFlags::Old {
            alpha_bits: 0,
            extra: 5,
            has_mipmaps: 0,
        },
        width: 64,
        height: 64,
        mipmap_locator: MipmapLocator::Internal {
            offsets: [1180, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            sizes: [4096, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        },
    };
    blp1_test("EditorGemDeact", blp_bytes, &header);
}

#[test]
fn test_green_firering2() {
    let blp_bytes = include_bytes!("../../../../assets/blp1/Green_Firering2.blp");
    let header = BlpHeader {
        version: BlpVersion::Blp1,
        content: BlpContentTag::Jpeg,
        flags: BlpFlags::Old {
            alpha_bits: 0,
            extra: 5,
            has_mipmaps: 1,
        },
        width: 64,
        height: 64,
        mipmap_locator: MipmapLocator::Internal {
            offsets: [
                1180, 3053, 3650, 3880, 3978, 4071, 4153, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            sizes: [1873, 597, 230, 98, 93, 82, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        },
    };
    blp1_test("Green_Firering2", blp_bytes, &header);
}

#[test]
fn test_human_base() {
    let blp_bytes = include_bytes!("../../../../assets/blp1/HumanBase.blp");
    let header = BlpHeader {
        version: BlpVersion::Blp1,
        content: BlpContentTag::Jpeg,
        flags: BlpFlags::Old {
            alpha_bits: 8,
            extra: 5,
            has_mipmaps: 1,
        },
        width: 128,
        height: 128,
        mipmap_locator: MipmapLocator::Internal {
            offsets: [
                1180, 16151, 20402, 21524, 21827, 21924, 22003, 22065, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            sizes: [
                14971, 4251, 1122, 303, 97, 79, 62, 47, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        },
    };
    blp1_test("HumanBase", blp_bytes, &header);
}

#[test]
fn test_minimap_item() {
    let blp_bytes = include_bytes!("../../../../assets/blp1/minimap-item.blp");
    let header = BlpHeader {
        version: BlpVersion::Blp1,
        content: BlpContentTag::Direct,
        flags: BlpFlags::Old {
            alpha_bits: 8,
            extra: 4,
            has_mipmaps: 1,
        },
        width: 16,
        height: 16,
        mipmap_locator: MipmapLocator::Internal {
            offsets: [
                1180, 1692, 1820, 1852, 1860, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            sizes: [512, 128, 32, 8, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        },
    };
    blp1_test("minimap-item", blp_bytes, &header);
}

#[test]
fn test_night_elf_sybmol() {
    let blp_bytes = include_bytes!("../../../../assets/blp1/NightElfSymbol.blp");
    let header = BlpHeader {
        version: BlpVersion::Blp1,
        content: BlpContentTag::Jpeg,
        flags: BlpFlags::Old {
            alpha_bits: 0,
            extra: 4,
            has_mipmaps: 1,
        },
        width: 256,
        height: 256,
        mipmap_locator: MipmapLocator::Internal {
            offsets: [
                1180, 45951, 61864, 67007, 68809, 69352, 69518, 69652, 69745, 0, 0, 0, 0, 0, 0, 0,
            ],
            sizes: [
                44771, 15913, 5143, 1802, 543, 166, 134, 93, 48, 0, 0, 0, 0, 0, 0, 0,
            ],
        },
    };
    blp1_test("NightElfSymbol", blp_bytes, &header);
}

#[test]
fn test_object_editor_custom_item() {
    let blp_bytes = include_bytes!("../../../../assets/blp1/ObjectEditor-CustomItem.blp");
    let header = BlpHeader {
        version: BlpVersion::Blp1,
        content: BlpContentTag::Direct,
        flags: BlpFlags::Old {
            alpha_bits: 8,
            extra: 5,
            has_mipmaps: 0,
        },
        width: 16,
        height: 16,
        mipmap_locator: MipmapLocator::Internal {
            offsets: [1180, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            sizes: [512, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        },
    };
    blp1_test("ObjectEditor-CustomItem", blp_bytes, &header);
}

#[test]
fn test_ping2() {
    let blp_bytes = include_bytes!("../../../../assets/blp1/ping2.blp");
    let header = BlpHeader {
        version: BlpVersion::Blp1,
        content: BlpContentTag::Direct,
        flags: BlpFlags::Old {
            alpha_bits: 0,
            extra: 5,
            has_mipmaps: 1,
        },
        width: 128,
        height: 128,
        mipmap_locator: MipmapLocator::Internal {
            offsets: [
                1180, 17564, 21660, 22684, 22940, 23004, 23020, 23024, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            sizes: [16384, 4096, 1024, 256, 64, 16, 4, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        },
    };
    blp1_test("ping2", blp_bytes, &header);
}

#[test]
fn test_stronghold() {
    let blp_bytes = include_bytes!("../../../../assets/blp1/Stronghold.blp");
    let header = BlpHeader {
        version: BlpVersion::Blp1,
        content: BlpContentTag::Jpeg,
        flags: BlpFlags::Old {
            alpha_bits: 8,
            extra: 4,
            has_mipmaps: 1,
        },
        width: 256,
        height: 256,
        mipmap_locator: MipmapLocator::Internal {
            offsets: [
                1180, 69750, 94264, 101511, 103450, 103975, 104137, 104282, 104376, 0, 0, 0, 0, 0,
                0, 0,
            ],
            sizes: [
                68570, 24514, 7247, 1939, 525, 162, 145, 94, 48, 0, 0, 0, 0, 0, 0, 0,
            ],
        },
    };
    blp1_test("Stronghold", blp_bytes, &header);
}
