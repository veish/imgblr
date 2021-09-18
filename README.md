# Imgblr

Imgblr is a command-line impleemntation of a [blurhash] encoder.

## Why?

I couldn't find a good encoder that I wanted to use on the server side.  So,
I made one.  This generates a blurhash of an image, and outputs it to stdout
if successful.  If unsuccessful, it does not print anything to stdout; only
stderr.

## How?

```
USAGE:
    imgblr [OPTIONS] <INPUT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --format <FORMAT>    The input format of the file.  It is automatically detected if not provided.
    -x <NX>                  The number of components on the x axis.  Clamped to 9.  Defaults to 4.
    -y <NY>                  The number of components on the y axis.  Clamped to 9.  Defaults to 3.

ARGS:
    <INPUT>    Sets the input file to use
```

Under the hood, imgblr uses the [image] rust library, and converts it into the
correct color type required for blurhash.  This discards the alpha channel, if
there is one, so be careful with using alpha with this.  See the [image] rust
library to see what formats it supports, but for a short list:

- `jpeg`
- `png`
- `gif`
- `webp`
- `tiff`
- `tga`
- `dds`
- `bmp`
- `ico`
- `hdr`
- `pnm`
- `ff`

## How fast?

The release binary can process [sumo.jpg] in less than a second on my 4-core,
8GB DO machine.

## Is it any good?

Yes.

[blurhash]: https://blurha.sh
[image]: https://github.com/image-rs/image
[sumo.jpg]: sumo.jpg
