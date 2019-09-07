use std::f64::consts::PI;

pub struct Mercator {
    tile_size: f64,
}

impl Mercator {
    /// Create a new Mercator with custom tile size. Tile sizes must be a power of two (256, 512,
    /// and so on).
    pub fn with_size(tile_size: usize) -> Mercator {
        Mercator { tile_size: tile_size as f64 }
    }

    /// Projects a given LL coordinate at a specific zoom level into decimal sub-pixel screen-coordinates.
    ///
    /// Zoom level is between 0 and 29 (inclusive). Every other zoom level will return a `None`.
    pub fn from_ll_to_subpixel<T: Coord>(&self, ll: &T, zoom: usize) -> Option<T> {
        if 30 > zoom {
            let c = self.tile_size * 2.0_f64.powi(zoom as i32);
            let bc = c / 360.0;
            let cc = c / (2.0 * PI);

            let d = c / 2.0;
            let e = d + ll.x() * bc;
            let f = ll.y().to_radians().sin().max(-0.9999).min(0.9999);
            let g = d + 0.5 * ((1.0 + f) / (1.0 - f)).ln() * -cc;

            Some(T::with_xy(e, g))
        } else {
            None
        }
    }


    /// Projects a given LL coordinate at a specific zoom level into integer pixel screen-coordinates.
    ///
    /// Zoom level is between 0 and 29 (inclusive). Every other zoom level will return a `None`.
    pub fn from_ll_to_pixel<T: Coord>(&self, ll: &T, zoom: usize) -> Option<T> {
        match self.from_ll_to_subpixel(ll, zoom) {
            Some(subpixel) => Some(T::with_xy(
                (subpixel.x() + 0.5).floor(),
                (subpixel.y() + 0.5).floor()
            )),
            None => None
        }
    }

    /// Projects a given pixel position at a specific zoom level into LL world-coordinates.
    ///
    /// Zoom level is between 0 and 29 (inclusive). Every other zoom level will return a `None`.
    pub fn from_pixel_to_ll<T: Coord>(&self, px: &T, zoom: usize) -> Option<T> {
        if 30 > zoom {
            let c = self.tile_size * 2.0_f64.powi(zoom as i32);
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
}

impl Default for Mercator {
    fn default() -> Mercator {
        Mercator { tile_size: 256.0 }
    }
}

/// Projects a given LL coordinate at a specific zoom level into decimal pixel screen-coordinates using a
/// default tile size of 256.
///
/// Zoom level is between 0 and 29 (inclusive). Every other zoom level will return a `None`.
///
/// ```rust
/// extern crate googleprojection;
///
/// let subpixel = googleprojection::from_ll_to_subpixel(&(13.2, 55.9), 2).unwrap();
///
/// assert!((subpixel.0 - 549.5466666666666).abs() < 1e-10);
/// assert!((subpixel.1 - 319.3747774937304).abs() < 1e-10);
/// ```
pub fn from_ll_to_subpixel<T: Coord>(ll: &T, zoom: usize) -> Option<T> {
    Mercator::with_size(256).from_ll_to_subpixel(&ll, zoom)
}

/// Projects a given LL coordinate at a specific zoom level into pixel screen-coordinates using a
/// default tile size of 256.
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
    Mercator::with_size(256).from_ll_to_pixel(&ll, zoom)
}

/// Projects a given pixel position at a specific zoom level into LL world-coordinates using a
/// default tile size of 256.
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
    Mercator::with_size(256).from_pixel_to_ll(&px, zoom)
}

/// A trait for everything that can be treated as a coordinate for a projection.
///
/// Implement this trait if you have a custom type to be able to project to and from it directly.
///
/// There exist an impl for this for `(f64, f64)` out of the box (formatted as `(x, y)`, or `(lon,
/// lat)`).
pub trait Coord {
    /// Return the first of the `f64` pair.
    fn x(&self) -> f64;

