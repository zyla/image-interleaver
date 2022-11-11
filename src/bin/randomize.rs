use anyhow::*;
use clap::Parser;
use image::{GenericImage, GenericImageView};
use std::cmp::min;
use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 64)]
    pixel_size: u32,

    file1: PathBuf,
    result_filename: PathBuf,
}

fn open_image(filename: &PathBuf) -> anyhow::Result<image::DynamicImage> {
    image::open(filename).with_context(|| anyhow!("opening image file {:?}", filename))
}

fn draw_rect(
    image: &mut image::RgbImage,
    xs: impl IntoIterator<Item = u32>,
    ys: impl IntoIterator<Item = u32> + Clone,
    color: image::Rgb<u8>,
) {
    for x in xs {
        let ys = ys.clone();
        for y in ys {
            image.put_pixel(x, y, color);
        }
    }
}

const BLACK: image::Rgb<u8> = image::Rgb([0, 0, 0]);
const WHITE: image::Rgb<u8> = image::Rgb([0xff, 0xff, 0xff]);

fn draw_big(
    target: &mut image::RgbImage,
    source: &image::RgbImage,
    offset_x: u32,
    offset_y: u32,
    pixel_size: u32,
) {
    for x in 0..=source.width() {
        draw_rect(
            target,
            [offset_x + x * pixel_size],
            offset_y..offset_y + pixel_size * source.height(),
            BLACK,
        );
    }
    for y in 0..=source.height() {
        draw_rect(
            target,
            offset_x..offset_x + pixel_size * source.width(),
            [offset_y + y * pixel_size],
            BLACK,
        );
    }
    for x in 0..source.width() {
        for y in 0..source.height() {
            draw_rect(
                target,
                offset_x + x * pixel_size+1..offset_x + (x + 1) * pixel_size,
                offset_y + y * pixel_size+1..offset_y + (y + 1) * pixel_size,
                *source.get_pixel(x, y),
            );
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let source = open_image(&args.file1)?.to_rgb8();

    let mut target1 = image::RgbImage::new(source.width(), source.height());
    let mut target2 = image::RgbImage::new(source.width(), source.height());
    let mut empty = image::RgbImage::new(source.width(), source.height());

    for x in 0..source.width() {
        for y in 0..source.height() {
            let val1: bool = rand::random();
            target1.put_pixel(x, y, if val1 { BLACK } else { WHITE });
            let val2: bool = (*source.get_pixel(x, y) == BLACK) ^ val1;
            target2.put_pixel(x, y, if val2 { BLACK } else { WHITE });
            empty.put_pixel(x, y, WHITE);
        }
    }

    let result_w = args.pixel_size * ((source.width() + 1) * 3 + 1);
    let result_h = args.pixel_size * (source.height() + 2);
    let mut result = image::RgbImage::new(result_w, result_h);

    draw_rect(&mut result, 0..result_w, 0..result_h, WHITE);

    draw_big(
        &mut result,
        &target1,
        args.pixel_size,
        args.pixel_size,
        args.pixel_size,
    );

    draw_big(
        &mut result,
        &target2,
        args.pixel_size * (2 + source.width()),
        args.pixel_size,
        args.pixel_size,
    );

    draw_big(
        &mut result,
        &empty,
        args.pixel_size * (3 + 2 * source.width()),
        args.pixel_size,
        args.pixel_size,
    );

    result.save(args.result_filename)?;

    Ok(())
}
