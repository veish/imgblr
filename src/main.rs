#[macro_use]
extern crate clap;

mod encode;

fn main() {
    let matches = clap_app!(imgblr =>
        (version: "0.1")
        (author: "Jeremy R. <me@retroc.at>")
        (about: "Generates a blurhash from an input image.  See <blurha.sh> for more details.")
        (@arg FORMAT: -f --format +takes_value "The input format of the file.  It is automatically detected if not provided.")
        (@arg NX: -x +takes_value "The number of components on the x axis.  Clamped to 9.  Defaults to 4.")
        (@arg NY: -y +takes_value "The number of components on the y axis.  Clamped to 9.  Defaults to 3.")
        (@arg INPUT: +required "Sets the input file to use")
    )
    .get_matches();

    let input = matches.value_of_os("INPUT").expect("input file required");
    let format = matches.value_of_os("FORMAT");
    let nx = matches
        .value_of("NX")
        .map(|v| {
            v.parse::<usize>()
                .expect("the number of x components should be a number")
        })
        .unwrap_or(4)
        .clamp(0, 9);
    let ny = matches
        .value_of("NY")
        .map(|v| {
            v.parse::<usize>()
                .expect("the number of y components should be a number")
        })
        .unwrap_or(3)
        .clamp(0, 9);
    let image = load_image(input, format).expect("could not load image");

    println!("{}", self::encode::encode(image, nx, ny))
}

fn load_image<P: AsRef<std::path::Path>>(
    path: P,
    format: Option<&std::ffi::OsStr>,
) -> Result<image::RgbaImage, anyhow::Error> {
    let file = std::io::BufReader::new(std::fs::OpenOptions::new().read(true).open(path)?);

    let mut reader = image::io::Reader::new(file);

    match format {
        None => {
            reader = reader.with_guessed_format()?;
        }
        Some(format) => {
            let format =
                image::ImageFormat::from_extension(format).expect("could not load given format");
            reader.set_format(format);
        }
    }

    reader
        .decode()
        .map(image::DynamicImage::into_rgba8)
        .map_err(anyhow::Error::from)
}
