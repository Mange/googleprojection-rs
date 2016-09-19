pub struct GoogleProjection {
    bc: Vec<f64>,
    cc: Vec<f64>,
    zc: Vec<(f64, f64)>,
    ac: Vec<f64>
}

pub trait Coord {
    fn x(&self) -> f64;
    fn y(&self) -> f64;

    fn with_xy(f64, f64) -> Self;
}

const PI: f64 = std::f64::consts::PI;

impl GoogleProjection {
    pub fn new() -> GoogleProjection {
        let mut bc = Vec::with_capacity(30);
        let mut cc = Vec::with_capacity(30);
        let mut zc = Vec::with_capacity(30);
        let mut ac = Vec::with_capacity(30);
        let mut c: f64 = 256.0;

        for _ in 0..30 {
            let e = c / 2.0;
            bc.push(c / 360.0);
            cc.push(c / (2.0 * PI));
            zc.push((e, e));
            ac.push(c);
            c *= 2.0;
        }

        GoogleProjection{
            bc: bc,
            cc: cc,
            zc: zc,
            ac: ac,
        }
    }

    pub fn from_ll_to_pixel<T: Coord>(&self, ll: &T, zoom: usize) -> Option<T> {
        if self.ac.len() > zoom {
            let d = self.zc[zoom];
            let e = ((d.0 + ll.x() * self.bc[zoom]) + 0.5).floor();
            let f = ll.y().to_radians().sin().max(-0.9999).min(0.9999);
            let g = ((d.1 + 0.5 * ((1.0+f)/(1.0-f)).ln() * -self.cc[zoom]) + 0.5).floor();
            Some(T::with_xy(e, g))
        } else {
            None
        }
    }

    pub fn from_pixel_to_ll<T: Coord>(&self, px: &T, zoom: usize) -> Option<T> {
        if self.ac.len() > zoom {
            let e = self.zc[zoom];
            let f = (px.x() - e.0) / self.bc[zoom];
            let g = (px.y() - e.1) / -self.cc[zoom];
            let h = (2.0 * g.exp().atan() - 0.5 * PI).to_degrees();
            Some(T::with_xy(f, h))
        } else {
            None
        }
    }
}

impl Coord for (f64, f64) {
    fn x(&self) -> f64 { self.0 }
    fn y(&self) -> f64 { self.1 }

    fn with_xy(x: f64, y: f64) -> (f64, f64) {
        (x, y)
    }
}

#[cfg(test)]
mod test {
    use Coord;

    const EPSILON: f64 = 1e-10;

    fn float_pair_close(pair: &(f64, f64), expected: &(f64, f64)) -> bool {
        ((pair.0 - expected.0).abs() < EPSILON) &&
            ((pair.1 - expected.1).abs() < EPSILON)
    }

    #[test]
    fn it_maps_coords_for_f64_tuple() {
        let coord: (f64, f64) = Coord::with_xy(45.0, 33.0);
        assert_eq!(coord.x(), 45.0);
        assert_eq!(coord.y(), 33.0);
    }

    #[test]
    fn it_projects_to_pixels() {
        use GoogleProjection;

        let answers = vec![
            ((0.0, 0.0), 0, (128.0, 128.0)),
            ((0.0, 0.0), 1, (256.0, 256.0)),
            ((0.0, 0.0), 29, (6.8719476736e10, 6.8719476736e10)),

            ((0.0, 1.0), 0, (128.0, 127.0)),
            ((1.0, 0.0), 0, (129.0, 128.0)),
            ((1.0, 1.0), 0, (129.0, 127.0)),

            ((5.5, 5.5), 5, (4221.0, 3971.0)),

            ((100.0, 54.0), 12, (815559.0, 336679.0)),

            ((-45.0, 12.0), 6, (6144.0, 7642.0)),
        ];

        let projection = GoogleProjection::new();

        for answer in answers {
            let ll = answer.0;
            let zoom = answer.1;
            let expected = answer.2;

            let actual = projection.from_ll_to_pixel(&ll, zoom).unwrap();

            assert!(
                float_pair_close(&actual, &expected),
                format!(
                    "Expected {:?} at zoom {} to be {:?} but was {:?}",
                    &ll, zoom, &expected, &actual
                )
            );
        }
    }

    #[test]
    fn it_projects_to_longlat() {
        use GoogleProjection;

        let answers = vec![
            ((128.0, 128.0), 0, (0.0, 0.0)),
            ((256.0, 256.0), 1, (0.0, 0.0)),
            ((6.8719476736e10, 6.8719476736e10), 29, (0.0, 0.0)),

            ((128.0, 127.0), 0, (0.0, 1.4061088354351594)),
            ((129.0, 128.0), 0, (1.40625, 0.0)),
            ((129.0, 127.0), 0, (1.40625, 1.4061088354351594)),

            ((20.0, 19.0), 0, (-151.875, 82.11838360691269)),

            ((78.0, 78.0), 12, (-179.9732208251953, 85.04881808980566)),

            ((-67.0, -100.0), 6, (-181.47216796875, 85.2371040233303)),
        ];

        let projection = GoogleProjection::new();

        for answer in answers {
            let pixel = answer.0;
            let zoom = answer.1;
            let expected = answer.2;

            let actual = projection.from_pixel_to_ll(&pixel, zoom).unwrap();

            assert!(
                float_pair_close(&actual, &expected),
                format!(
                    "Expected {:?} at zoom {} to be {:?} but was {:?}",
                    &pixel, zoom, &expected, &actual
                )
            );
        }
    }

    #[test]
    fn it_returns_none_when_zooming_too_far() {
        use GoogleProjection;

        let projection = GoogleProjection::new();
        assert_eq!(
            projection.from_ll_to_pixel(&(0.0, 0.0), 30),
            None
        );

        assert_eq!(
            projection.from_pixel_to_ll(&(0.0, 0.0), 30),
            None
        );
    }
}
