const GRAD3: [[i8; 3]; 12] = [
    [1,1,0],[-1,1,0],[1,-1,0],[-1,-1,0],
    [1,0,1],[-1,0,1],[1,0,-1],[-1,0,-1],
    [0,1,1],[0,-1,1],[0,1,-1],[0,-1,-1]
];

const PERM: [u8; 512] = [
    151,160,137,91,90,15,131,13,201,95,96,53,194,233,7,225,140,36,103,30,69,142,8,99,37,240,21,10,23,190,6,148,247,120,234,75,0,26,197,62,94,252,219,203,117,35,11,32,57,177,33,88,237,149,56,87,174,20,125,136,171,168,68,175,74,165,71,134,139,48,27,166,77,146,158,231,83,111,229,122,60,211,133,230,220,105,92,41,55,46,245,40,244,102,143,54,65,25,63,161,1,216,80,73,209,76,132,187,208,89,18,169,200,196,135,130,116,188,159,86,164,100,109,198,173,186,3,64,52,217,226,250,124,123,5,202,38,147,118,126,255,82,85,212,207,206,59,227,47,16,58,17,182,189,28,42,223,183,170,213,119,248,152,2,44,154,163,70,221,153,101,155,167,43,172,9,129,22,39,253,19,98,108,110,79,113,224,232,178,185,112,104,218,246,97,228,251,34,242,193,238,210,144,12,191,179,162,241,81,51,145,235,249,14,239,107,49,192,214,31,181,199,106,157,184,84,204,176,115,121,50,45,127,4,150,254,138,236,205,93,222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,180,
    151,160,137,91,90,15,131,13,201,95,96,53,194,233,7,225,140,36,103,30,69,142,8,99,37,240,21,10,23,190,6,148,247,120,234,75,0,26,197,62,94,252,219,203,117,35,11,32,57,177,33,88,237,149,56,87,174,20,125,136,171,168,68,175,74,165,71,134,139,48,27,166,77,146,158,231,83,111,229,122,60,211,133,230,220,105,92,41,55,46,245,40,244,102,143,54,65,25,63,161,1,216,80,73,209,76,132,187,208,89,18,169,200,196,135,130,116,188,159,86,164,100,109,198,173,186,3,64,52,217,226,250,124,123,5,202,38,147,118,126,255,82,85,212,207,206,59,227,47,16,58,17,182,189,28,42,223,183,170,213,119,248,152,2,44,154,163,70,221,153,101,155,167,43,172,9,129,22,39,253,19,98,108,110,79,113,224,232,178,185,112,104,218,246,97,228,251,34,242,193,238,210,144,12,191,179,162,241,81,51,145,235,249,14,239,107,49,192,214,31,181,199,106,157,184,84,204,176,115,121,50,45,127,4,150,254,138,236,205,93,222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,180
];

pub fn fast_floor(x: f64) -> i32 {
    if x >= 0.0 { x as i32 } else { x as i32 - 1 }
}
    
fn dot(g: &[i8], x: f64, y: f64, z: f64) -> f64 {
    g[0] as f64 * x + g[1] as f64 * y + g[2] as f64 * z
}

pub fn simplex(xin: f64, yin: f64, zin: f64) -> f64 {
    let n0: f64;
    let n1: f64;
    let n2: f64;
    let n3: f64;

    const F3: f64 = 1.0 / 3.0;
    let s = (xin + yin + zin) * F3;
    let i = fast_floor(xin + s);
    let j = fast_floor(yin + s);
    let k = fast_floor(zin + s);

    const G3: f64 = 1.0 / 6.0;
    let t = (i + j + k) as f64 * G3;
    let x0 = xin - i as f64 + t;
    let y0 = yin - j as f64 + t;
    let z0 = zin - k as f64 + t;

    let (i1, j1, k1, i2, j2, k2) = if x0 >= y0 {
        if y0 >= z0 {
            (1, 0, 0, 1, 1, 0)
        } else if x0 >= z0 {
            (1, 0, 0, 1, 0, 1)
        } else {
            (0, 0, 1, 1, 0, 1)
        }
    } else {
        if y0 < z0 {
            (0, 0, 1, 0, 1, 1)
        } else if x0 < z0 {
            (0, 1, 0, 0, 1, 1)
        } else {
            (0, 1, 0, 1, 1, 0)
        }
    };

    let x1 = x0 - i1 as f64 + G3;
    let y1 = y0 - j1 as f64 + G3;
    let z1 = z0 - k1 as f64 + G3;

    let x2 = x0 - i2 as f64 + 2.0 * G3;
    let y2 = y0 - j2 as f64 + 2.0 * G3;
    let z2 = z0 - k2 as f64 + 2.0 * G3;

    let x3 = x0 - 1.0 + 3.0 * G3;
    let y3 = y0 - 1.0 + 3.0 * G3;
    let z3 = z0 - 1.0 + 3.0 * G3;

    let ii = (i & 255) as usize;
    let jj = (j & 255) as usize;
    let kk = (k & 255) as usize;

    let gi0 = PERM[ii + PERM[jj + PERM[kk] as usize] as usize] as usize % 12;
    let gi1 = PERM[ii + i1 + PERM[jj + j1 + PERM[kk + k1] as usize] as usize] as usize % 12;
    let gi2 = PERM[ii + i2 + PERM[jj + j2 + PERM[kk + k2] as usize] as usize] as usize % 12;
    let gi3 = PERM[ii + 1 + PERM[jj + 1 + PERM[kk + 1] as usize] as usize] as usize % 12;

    n0 = {
        let t0 = 0.6 - x0 * x0 - y0 * y0 - z0 * z0;
        if t0 < 0.0 {
            0.0
        } else {
            let t0_sq = t0 * t0;
            t0_sq * t0_sq * dot(&GRAD3[gi0], x0, y0, z0)
        }
    };

    n1 = {
        let t1 = 0.6 - x1 * x1 - y1 * y1 - z1 * z1;
        if t1 < 0.0 {
            0.0
        } else {
            let t1_sq = t1 * t1;
            t1_sq * t1_sq * dot(&GRAD3[gi1], x1, y1, z1)
        }
    };

    n2 = {
        let t2 = 0.6 - x2 * x2 - y2 * y2 - z2 * z2;
        if t2 < 0.0 {
            0.0
        } else {
            let t2_sq = t2 * t2;
            t2_sq * t2_sq * dot(&GRAD3[gi2], x2, y2, z2)
        }
    };

    n3 = {
        let t3 = 0.6 - x3 * x3 - y3 * y3 - z3 * z3;
        if t3 < 0.0 {
            0.0
        } else {
            let t3_sq = t3 * t3;
            t3_sq * t3_sq * dot(&GRAD3[gi3], x3, y3, z3)
        }
    };

    32.0 * (n0 + n1 + n2 + n3)
}

