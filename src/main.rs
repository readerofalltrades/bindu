use image::RgbImage;
use resvg::tiny_skia;
use resvg::usvg;

fn brightness(pixel: tiny_skia::PremultipliedColorU8) -> f32 {
    (0.299 * pixel.red() as f32 + 0.587 * pixel.green() as f32 + 0.114 * pixel.blue() as f32)
        / 255.0
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let svg_path = &args[1];
    let output_path = &args[2];

    let svg_data = std::fs::read_to_string(svg_path).unwrap();
    let tree = usvg::Tree::from_str(&svg_data, &usvg::Options::default()).unwrap();

    let svg_size = tree.size();
    println!("SVG size: {}x{}", svg_size.width(), svg_size.height());

    let width = svg_size.width() as u32;
    let height = svg_size.height() as u32;

    let mut pixmap = tiny_skia::Pixmap::new(width, height).unwrap();
    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

    let pixels = pixmap.pixels();
    let spacing = 15u32;
    let max_radius = 7.0f32;

    let mut output = RgbImage::new(width, height);

    for y in (0..height).step_by(spacing as usize) {
        for x in (0..width).step_by(spacing as usize) {
            let pixel = pixels[(y * width + x) as usize];
            let b = brightness(pixel);

            if b < 0.01 {
                continue;
            }

            let radius = b * max_radius;

            for py in (y as i32 - radius as i32)..=(y as i32 + radius as i32) {
                for px in (x as i32 - radius as i32)..=(x as i32 + radius as i32) {
                    if px < 0 || py < 0 || px >= width as i32 || py >= height as i32 {
                        continue;
                    }

                    let inside =
                        (px - x as i32).pow(2) + (py - y as i32).pow(2) <= (radius as i32).pow(2);

                    if inside {
                        output.put_pixel(px as u32, py as u32, image::Rgb([255, 255, 255]));
                    }
                }
            }
        }
    }

    output.save(output_path).unwrap();
    println!("Done! Saved to output_path");
}
