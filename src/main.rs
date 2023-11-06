// rwtodo "Rng" here is a trait. What is a trait? is "Write" a trait?
use glam::Vec3;
use rand::Rng; // Specified in the toml as "0.8.0", so it currently jumps to "0.8.5" put will never choose "0.9.0"
use show_image::{create_window, ImageInfo, ImageView};
use std::{f32::INFINITY, vec::Vec};

// rwtodo can I put this declaration inside Surface?
enum SurfaceReflectivity {
    Light,
    Rough,
}

// rwtodo this will be come a SphereSurface, then we can wrap it in a Surface enum (Sphere, Triangle etc)
struct Surface {
    position: Vec3,
    radius: f32,
    reflectivity: SurfaceReflectivity,
}

impl Surface {
    // rwtodo i think I actually want to return the world pos of the intersection?
    fn intersection(&self, ray: &Ray) -> Option<f32> {
        // Project the sphere's position onto the ray direction
        let sphere_relative_pos = self.position - ray.position;
        let projection = sphere_relative_pos.dot(ray.direction);

        // Check the distance of the ray-relative position of projection against the radius
        let projection_relative_pos = ray.direction * projection;
        if projection_relative_pos.distance(sphere_relative_pos) <= self.radius {
            Some(42.0) // rwtodo return actual intersection. it's not the projection.
        } else {
            None
        }
    }
}

struct Ray {
    direction: Vec3, // rwtodo can I do something clever to enforce normalization?
    position: Vec3,
}

fn trace_ray(ray: &Ray, surfaces: &Vec<Surface>) -> f32 {
    // rwtodo I could combine these into an Option<struct> but I need to understand lifetime specifiers first.
    let mut closest_intersecting_surface_distance: f32 = INFINITY;
    let mut closest_intersecting_surface: Option<&Surface> = None;

    for surface in surfaces.iter() {
        // rwtodo skip the originating surface of the ray. how about having a Surface reference in the Ray struct?
        if let Some(intersection_distance) = surface.intersection(ray) {
            if intersection_distance < closest_intersecting_surface_distance {
                closest_intersecting_surface_distance = intersection_distance;
                closest_intersecting_surface = Some(surface);
            }
        }
    }

    // Spawn ray(s) at the nearest surface
    if let Some(surface) = closest_intersecting_surface {
        match surface.reflectivity {
            SurfaceReflectivity::Light => 1.0,
            SurfaceReflectivity::Rough => {
                let ray = Ray {
                    direction: random_normalized(),
                    position: Vec3::new(0.0, 0.0, 0.0), // rwtodo position of intersection
                };
                trace_ray(&ray, surfaces) // rwtodo spawn a random ray in the hemisphere of the surface normal.
                                          // rwtodo spawn multiple rays.
            }
        }
    } else {
        0.1 // rwtodo 0.0
    }
}

// rwtodo make hemisphere style
fn random_normalized() -> Vec3 {
    // rwtodo I'm not fond of this brute force approach
    loop {
        let mut v = Vec3::new(
            rand::random::<f32>() * 2.0 - 1.0,
            rand::random::<f32>() * 2.0 - 1.0,
            rand::random::<f32>() * 2.0 - 1.0,
        );

        let len = v.length();
        if len > 0.001 && len < 1.0 {
            return v.normalize(); // rwtodo can't do the shorthand return here and I don't know why
        }
    }
}

#[show_image::main]
fn main() {
    println!("begin");
    let surfaces = vec![
        Surface {
            position: Vec3::new(-20.0, 10.0, 10.0),
            radius: 10.0,
            reflectivity: SurfaceReflectivity::Light,
        },
        Surface {
            position: Vec3::new(20.0, -10.0, 10.0),
            radius: 10.0,
            reflectivity: SurfaceReflectivity::Rough,
        },
    ];

    let mut pixel_data = Vec::new();

    let width = 64;
    let height = 64;

    for y in 0..height {
        for x in 0..width {
            let ray = Ray {
                direction: Vec3::new(0.0, 0.0, 1.0),
                position: Vec3::new(
                    x as f32 - ((width as f32) / 2.0),
                    -(y as f32) + ((height as f32) / 2.0),
                    0.0,
                ),
            };
            let intensity = trace_ray(&ray, &surfaces);
            pixel_data.push((intensity * 255.0) as u8);
        }
    }

    let image = ImageView::new(ImageInfo::mono8(width, height), &pixel_data);
    let window = create_window("image", Default::default()).expect("Couldn't create window");
    window
        .set_image("image-001", image)
        .expect("Couldn't set image");

    std::thread::sleep(std::time::Duration::new(2, 0));

    println!("end");
}

// rwtodo learn the ins and outs of specifying crate versions. e.g. rand
