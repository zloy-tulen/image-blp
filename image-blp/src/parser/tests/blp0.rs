use super::super::*;
use crate::encode::encode_blp0;
use crate::path::make_mipmap_path;
use test_log::test;

fn test_blp0(
    name: &str,
    blp_bytes: &[u8],
    blp_mipmaps: &[Vec<u8>],
    expected_header: &BlpHeader,
    expected_images: usize,
) {
    let (_, parsed) = parse_blp_with_externals(blp_bytes, |i| preloaded_mipmaps(blp_mipmaps, i))
        .expect("successfull parsing");
    assert_eq!(&parsed.header, expected_header);
    assert_eq!(parsed.get_image_count(), expected_images);
    let encoded = encode_blp0(&parsed).expect("encoded blp");
    // There are zeros in the original file at the end
    let blp_bytes_n: Vec<u8> = blp_bytes
        .iter()
        .take(encoded.blp_bytes.len())
        .copied()
        .collect();
    assert_eq!(encoded.blp_bytes, blp_bytes_n);
    assert_eq!(encoded.blp_mipmaps, blp_mipmaps);

    // Test File API
    {
        let dir = tempfile::tempdir().expect("temporary directory");
        let blp_name = format!("{}.blp", name);
        let blp_path = dir.path().join(Path::new(&blp_name));
        std::fs::write(&blp_path, blp_bytes).expect("write");
        for (i, mipmap) in blp_mipmaps.iter().enumerate() {
            let mipmap_name = make_mipmap_path(&blp_path, i).expect("mipmap name");
            std::fs::write(mipmap_name, mipmap).expect("write");
        }

        let loaded = load_blp(&blp_path).expect("loaded");
        assert_eq!(loaded, parsed);
    }
}

#[test]
fn test_wyvern_rider() {
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
    test_blp0(
        "WyvernRider",
        blp_bytes,
        &blp_mipmaps,
        &header,
        blp_mipmaps.len(),
    );
}

#[test]
fn test_hero_level_border() {
    let blp_bytes = include_bytes!("../../../../assets/blp0/HeroLevel-Border.blp");
    let blp_mipmaps = vec![
        include_bytes!("../../../../assets/blp0/HeroLevel-Border.b00").to_vec(),
        include_bytes!("../../../../assets/blp0/HeroLevel-Border.b01").to_vec(),
        include_bytes!("../../../../assets/blp0/HeroLevel-Border.b02").to_vec(),
        include_bytes!("../../../../assets/blp0/HeroLevel-Border.b03").to_vec(),
        include_bytes!("../../../../assets/blp0/HeroLevel-Border.b04").to_vec(),
        include_bytes!("../../../../assets/blp0/HeroLevel-Border.b05").to_vec(),
        include_bytes!("../../../../assets/blp0/HeroLevel-Border.b06").to_vec(),
    ];
    let header = BlpHeader {
        version: BlpVersion::Blp0,
        content: BlpContentTag::Direct,
        flags: BlpFlags::Old {
            alpha_bits: 0,
            extra: 5,
            has_mipmaps: 1,
        },
        width: 64,
        height: 64,
        mipmap_locator: MipmapLocator::External,
    };
    test_blp0(
        "HeroLevel-Border",
        blp_bytes,
        &blp_mipmaps,
        &header,
        blp_mipmaps.len(),
    );
}

#[test]
fn test_acid_splash1() {
    let blp_bytes = include_bytes!("../../../../assets/blp0/AcidSplash1.blp");
    let blp_mipmaps = vec![
        include_bytes!("../../../../assets/blp0/AcidSplash1.b00").to_vec(),
        include_bytes!("../../../../assets/blp0/AcidSplash1.b01").to_vec(),
        include_bytes!("../../../../assets/blp0/AcidSplash1.b02").to_vec(),
        include_bytes!("../../../../assets/blp0/AcidSplash1.b03").to_vec(),
        include_bytes!("../../../../assets/blp0/AcidSplash1.b04").to_vec(),
        include_bytes!("../../../../assets/blp0/AcidSplash1.b05").to_vec(),
        include_bytes!("../../../../assets/blp0/AcidSplash1.b06").to_vec(),
    ];
    let header = BlpHeader {
        version: BlpVersion::Blp0,
        content: BlpContentTag::Jpeg,
        flags: BlpFlags::Old {
            alpha_bits: 8,
            extra: 4,
            has_mipmaps: 1,
        },
        width: 64,
        height: 64,
        mipmap_locator: MipmapLocator::External,
    };
    test_blp0(
        "AcidSplash1",
        blp_bytes,
        &blp_mipmaps,
        &header,
        blp_mipmaps.len(),
    );
}
