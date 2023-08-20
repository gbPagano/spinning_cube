pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub mesh: char,
}

pub struct Cube {
    pub points: Vec<Point>,
}

impl Cube {
    pub fn new(size: i16) -> Self {
        let mut points: Vec<Point> = Vec::new();
        let z = size;
        for x in -size..=size {
            for y in -size..=size {
                points.push(Point {
                    x: x.into(),
                    y: y.into(),
                    z: z.into(),
                    mesh: '#',
                });
                points.push(Point {
                    x: (-z).into(),
                    y: y.into(),
                    z: x.into(),
                    mesh: '+',
                });
                points.push(Point {
                    x: z.into(),
                    y: y.into(),
                    z: (-x).into(),
                    mesh: '%',
                });
                points.push(Point {
                    x: (-x).into(),
                    y: y.into(),
                    z: (-z).into(),
                    mesh: '*',
                });
                points.push(Point {
                    x: x.into(),
                    y: z.into(),
                    z: (-y).into(),
                    mesh: '$',
                });
                points.push(Point {
                    x: x.into(),
                    y: (-z).into(),
                    z: y.into(),
                    mesh: '@',
                });
            }
        }
        Cube { points }
    }
}
