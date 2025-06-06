use num::Complex;

pub struct FieldMap {
    pub re_resolution: usize,
    pub im_resolution: usize,
    pub precomputed_re: Vec<f64>,
    pub precomputed_im: Vec<f64>,
}

impl FieldMap {
    pub fn new(
        upper_left: Complex<f64>,
        lower_right: Complex<f64>,
        re_resolution: usize,
        im_resolution: usize,
    ) -> Self {
        let re_delta = (lower_right.re - upper_left.re) / re_resolution as f64;
        let im_delta = (upper_left.im - lower_right.im) / im_resolution as f64;

        let precomputed_re: Vec<f64> = (0..re_resolution)
            .map(|x| upper_left.re + x as f64 * re_delta)
            .collect();

        let precomputed_im: Vec<f64> = (0..im_resolution)
            .map(|y| upper_left.im - y as f64 * im_delta)
            .collect();

        FieldMap {
            re_resolution,
            im_resolution,
            precomputed_re,
            precomputed_im,
        }
    }

    #[inline(always)]
    pub fn get_point(&self, index: usize) -> Complex<f64> {
        let (x, y) = (index % self.re_resolution, index / self.re_resolution);

        Complex::new(self.precomputed_re[x], self.precomputed_im[y])
    }

    pub fn get_limit(&self) -> usize {
        self.re_resolution * self.im_resolution
    }

    #[inline(always)]
    pub fn escape_time(c: Complex<f64>, max_iters: usize) -> u8 {
        let mut z = Complex::new(0.0, 0.0);
        for i in 0..max_iters {
            if z.norm_sqr() > 4.0 {
                return ((max_iters - i) & 0xff) as u8;
            }
            z = z * z + c;
        }
        0
    }
}
