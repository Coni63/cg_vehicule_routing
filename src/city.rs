pub struct City {
    index: usize,
    x: i16,
    y: i16,
    demand: u16,
}

impl City {
    pub fn new(index: usize, x: i16, y: i16, demand: u16) -> City {
        City {
            index,
            x,
            y,
            demand,
        }
    }

    pub fn distance(&self, other: &City) -> u32 {
        let dx = self.x as f32 - other.x as f32;
        let dy = self.y as f32 - other.y as f32;
        (dx * dx + dy * dy).sqrt().round() as u32
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_demand(&self) -> u16 {
        self.demand
    }
}
