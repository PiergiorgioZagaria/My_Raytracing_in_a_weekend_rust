mod hitables;
mod materials;
mod mylib;
mod vecmath;

use hitables::*;
use materials::*;
use mylib::*;

// use mylib::{HitableList,Camera};
use vecmath::Vec3;

use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use rayon::prelude::*;
use std::io::Write;
use std::sync::Arc;

const WIDTH: usize = 640;
const HEIGHT: usize = 320;
const SECS: usize = 65; //100

const USE_RANDOM_SCENE: bool = true;
const USE_MULTITHREADING: bool = true;

fn main() {
    let mut stdout = std::io::stdout();
    let now = std::time::Instant::now();
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let lookfrom = Vec3::new(13., 2., 3.);
    let lookat = Vec3::new(0., 0., 0.);
    let dist_to_focus = 10.;//(lookfrom - lookat).length();
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0., 1., 0.),
        20.,
        WIDTH as f32 / HEIGHT as f32,
        aperture,
        dist_to_focus,
    );

    let world;
    if !USE_RANDOM_SCENE {
        world = HitableList {
            list: vec![
                Box::new(Sphere::new(
                    Vec3::new(0., 0., -1.),
                    0.5,
                    Arc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))),
                )),
                Box::new(Sphere::new(
                    Vec3::new(0., -100.5, -1.),
                    100.,
                    Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.))),
                )),
                Box::new(Sphere::new(
                    Vec3::new(1., 0., -1.),
                    0.5,
                    Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.), 0.3)),
                )),
                Box::new(Sphere::new(
                    Vec3::new(-1., 0., -1.),
                    0.5,
                    Arc::new(Dieletric::new(1.5)),
                )),
                // Box::new(Sphere::new(Vec3::new(-1.,0.,-1.),-0.45,Materials::Dieletric(1.5))),
            ],
        };
    } else {
        world = HitableList::random_scene();
    }

    if USE_MULTITHREADING {
        buffer.par_iter_mut().enumerate().for_each(|(k, pixel)| {
            let i = k % WIDTH;
            let j = k / WIDTH;
            *pixel = u32::from(calc_col(i, j, &world, &cam) * 255.99);
        });
    } else {
        for j in 0..HEIGHT {
            let perc: f32 = j as f32 / HEIGHT as f32;
            let mut tmp = String::with_capacity(20);
            let tmpp = (20. * perc) as usize;
            for _ in 0..tmpp {
                tmp.push('=');
            }
            tmp.push('>');
            for _ in 0..(20 - tmpp - 1) {
                tmp.push(' ');
            }
            print!("\r[{}]{:.4}%", tmp, perc * 100.);
            stdout.flush().unwrap();

            for i in 0..WIDTH {
                buffer[i + j * WIDTH] = u32::from(calc_col(i, j, &world, &cam) * 255.99);
            }
        }
    }
    println!(
        "\nFinished rendering after {}s     ",
        now.elapsed().as_secs()
    );

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn calc_col(i: usize, j: usize, world: &HitableList, cam: &Camera) -> Vec3 {
    let mut col = Vec3::new(0., 0., 0.);

    // The following block is to apply antialiasing to the image,
    // We take random colors around us and average them, so that
    // color transitions are smoother
    let mut rng = rand::thread_rng();
    for _ in 0..SECS {
        let u = (i as f32 + rng.gen::<f32>()) / WIDTH as f32;
        let v = ((HEIGHT - 1 - j) as f32 + rng.gen::<f32>()) / HEIGHT as f32;
        let r = cam.get_ray(u, v);
        col += Vec3::color_material(&r, &world, 0);
    }
    col /= SECS as f32;

    // The following inreases the gamma, the guide mentions that
    // Image viewers lower the gamma making the picture appear
    // darker, with this we can increase the gamma value and
    // make it brighter
    col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
    col
}