    /// Return the second of the `f64` pair.
    fn y(&self) -> f64;

    /// Construct a new Coord implementation from two `f64`.
    fn with_xy(f64, f64) -> Self;
}

impl Coord for (f64, f64) {
    fn x(&self) -> f64 {
        self.0
    }
    fn y(&self) -> f64 {
        self.1
    }

    fn with_xy(x: f64, y: f64) -> (f64, f64) {
        (x, y)
    }
}

#[cfg(test)]
mod test {
    use Coord;

    const EPSILON: f64 = 1e-10;

    fn float_pair_close(pair: &(f64, f64), expected: &(f64, f64)) -> bool {
        ((pair.0 - expected.0).abs() < EPSILON) && ((pair.1 - expected.1).abs() < EPSILON)
    }

    #[test]
    fn it_maps_coords_for_f64_tuple() {
        let coord: (f64, f64) = Coord::with_xy(45.0, 33.0);
        assert_eq!(coord.x(), 45.0);
        assert_eq!(coord.y(), 33.0);
    }

    #[test]
    fn it_matches_google_maps_projection_extension() {
        let pixel_extension = 60.0;
        let starting_coord = (43.6532, -79.3832);

        // These data collected using Google Maps API OverlayView.getProjection().fromLatLngToDivPixel(), 
        // adding 60 to x, subtracting 60 from y, then converting back using OverlayView.getProjection().fromDivPixelToLatLng()
        let answers = vec![
            // ((lat, lng), zoom)
            ((50.800061065188856, -68.83632499999999),  3),
            ((47.34741387849921,  -74.10976249999999),  4),
            ((45.530626397270055, -76.74648124999999),  5),
            ((44.599495541698985, -78.06484062499999),  6),
            ((44.12824279122392,  -78.72402031249999),  7),
            ((43.891195023324286, -79.05361015624999),  8),
            ((43.77231589906095,  -79.21840507812499),  9),
            ((43.712787543711634, -79.30080253906249), 10),
            ((43.68300117005328,  -79.34200126953124), 11),
            ((43.66810243453164,  -79.36260063476561), 12),
            ((43.66065167963645,  -79.3729003173828),  13),
            ((43.65692595541019,  -79.3780501586914),  14),
            ((43.65506300660299,  -79.38062507934569), 15),
            ((43.65413151052596,  -79.38191253967284), 16),
            ((43.653665757069085, -79.38255626983641), 17),
            ((43.653432878986074, -79.3828781349182),  18),
            ((43.6533164396059,   -79.3830390674591),  19),
        ];

        for answer in answers {
            let expected_ll = answer.0;
            let expected = (expected_ll.1, expected_ll.0);
            let zoom = answer.1;

            let subpixel = super::from_ll_to_subpixel(&(starting_coord.1, starting_coord.0), zoom).unwrap();
            let actual = super::from_pixel_to_ll(&(subpixel.0 + pixel_extension, subpixel.1 - pixel_extension), zoom).unwrap();

            assert!(float_pair_close(&actual, &expected),
                    format!("Expected {:?} at zoom {} to be {:?} but was {:?}",
                            &starting_coord,
                            zoom,
                            &expected,
                            &actual));
        }
    }

    #[test]
    fn it_projects_to_subpixels() {
        let answers = vec![((0.0, 0.0), 0, (128.0, 128.0)),
                           ((0.0, 0.0), 1, (256.0, 256.0)),
                           ((0.0, 0.0), 29, (6.8719476736e10, 6.8719476736e10)),

                           ((0.0, 1.0), 0, (128.0, 127.2888527833)),
                           ((1.0, 0.0), 0, (128.7111111111, 128.0)),
                           ((1.0, 1.0), 0, (128.7111111111, 127.2888527833)),

                           ((5.5, 5.5), 5, (4221.1555555555, 3970.6517891289)),

                           ((100.0, 54.0), 12, (815559.1111111111, 336678.5009166745)),

                           ((-45.0, 12.0), 6, (6144.0, 7641.8296348480))];

        for answer in answers {
            let ll = answer.0;
            let zoom = answer.1;
            let expected = answer.2;

            let actual = super::from_ll_to_subpixel(&ll, zoom).unwrap();

            assert!(float_pair_close(&actual, &expected),
                    format!("Expected {:?} at zoom {} to be {:?} but was {:?}",
                            &ll,
                            zoom,
                            &expected,
                            &actual));
        }
    }

