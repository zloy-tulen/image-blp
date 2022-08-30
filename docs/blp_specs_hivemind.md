Original: 
[hiveworkshop.com](https://www.hiveworkshop.com/threads/blp-specifications-wc3.279306/)

# BLP Specifications (WC3)

Dr Super Good

**What is a BLP file?**

This manual covers the specification of Warcraft III BLP files. World of
Warcraft BLP files are not fully covered but they are specified quite
well by other sources. It is an update of existing specifications based
on observations and experimentation results to tidy it up.

A BLP (extension .blp) file is an image container. Each BLP file
contains a single image with an optional full set of mipmap images. The
format supports images up to 65,535 pixels in either dimension. BLP0
files are logically limited to at most 512 pixels in either dimension.
BLP1 used to have the same dimension limit as BLP0 until Warcraft III
The Frozen Throne patch 1.27b. BLP1 also used to limit the largest
useful mipmap level to at most 512 pixels in any dimension until
Warcraft III The Frozen Throne patch 1.29.

Pixels produce linear RGB colour components and optionally an alpha
component. The size of colour components varies depending on the
encoding type. Some types of decoded component data are intended to be
directly loaded as a linear colour space textures in by a graphics API.

It is important to note that BLP files technically contain linear RGB
colour space images. Most images such as from JPEG files or drawn using
art tools are manipulated in the sRGB or other non-linear colour spaces.
This means that in theory a colour space conversion should be performed
in order for the BLP image to represent the correct colours. However
there is strong evidence to suggest that Blizzard treated the image
components as sRGB when encoding the assets of Warcraft III, possibly
due to a lack of understanding about colour space management. So that
encoded and decoded assets generally appear as intended is not
recommended to perform colour space correction when processing BLP
files. If a colour space is required for output pixel components then it
is recommended that sRGB is used as that will likely be most visually
correct. When encoding it is recommended that pixel component values are
used directly with color space being ignored if possible.

In Warcraft III one can apply an approximate colour space correction
after scene rendering as part of the \"gamma\" setting. Warcraft III
gamma runs approximately from exponent 1.2 (minimum, setting 0) to
exponent 0.2 (maximum, setting 100, gamma 5.0). For semi-accurate
results gamma could be set to \~2.2 (registry setting 74-75). The way
WC3 generates the gamma correction LUTs appears to suffer from minor
rounding error or possibly uses a more correct sRGB curve rather than a
plain gamma curve. It is worth noting that despite this being
technically correct, the game will look terrible and over exposed
hinting that textures were not correctly colour space managed.

**File Structure**

BLP header file structure is stream based. This means that it is
recommended to read it 1 field at a time into an object rather than
mapping directly into a structure. Do note that the below code is pseudo
code of sorts so not intended for direct use by a programming language,
it is only to give an idea of the file stream structure. All fields are
in **little-endian** byte order.

.blp File Structure:
``` 
// from start of file stream the following objects are present.
BLPHeader_t blpHeader;;
if (version >= 1) { // determined by BLPHeader
    MipmapLocator_t mipmapLocator;
}
if (content == CONTENT_DIRECT) {
    DirectContentHeader_t contentHeader;
} else {
    JPEGContentHeader_t contentHeader;
}
```

BLPHeader_t Structure:
```
// BLPHeader_t
uint32_t magic; // determines version
uint32_t content;
if (version >= 2) {
    uint8_t encodingType; // not documented
    uint8_t alphaBits;
    uint8_t sampleType; // not documented
    uint8_t hasMipmaps;
} else {
    uint32_t alphaBits;
}
uint32_t width;
uint32_t height;
if (version < 2) {
    uint32_t extra;
    uint32_t hasMipmaps;
}

// valid magic values
uint32_t const MAGIC_BLP0 = decodeMagic("BLP0"); // version = 0
uint32_t const MAGIC_BLP1 = decodeMagic("BLP1"); // version = 1
uint32_t const MAGIC_BLP2 = decodeMagic("BLP2"); // version = 2

// valid content values
uint32_t const CONTENT_JPEG = 0; // JPEG encoded content.
uint32_t const CONTENT_DIRECT = 1; // Directly encoded content

// default values if invalid or new
content = CONTENT_JPEG;
alphaBits = 0;
extra = 5;
```

MipmapLocator_t Structure:
``` 
// MipmapLocator_t
uint32_t mmOffsets[16];
uint32_t mmSizes[16];
```

DirectContentHeader_t Structure:
``` 
// DirectContentHeader_t
uint32_t cmap[256];
``` 

**BLP File Header**

All fields assume little-endian byte order unless otherwise stated.

The BLP file header:
```
uint32_t magic;
uint32_t content;
if (version >= 2) {
    uint8_t encodingType; // not documented
    uint8_t alphaBits;
    uint8_t sampleType; // not documented
    uint8_t hasMipmaps;
} else {
    uint32_t alphaBits;
}
uint32_t width;
uint32_t height;
if (version < 2) {
    uint32_t extra;
    uint32_t hasMipmaps;
}
```

The magic field is used to identify files as BLP image files and what
format version they are. It consists of 4 ASCII characters that
sequentially represent something human readable. The characters are read
from lowest byte to highest byte when forming the strings. This should
be decoded into a constant version number to make subsequent reading and
processing easier.

```
// converts a 4 character ASCII string into a single 32 bit unsigned integer.
uint32_t decodeMagic(char const * magicstr);

// Version 0 used by Warcraft III: Reign of Chaos beta.
uint32_t const MAGIC_BLP_V0 = decodeMagic("BLP0");
// Version 1 used by Warcraft III.
uint32_t const MAGIC_BLP_V1 = decodeMagic("BLP1");
// Version 2 used by World of Warcraft.
uint32_t const MAGIC_BLP_V2 = decodeMagic("BLP2");

uint32_t const BLP_VERSION_MAP[] = {MAGIC_BLP_V0,
    MAGIC_BLP_V1, MAGIC_BLP_V2};
```

The content field determines how the image data is stored. CONTENT_JPEG
uses non-standard JPEG (JFIF) file compression of BGRA colour component
values rather than the usual Y′CbCr color component values.
CONTENT_DIRECT refers to a variety of storage formats which can be
directly read as pixel values. If content field is invalid then
CONTENT_JPEG must be assumed and it is recommended a warning be
generated.

```
// JPEG content.
uint32_t const CONTENT_JPEG = 0;
// Pixmap content.
uint32_t const CONTENT_DIRECT = 1;
```

The encodingType and sampleType fields determine the encoding used for
CONTENT_DIRECT images. The exact mechanics of the fields are not
documented here. For version 0 and 1 these can be assumed to correspond
to indexed encoding.

The alphaBits field determines the alpha component precision of the
stored image. CONTENT_JPEG must have either 0 or 8 bits due to the
mechanics of how JPEG stores components, and a JPEG alpha component must
still physically exist even for 0 bits. Direct content has been known to
support values of 8, 4, 1 or 0 bit and depends on the encoding type.
Invalid values of alphaBits for the content or encoding used must
evaluate as 0 bit and it is recommended a warning be generated.

Warcraft III has inconsistent processing for CONTENT_JPEG with 0 bit
alpha as UI images are correctly opaque but alpha component values are
still used when blending model textures. As such when writing
CONTENT_JPEG with 0 bit alpha it is required that the alpha band be
assigned opaque component values (0xFF).

With some direct encoding types in version 2 it is possible the
alphaBits field takes on another, unknown meaning according to various
World of Warcraft BLP2 specifications.

The width and height fields define the pixel dimensions of the full
scale image stored in the BLP file. Although often a power of two, it is
not required to be such.

The extra field appears to serve no purpose at all. There is no
particularly strong correlation between BLP file usage and value.
Testing also shows the value to have no visual impact on how the
textures is processed. Some legacy documentation stated that it effected
the team colour blending on unit models in Warcraft III however tests
have been unable to recreate any such effect. Other people speculate
that it might be a version field for the encoder used to create the BLP
files. Another guess might be that it was used internally by Blizzard to
perform some kind of image classification during development. Images
extracted without processing the field show no noticeable artefacts. A
recommended value to use would be 5 as that is used by the WorldEdit
generated war3mapMap.blp (mini-map) file.

The hasMipmaps field is a boolean for if mipmaps are present for the
image. If 0 then no mipmaps exist and the image will be present at full
resolution at mipmap level 0. If non 0 then a full compliment of mipmaps
exist ending with a 1\*1 pixel image at the highest mipmap level (eg for
the maximum size 65,535 x 65,535 image all 16 mipmaps are required). It
is recommended that an exception or error be generated when trying to
access mipmap levels which do not logically exist. It is also
recommended that an exception or error be generated when trying to
encode or decode an image with mipmaps that has any dimension larger
than 65,535 pixels. Mipmaps may be treated as thumbnails when it comes
to previewing the image content.

Mipmaps are required by textures which use mipmaps for filtering, such
as used by models. Warcraft III does not automatically generate mipmaps
for BLP textures, it assumes 0 value RGBA components (transparent black)
for all non-0 mipmap levels when using an image with hasMipmaps set to
1. Mipmaps are not required by textures which do not need mipmaps (only
tested on windows) such as command card buttons, shadows and minimap
images. Mipmaps are also needed by textures that are to scale with
Warcraft III texture quality.

**Mipmap Location**

If version is greater than or equal to 1 then a mipmap chunk location
structure is present after the BLP header.

The mipmap location header:
```
if (version >= 1) {
    uint32_t mmOffsets[16];
    uint32_t mmSizes[16];
}
```

The mmOffsets array field determines the file stream offset where the
mipmap data chunk is located counting from the start of the file. The
mmSizes array field determines how many bytes must be read for the
mipmap data chunk. With both arrays the index refers to the mipmap level
with mipmap 0 being the full sized image and higher indexes representing
smaller mipmap levels.

If hasMipmaps is 0 then only index 0 is used. If hasMipmaps is not 0
then the number of indices used should be the number of mipmap levels
required for the full sized image which is determined by the maximum of
height and width. There are no strict positioning or ordering
requirements with mipmap data chunks, and padding can even be mixed
between. However it is recommended that mipmap data chunks be placed
sequentially in ascending mipmap level order with no padding in between.

Mipmap data chunks must be sourced from within the file. WC3 does not
bounds check the buffers used after reading past EOF of a BLP file
resulting in undefined and crash prone behaviour. To allow some image
data to be read from technically malformed files it is recommended that
if mipmap data exceeds EOF then the EOF cutoff be used to truncate the
mipmap data chunk and a warning be generated.

![Example of Indexed Color Model texture where pixel
values are sourced unsafely from outside buffer bounds](./WC3%20BLP%20Buffer%20Overflow.jpg)

Warcraft 3 1.27 example of Indexed Color Model texture where pixel
values are sourced unsafely from outside buffer bounds. The circular
features are likely memory garbage leftover from processing parts of the
clock UI graphics.


If version is 0 then the mipmap data chunks for different mipmap levels
are stored in separate files. The full scale image, mipmap 0, is in a
file in the same directory with the same root name as the BLP file but
with extension \"b00\". Each mipmap level is assigned a unique file
extension derived from its level in the form of \"bXX\" where \"XX\" are
replaced with the 0 leading mipmap level number such as \"b08\" for
mipmap level 8 and \"b10\" for mipmap level 10. The mipmap data chunk
size is the file size of the file it is located in.

The way version 0 stores mipmap data chunks might not be directly
compatible with some image IO APIs. To decode them the API must support
processing or acquiring a file system path rather than only dealing with
a file stream to the BLP file. Due to this and how uncommon version 0
files are support is considered optional. With exception of how mipmap
data chunks are sourced version 0 and 1 are mostly identical so a
lossless version 0 to 1 converter could be made to provide version 0
support to a version 1 reader.

**Content Headers**

Next in the stream is the content header which is determined by the
content field. The content header is the final sequential structure that
can be read from a BLP file.

Content headers are used to setup the decoder for mipmap data chunks.

For CONTENT_JPEG BLP files the following header:

```
uint32_t jpegHeaderSize;
uint8_t jpegHeaderChunk[JpegHeaderSize];
```

The jpegHeaderSize field determines the byte size of a header chunk
appended to the beginning of all mipmap data blocks to produce a valid
JFIF image. It is followed immediately by the jpegHeaderChunk itself.
The mechanics of the chunk concatenation mean that it is not required
that jpegHeaderSize be non-0 as the mipmap data chunk can incorporate a
complete JPEG file. When dealing with blp files with hasMipmaps equal to
0 the mipmap data does not require a non-0 size as the jpegHeaderChunk
itself could be a complete JPEG file.

The maximum valid value of jpegHeaderSize is 624 bytes (0x270). Larger
values are prone to causing image corruption and crashes in some BLP
reader implementations like Warcraft III 1.27b where buffer bounds are
not strongly enforced. This limit applies especially when generating a
BLP file with JPEG content and without mipmaps as it can prevent dumping
the entire full scale image JPEG file into jpegHeaderChunk and using an
empty mipmap block. If values larger than 624 are encountered it is
recommended that a warning be generated and loading continues as normal
using the larger size.

If jpegHeaderSize causes jpegHeaderChunk to extend past EOF or is
stupidly large then Warcraft III is susceptible to a buffer over-run
crash. In such a case the JPEG header chunk size should be reduced to
fit within the file and it is recommended a warning be generated.

## Direct Content

For CONTENT_DIRECT BLP files the following header\...

The cmap field array is the colour look up table used for an indexed
colour model. Each element represents 24 bit RGB colour component values
in the order of 0xBBGGRR. The final byte is alignment padding and will
not alter the decoded image in any way. One might be able to improve the
file compressibility by carefully choosing padding values.

It is important to note that cmap must always be present for
CONTENT_DIRECT even if it is not required to decode an image. This can
happen for version 2 files where non index colour model based direct
formats exist. In such a case cmap should be left zeroed to allow for
better compression.

**Mipmap Data Chunks** 

Mipmap data chunks contain the image data which produces a mipmap level
when decoded.

## JPEG Mipmap Data

CONTENT_JPEG mipmap data chunks have the following format.

``` 
uint8_t jpegChunk[mipmapSize[this]];
```

Component values can be obtained by appending the jpegChunk to the
jpegHeaderChunk and decoding the resulting JPEG file with a compliant
JPEG codec. The JPEG file contains 8 bit BGRA colour components. This is
non-standard for JPEG files which usually consist of 8 bit Y′CbCr colour
components. As such the stored components have to be used directly
before codecs perform Y′CbCr colour conversion, something that many
codecs will do automatically. For example when programming in JAVA the
standard ImageIO JPEG decoder requires that the JPEG be decoded as a
raster and not as a BufferedImage to avoid automatic colour space
conversion. Blizzard uses the now discontinued Intel Imaging Framework
to produce/decode the JPEG content meaning that each jpegHeaderChunk of
official BLP files contains its signature. The use of an embedded
signature is optional and likely up to the JPEG encoder used.

Chroma subsampling is not possible or will likely produce bad results
since BGRA colour components are used instead of the normal Y′CbCr. Due
to the linear colour space used by BLP files the lossy compression of
JPEG should visually affect high component values less than low
component values which is not ideal and likely why BLP2 files never
contain JPEG content.

Since jpegChunk is a JPEG image it defines its own width and height
separate from the width and height fields of the BLP header. The
internal height and width should match the expected dimensions for the
mipmap level the jpegChunk is for. Some badly written BLP composition
tools fail to do this for high mipmap levels resulting in decoded mipmap
images that fail to comply with mipmap scaling logic. Such mipmaps are
known to cause a fatal error on some Mac versions. For feature parity
with the Windows version of Warcraft III such images should be loaded
with the right and bottom edges being used to crop or expand the image
into appropriate dimensions. Padding is done with transparent black (0,
0, 0, 0).

## Indexed Mipmap Data

CONTENT_DIRECT mipmap data chunks representing an indexed color model
have the following format.

``` 
uint8_t indexedRGB[getMipmapPixelCount(this)];
uint8_t indexedAlpha[(getMipmapPixelCount(this) * alphaBits + 7) / 8];
```

The function getMipmapPixelCount returns the product of height and width
field after being scaled for the appropriate mipmap level using mipmap
scaling logic. In Warcraft III the number of bytes read for the chunk
does not depend on mipmapSize\[this\], with it always reading the number
of bytes required, even if those bytes are unsafely outside buffer
bounds. The total length in bytes of both array fields should match
mipmapSize\[this\]. If the expected chunk size does not match mipmapSize
it is recommended that the underlying buffer be resized in an
implementation specific way to the expected size and a warning is
generated.

BGR component values can be obtained by using indexedRGB values as an
index in lutBGR. When producing such values using color matching be
aware of the linear nature of the color space. For best results it is
recommended that color matching be performed in sRGB or other perceptual
color spaces.

Alpha component can be obtained by breaking indexedAlpha into a bit
field of alphaBits bit length fragments and then using the bit fragment
as the alpha value for the pixel. The alpha pixel components are ordered
from least significant to most significant bit with bytes following the
same pixel order as indexedRGB. Since the alpha is to alphaBits
precision it may need to be resampled to 8 bits be useful depending on
the imaging framework used.

Example of different alpha packing in a byte.

```
MSB <-> LSB where number indicates the sequential pixel the bits belong to
ALPHA_8B -> 11111111
ALPHA_4B -> 22221111
ALPHA_1B -> 87654321
```

Pixels in the arrays run in scan lines left to right from top to bottom.
The scan line length and number of scan lines is determined from the
width and height fields after having mipmap scaling logic applied.

When producing mipmap data an algorithm appropriate for the source
colour space should be used. Simple averaging is only suitable for the
linear RGB colour space.

For most purposes high level mipmaps are usually so visually
insignificant that visual correctness can be sacrificed for filesize. In
the case of indexed content this can be achieved by pointing the image
data chunk inside one of the lower level mipmaps. In the case of JPEG
content it may be possible to use standard jpegChunk fields which have
been hand optimized for file size. In all cases it is required that
mipmapSize be correct, the appropriate number of mipmaps exist and that
the mipmaps are the correct dimensions.

