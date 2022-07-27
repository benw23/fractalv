extern crate minifb;
extern crate rayon;
extern crate num_complex;

use minifb::{Key, Window, WindowOptions};
use rayon::prelude::*;
use num_complex::Complex;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

struct FractalContext {
    dimensions: (usize, usize),
    pan: (f64, f64),
    scale: f64,
    updated: bool,
    pixels: Vec<u32>
}

impl FractalContext {
    fn new() -> FractalContext {
        FractalContext {
            dimensions: (WIDTH, HEIGHT),
            pan: (0.0, 0.0),
            scale: 100.,
            updated: true,
            pixels: vec![0; WIDTH * HEIGHT]
        }
    }
}

enum Fractal {
    Mandelbrot(usize),
    BurningShip(usize)
}

impl Fractal {
    fn render(&self, ctx: &mut FractalContext) {
        if !ctx.updated {return;};
        ctx.pixels.resize(ctx.dimensions.0 * ctx.dimensions.1, 0);
        
        match self {
            Fractal::Mandelbrot(max) => {
                Self::mandelbrot(ctx, *max);
            }
            Fractal::BurningShip(max) => {
                Self::burning_ship(ctx, *max);
            }
        }

        ctx.pixels[(ctx.dimensions.0 / 2)+(ctx.dimensions.1 / 2)*ctx.dimensions.0] = 0xFF0000;

        ctx.updated = false;
    }
    fn mandelbrot(ctx: &mut FractalContext, maxiter: usize) {
        (0..ctx.pixels.len()).into_par_iter().for_each(|i| {
            let (x, y) = ((i % ctx.dimensions.0) as f64 - (ctx.dimensions.0 as f64 / 2.), (i / ctx.dimensions.0) as f64 - (ctx.dimensions.1 as f64 / 2.));
            
            let c = Complex::new(x / ctx.scale + ctx.pan.0, y / ctx.scale + ctx.pan.1);
            let mut z = Complex::new(0., 0.);

            let mut escaped = 0;
            for _ in 0..maxiter {
                z = z * z + c;
                if z.norm_sqr() > 4. {escaped += 1;}
            }

            unsafe {
                let px_ptr = ctx.pixels.as_ptr() as *mut u32;

                *px_ptr.add(i) = ((escaped as f64 / maxiter as f64).sqrt() * 255.) as u32 * 0x010101;
            }
        });
    }

    fn burning_ship(ctx: &mut FractalContext, maxiter: usize) {
        (0..ctx.pixels.len()).into_par_iter().for_each(|i| {
            let (x, y) = ((i % ctx.dimensions.0) as f64 - (ctx.dimensions.0 as f64 / 2.), (i / ctx.dimensions.0) as f64 - (ctx.dimensions.1 as f64 / 2.));
            
            let c: Complex<f64> = Complex::new(x / ctx.scale + ctx.pan.0, y / ctx.scale + ctx.pan.1);
            let mut z: Complex<f64> = Complex::new(0., 0.);

            let mut escaped = 0;
            for _ in 0..maxiter {
                let abs_z = Complex::new(z.re.abs(), z.im.abs());  
                z = (abs_z * abs_z) + c;
                if z.norm_sqr() > 4. {escaped += 1;}
            }

            unsafe {
                let px_ptr = ctx.pixels.as_ptr() as *mut u32;

                *px_ptr.add(i) = ((escaped as f64 / maxiter as f64).sqrt() * 255.) as u32 * 0x010101;
            }
        });
    }
}

fn main() {
    let mut fract = Fractal::BurningShip(50);
    let mut ctx = FractalContext::new();

    let mut window = Window::new(
        "Fractal Viewer",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {

        if(window.is_key_down(Key::I)) {
            ctx.scale *= 1.1;
            ctx.updated = true;
        }

        if(window.is_key_down(Key::O)) {
            ctx.scale /= 1.1;
            ctx.updated = true;
        }

        if(window.is_key_down(Key::Up)) {
            ctx.pan.1 -= 1. / ctx.scale;
            ctx.updated = true;
        }

        if(window.is_key_down(Key::Down)) {
            ctx.pan.1 += 1. / ctx.scale;
            ctx.updated = true;
        }

        if(window.is_key_down(Key::Left)) {
            ctx.pan.0 -= 1. / ctx.scale;
            ctx.updated = true;
        }

        if(window.is_key_down(Key::Right)) {
            ctx.pan.0 += 1. / ctx.scale;
            ctx.updated = true;
        }

        fract.render(&mut ctx);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&ctx.pixels, WIDTH, HEIGHT)
            .unwrap();
    }
}