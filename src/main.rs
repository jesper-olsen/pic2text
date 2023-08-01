use image::{imageops, GenericImage, GenericImageView, ImageBuffer, RgbImage};
use std::collections::hash_map::{Entry, HashMap};
use std::collections::BinaryHeap;
pub mod font8x8;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    ///invert
    i: bool,
    #[arg(short, long, default_value_t = 80)]
    ///max picture width in characters  
    w: u32,
    #[arg(short, long, default_value_t = String::from("latin"))]
    ///range all/ascii/latin/greek/hiragana/box/sga
    r: String,
    #[arg(short, long, default_value_t = String::from(""))]
    ///each char in string
    u: String,
}
// TODO - hardcode example string - ascii, latin, all filtered for bits set
//
fn main() {
    let args = Args::parse();

    let mut heap = BinaryHeap::new();

//    if args.u.len() > 0 {
//        for c in args.u.chars() {
//            let u = c as u16;
//            let b = font8x8::unicode2bitmap(u);
//
//            heap.push((b.count_ones(), c, u));
//        }
//    } else {

    let mut s1: std::collections::HashSet<u8> = std::collections::HashSet::<u8>::new();
    for u in font8x8::UNICODE_LATIN {
        //  let u = c as u16;
        if let Some(c) = char::from_u32(u.into()) {
            //let c = char::frm_u32(u.into()).unwrap();
            let b = font8x8::unicode2bitmap(u);
            if b == 0 && u != 0x20 {
                continue;
            }

            let n: u8 = b.count_ones() as u8;
            if !s1.contains(&n) {
                s1.insert(n);
                heap.push((n, c, u));
            }
        }
    }
    let mut l: Vec<(u8, char)> = Vec::new();
    let mut n = 0;
    let mut last: i32 = -1;
    while !heap.is_empty() {
        if let Some((x, c, u)) = heap.pop() {
            if x as i32 != last && (x!=0 || u== 0x20) {
                last = x as i32;
                n += 1;
                l.push((x, c));
            }
        }
    }
    l.sort();
    let bmax = if let Some((x,_)) = l.last() {
        *x as f64
    } else {
        panic!("no usable symbols")
    };
    println!("#Symbols: {}", s1.len());
    println!("{l:?}");

    let mut i = 0;
    let mut j = 1;
    let mut pixel2char: Vec<char> = Vec::with_capacity(256);
    let mut last = ' ';
    while i < 256 {
        let (q, c) = l[j];
        if i as f64 / 256f64 >= q as f64 / bmax {
            last = c;
            j += 1;
        }
        pixel2char.push(last);
        i += 1;
    }

    let mut img = image::open("baimou.jpg").unwrap();
    let (width, height) = img.dimensions();

    if args.w > 0 && args.w < width {
        let h = args.w * height / width;
        println!("rescale {width},{height} -> {}, {h}", args.w);
        img = image::DynamicImage::resize(&img, args.w, h, imageops::FilterType::Lanczos3);
        imageops::invert(&mut img);
    };

    let (width, height) = img.dimensions();
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let a = (pixel[0] as usize + pixel[1] as usize + pixel[2] as usize)/3;
            let a = if args.i {
                256 - a
            } else {
                a
            };
            print!("{}", pixel2char[a]);
        }
        println!();
    }
}
