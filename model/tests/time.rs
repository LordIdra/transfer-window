use transfer_window_model::Model;

#[test]
fn test_simple_time_step() {
    let mut model = Model::default();
    model.update(5.0);
    assert!(model.get_time() == 5.0);
}

#[test]
fn test_time_step() {
    let mut model = Model::default();
    model.increase_time_step_level();
    model.update(1.0);
    assert!(model.get_time() == model.get_time_step().get_time_step());
}