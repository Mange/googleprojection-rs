extern crate googleprojection;

use googleprojection::GoogleProjection;

#[test]
fn it_works() {
    let projection = GoogleProjection::new();
    let pixel = projection.from_ll_to_pixel(&(13.2, 55.9), 2).unwrap();
    assert_eq!(pixel.0, 550.0);
    assert_eq!(pixel.1, 319.0);
}

#[test]
fn it_can_be_shared_over_threads() {
    use std::thread;
    use std::sync::Arc;

    let projection = Arc::new(GoogleProjection::new());
    let mut threads = vec![];

    for _ in 0..3 {
        let reference = projection.clone();
        threads.push(thread::spawn(move || {
            reference.from_ll_to_pixel(&(13.2, 55.9), 2).unwrap();
        }));
    }

    for thread in threads {
        let _ = thread.join();
    }
}
