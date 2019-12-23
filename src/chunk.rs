use rand::prelude::*;

pub struct Chunk {
    height_map: Vec<usize>,
}

const CHUNK_X_SIZE: usize = 32;
const CHUNK_Y_SIZE: usize = 32;
const CHUNK_Z_SIZE: usize = 32;

fn bilinear_interpolation(
    q11: f32,
    q12: f32,
    q21: f32,
    q22: f32,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
    x: f32,
    y: f32,
) -> f32 {
    let x2x1 = x2 - x1;
    let y2y1 = y2 - y1;
    let x2x = x2 - x;
    let y2y = y2 - y;
    let yy1 = y - y1;
    let xx1 = x - x1;
    return 1.0 / (x2x1 * y2y1)
        * (q11 * x2x * y2y + q21 * xx1 * y2y + q12 * x2x * yy1 + q22 * xx1 * yy1);
}

impl Chunk {
    // TODO(andrea): make this much much cooler.
    // See: Perlin noise, Simplex noise, Value noise, Gradient noise, fractional Brownian Motion
    // Support multiple chunks, and make terrain generation consistent
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let mut height_map: Vec<usize> = vec![0; CHUNK_X_SIZE * CHUNK_Y_SIZE * CHUNK_Z_SIZE];

        // let octave = 4;
        // for y in (0..CHUNK_Y_SIZE).step_by(CHUNK_Y_SIZE / octave) {
        //     for x in (0..CHUNK_X_SIZE).step_by(CHUNK_X_SIZE / octave) {
        //         height_map[y * CHUNK_X_SIZE + x] = rng.gen_range(0, 32 / octave);
        //     }
        // }

        // for y in 0..CHUNK_Y_SIZE {
        //     for x in 0..CHUNK_X_SIZE {
        //         if ((x % (CHUNK_X_SIZE / octave)) != 0) || ((y % (CHUNK_Y_SIZE / octave)) != 0) {
        //             let x1 = x - (x % (CHUNK_X_SIZE / octave));
        //             let y1 = y - (y % (CHUNK_Y_SIZE / octave));
        //             let x2 = x1 + CHUNK_X_SIZE / octave;
        //             let y2 = y1 + CHUNK_Y_SIZE / octave;

        //             let q11 = height_map[(y1 * CHUNK_X_SIZE + x1) % (CHUNK_X_SIZE * CHUNK_Y_SIZE)];
        //             let q12 = height_map[(y2 * CHUNK_X_SIZE + x1) % (CHUNK_X_SIZE * CHUNK_Y_SIZE)];

        //             let q21 = height_map[(y1 * CHUNK_X_SIZE + x2) % (CHUNK_X_SIZE * CHUNK_Y_SIZE)];
        //             let q22 = height_map[(y2 * CHUNK_X_SIZE + x2) % (CHUNK_X_SIZE * CHUNK_Y_SIZE)];

        //             height_map[y * CHUNK_X_SIZE + x] = bilinear_interpolation(
        //                 q11 as f32, q12 as f32, q21 as f32, q22 as f32, x1 as f32, x2 as f32,
        //                 y1 as f32, y2 as f32, x as f32, y as f32,
        //             ) as usize;
        //         }
        //     }
        // }

        Self { height_map }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> usize {
        self.height_map[z * self.y_blocks() * self.x_blocks() + y * self.x_blocks() + x]
    }
    pub fn x_blocks(&self) -> usize {
        CHUNK_X_SIZE
    }
    pub fn y_blocks(&self) -> usize {
        CHUNK_Y_SIZE
    }
    pub fn z_blocks(&self) -> usize {
        CHUNK_Z_SIZE
    }
}
