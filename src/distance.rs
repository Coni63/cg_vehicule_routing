use std::fmt;

use crate::city::City;

pub struct Distance {
    dist: Vec<Vec<u32>>,
}

impl Distance {
    pub fn new(cities: &[City]) -> Self {
        let n = cities.len();
        let mut dist = vec![vec![0; n]; n];
        for (i, ci) in cities.iter().enumerate() {
            for (j, cj) in cities.iter().skip(i).enumerate() {
                dist[i][i + j] = ci.distance(cj);
                dist[i + j][i] = dist[i][i + j];
            }
        }
        Distance { dist }
    }

    pub fn get(&self, i: usize, j: usize) -> u32 {
        self.dist[i][j]
    }
}

impl fmt::Debug for Distance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.dist {
            for value in row.iter() {
                write!(f, "{:.1} ", value)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
