use image::{GenericImageView, ImageBuffer, Rgba};
use std::path::Path;
use std::thread;
use std::time::Instant;

const NUM_THREADS: u32 = 8; // Количество потоков

fn main() {
  let start_time = Instant::now();

  let img = image::open(&Path::new("original.jpg")).unwrap();
  let (width, height) = img.dimensions();

  let r = 1i32; // радиус по горизонталі
  let k = 1i32; // радиус по вертикалі

  // Создаем вектор для хранения дескрипторов потоков
  let mut thread_handles = vec![];

  for t in 0..NUM_THREADS {
    // Каждый поток обрабатывает свою часть изображения
    let start_y = (height as i32 / NUM_THREADS as i32 * t as i32) as u32;
    let end_y = (height as i32 / NUM_THREADS as i32 * (t + 1) as i32) as u32;

    // Клонируем изображение для каждого потока
    let img_clone = img.clone();

    let handle = thread::spawn(move || {
      let mut output_img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::new(width, end_y - start_y);

      for y in start_y..end_y {
        for x in 0..width {
          let mut rgba_sum = [0u32; 4];
          let mut count = 0;

          for ky in -k..=k {
            for rx in -r..=r {
              if x.wrapping_add(rx as u32) < width && y.wrapping_add(ky as u32) < height {
                let pixel = img_clone.get_pixel((x as i32 + rx) as u32, (y as i32 + ky) as u32);
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
              y - start_y,
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

      output_img
    });

    thread_handles.push(handle);
  }

  // Объединяем результаты всех потоков в одно изображение
  let mut output_img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::new(width, height);

  for (t, handle) in thread_handles.into_iter().enumerate() {
    let partial_img = handle.join().unwrap();
    for y in 0..partial_img.height() {
      for x in 0..width {
        output_img.put_pixel(
          x,
          y + (height / NUM_THREADS * t as u32),
          *partial_img.get_pixel(x, y),
        );
      }
    }
  }

  output_img.save("output.jpg").unwrap();

  let duration_time: std::time::Duration = start_time.elapsed();

  println!(
    "A program with {} threads processed the image in {:?}",
    NUM_THREADS, duration_time
  );
}
