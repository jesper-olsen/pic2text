use clap::Parser;
use image::GenericImageView;
use std::array;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    ///invert grayscale
    invert: bool,

    #[arg(short, long, default_value_t = 2.0)]
    ///aspect ratio (character height to width)
    aratio: f64,

    #[arg(short, long, default_value_t = 80)]
    ///max picture width in characters  
    width: u32,

    ///image
    image: String,

    #[arg(short = 's', long, conflicts_with_all = ["ascii", "braille"])]
    /// Custom character set (ordered from dark to bright)
    symbols: Option<String>,

    #[arg(long, conflicts_with_all = ["symbols", "braille"])]
    /// Use ASCII character set (default)
    ascii: bool,

    #[arg(long, conflicts_with_all = ["symbols", "ascii"])]
    /// Use Braille character set for high resolution
    braille: bool,
}

fn main() {
    let args = Args::parse();

    let symbols: Vec<(u8, char)> = {
        if let Some(custom) = &args.symbols {
            // User-provided: assume already ordered, assign evenly spaced brightness
            let chars: Vec<char> = custom.chars().collect();
            let n = chars.len();
            chars
                .into_iter()
                .enumerate()
                .map(|(i, c)| {
                    let brightness = if n == 1 {
                        0u8
                    } else {
                        (i * 255 / (n - 1)) as u8
                    };
                    (brightness, c)
                })
                .collect()
        } else {
            let mut sym = if args.braille {
                (0x2800..=0x28FF)
                    .filter_map(|u| {
                        let c = char::from_u32(u)?;
                        let pattern = u - 0x2800;
                        let density = pattern.count_ones() as u8;
                        Some((density, c))
                    })
                    .collect::<Vec<_>>()
            } else {
                font8x8::UNICODE_ASCII
                    .filter_map(|u: u16| char::from_u32(u.into()).map(|c| (u, c)))
                    .filter_map(|(u, c)| {
                        let bitmap = font8x8::unicode2bitmap(u)?;
                        let density = bitmap.count_ones() as u8;
                        (density > 0 || c == ' ').then_some((density, c))
                    })
                    .collect::<Vec<_>>()
            };
            sym.sort_unstable_by_key(|&(brightness, _)| brightness);
            sym.dedup_by_key(|(brightness, _)| *brightness);
            sym
        }
    };

    let Some(&(density_max, _)) = symbols.last() else {
        panic!("no usable symbols")
    };
    println!("#Symbols: {}; {symbols:?}", symbols.len());

    // calculate ascii ramp; 1st symbol is space
    let mut j = 1;
    let mut last = ' ';
    let intensity2char: [char; 256] = array::from_fn(|i| {
        if j < symbols.len() {
            let (density, c) = symbols[j];
            if i as f64 / 255f64 >= density as f64 / density_max as f64 {
                last = c;
                j += 1;
            }
        }
        last
    });

    let mut img = image::open(args.image).expect("Failed to open the image file");

    if args.width > 0 && args.aratio >= 0.1 {
        // rescale image - take into account that characters are not square
        let (width, height) = img.dimensions();
        let h = (args.width * height) as f64 / (args.aratio * width as f64);
        img = img.resize_exact(
            args.width,
            h.round() as u32,
            image::imageops::FilterType::Lanczos3,
        );
    };

    let grayscale_img = img.to_luma8();
    let (width, height) = grayscale_img.dimensions();
    for y in 0..height {
        let row: String = (0..width)
            .map(|x| {
                let pixel = grayscale_img.get_pixel(x, y)[0] as usize;
                let intensity = if args.invert { 255 - pixel } else { pixel };
                intensity2char[intensity]
            })
            .collect();
        println!("{row}");
    }
}
