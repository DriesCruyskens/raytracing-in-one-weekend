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
        let i = (4.0*p.x) as usize & 255;
        let j = (4.0*p.y) as usize & 255;
        let k = (4.0*p.z) as usize & 255;

        self.ranfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }

    fn perlin_generate_perm() -> Perm {
        let mut p: Perm = [0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            p[i] = i as usize;
        };

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