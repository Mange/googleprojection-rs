extern crate googleprojection;

#[test]
fn it_works() {
    let pixel = googleprojection::from_ll_to_pixel(&(13.2, 55.9), 2).unwrap();
    assert_eq!(pixel.0, 550.0);
    assert_eq!(pixel.1, 319.0);
}
