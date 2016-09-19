#![feature(test)]

extern crate test;
extern crate googleprojection;

use test::Bencher;

#[bench]
fn it_is_fast(b: &mut Bencher) {
    let ll = (67.34, 190.34);
    b.iter(|| {
        let px = googleprojection::from_ll_to_pixel(&ll, 25);
        googleprojection::from_pixel_to_ll(&px.unwrap(), 25)
    });
}
