use std::f64::consts::PI;

/// Projects a given LL coordinate at a specific zoom level into pixel screen-coordinates.
///
/// Zoom level is between 0 and 29 (inclusive). Every other zoom level will return a `None`.
///
/// ```rust
/// extern crate googleprojection;
///
/// let pixel = googleprojection::from_ll_to_pixel(&(13.2, 55.9), 2).unwrap();
///
/// assert_eq!(pixel.0, 550.0);
/// assert_eq!(pixel.1, 319.0);
/// ```
pub fn from_ll_to_pixel<T: Coord>(ll: &T, zoom: usize) -> Option<T> {
    if 30 > zoom {
        let c = 256.0 * 2.0_f64.powi(zoom as i32);
        let bc = c / 360.0;
        let cc = c / (2.0 * PI);

        let d = c / 2.0;
        let e = ((d + ll.x() * bc) + 0.5).floor();
        let f = ll.y().to_radians().sin().max(-0.9999).min(0.9999);
        let g = ((d + 0.5 * ((1.0+f)/(1.0-f)).ln() * -cc) + 0.5).floor();

        Some(T::with_xy(e, g))
    } else {
        None
    }
}

/// Projects a given pixel position at a specific zoom level into LL world-coordinates.
///
/// Zoom level is between 0 and 29 (inclusive). Every other zoom level will return a `None`.
///
/// ```rust
/// extern crate googleprojection;
///
/// let ll = googleprojection::from_pixel_to_ll(&(78.0, 78.0), 12).unwrap();
///
/// assert!((ll.0 - -179.9732208251953).abs() < 1e-10);
/// assert!((ll.1 - 85.04881808980566).abs() < 1e-10);
/// ```
pub fn from_pixel_to_ll<T: Coord>(px: &T, zoom: usize) -> Option<T> {
    if 30 > zoom {
        let c = 256.0 * 2.0_f64.powi(zoom as i32);
        let bc = c / 360.0;
        let cc = c / (2.0 * PI);

        let e = c / 2.0;
        let f = (px.x() - e) / bc;
        let g = (px.y() - e) / -cc;
        let h = (2.0 * g.exp().atan() - 0.5 * PI).to_degrees();

        Some(T::with_xy(f, h))
    } else {
        None
    }
}

/// A trait for everything that can be treated as a coordinate for a projection.
///
/// Implement this trait if you have a custom type to be able to project to and from it directly.
///
/// There exist an impl for this for `(f64, f64)` out of the box.
pub trait Coord {
    /// Return the first of the `f64` pair.
    fn x(&self) -> f64;

    /// Return the second of the `f64` pair.
    fn y(&self) -> f64;

    /// Construct a new Coord implementation from two `f64`.
    fn with_xy(f64, f64) -> Self;
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

        for answer in answers {
            let ll = answer.0;
            let zoom = answer.1;
            let expected = answer.2;

            let actual = super::from_ll_to_pixel(&ll, zoom).unwrap();

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

        for answer in answers {
            let pixel = answer.0;
            let zoom = answer.1;
            let expected = answer.2;

            let actual = super::from_pixel_to_ll(&pixel, zoom).unwrap();

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
        assert_eq!(
            super::from_ll_to_pixel(&(0.0, 0.0), 30),
            None
        );

        assert_eq!(
            super::from_pixel_to_ll(&(0.0, 0.0), 30),
            None
        );
    }
}
