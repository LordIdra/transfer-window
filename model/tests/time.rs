use transfer_window_model::model::Model;

#[test]
fn test_simple_time_step() {
    let mut model = Model::default();
    model.update(5.0);
    assert_eq!(model.time(), 5.0);
}

#[test]
fn test_time_step() {
    let mut model = Model::default();
    model.increase_time_step_level();
    model.update(1.0);
    assert_eq!(model.time(), model.time_step().time_step());
}

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