/// Linearly interpolate between `a` and `b` using a blend factor `t`.
fn lerp(a: f64, b: f64, t: f64) -> f64 {
    (1.0 - t) * a + t * b
}

fn random(x: f64, y: f64, z: f64) -> f64 {
    let seed = (x * 40_157.0 + y * 57_289.0 + z * 789_221.0 + 1_013.0) as u64;
    let m = 2u64.pow(48); // modulus
    let a = 25_214_903_917u64; // multiplier
    let c = 11u64; // increment

    // Basic LCG formula: (a * seed + c) % m
    let result = (a.wrapping_mul(seed).wrapping_add(c)) % m;

    // Normalize the result to [0, 1)
    (result as f64) / (m as f64)
}

/// Generate linear interpolated noise for the given `x`, `y`, and `z`.
pub fn lerp_noise(x: f64, y: f64, z: f64) -> f64 {
    let x_floor = (x as i64) as f64;
    let y_floor = (y as i64) as f64;
    let z_floor = (z as i64) as f64;

    let tx = (x - x_floor) as f64;  // fractional part of x
    let ty = (y - y_floor) as f64;  // fractional part of y
    let tz = (z - z_floor) as f64;  // fractional part of z

    // Optional: Smooth tx, ty, and tz for smoother curves
    let tx_smooth = tx * tx * (3.0 - 2.0 * tx);
    let ty_smooth = ty * ty * (3.0 - 2.0 * ty);
    let tz_smooth = tz * tz * (3.0 - 2.0 * tz);

    // Corners
    let c000 = random(x_floor, y_floor, z_floor);
    let c001 = random(x_floor, y_floor, z_floor + 1.0);
    let c010 = random(x_floor, y_floor + 1.0, z_floor);
    let c011 = random(x_floor, y_floor + 1.0, z_floor + 1.0);
    let c100 = random(x_floor + 1.0, y_floor, z_floor);
    let c101 = random(x_floor + 1.0, y_floor, z_floor + 1.0);
    let c110 = random(x_floor + 1.0, y_floor + 1.0, z_floor);
    let c111 = random(x_floor + 1.0, y_floor + 1.0, z_floor + 1.0);

    // Interpolate
    let cx00 = lerp(c000, c100, tx_smooth);
    let cx01 = lerp(c001, c101, tx_smooth);
    let cx10 = lerp(c010, c110, tx_smooth);
    let cx11 = lerp(c011, c111, tx_smooth);

    let cxy0 = lerp(cx00, cx10, ty_smooth);
    let cxy1 = lerp(cx01, cx11, ty_smooth);

    let cxyz = lerp(cxy0, cxy1, tz_smooth);

    cxyz
}


const GRID_SIZE: usize = 16;
const SPARSE_GRID_SIZE: usize = 4;
const CELL_SIZE: usize = GRID_SIZE / SPARSE_GRID_SIZE;

fn random_value(rng: &mut Rng) -> f64 {
    1.0 + get_rando_val(rng.random()) as f64 * 4.0 // generates a random value between 1 and 5
}

fn get_rando_val(value: u32) -> f64 {
    // Mask the top 52 bits
    let masked = value >> (32 - 23);
    // Convert to float and normalize
    masked as f64 / (1u32 << 23) as f64
}

fn bilinear_interpolate(f00: f64, f10: f64, f01: f64, f11: f64, x: f64, y: f64) -> f64 {
    let f_top = (1.0 - x) * f00 + x * f10;
    let f_bottom = (1.0 - x) * f01 + x * f11;
    (1.0 - y) * f_top + y * f_bottom
}

fn get_dense_grid() {
    let mut sparse_grid = [[0.0; SPARSE_GRID_SIZE + 1]; SPARSE_GRID_SIZE + 1];
        for i in 0..=SPARSE_GRID_SIZE {
            for j in 0..=SPARSE_GRID_SIZE {
                sparse_grid[i][j] = random_value(&mut self.rng);
            }
        }

        // 2. Fill the dense grid using bilinear interpolation
        let mut dense_grid = [[0.0; GRID_SIZE]; GRID_SIZE];
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                let cell_x = i / CELL_SIZE;
                let cell_y = j / CELL_SIZE;

                let x = (i % CELL_SIZE) as f64 / CELL_SIZE as f64;
                let y = (j % CELL_SIZE) as f64 / CELL_SIZE as f64;

                dense_grid[i][j] = bilinear_interpolate(
                    sparse_grid[cell_x][cell_y],
                    sparse_grid[cell_x + 1][cell_y],
                    sparse_grid[cell_x][cell_y + 1],
                    sparse_grid[cell_x + 1][cell_y + 1],
                    x,
                    y,
                );
            }
        }
}