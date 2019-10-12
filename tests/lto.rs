#[test]
#[cfg(feature = "unstable-internal-test")]
fn opt_level_lto() {
    match reachability::tests::grey_box(1) {
        0 => reachability::unreachable_static!(),
        _ => (),
    }
}
