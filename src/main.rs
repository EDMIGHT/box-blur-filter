use image::{GenericImageView, ImageBuffer, Rgba};
use std::path::Path;

fn main() {
  let img = image::open(&Path::new("original.jpg")).unwrap();
  let (width, height) = img.dimensions();

  let mut output_img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::new(width, height);

  let r = 1i32; // радиус по горизонталі
  let k = 1i32; // радиус по вертикалі

  for y in 0..height {
    for x in 0..width {
      let mut rgba_sum = [0u32; 4];
      let mut count = 0;

      for ky in -k..=k {
        for rx in -r..=r {
          if (x as i32 + rx) >= 0
            && (x as i32 + rx) < width as i32
            && (y as i32 + ky) >= 0
            && (y as i32 + ky) < height as i32
          {
            let pixel = img.get_pixel((x as i32 + rx) as u32, (y as i32 + ky) as u32);
            rgba_sum[0] += pixel[0] as u32;
            rgba_sum[1] += pixel[1] as u32;
            rgba_sum[2] += pixel[2] as u32;
            rgba_sum[3] += pixel[3] as u32;
            count += 1;
          }
        }
      }

      if count > 0 {
        output_img.put_pixel(
          x,
          y,
          Rgba([
            (rgba_sum[0] / count) as u8,
            (rgba_sum[1] / count) as u8,
            (rgba_sum[2] / count) as u8,
            (rgba_sum[3] / count) as u8,
          ]),
        );
      }
    }
  }

  output_img.save("output.jpg").unwrap();
}
