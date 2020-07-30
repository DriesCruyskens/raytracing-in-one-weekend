use crate::hit::{HitRecord, Hittable, HittableList, HittablePtr};
use crate::material::{Isotropic, Lambertian, Material};
use crate::ray::Ray;
use crate::texture::TexturePtr;
use rand::Rng;
use std::f64::consts::{E, PI};
use std::sync::Arc;
use vec3::{Color, Point3, Vec3};

type MaterialPtr = Arc<dyn Material + Send + Sync>;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: MaterialPtr,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat_ptr: MaterialPtr) -> Sphere {
        Sphere {
            center,
            radius,
            mat_ptr,
        }
    }

    /// Returns the u,v coordinates for a given point as a tuple (u,v).
    pub fn get_sphere_uv(&self, p: &Vec3) -> (f64, f64) {
        let phi = p.z.atan2(p.x);
        let theta = p.y.asin();
        (1.0 - (phi + PI) / (2.0 * PI), (theta + PI / 2.0) / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.origin - self.center;
        let a: f64 = r.direction.length_squared();
        let half_b: f64 = oc.dot(r.direction);
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut hit_record = HitRecord::default();

            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = r.at(hit_record.t);
                let outward_normal: Vec3 = (hit_record.p - self.center) / self.radius;
                hit_record.set_face_normal(r, &outward_normal);
                hit_record.mat_ptr = Arc::clone(&self.mat_ptr);
                let (u, v) = self.get_sphere_uv(&((hit_record.p - self.center) / self.radius));
                hit_record.u = u;
                hit_record.v = v;
                return Some(hit_record);
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = r.at(hit_record.t);
                let outward_normal: Vec3 = (hit_record.p - self.center) / self.radius;
                hit_record.set_face_normal(r, &outward_normal);
                hit_record.mat_ptr = Arc::clone(&self.mat_ptr);
                let (u, v) = self.get_sphere_uv(&((hit_record.p - self.center) / self.radius));
                hit_record.u = u;
                hit_record.v = v;
                return Some(hit_record);
            }
        }

        None
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere::new(Point3::default(), 1.0, Arc::new(Lambertian::default()))
    }
}

pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    t0: f64,
    t1: f64,
    radius: f64,
    mat_ptr: MaterialPtr,
}

impl MovingSphere {
    pub fn new(
        center0: Point3,
        center1: Point3,
        t0: f64,
        t1: f64,
        radius: f64,
        mat_ptr: MaterialPtr,
    ) -> Self {
        MovingSphere {
            center0,
            center1,
            t0,
            t1,
            radius,
            mat_ptr,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.center0 + (self.center1 - self.center0) * ((time - self.t0) / (self.t1 - self.t0))
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.origin - self.center(r.time);
        let a: f64 = r.direction.length_squared();
        let half_b: f64 = oc.dot(r.direction);
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut hit_record = HitRecord::default();

            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = r.at(hit_record.t);
                let outward_normal: Vec3 = (hit_record.p - self.center(r.time)) / self.radius;
                hit_record.set_face_normal(r, &outward_normal);
                hit_record.mat_ptr = Arc::clone(&self.mat_ptr);
                return Some(hit_record);
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = r.at(hit_record.t);
                let outward_normal: Vec3 = (hit_record.p - self.center(r.time)) / self.radius;
                hit_record.set_face_normal(r, &outward_normal);
                hit_record.mat_ptr = Arc::clone(&self.mat_ptr);
                return Some(hit_record);
            }
        }

        None
    }
}

pub struct XyRect {
    mp: MaterialPtr,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl XyRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mp: MaterialPtr) -> Self {
        XyRect {
            x0,
            x1,
            y0,
            y1,
            k,
            mp,
        }
    }
}

impl Hittable for XyRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin.z) / r.direction.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin.x + t * r.direction.x;
        let y = r.origin.y + t * r.direction.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let mut rec = HitRecord::new(
            r.at(t),
            Vec3::default(),
            t,
            (x - self.x0) / (self.x1 - self.x0),
            (y - self.y0) / (self.y1 - self.y0),
            true,
            Arc::clone(&self.mp),
        );
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}

