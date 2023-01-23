use colorgrad::{CustomGradient, Gradient};
use image::RgbImage;
use num_complex::Complex;
use rayon::prelude::*;

struct Canvas {
    w: u32,
    h: u32,
    pallette: Gradient,
    buffer: RgbImage,
}
struct View {
    x0: f64,
    y0: f64,
    w: f64,
    h: f64,
}

impl Canvas {
    fn new(w: u32, h: u32, pallette: Gradient) -> Self {
        Canvas {
            w,
            h,
            pallette,
            buffer: RgbImage::new(w, h),
        }
    }
    fn save(&self, location: &str) {
        self.buffer.save(location).unwrap();
    }
}

fn pix_to_ccoord(pixel: (u32, u32), canvas_size: (u32, u32), view: &View) -> Complex<f64> {
    let re = (view.w / (canvas_size.0 as f64)) * (pixel.0 as f64) + view.x0;
    let im = -((view.h / (canvas_size.1 as f64)) * (pixel.1 as f64)) + view.y0;
    Complex::<f64> { re, im }
}

fn render_mandelbrot(canvas: &mut Canvas, view: &View, max_iter: u32, bound: u32) {
    let Canvas { w, h, pallette, .. } = canvas;
    canvas
        .buffer
        .enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            let ccoord = pix_to_ccoord((x, y), (*w, *h), view);
            let mut z = Complex { re: 0.0, im: 0.0 };
            let mut iter_count = 0;
            while z.norm() < bound as f64 && iter_count < max_iter {
                z = z * z + ccoord;
                iter_count += 1;
            }
            let (r, g, b, _) = pallette.at(iter_count as f64 / max_iter as f64).rgba_u8();
            *pixel = image::Rgb([r, g, b]);
        })
}

fn main() {
    let pallette = CustomGradient::new().domain(&[0.0, 1.0]).build().unwrap();
    // let pallette = colorgrad::viridis();

    let mut canvas = Canvas::new(20000, 10000, pallette);
    let view = View {
        x0: -2.5,
        y0: 1.0,
        w: 4.0,
        h: 2.0,
    };
    // let view = View { x0: 0.3, y0: 0.15, w: 0.1, h: 0.05 };
    render_mandelbrot(&mut canvas, &view, 250, 2);
    canvas.save("/home/qtqbpo/0.png");
}
