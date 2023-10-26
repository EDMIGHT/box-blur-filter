use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

const NUM_THREADS: u32 = 1; // Количество потоков

fn main() {
  let start_time = Instant::now();
  let img = Arc::new(image::open(&Path::new("original.jpg")).unwrap());
  let (width, height) = img.dimensions();

  let r = 1; // радиус по горизонталі
  let k = 1; // радиус по вертикалі

  // Создаем вектор для хранения дескрипторов потоков
  let mut thread_handles = vec![];

  for t in 0..NUM_THREADS {
    // Каждый поток обрабатывает свою часть изображения
    let start_y = height / NUM_THREADS * t;
    let end_y = height / NUM_THREADS * (t + 1);

    // Клонируем Arc, а не само изображение
    let img_clone = Arc::clone(&img);

    let handle = thread::spawn(move || process_image_part(img_clone, width, start_y, end_y, r, k));

    thread_handles.push(handle);
  }

  // Объединяем результаты всех потоков в одно изображение
  let output_img = merge_images(thread_handles, width, height);

  output_img.save("output.jpg").unwrap();

  let duration_time: std::time::Duration = start_time.elapsed();

  println!(
    "A program with {} threads processed the image in {:?}",
    NUM_THREADS, duration_time
  );
}

fn process_image_part(
  img: Arc<DynamicImage>,
  width: u32,
  start_y: u32,
  end_y: u32,
  r: i32,
  k: i32,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
  let mut output_img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::new(width, end_y - start_y);

  for y in start_y..end_y {
    for x in 0..width {
      let (rgba_avg, count) = calculate_average_color(&img, x as i32, y as i32, r, k);
      if count > 0 {
        output_img.put_pixel(
          x,
          y - start_y,
          Rgba([
            (rgba_avg[0] / count) as u8,
            (rgba_avg[1] / count) as u8,
            (rgba_avg[2] / count) as u8,
            (rgba_avg[3] / count) as u8,
          ]),
        );
      }
    }
  }

  output_img
}

fn calculate_average_color(
  img: &image::DynamicImage,
  x: i32,
  y: i32,
  r: i32,
  k: i32,
) -> ([u32; 4], u32) {
  let mut rgba_sum = [0u32; 4];
  let mut count = 0;

  for ky in -k..=k {
    for rx in -r..=r {
      let new_x = x + rx;
      let new_y = y + ky;
      if new_x >= 0 && new_x < img.width() as i32 && new_y >= 0 && new_y < img.height() as i32 {
        let pixel = img.get_pixel(new_x as u32, new_y as u32);
        rgba_sum[0] += u32::from(pixel[0]);
        rgba_sum[1] += u32::from(pixel[1]);
        rgba_sum[2] += u32::from(pixel[2]);
        rgba_sum[3] += u32::from(pixel[3]);
        count += 1;
      }
    }
  }

  (rgba_sum, count)
}

fn merge_images(
  handles: Vec<thread::JoinHandle<ImageBuffer<Rgba<u8>, Vec<u8>>>>,
  width: u32,
  height: u32,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
  let mut output_img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::new(width, height);

  for (t, handle) in handles.into_iter().enumerate() {
    let partial_img = handle.join().unwrap();
    for y in 0..partial_img.height() {
      for x in 0..width {
        output_img.put_pixel(
          x,
          y + height / NUM_THREADS * t as u32,
          *partial_img.get_pixel(x, y),
        );
      }
    }
  }

  output_img
}
