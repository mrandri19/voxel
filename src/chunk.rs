use gl::types::*;

pub const AIR: GLuint = 0;
pub const COBBLESTONE: GLuint = 1;

pub struct Chunk {
    pub blocks: Vec<GLuint>,
}

const CHUNK_X_SIZE: GLuint = 64;
const CHUNK_Y_SIZE: GLuint = 64;
const CHUNK_Z_SIZE: GLuint = 64;

impl Chunk {
    // TODO(andrea): make this much much cooler.
    // See: Perlin noise, Simplex noise, Value noise, Gradient noise, fractional Brownian Motion
    // Support multiple chunks, and make terrain generation consistent
    pub fn new() -> Self {
        let mut blocks: Vec<GLuint> =
            vec![AIR; (CHUNK_X_SIZE * CHUNK_Y_SIZE * CHUNK_Z_SIZE) as usize];

        // Create a half-air, half-cobblestone chunk
        for z in 0..(CHUNK_Z_SIZE / 16) {
            for y in 0..CHUNK_Y_SIZE {
                for x in 0..CHUNK_X_SIZE {
                    blocks[(z * CHUNK_Y_SIZE * CHUNK_X_SIZE + y * CHUNK_X_SIZE + x) as usize] =
                        COBBLESTONE;
                }
            }
        }
        for z in (CHUNK_Z_SIZE / 16)..(CHUNK_Z_SIZE / 8) {
            for y in (CHUNK_Y_SIZE / 16)..(CHUNK_Y_SIZE / 8) {
                for x in (CHUNK_Y_SIZE / 16)..(CHUNK_X_SIZE / 8) {
                    blocks[(z * CHUNK_Y_SIZE * CHUNK_X_SIZE + y * CHUNK_X_SIZE + x) as usize] =
                        COBBLESTONE;
                }
            }
        }

        Self { blocks }
    }

    #[inline(always)]
    pub fn get(&self, x: GLuint, y: GLuint, z: GLuint) -> GLuint {
        self.blocks[(z * self.y_blocks() * self.x_blocks() + y * self.x_blocks() + x) as usize]
    }
    pub fn x_blocks(&self) -> GLuint {
        CHUNK_X_SIZE
    }
    pub fn y_blocks(&self) -> GLuint {
        CHUNK_Y_SIZE
    }
    pub fn z_blocks(&self) -> GLuint {
        CHUNK_Z_SIZE
    }
}
