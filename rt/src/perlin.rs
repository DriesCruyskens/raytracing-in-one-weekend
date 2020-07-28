use rand::Rng;
use vec3::{Point3, Vec3};

pub const POINT_COUNT: usize = 256;

pub type Perm = [usize; POINT_COUNT];
pub type RandVec = [Vec3; POINT_COUNT];

pub struct Perlin {
    ranvec: RandVec,
    perm_x: Perm,
    perm_y: Perm,
    perm_z: Perm,
}

impl Perlin {
    pub fn new() -> Self {
        let mut ranvec = [Vec3::default(); POINT_COUNT];

        for i in 0..POINT_COUNT {
            ranvec[i] = Vec3::random_range(-1.0, 1.0).unit_vector();
        }

        Perlin {
            ranvec,
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as usize;
        let j = p.y.floor() as usize;
        let k = p.z.floor() as usize;
        let mut c = [[[Vec3::default(); 2]; 2]; 3];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255]]
                }
            }
        }

        Perlin::perlin_interp(c, u, v, w)
    }

    fn perlin_interp(c: [[[Vec3; 2]; 2]; 3], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        let mut weight_v: Vec3;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * c[i][j][k].dot(weight_v);
                }
            }
        }
        accum
    }

    fn perlin_generate_perm() -> Perm {
        let mut p: Perm = [0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            p[i] = i as usize;
        }

        Perlin::permute(&mut p, POINT_COUNT);

        p
    }

    fn permute(p: &mut Perm, n: usize) {
        let mut rng = rand::thread_rng();
        for i in (1..n).rev() {
            let target = rng.gen_range(0, i);
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }
}
