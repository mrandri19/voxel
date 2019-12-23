pub struct Chunk {
    blocks: Vec<usize>,
}

const CHUNK_X_SIZE: usize = 64;
const CHUNK_Y_SIZE: usize = 64;
const CHUNK_Z_SIZE: usize = 64;

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
        let blocks: Vec<usize> = vec![0; CHUNK_X_SIZE * CHUNK_Y_SIZE * CHUNK_Z_SIZE];

        Self { blocks }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> usize {
        self.blocks[(z * self.y_blocks() * self.x_blocks()) + (y * self.x_blocks()) + x]
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
