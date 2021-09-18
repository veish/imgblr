type Triplet = (f64, f64, f64);

pub fn encode(image: image::RgbaImage, nx: usize, ny: usize) -> String {
    use std::f64::consts::PI;

    let mut dc: Triplet = (0.0, 0.0, 0.0);
    let mut components = Vec::with_capacity((nx * ny) - 1);

    for j in 0..ny {
        for i in 0..nx {
            let normalization = if j == 0 && i == 0 { 1f64 } else { 2f64 };

            let factor = multiply_basis_function(&image, |x, y| {
                normalization
                    * ((PI * i as f64 * x) / (image.width() as f64)).cos()
                    * ((PI * j as f64 * y) / (image.height() as f64)).cos()
            });

            if j == 0 && i == 0 {
                dc = factor;
            } else {
                components.push(factor);
            }
        }
    }

    let mut hash = String::with_capacity(6 + components.len() * 2);

    // Encode the size vector.
    hash.extend(encode_base83(((nx - 1) + (ny - 1) * 9) as u32, 1));

    // Encode the maximum AC component value.
    let actual_maximum = components
        .iter()
        .copied()
        .map(|(r, g, b)| r.abs().max(g.abs().max(b.abs())))
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let maximum = if let Some(actual) = actual_maximum {
        let quantized = (actual * 166.0 - 0.5).floor().clamp(0.0, 82.0) as u32;
        let maximum = (quantized + 1) as f64 / 166.0;
        hash.extend(encode_base83(quantized as u32, 1));
        maximum
    } else {
        hash.extend(encode_base83(0, 1));
        1.0
    };

    dbg!(maximum);
    hash.extend(encode_base83(encode_dc(dc), 4));
    for component in components {
        hash.extend(encode_base83(encode_ac(component, maximum), 2));
    }

    hash
}

fn multiply_basis_function<F: FnMut(f64, f64) -> f64>(
    image: &image::RgbaImage,
    mut basis: F,
) -> Triplet {
    let mut r = 0f64;
    let mut g = 0f64;
    let mut b = 0f64;

    for (x, y, pixel) in image.enumerate_pixels() {
        let basis = basis(x as f64, y as f64);

        r += basis * srgb_linear(pixel.0[0]);
        g += basis * srgb_linear(pixel.0[1]);
        b += basis * srgb_linear(pixel.0[2]);
    }

    let scale = 1.0 / ((image.width() as f64) * (image.height() as f64));
    (r * scale, g * scale, b * scale)
}

// srgb -> linear
fn srgb_linear(value: u8) -> f64 {
    let v = (value as f64) / 255.0;
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

// linear -> srgb
fn linear_srgb(value: f64) -> u8 {
    let value = value.clamp(0.0, 1.0);
    if value <= 0.0031308 {
        (value * 12.92 * 255.0 + 0.5) as u8
    } else {
        ((value.powf(1.0 / 2.4) * 1.055 - 0.055) * 255.0 + 0.5) as u8
    }
}

static BASE83_ALPHABET: &[u8] =
    b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz#$%*+,-.:;=?@[]^_{|}~";

fn encode_base83(value: u32, length: usize) -> impl Iterator<Item = char> {
    (1..=length)
        .map(move |i| (value / 83u32.pow((length - i) as u32)) % 83)
        .map(|digit| (BASE83_ALPHABET[digit as usize] as char))
}

fn encode_dc(component: Triplet) -> u32 {
    let r = linear_srgb(component.0);
    let g = linear_srgb(component.1);
    let b = linear_srgb(component.2);

    ((r as u32) << 16) + ((g as u32) << 8) + (b as u32)
}

fn encode_ac(component: Triplet, max: f64) -> u32 {
    fn quant(value: f64, max: f64) -> u32 {
        (sign_pow(value / max, 0.5) * 9.0 + 9.5)
            .floor()
            .clamp(0.0, 18.0) as u32
    }
    let r = quant(component.0, max);
    let g = quant(component.1, max);
    let b = quant(component.2, max);
    r * 19 * 19 + g * 19 + b
}

fn sign_pow(value: f64, exp: f64) -> f64 {
    value.abs().powf(exp).copysign(value)
}
