//! Lightning fast two dimensional perlin noise generation library using prime numbers.
//! It is small and efficient with zero dependencies.
//!
//! # Example
//!
//! ```no run
//! // For example, to generate a Perlin noise 2D terrain:
//! use perlin2d::PerlinNoise2D;
//!
//! let perlin_obj = PerlinNoise2D::new(6, 10.0, 0.5, 1.0, 2.0, (100.0, 100.0), 101);
//!
//! let noise = perlin_obj.get_noise(5.0, 10.0);
//! ```

/// Perlin Noise struct
///
/// Member variables:
///
/// * `octaves` - The amount of detail in Perlin noise.
/// * `amplitude` - The maximum absolute value that the Perlin noise can output.
/// * `frequeny` - The number of cycles per unit length that the Perlin noise outputs.
/// * `persistence` - A multiplier that determines how quickly the amplitudes diminish for each successive octave in a Perlin-noise function.
/// * `lacunarity` - A multiplier that determines how quickly the frequency increases for each successive octave in a Perlin-noise function.
/// * `scale` - A Tuple. A number that determines at what distance to view the noisemap.
/// * `seed` -  A value that changes the output of a coherent-noise function.
/// * `bias` - Amount of change in Perlin noise. Used , for example, to make all Perlin noise values positive.
///
/// Additional Info:
/// http://libnoise.sourceforge.net/glossary/
pub struct PerlinNoise2D {
    octaves: i32,
    amplitude: f64,
    frequency: f64,
    persistence: f64,
    lacunarity: f64,
    scale: (f64, f64),
    bias: f64,
    seed: i32,
}

impl PerlinNoise2D {
    /// Create and return a new PerlinNoise2D object
    pub fn new(
        octaves: i32,
        amplitude: f64,
        frequency: f64,
        persistence: f64,
        lacunarity: f64,
        scale: (f64, f64),
        bias: f64,
        seed: i32,
    ) -> Self {
        Self {
            octaves,
            amplitude,
            frequency,
            persistence,
            lacunarity,
            scale,
            bias,
            seed,
        }
    }

    /// Getter function for octaves
    pub fn get_octaves(&self) -> i32 {
        self.octaves
    }
    //// Getter function for amplitude
    pub fn get_amplitude(&self) -> f64 {
        self.amplitude
    }
    /// Getter function for frequency
    pub fn get_frequency(&self) -> f64 {
        self.frequency
    }
    /// Getter function for persistence
    pub fn get_persistence(&self) -> f64 {
        self.persistence
    }
    /// Getter function for lacunarity
    pub fn get_lacunarity(&self) -> f64 {
        self.lacunarity
    }
    /// Getter function for scale
    pub fn get_scale(&self) -> (f64, f64) {
        self.scale
    }
    /// Getter function for bias
    pub fn get_bias(&self) -> f64 {
        self.bias
    }
    /// Getter function for seed
    pub fn get_seed(&self) -> i32 {
        self.seed
    }

    /// Setter function for octaves
    pub fn set_octaves(&mut self, octaves: i32) {
        self.octaves = octaves;
    }
    /// Setter function for amplitude
    pub fn set_amplitude(&mut self, amplitude: f64) {
        self.amplitude = amplitude;
    }
    /// Setter function for frequency
    pub fn set_frequency(&mut self, frequency: f64) {
        self.frequency = frequency;
    }
    /// Setter function for persistence
    pub fn set_persistence(&mut self, persistence: f64) {
        self.persistence = persistence;
    }
    /// Setter function for lacunarity
    pub fn set_lacunarity(&mut self, lacunarity: f64) {
        self.lacunarity = lacunarity;
    }
    /// Setter function for scale
    pub fn set_scale(&mut self, scale: (f64, f64)) {
        self.scale = scale;
    }
    /// Setter function for bias
    pub fn set_bias(&mut self, bias: f64) {
        self.bias = bias;
    }
    /// Setter function for seed
    pub fn set_seed(&mut self, seed: i32) {
        self.seed = seed;
    }

