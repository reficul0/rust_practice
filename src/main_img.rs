mod args;

use image::{DynamicImage, GenericImageView, ImageFormat};
use image::imageops::Triangle;
use image::io::Reader;
use args::Args;

#[derive(Debug)]
enum ImageReadingErrors {
    ImageFormatsDontMatch
}

struct FloatImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String
}

impl FloatImage {
    fn new(w: u32, h: u32, name: String) -> Self {
        let buf_capacity = h * w * 4/*RGB*/;
        let buffer = Vec::with_capacity(buf_capacity.try_into().unwrap());
        FloatImage { width: w, height: h, data: buffer, name }
    }
}

fn main() -> Result<(), ImageReadingErrors> {
    let args = Args::new();
    let (image_1, image_1_format) = find_image_from_path(args.image_1);
    let (image_2, image_2_format) = find_image_from_path(args.image_2);

    if image_1_format != image_2_format {
        return Err(ImageReadingErrors::ImageFormatsDontMatch);
    }

    let (image_1, image_2) = strardise_size(image_1, image_2);
    let output = FloatImage::new(image_1.width(), image_2.height(),
                                 args.output.into_os_string().into_string().unwrap());

    Ok(())
}

fn find_image_from_path<P>(path: P) -> (DynamicImage, ImageFormat)
where
    P: AsRef<std::path::Path>
{
    let image_reader = Reader::open(path).unwrap();
    let image_format = image_reader.format().unwrap();
    let image = image_reader.decode().unwrap();
    (image, image_format)
}

fn get_smallest_dimension<N1, N2>(d_1: (N1, N2), d_2: (N1, N2)) -> (N1, N2)
where
    N1: std::ops::Mul<N2, Output = N1> + PartialOrd + Copy,
    N2: std::ops::Mul<N1, Output = N1> + Copy
{
    let points_1 = d_1.0 * d_1.1;
    let points_2 = d_2.0 * d_2.1;
    return if points_1 < points_2 { d_1 } else { d_2 }
}

fn strardise_size(mut first: DynamicImage, mut second: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dimension(first.dimensions(), second.dimensions());
    println!("width: {width}, height:{height}");

    let img_to_resize: &mut DynamicImage = if first.dimensions() == (width, height) {&mut second} else {&mut first};
    *img_to_resize = img_to_resize.resize_exact(width, height, Triangle);

    (first, second)
}

