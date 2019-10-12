#[test]
fn opt_level_1() {
    match [0].is_empty() {
        true => reachability::unreachable_static!(),
        _ => (),
    }
}
