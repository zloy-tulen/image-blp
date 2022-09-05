use super::super::*;
use crate::encode::encode_blp;
use test_log::test;

// #[test]
// fn blp1_simplest_direct_alpha() {
//     let blp_bytes = include_bytes!("../../../../assets/blp1/simple_with_alpha.blp");
//     let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
//     let header = BlpHeader {
//         version: BlpVersion::Blp1,
//         content: BlpContentTag::Direct,
//         flags: BlpFlags::Old {
//             alpha_bits: 8,
//             extra: 3,
//             has_mipmaps: 5,
//         },
//         width: 2,
//         height: 2,
//         mipmap_locator: MipmapLocator::Internal {
//             offsets: [1180; 16],
//             sizes: [8; 16],
//         },
//     };
//     assert_eq!(parsed.header, header);
//     let encoded = encode_blp(&parsed).expect("encoded blp");
//     assert_eq!(encoded, blp_bytes);
// }

// #[test]
// fn blp1_simplest_direct_noalpha() {
//     let blp_bytes = include_bytes!("../../../../assets/blp1/simple_without_alpha.blp");
//     let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
//     let header = BlpHeader {
//         version: BlpVersion::Blp1,
//         content: BlpContentTag::Direct,
//         flags: BlpFlags::Old {
//             alpha_bits: 0,
//             extra: 5,
//             has_mipmaps: 5,
//         },
//         width: 2,
//         height: 2,
//         mipmap_locator: MipmapLocator::Internal {
//             offsets: [1180; 16],
//             sizes: [4; 16],
//         },
//     };
//     assert_eq!(parsed.header, header);
//     let encoded = encode_blp(&parsed).expect("encoded blp");
//     assert_eq!(encoded, blp_bytes);
// }

// #[test]
// fn blp1_simplest_blp() {
//     let blp_bytes = include_bytes!("../../../../assets/blp1/simple_jpg.blp");
//     let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
//     let header = BlpHeader {
//         version: BlpVersion::Blp1,
//         content: BlpContentTag::Jpeg,
//         flags: BlpFlags::Old {
//             alpha_bits: 8,
//             extra: 3,
//             has_mipmaps: 5,
//         },
//         width: 2,
//         height: 2,
//         mipmap_locator: MipmapLocator::Internal {
//             offsets: [160; 16],
//             sizes: [482; 16],
//         },
//     };
//     assert_eq!(parsed.header, header);
//     let encoded = encode_blp(&parsed).expect("encoded blp");
//     assert_eq!(encoded, blp_bytes);
// }

// #[test]
// fn blp1_rect_direct() {
//     let blp_bytes = include_bytes!("../../../../assets/blp1/rect_with_alpha.blp");
//     let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
//     let header = BlpHeader {
//         version: BlpVersion::Blp1,
//         content: BlpContentTag::Direct,
//         flags: BlpFlags::Old {
//             alpha_bits: 8,
//             extra: 3,
//             has_mipmaps: 5,
//         },
//         width: 2,
//         height: 3,
//         mipmap_locator: MipmapLocator::Internal {
//             offsets: [1180; 16],
//             sizes: [12; 16],
//         },
//     };
//     assert_eq!(parsed.header, header);
//     let encoded = encode_blp(&parsed).expect("encoded blp");
//     assert_eq!(encoded, blp_bytes);
// }

// #[test]
// fn blp1_rect_jpg_no_alpha() {
//     let blp_bytes = include_bytes!("../../../../assets/blp1/rect_without_alpha.blp");
//     let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
//     let header = BlpHeader {
//         version: BlpVersion::Blp1,
//         content: BlpContentTag::Jpeg,
//         flags: BlpFlags::Old {
//             alpha_bits: 0,
//             extra: 5,
//             has_mipmaps: 5,
//         },
//         width: 2,
//         height: 3,
//         mipmap_locator: MipmapLocator::Internal {
//             offsets: [160; 16],
//             sizes: [448; 16],
//         },
//     };
//     assert_eq!(parsed.header, header);
//     assert_eq!(parsed.get_content_jpeg().expect("jpg").images.len(), 2);
//     let encoded = encode_blp(&parsed).expect("encoded blp");
//     assert_eq!(encoded, blp_bytes);
// }

#[test]
fn blp1_cliff_brush01_blp_compression_uncompressed() {
    let blp_bytes = include_bytes!("../../../../assets/blp1/CliffBrush01-BlpCompressionUncompressed.blp");
    let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
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
    assert_eq!(parsed.header, header);
    assert_eq!(parsed.get_content_direct().expect("image").images.len(), 1);
    let encoded = encode_blp(&parsed).expect("encoded blp");
    assert_eq!(encoded, blp_bytes);
}

#[test]
fn blp1_day_indicator_texture3() {
    let blp_bytes = include_bytes!("../../../../assets/blp1/day-indicator-texture3.blp");
    let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
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
            offsets: [1180, 263324, 328860, 345244, 349340, 350364, 350620, 350684, 350700, 350704, 0, 0, 0, 0, 0, 0],
            sizes: [262144, 65536, 16384, 4096, 1024, 256, 64, 16, 4, 2, 0, 0, 0, 0, 0, 0],
        },
    };
    assert_eq!(parsed.header, header);
    assert_eq!(parsed.get_content_direct().expect("image").images.len(), 10);
    let encoded = encode_blp(&parsed).expect("encoded blp");
    assert_eq!(encoded, blp_bytes);
}


