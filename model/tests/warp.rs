use transfer_window_model::Model;

#[test]
fn test_warp_1() {
    let mut model = Model::default();
    model.start_warp(100.0);
    model.update(10.0);
    assert!((model.time() - 95.0).abs() < 0.1);
}

#[test]
fn test_warp_2() {
    let mut model = Model::default();
    model.update(15.0);
    model.start_warp(100.0);
    model.update(7.0);
    assert!((model.time() - 95.0).abs() < 0.1);
}
