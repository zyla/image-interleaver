use clap::Parser;
use anyhow::*;
use std::path::PathBuf;
use std::cmp::min;
use image::{GenericImage,GenericImageView};

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 16)]
    num_segments: u32,

    file1: PathBuf,
    file2: PathBuf,
}

fn open_image(filename: &PathBuf) -> anyhow::Result<image::DynamicImage> {
    image::open(filename).with_context(|| anyhow!("opening image file {:?}", filename))
}

fn draw_vertical_line(image: &mut image::RgbImage, x: u32, y1: u32, y2: u32) {
    for y in y1..=y2 {
        image.put_pixel(x, y, image::Rgb([0,0,0]));
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let image1 = open_image(&args.file1)?.to_rgb8();
    let image2 = open_image(&args.file2)?.to_rgb8();

    if image1.width() != image2.width() {
        bail!("images must have the same width ({} != {})", image1.width(), image2.width());
    }

    if image1.height() != image2.height() {
        bail!("images must have the same height ({} != {})", image1.height(), image2.height());
    }

    let segment_width = image1.width() / args.num_segments;

    let mut result = image::RgbImage::new(2 * image1.width(), image1.height());

    let mut source_x = 0;
    while source_x < image1.width() {
        let this_segment_width = min(segment_width, image1.width() - source_x);
        result.copy_from(&*image1.view(source_x, 0, this_segment_width, image1.height()), 2 * source_x, 0)?;
        draw_vertical_line(&mut result, 2*source_x, 0, 10);
        draw_vertical_line(&mut result, 2*source_x, image1.height()-11, image1.height()-1);
        result.copy_from(&*image2.view(source_x, 0, this_segment_width, image1.height()), 2 * source_x + this_segment_width, 0)?;
        draw_vertical_line(&mut result, 2*source_x+this_segment_width, 0, 10);
        draw_vertical_line(&mut result, 2*source_x+this_segment_width, image1.height()-11, image1.height()-1);
        source_x += this_segment_width;
    }

    result.save("result.png")?;

    Ok(())
}
