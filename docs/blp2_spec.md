Original: https://wowpedia.fandom.com/wiki/BLP_files

# BLP files

**BLP files** are Blizzard\'s texture format, used for many games.
*World of Warcraft* uses the BLP2 format in particular. The BLP file
structure consists of a header, up to 16 mipmaps of the texture, and a
palette. Texture sizes must be powers of two, though the two dimensions
do not have to be equal; 512x256 is valid, but 512x200 is not. The first
mipmap (mipmap #0) is the full size image; each subsequent mipmap halves
both dimensions. The final mipmap should be 1x1.

Instead of converting to BLP files (see the end of this article for
converters), the WoW engine also accepts [TGA
files](https://wowpedia.fandom.com/wiki/TGA_files "TGA files"), which
can be edited directly in graphics editors.

```
    struct blp2header {
      uint8_t    ident[4];           // "BLP2" magic number
      uint32_t   type;               // Texture type: 0 = JPG, 1 = S3TC
      uint8_t    compression;        // Compression mode: 1 = raw, 2 = DXTC
      uint8_t    alpha_bits;         // 0, 1, 4, or 8
      uint8_t    alpha_type;         // 0, 1, 7, or 8
      uint8_t    has_mips;           // 0 = no mips levels, 1 = has mips (number of levels determined by image size)
      uint32_t   width;              // Image width in pixels
      uint32_t   height;             // Image height in pixels
      uint32_t   mipmap_offsets[16]; // The file offsets of each mipmap, 0 for unused
      uint32_t   mipmap_lengths[16]; // The length of each mipmap data block
    } blp2header;
```

*World of Warcraft* does not use JPG textures. A discussion of BLP files
using JPG compression is beyond the scope of this article. The type flag
should always be 1, which indicates the use of either RAW, DXT1, DXT3,
or DXT5 compression. Each compression type will be covered separately.

### The .BLP color palette

The header is always followed by a 256-entry color table. Each entry a
32-bit BGRA 8888 value. This table is only used for RAW images, but is
present in all BLPs regardless.

### RAW compression

#### RAW1

If `compression` is set to 1, each mipmap is stored as an array of 8-bit
values, one per pixel, left to right, top to bottom. Each value is an
index to the palette.

If `alpha_bits` is greater than 0, an alpha channel will immediately
follow the image data, and comes in 1, 4, and 8 bit varieties. The 1 and
4 bit versions have multiple values packed into a single byte, with the
least significant bit belonging to the first packed value.

#### RAW3

With the `compression` set to 3, each mipmap contains what appears to be
32 bit BGRA data. `alpha_bits` seems to represent a set of bit flags
rather than depth, as all images of this type seem to have 4 bytes per
pixel regardless of depth, and it has been seen to exceed 8. Their
meaning is unknown.

### DXTn compression

If `compression` is set to 2, each mipmap is composed of 4×4 blocks of
pixels. The blocks and the pixels within each block are ordered from
left to right, top to bottom.

See [Wikipedia\'s entry on DXT
compression](http://en.wikipedia.org/wiki/S3TC "wikipedia:S3TC") for the
technical details.

#### DXT1

If `alpha_type` is 0, then DXT1 compression is used.

Each block is 64 bits and begins with two 16 bit values, and are used to
derived a 4 color palette.

The values are interpreted as 565 RGB colors, with the least significant
bits corresponding to blue, to create the first two colors in the
palette.

If the first value is less than or equal to the second, the final entry
of the palette is reserved. If `alpha_bits` is 0, the reserved color is
black. If `alpha_bits` is 1, the reserved color is transparent.

The remaining colors are created by interpolating between the first two
colors in the palette.

The remaining 32 bits are 16 2-bit values acting as a lookups to specify
the colors in the block.

#### DXT3

If `alpha_type` is 1, then DXT3 compression is used.

Each block is 128 bits and begins identically to DXT1, except that no
special color is reserved in the palette.

It is followed by 16 4-bit values corresponding to the alpha values for
each of the pixels in the block.

#### DXT5

If `alpha_type` is 7, then DXT5 compression is used. This format was
first used for Burning Crusade images.

Each block is 128 bits and begins with two 8-bit values to create an 8
element lookup table for alpha values.

The first two elements in the lookup table are copies of those values.

If the first value is less than or equal to the second, the final two
entries of the lookup table are reserved for transparent and opaque.

The remaining entries are created by interpolating between the first two
entries in the lookup table.

The next 48 bits make up 16 3-bit values acting as lookups specifying
the alpha values for each of the pixels in the block.

The remaining 64 bits are identical to DXT1, except that no special
color is reserved in the palette.

### Sample Files

Below is a list of BLP files, each with some unique characteristic.

  --------------------------------------------------- ---------- -------------
  File                                                Encoding   Features
  GLUES\\LoadingBar\\Loading-BarGlow.blp              RAW1       No alpha
  CURSOR\\Attack.blp                                  RAW1       1-bit alpha
  CURSOR\\Buy.blp                                     RAW1       8-bit alpha
  Icons\\Trade_Alchemy.blp                            DXT1       No alpha
  AuctionFrame\\BuyoutIcon.blp                        DXT1       1-bit alpha
  Icons\\INV_Fishingpole_02.blp                       DXT3       4-bit alpha
  Icons\\Ability_Rogue_Shadowstep.blp                 DXT5       8-bit alpha
  BUTTONS\\UI-PaidCharacterCustomization-Button.blp   RAW3       BGRA color
  --------------------------------------------------- ---------- -------------

The following files require an MPQ viewer to access, as they aren\'t
exposed by Blizzard\'s [AddOn
Kit](http://us.blizzard.com/support/article.xml?locale=en_US&articleId=21466){target="_self"
rel="nofollow"}.

  ------------------------------------------------------------ ---------- ----------------
  File                                                         Encoding   Features
  Character\\Tauren\\Female\\TAURENFEMALESKIN00_01_EXTRA.blp   RAW1       4-bit alpha
  Environments\\Stars\\HellFireSkyNebula03.blp                 DXT5       No alpha
  Textures\\SunGlare.blp                                       RAW3       alpha_bits=136
  TILESET\\Terrain Cube Maps\\TCB_CrystalSong_A.blp            DXT1       width=768
  TILESET\\Terrain Cube Maps\\oilslickenvA.blp                 RAW3       alpha_bits=1
  ------------------------------------------------------------ ---------- ----------------

### Conversion tools

There exist BLP tools to convert both from and to the BLP format. They
can be found in [UI Authors resource
list](https://wowpedia.fandom.com/wiki/UI_FAQ/AddOn_Author_Resources#Resources "UI FAQ/AddOn Author Resources").
