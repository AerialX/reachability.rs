#![cfg_attr(feature = "unstable", feature(test))]

#[test]
#[cfg(feature = "unstable")]
fn black_box() {
    extern crate test;
    use test::black_box;

    match black_box(Some(1)) {
        Some(0) => reachability::unreachable_static!(),
        _ => (),
    }
}

#[test]
fn black_box_stable() {
    fn blackish_box<T>(dummy: T) -> T {
        unsafe {
            let ret = std::ptr::read_volatile(&dummy);
            std::mem::forget(dummy);
            ret
        }
    }

    match blackish_box(Some(1)) {
        Some(0) => reachability::unreachable_static!(),
        _ => (),
    }
}
