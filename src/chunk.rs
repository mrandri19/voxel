const AIR: usize = 0;
const COBBLESTONE: usize = 1;

pub struct Chunk {
    blocks: Vec<usize>,
}

const CHUNK_X_SIZE: usize = 96;
const CHUNK_Y_SIZE: usize = 96;
const CHUNK_Z_SIZE: usize = 96;

impl Chunk {
    // TODO(andrea): make this much much cooler.
    // See: Perlin noise, Simplex noise, Value noise, Gradient noise, fractional Brownian Motion
    // Support multiple chunks, and make terrain generation consistent
    pub fn new() -> Self {
        let blocks: Vec<usize> = vec![COBBLESTONE; CHUNK_X_SIZE * CHUNK_Y_SIZE * CHUNK_Z_SIZE];

        Self { blocks }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> usize {
        if (x >= self.x_blocks()) || (y >= self.y_blocks()) || (z >= self.z_blocks()) {
            return AIR;
        }
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