#[test]
fn blp1_editor_gem_deact() {
    let blp_bytes = include_bytes!("../../../../assets/blp1/EditorGemDeact.blp");
    let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
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
    assert_eq!(parsed.header, header);
    assert_eq!(parsed.get_content_direct().expect("image").images.len(), 1);
    let encoded = encode_blp(&parsed).expect("encoded blp");
    assert_eq!(encoded, blp_bytes);
}

#[test]
fn blp1_green_firering2() {
    let blp_bytes = include_bytes!("../../../../assets/blp1/Green_Firering2.blp");
    let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
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
            offsets: [1180, 3053, 3650, 3880, 3978, 4071, 4153, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            sizes: [1873, 597, 230, 98, 93, 82, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        },
    };
    assert_eq!(parsed.header, header);
    assert_eq!(parsed.get_content_jpeg().expect("image").images.len(), 7);
    let encoded = encode_blp(&parsed).expect("encoded blp");
    assert_eq!(encoded, blp_bytes, "Sizes: {} ?= {}", encoded.len(), blp_bytes.len());
}

#[test]
fn blp1_human_base() {
    let blp_bytes = include_bytes!("../../../../assets/blp1/HumanBase.blp");
    let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
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
            offsets: [1180, 16151, 20402, 21524, 21827, 21924, 22003, 22065, 0, 0, 0, 0, 0, 0, 0, 0],
            sizes: [14971, 4251, 1122, 303, 97, 79, 62, 47, 0, 0, 0, 0, 0, 0, 0, 0],
        },
    };
    assert_eq!(parsed.header, header);
    assert_eq!(parsed.get_content_jpeg().expect("image").images.len(), 8);
    let encoded = encode_blp(&parsed).expect("encoded blp");
    assert_eq!(encoded, blp_bytes, "Sizes: {} ?= {}", encoded.len(), blp_bytes.len());
}

#[test]
fn blp1_minimap_item() {
    let blp_bytes = include_bytes!("../../../../assets/blp1/minimap-item.blp");
    let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
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
            offsets: [1180, 1692, 1820, 1852, 1860, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            sizes: [512, 128, 32, 8, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        },
    };
    assert_eq!(parsed.header, header);
    assert_eq!(parsed.get_content_direct().expect("image").images.len(), 5);
    let encoded = encode_blp(&parsed).expect("encoded blp");
    assert_eq!(encoded, blp_bytes, "Sizes: {} ?= {}", encoded.len(), blp_bytes.len());
}

#[test]
fn blp1_night_elf_sybmol() {
    let blp_bytes = include_bytes!("../../../../assets/blp1/NightElfSymbol.blp");
    let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
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
            offsets: [1180, 45951, 61864, 67007, 68809, 69352, 69518, 69652, 69745, 0, 0, 0, 0, 0, 0, 0],
            sizes: [44771, 15913, 5143, 1802, 543, 166, 134, 93, 48, 0, 0, 0, 0, 0, 0, 0],
        },
    };
    assert_eq!(parsed.header, header);
    assert_eq!(parsed.get_content_jpeg().expect("image").images.len(), 9);
    let encoded = encode_blp(&parsed).expect("encoded blp");
    assert_eq!(encoded, blp_bytes, "Sizes: {} ?= {}", encoded.len(), blp_bytes.len());
}

#[test]
fn blp1_object_editor_custom_item() {
    let blp_bytes = include_bytes!("../../../../assets/blp1/ObjectEditor-CustomItem.blp");
    let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
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
    assert_eq!(parsed.header, header);
    assert_eq!(parsed.get_content_direct().expect("image").images.len(), 1);
    let encoded = encode_blp(&parsed).expect("encoded blp");
    assert_eq!(encoded, blp_bytes, "Sizes: {} ?= {}", encoded.len(), blp_bytes.len());
}

#[test]
fn blp1_ping2() {
    let blp_bytes = include_bytes!("../../../../assets/blp1/ping2.blp");
    let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
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
            offsets: [1180, 17564, 21660, 22684, 22940, 23004, 23020, 23024, 0, 0, 0, 0, 0, 0, 0, 0],
            sizes: [16384, 4096, 1024, 256, 64, 16, 4, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        },
    };
    assert_eq!(parsed.header, header);
    assert_eq!(parsed.get_content_direct().expect("image").images.len(), 8);
    let encoded = encode_blp(&parsed).expect("encoded blp");
    assert_eq!(encoded, blp_bytes, "Sizes: {} ?= {}", encoded.len(), blp_bytes.len());
}

#[test]
fn blp1_stronghold() {
    let blp_bytes = include_bytes!("../../../../assets/blp1/Stronghold.blp");
    let (_, parsed) = parse_blp(blp_bytes).expect("successfull parsing");
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
            offsets: [1180, 69750, 94264, 101511, 103450, 103975, 104137, 104282, 104376, 0, 0, 0, 0, 0, 0, 0],
            sizes: [68570, 24514, 7247, 1939, 525, 162, 145, 94, 48, 0, 0, 0, 0, 0, 0, 0],
        },
    };
    assert_eq!(parsed.header, header);
    assert_eq!(parsed.get_content_jpeg().expect("image").images.len(), 9);
    let encoded = encode_blp(&parsed).expect("encoded blp");
    assert_eq!(encoded, blp_bytes, "Sizes: {} ?= {}", encoded.len(), blp_bytes.len());
}