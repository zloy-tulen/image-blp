This crate provides decoding/encoding for Blizzard BLP texture
format that is used across several games like Wacraft III and
World of Warcraft. You can load any BLP file into [DynamicImage]
from [image](https://crates.io/crates/image) crate and save any [DynamicImage] into BLP file.

# Usage
This crate is on crates.io and can be used by adding image-blp to
your dependencies in your project's Cargo.toml.

``` toml
[dependencies]
image-blp = "1"
```

# Example: loading

The crate separates loading of BLP file into parsing and converting.
That allows to process BLP files without loosing information due unessesary
conversion back and forth. Typically loading of BLP image into usable RGBA
image looks like:

```rust
let blp_file = load_blp(blp_filename).expect("loaded blp");
let mipmap_level = 0;
let image = blp_to_image(&blp_file, mipmap_level).expect("converted");
```
See example [examples/load.rs](./image-blp/examples/load.rs) for full code.

# Example: saving

The crate provides simplified API for specifing which type of BLP do you want to use.
See [convert::BlpTarget] type for more info. Here the typical way to save image as
```rust
let img_file: DynamicImage = Reader::open(input_filename)
    .expect("open")
    .decode()
    .expect("decode");
let make_mipmaps = true;
let blp = image_to_blp(
    img_file,
    make_mipmaps,
    BlpTarget::Blp1(BlpOldFormat::Raw1 {
        alpha_bits: AlphaBits::Bit1,
    }),
    FilterType::Nearest,
)
.expect("converted");
save_blp(&blp, output_filename).expect("saved");
```  
See example [examples/save.rs](./image-blp/examples/save.rs) for full code.

# CLI tool

The library is used to build universal CLI tool [blp-conv]( https://crates.io/crates/blp-conv) that allows
to convert from/into BLP format for wide range of image formats. You
can install it via:

```bash
cargo install blp-conv
```

# Features

The crate supports all known BLP versions like:
* `BLP0` -- is used in old Wacraft III ROC Beta builds.
* `BLP1` -- is common for Wacraft III TFT.
* `BLP2` -- is used in World of Warcraft.

The crate supports also all known encodings:
* `RAW1` -- paletted images with 256 colors. We use [color_quant]
package for compressing generic images to the format.
* `RAW3` -- like ordinary RGBA bitmaps.
* `JPEG` -- ordinary jpeg compressed image.
* `DXTn` -- [S3TC] compression algorithms for `BLP2` version. We use
[texpresso] for the compression/decompression.

# Tests

Tests of the library use original files of Blizzard games. So, they cannot
be distributed with the library due license issues. You should buy the
original games to extract the files for testing.

[DynamicImage]: https://docs.rs/image/latest/image/enum.DynamicImage.html
[color_quant]: https://crates.io/crates/color_quant
[texpresso]: https://crates.io/crates/texpresso
[S3TC]: http://en.wikipedia.org/wiki/S3TC