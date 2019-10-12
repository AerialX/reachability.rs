#[test]
fn never() {
    if std::env::var_os("I EXPECT THIS TO NOT EXIST KTHX").is_some() {
        reachability::unreachable_static!()
    }
}