pub struct XzRect {
    mp: MaterialPtr,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl XzRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mp: MaterialPtr) -> Self {
        XzRect {
            x0,
            x1,
            z0,
            z1,
            k,
            mp,
        }
    }
}

impl Hittable for XzRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin.y) / r.direction.y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin.x + t * r.direction.x;
        let z = r.origin.z + t * r.direction.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut rec = HitRecord::new(
            r.at(t),
            Vec3::default(),
            t,
            (x - self.x0) / (self.x1 - self.x0),
            (z - self.z0) / (self.z1 - self.z0),
            true,
            Arc::clone(&self.mp),
        );
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}

pub struct YzRect {
    mp: MaterialPtr,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl YzRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mp: MaterialPtr) -> Self {
        YzRect {
            y0,
            y1,
            z0,
            z1,
            k,
            mp,
        }
    }
}

impl Hittable for YzRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin.x) / r.direction.x;
        if t < t_min || t > t_max {
            return None;
        }
        let y = r.origin.y + t * r.direction.y;
        let z = r.origin.z + t * r.direction.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut rec = HitRecord::new(
            r.at(t),
            Vec3::default(),
            t,
            (y - self.y0) / (self.y1 - self.y0),
            (z - self.z0) / (self.z1 - self.z0),
            true,
            Arc::clone(&self.mp),
        );
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}

pub struct Cube {
    _cube_min: Point3,
    _cube_max: Point3,
    sides: HittableList,
}

impl Cube {
    pub fn new(p0: Point3, p1: Point3, mat: MaterialPtr) -> Self {
        let mut cube = Cube {
            _cube_min: p0,
            _cube_max: p1,
            sides: HittableList::default(),
        };

        cube.sides.add(Arc::new(XyRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
            Arc::clone(&mat),
        )));
        cube.sides.add(Arc::new(XyRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
            Arc::clone(&mat),
        )));

        cube.sides.add(Arc::new(XzRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
            Arc::clone(&mat),
        )));
        cube.sides.add(Arc::new(XzRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
            Arc::clone(&mat),
        )));

        cube.sides.add(Arc::new(YzRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
            Arc::clone(&mat),
        )));
        cube.sides.add(Arc::new(YzRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p0.x,
            Arc::clone(&mat),
        )));
        cube
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
}

pub struct ConstantMedium {
    boundary: HittablePtr,
    phase_function: MaterialPtr,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new_from_texture(boundary: HittablePtr, d: f64, a: TexturePtr) -> Self {
        ConstantMedium {
            boundary,
            phase_function: Arc::new(Isotropic::new_from_texture(a)),
            neg_inv_density: -1.0 / d,
        }
    }

    pub fn new_from_color(boundary: HittablePtr, d: f64, c: Color) -> Self {
        ConstantMedium {
            boundary,
            neg_inv_density: -1.0 / d,
            phase_function: Arc::new(Isotropic::new_from_color(c)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rng = rand::thread_rng();

        if let Some(mut rec1) = self.boundary.hit(r, f64::NEG_INFINITY, f64::INFINITY) {
            if let Some(mut rec2) = self.boundary.hit(r, rec1.t + 0.0001, f64::INFINITY) {
                if rec1.t < t_min {
                    rec1.t = t_min;
                }
                if rec2.t > t_max {
                    rec2.t = t_max;
                }
                if rec1.t > rec2.t {
                    return None;
                }
                if rec1.t < 0.0 {
                    rec1.t = 0.0;
                }

                let ray_length = r.direction.length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * rng.gen::<f64>().log(E);

                if hit_distance > distance_inside_boundary {
                    return None;
                }

                let mut rec = HitRecord::default();
                rec.t = rec1.t + hit_distance / ray_length;
                rec.p = r.at(rec.t);
                rec.normal = Vec3::new(1.0, 0.0, 0.0); // Arbitrary
                rec.front_face = true; // Arbitrary.
                rec.mat_ptr = Arc::clone(&self.phase_function);
                return Some(rec);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
}
