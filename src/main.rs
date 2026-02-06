use image::GenericImageView;
use std::collections::BinaryHeap;
pub mod font8x8;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    ///invert
    i: bool,
    #[arg(short, long, default_value_t = 2.0)]
    ///aspect ratio (character height to width)
    r: f64,
    #[arg(short, long, default_value_t = 80)]
    ///max picture width in characters  
    w: u32,
    #[arg(short, long, default_value_t = String::from(""))]
    ///each char in string
    u: String,
    ///image
    image: String,
}

fn main() {
    let args = Args::parse();

    // Sort and dedup symbols
    let mut symbols: Vec<(u8, char)> = if args.u.is_empty() {
        font8x8::UNICODE_ASCII
            .filter_map(|u| char::from_u32(u.into()).map(|c| (u, c)))
            .map(|(u, c)| (font8x8::unicode2bitmap(u).count_ones() as u8, c))
            .filter(|&(density, c)| density > 0 || c == ' ')
            .map(|(density, u)| (density, char::from_u32(u.into()).unwrap()))
            .collect()
    } else {
        args.u
            .chars()
            .map(|c| (font8x8::unicode2bitmap(c as u16).count_ones() as u8, c))
            .filter(|&(density, c)| density > 0 || c == ' ')
            .collect()
    };
    symbols.sort_unstable_by_key(|&(brightness, _)| brightness);
    symbols.dedup_by_key(|(brightness, _)| *brightness);

    let Some((bmax, _)) = symbols.last() else {
        panic!("no usable symbols")
    };
    println!("#Symbols: {}; {symbols:?}", symbols.len());

    let mut i = 0;
    let mut j = 1;
    let mut pixel2char: Vec<char> = Vec::with_capacity(256);
    let mut last = ' ';
    while i < 256 {
        let (q, c) = symbols[j];
        if i as f64 / 256f64 >= q as f64 / *bmax as f64 {
            last = c;
            j += 1;
        }
        pixel2char.push(last);
        i += 1;
    }

    let mut img = image::open(args.image).expect("Failed to open the image file");
    let (width, height) = img.dimensions();

    if args.w > 0 && args.r >= 0.1 {
        // rescale image - take into account that characters are not square
        //let h = args.w * height / width;
        let h = (args.w * height) as f64 / (args.r * width as f64);
        let h: u32 = h.round() as u32;
        if h >= 1 {
            img = image::DynamicImage::resize_exact(
                &img,
                args.w,
                h,
                image::imageops::FilterType::Lanczos3,
            );
        }
    };

    let (width, height) = img.dimensions();
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let a = (pixel[0] as usize + pixel[1] as usize + pixel[2] as usize) / 3;
            let a = if args.i { 255 - a } else { a };
            print!("{}", pixel2char[a]);
        }
        println!();
    }
}
