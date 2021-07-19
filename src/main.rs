mod mylib;
mod vecmath;
use mylib::{Materials,HitableList,Sphere,Camera};
use vecmath::Vec3;
use minifb::{Key, Window, WindowOptions};
use rand::Rng;

const WIDTH: usize = 1200;
const HEIGHT: usize = 600;
const SECS: usize = 100;
fn main() {
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

    let cam = Camera::new(Vec3::new(0.,0.,0.),Vec3::new(-2.,-1.,-1.),Vec3::new(4.,0.,0.),Vec3::new(0.,2.,0.));
    // let cam = Camera::new(Vec3::new(0.,0.,2.),Vec3::new(-2.,-1.,1.),Vec3::new(4.,0.,0.),Vec3::new(0.,2.,0.));
    // let cam = Camera::new(Vec3::new(3.,0.,-1.),Vec3::new(2.,-1.,1.),Vec3::new(4.,0.,0.),Vec3::new(0.,2.,0.));
    let world = HitableList {
        list: vec![
            Box::new(Sphere::new(Vec3::new(0.,0.,-1.),0.5,Materials::Lambertian(Vec3::new(0.8,0.3,0.3)))),
            Box::new(Sphere::new(Vec3::new(0.,-100.5,-1.),100.,Materials::Lambertian(Vec3::new(0.8,0.8,0.)))),
            Box::new(Sphere::new(Vec3::new(1.,0.,-1.),0.5,Materials::Metal(Vec3::new(0.8,0.6,0.2),0.3))),
            Box::new(Sphere::new(Vec3::new(-1.,0.,-1.),0.5,Materials::Dieletric(1.5))),
            Box::new(Sphere::new(Vec3::new(-1.,0.,-1.),-0.45,Materials::Dieletric(1.5))),
        ],
    };

    // TODO maybe make it multi-threaded?
    // r and world as Arc and use mpsc for the buffer?
    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let mut col = Vec3::new(0.,0.,0.);

            // The following block is to apply antialiasing to the image,
            // We take random colors around us and average them, so that 
            // color transitions are smoother
            let mut rng = rand::thread_rng();
            for _ in 0..SECS {
                let u = (i as f32 + rng.gen::<f32>())/ WIDTH as f32;
                let v = ((HEIGHT - 1 - j) as f32 + rng.gen::<f32>()) / HEIGHT as f32;
                let r = cam.get_ray(u,v);
                col += Vec3::color_material(&r,&world,0);
            }
            col /= SECS as f32;

            // The following inreases the gamma, the guide mentions that
            // Image viewers lower the gamma making the picture appear
            // darker, with this we can increase the gamma value and 
            // make it brighter
            col = Vec3::new(col.x.sqrt(),col.y.sqrt(),col.z.sqrt());

            buffer[i + j * WIDTH] = u32::from(col * 255.99);
        }
    }
    println!("Finished rendering after {}s",now.elapsed().as_secs());

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

