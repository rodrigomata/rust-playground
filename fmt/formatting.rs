use std::fmt::{Display, Formatter, Result};

struct City {
    name: &'static str,
    latitude: f32,
    longitude: f32,
}

impl Display for City {
    fn fmt(&self, buffer: &mut Formatter) -> Result {
        let vertical_direction = if self.latitude >= 0.0 { 'N' } else { 'S' };
        let horizontal_direction = if self.longitude >= 0.0 { 'E' } else { 'W' };

        write!(
            buffer,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.latitude.abs(),
            vertical_direction,
            self.longitude.abs(),
            horizontal_direction
        )
    }
}

fn main() {
    for city in [
        City {
            name: "Dublin",
            latitude: 53.347778,
            longitude: -6.259722,
        },
        City {
            name: "Vancouver",
            latitude: 49.25,
            longitude: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", *city)
    }
}
