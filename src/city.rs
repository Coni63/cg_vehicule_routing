pub struct City {
    index: u16,
    x: i16,
    y: i16,
    demand: u16,
}

impl City {
    pub fn new(index: u16, x: i16, y: i16, demand: u16) -> City {
        City {
            index,
            x,
            y,
            demand,
        }
    }

    pub fn distance(&self, other: &City) -> f32 {
        let dx = self.x as f32 - other.x as f32;
        let dy = self.y as f32 - other.y as f32;
        (dx * dx + dy * dy).sqrt()
    }
}
