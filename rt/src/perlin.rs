use rand::Rng;
use vec3::Point3;

pub const POINT_COUNT: usize = 256;

pub type Perm = [usize; POINT_COUNT];
pub type RandFloat = [f64; POINT_COUNT];

pub struct Perlin {
    ranfloat: RandFloat,
    perm_x: Perm,
    perm_y: Perm,
    perm_z: Perm,
}

fn trilinear_interp(c: [[[f64; 2]; 2]; 3], u: f64, v: f64, w: f64) -> f64 {
    let mut accum = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                accum += (i as f64 * u + (1.0 - i as f64) * (1.0 - u))
                    * (j as f64 * v + (1.0 - j as f64) * (1.0 - v))
                    * (k as f64 * w + (1.0 - k as f64) * (1.0 - w))
                    * c[i][j][k];
            }
        }
    }
    accum
}

impl Perlin {
    pub fn new() -> Self {
        let mut ranfloat = [0.0; POINT_COUNT];
        let mut rng = rand::thread_rng();
        for i in 0..POINT_COUNT {
            ranfloat[i] = rng.gen::<f64>();
        }

        Perlin {
            ranfloat,
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let mut u = p.x - p.x.floor();
        let mut v = p.y - p.y.floor();
        let mut w = p.z - p.z.floor();

        u = u * u * (3.0 - 2.0 * u);
        v = v * v * (3.0 - 2.0 * v);
        w = w * w * (3.0 - 2.0 * w);

        let i = p.x.floor() as usize;
        let j = p.y.floor() as usize;
        let k = p.z.floor() as usize;
        let mut c = [[[0.0; 2]; 2]; 3];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranfloat[self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255]]
                }
            }
        }

        trilinear_interp(c, u, v, w)
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