    #[test]
    fn it_projects_to_pixels() {
        let answers = vec![((0.0, 0.0), 0, (128.0, 128.0)),
                           ((0.0, 0.0), 1, (256.0, 256.0)),
                           ((0.0, 0.0), 29, (6.8719476736e10, 6.8719476736e10)),

                           ((0.0, 1.0), 0, (128.0, 127.0)),
                           ((1.0, 0.0), 0, (129.0, 128.0)),
                           ((1.0, 1.0), 0, (129.0, 127.0)),

                           ((5.5, 5.5), 5, (4221.0, 3971.0)),

                           ((100.0, 54.0), 12, (815559.0, 336679.0)),

                           ((-45.0, 12.0), 6, (6144.0, 7642.0))];

        for answer in answers {
            let ll = answer.0;
            let zoom = answer.1;
            let expected = answer.2;

            let actual = super::from_ll_to_pixel(&ll, zoom).unwrap();

            assert!(float_pair_close(&actual, &expected),
                    format!("Expected {:?} at zoom {} to be {:?} but was {:?}",
                            &ll,
                            zoom,
                            &expected,
                            &actual));
        }
    }

    #[test]
    fn it_projects_to_longlat() {
        let answers = vec![((128.0, 128.0), 0, (0.0, 0.0)),
                           ((256.0, 256.0), 1, (0.0, 0.0)),
                           ((6.8719476736e10, 6.8719476736e10), 29, (0.0, 0.0)),

                           ((128.0, 127.0), 0, (0.0, 1.4061088354351594)),
                           ((129.0, 128.0), 0, (1.40625, 0.0)),
                           ((129.0, 127.0), 0, (1.40625, 1.4061088354351594)),

                           ((20.0, 19.0), 0, (-151.875, 82.11838360691269)),

                           ((78.0, 78.0), 12, (-179.9732208251953, 85.04881808980566)),

                           ((-67.0, -100.0), 6, (-181.47216796875, 85.2371040233303))];

        for answer in answers {
            let pixel = answer.0;
            let zoom = answer.1;
            let expected = answer.2;

            let actual = super::from_pixel_to_ll(&pixel, zoom).unwrap();

            assert!(float_pair_close(&actual, &expected),
                    format!("Expected {:?} at zoom {} to be {:?} but was {:?}",
                            &pixel,
                            zoom,
                            &expected,
                            &actual));
        }
    }

    #[test]
    fn it_returns_none_when_zooming_too_far() {
        assert_eq!(super::from_ll_to_pixel(&(0.0, 0.0), 30), None);

        assert_eq!(super::from_pixel_to_ll(&(0.0, 0.0), 30), None);
    }

    #[test]
    fn it_projects_with_custom_size() {
        use super::Mercator;
        let mercator = Mercator::with_size(512);

        let ll = mercator.from_pixel_to_ll(&(512.0, 512.0), 1).unwrap();
        assert!(ll == (0.0, 0.0),
                format!("Pixels 512,512 is on LL 0,0 on zoom 1 on a mercator with 512 pixels \
                         per tile, but got result: {:?}",
                        ll));

        let px = mercator.from_ll_to_pixel(&(0.0, 0.0), 1).unwrap();
        assert!(px == (512.0, 512.0),
                format!("LL 0,0 is on pixels 512,512 on zoom 1 on a mercator with 512 pixels \
                         per tile, but got result: {:?}",
                        px));
    }
}
