// rwtodo "Rng" here is a trait. What is a trait? is "Write" a trait?
use glam::Vec3;
use rand::Rng; // Specified in the toml as "0.8.0", so it currently jumps to "0.8.5" put will never choose "0.9.0"
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

fn trace_ray(ray: &Ray, surfaces: Vec<Surface>) -> f32 {
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
                42.0 // rwtodo spawn a random ray in the hemisphere of the surface normal.
                     // rwtodo spawn multiple rays.
            }
        }
    } else {
        INFINITY
    }
}

fn main() {
    println!("begin");
    let surfaces = vec![Surface {
        position: Vec3::new(0.0, 0.0, 10.0),
        radius: 1.0,
        reflectivity: SurfaceReflectivity::Light,
    }];

    let first_ray = Ray {
        direction: Vec3::new(0.0, 0.0, 1.0),
        position: Vec3::new(0.0, 0.0, 0.0),
    };

    let intensity = trace_ray(&first_ray, surfaces);
    println!("intensity: {}", intensity);
    println!("end");
}

// rwtodo learn the ins and outs of specifying crate versions. e.g. rand
