use std::fs::File;
use std::io::Write;

use rand::{random, Rng};

use rust_ray_tracing::engine::camera::Camera;
use rust_ray_tracing::engine::hittables::hittable::{HitRecord, Hittable};
use rust_ray_tracing::engine::hittables::hittable_collection::HittableCollection;
use rust_ray_tracing::engine::hittables::sphere::Sphere;
use rust_ray_tracing::engine::materials::lambertian::Lambertian;
use rust_ray_tracing::engine::materials::dielectric::Dielectric;

use rust_ray_tracing::engine::materials::material::{Material, ScatterResult};
use rust_ray_tracing::engine::Ray;
use rust_ray_tracing::engine::utils::random_float;
use rust_ray_tracing::utils::ppm_writer::PPMWriter;
use rust_ray_tracing::vectors::{Color, Point, Vector};
use rust_ray_tracing::consts::ASPECT_RATIO;
use rust_ray_tracing::engine::materials::metal::Metal;

fn ray_color<'a, 'b, T: Hittable<'a>>(ray: &Ray, world: &'b T, depth: usize) -> Color {
    let record = world.hit(ray, 0.001, f64::INFINITY);
    if depth <= 0 {
        return Color::zeroes();
    }

    match record {
        Some(record) => {
            let material = record.material.clone();
            let scatter_result = material.scatter(ray, &record);

            if let Some(scatter_result) = scatter_result {
                let ScatterResult { scattered, attenuation } = scatter_result;
                return attenuation * ray_color(&scattered, world, depth - 1);
            }

            Color::zeroes()
        }
        None => {
            let t = (ray.direction.unit().y + 1.0) / 2.0;
            let blue = Color::new(0.5, 0.7, 1.0);
            let white = Color::new(1.0, 1.0, 1.0);

            (1.0 - t) * white + t * blue
        }
    }
}

// fn generate_random_scene<'a>() -> HittableCollection<'a> {
//     let mut world = HittableCollection::new();
//     let ground_material: Box<dyn Material> = Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
//     let ground = Sphere::new(
//         Point::new(0.0, -1000.0, 0.0),
//         1000.0,
//         &ground_material
//     );
//
//     for a in -11..11 {
//         for b in -11..11 {
//             let choose_material = random_float(0.0, 1.0);
//             let center = Point::new(
//                 (a as f64) + 0.9 * random_float(0.0, 1.0),
//                 0.2,
//                 (b as f64) + 0.9 * random_float(0.0, 1.0)
//             );
//
//             if (center - Point::new(4.0, 0.2, 0.0)).size() > 0.9 {
//                 let sphere_material: Box<dyn Material>;
//                 if choose_material < 0.8 {
//                     // Diffuse
//                     let albedo = Color::random() * Color::random();
//                     sphere_material = Box::new(Lambertian::new(albedo));
//                     let sphere = Sphere::new(center, 0.2, &sphere_material);
//                     world.add(&sphere);
//                 }
//             }
//         }
//     }
//
//     world
// }

fn main() {
    // Image
    let aspect_ratio = ASPECT_RATIO;
    let image_width: usize = 400;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;

    // World
    let material_ground: Box<dyn Material> = Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center: Box<dyn Material> = Box::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left: Box<dyn Material> = Box::new(Dielectric::new(1.5));
    let material_right: Box<dyn Material> = Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    let ground = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, material_ground);
    let center_sphere = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, material_center);
    let left_sphere = Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, material_left);
    let right_sphere = Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, material_right);

    let mut world = HittableCollection::new();
    world.add(Box::new(ground));
    world.add(Box::new(center_sphere));
    world.add(Box::new(left_sphere));
    world.add(Box::new(right_sphere));

    // Camera
    let look_from = Point::new(3.0, 3.0, 2.0);
    let look_at = Point::new(0.0, 0.0, -1.0);
    let camera = Camera::new(
        look_from,
        look_at,
        Point::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        2.0,
        (look_from - look_at).size(),
    );

    // Render
    let samples_count = 80;
    let depth = 50;

    let mut writer = PPMWriter::get_file_writer("test.ppm");
    writer.write_size(image_height as usize, image_width as usize);

    for j in (0..image_height).rev() {
        for i in 0..(image_width) {
            let mut color = Color::zeroes();

            for _ in 0..samples_count {
                let random_bias_x = random_float(0.0, 1.0);
                let random_bias_y = random_float(0.0, 1.0);

                let x = (i as f64 + random_bias_x) / ((image_width - 1) as f64);
                let y = (j as f64 + random_bias_y) / ((image_height - 1) as f64);

                let ray = camera.get_ray(x, y);
                color = color + ray_color(&ray, &world, depth);
            }

            writer.write_color(color, samples_count);
        }
    }
}
