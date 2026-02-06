use image::GenericImageView;
use std::array;
pub mod font8x8;
use clap::Parser;

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
    #[arg(short, long, default_value_t = String::from(""))]
    ///each char in string
    symbols: String,
    ///image
    image: String,
}

fn main() {
    let args = Args::parse();

    // Sort and dedup symbols
    let mut symbols: Vec<(u8, char)> = if args.symbols.is_empty() {
        font8x8::UNICODE_ASCII
            .filter_map(|u| {
                let c = char::from_u32(u.into())?;
                let density = font8x8::unicode2bitmap(u).count_ones() as u8;
                if density > 0 || c == ' ' {
                    Some((density, c))
                } else {
                    None
                }
            })
            .collect()
    } else {
        args.symbols
            .chars()
            .map(|c| (font8x8::unicode2bitmap(c as u16).count_ones() as u8, c))
            .filter(|&(density, c)| density > 0 || c == ' ')
            .collect()
    };
    symbols.sort_unstable_by_key(|&(brightness, _)| brightness);
    symbols.dedup_by_key(|(brightness, _)| *brightness);

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