    /// generates and returns 2D perlin noise
    pub fn get_noise(&self, x: f64, y: f64) -> f64 {
        self.bias + self.amplitude * self.total(x / self.scale.0, y / self.scale.1)
    }

    fn total(&self, x: f64, y: f64) -> f64 {
        let mut t = 0.0;
        let mut amp = 1.0;
        let mut freq = self.frequency;

        for _ in 0..self.octaves {
            t += self.get_value(y * freq + self.seed as f64, x * freq + self.seed as f64) * amp;
            amp *= self.persistence;
            freq *= self.lacunarity;
        }
        t
    }

    fn interpolate(&self, x: f64, y: f64, a: f64) -> f64 {
        let neg_a: f64 = 1.0 - a;
        let neg_a_sqr: f64 = neg_a * neg_a;
        let fac1: f64 = 3.0 * (neg_a_sqr) - 2.0 * (neg_a_sqr * neg_a);
        let a_sqr: f64 = a * a;
        let fac2: f64 = 3.0 * a_sqr - 2.0 * (a_sqr * a);

        x * fac1 + y * fac2 // add the weighted factors
    }

    fn noise(&self, x: i32, y: i32) -> f64 {
        let mut n: i32 = x + y * 57;
        n = (n << 13) ^ n;
        let t: i32 = (n * (n * n * 15731 + 789221) + 1376312589) & 0x7fffffff;
        1.0 - (t as f64) * 0.931322574615478515625e-9
    }

    fn get_value(&self, x: f64, y: f64) -> f64 {
        let x_int: i32 = x as i32;
        let y_int: i32 = y as i32;
        let x_frac: f64 = x - f64::floor(x);
        let y_frac: f64 = y - f64::floor(y);

        // noise values
        let n01: f64 = self.noise(x_int - 1, y_int - 1);
        let n02: f64 = self.noise(x_int + 1, y_int - 1);
        let n03: f64 = self.noise(x_int - 1, y_int + 1);
        let n04: f64 = self.noise(x_int + 1, y_int + 1);
        let n05: f64 = self.noise(x_int - 1, y_int);
        let n06: f64 = self.noise(x_int + 1, y_int);
        let n07: f64 = self.noise(x_int, y_int - 1);
        let n08: f64 = self.noise(x_int, y_int + 1);
        let n09: f64 = self.noise(x_int, y_int);

        let n12: f64 = self.noise(x_int + 2, y_int - 1);
        let n14: f64 = self.noise(x_int + 2, y_int + 1);
        let n16: f64 = self.noise(x_int + 2, y_int);

        let n23: f64 = self.noise(x_int - 1, y_int + 2);
        let n24: f64 = self.noise(x_int + 1, y_int + 2);
        let n28: f64 = self.noise(x_int, y_int + 2);

        let n34: f64 = self.noise(x_int + 2, y_int + 2);

        // find the noise values of the four corners
        let x0y0: f64 = 0.0625 * (n01 + n02 + n03 + n04) + 0.125 * (n05 + n06 + n07 + n08) + 0.25 * (n09);
        let x1y0: f64 = 0.0625 * (n07 + n12 + n08 + n14) + 0.125 * (n09 + n16 + n02 + n04) + 0.25 * (n06);
        let x0y1: f64 = 0.0625 * (n05 + n06 + n23 + n24) + 0.125 * (n03 + n04 + n09 + n28) + 0.25 * (n08);
        let x1y1: f64 = 0.0625 * (n09 + n16 + n28 + n34) + 0.125 * (n08 + n14 + n06 + n24) + 0.25 * (n04);

        // interpolate between those values according to the x and y fractions
        let v1: f64 = self.interpolate(x0y0, x1y0, x_frac); // interpolate in x
                                                            // direction (y)
        let v2: f64 = self.interpolate(x0y1, x1y1, x_frac); // interpolate in x
                                                            // direction (y+1)
        let fin: f64 = self.interpolate(v1, v2, y_frac); // interpolate in y direction

        return fin;
    }
}
