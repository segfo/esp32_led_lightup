pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
    // 色空間の変換メソッド
    pub fn hsv2rgb(&mut self, h: u8, s: u8, v: u8) {
        let h = h % 255;
        let h = h as f64 / 255.0;
        let s = s as f64 / 255.0;
        let v = v as f64 / 255.0;
        let mut r = v;
        let mut g = v;
        let mut b = v;

        let mut h = h;
        if s > 0.0 {
            h *= 6.0;
            let i = h as u32;
            let f = h - (i as f64);
            match i {
                0 => {
                    g *= 1.0 - s * (1.0 - f);
                    b *= 1.0 - s;
                }
                1 => {
                    r *= 1.0 - s * f;
                    b *= 1.0 - s;
                }
                2 => {
                    r *= 1.0 - s;
                    b *= 1.0 - s * (1.0 - f);
                }
                3 => {
                    r *= 1.0 - s;
                    g *= 1.0 - s * f;
                }
                4 => {
                    r *= 1.0 - s * (1.0 - f);
                    g *= 1.0 - s;
                }
                5 => {
                    g *= 1.0 - s;
                    b *= 1.0 - s * f;
                }
                _ => {}
            }
        }
        self.r = (r * (255.0)) as u8;
        self.g = (g * (255.0)) as u8;
        self.b = (b * (255.0)) as u8;
    }
    pub fn to_rgb_array(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}