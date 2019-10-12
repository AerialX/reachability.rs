#[test]
fn opt_level_2() {
    match Some(1).take() {
        Some(0) => reachability::unreachable_static!(),
        _ => (),
    }
}

