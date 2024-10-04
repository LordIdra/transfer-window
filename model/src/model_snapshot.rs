/// The state of the model at a given time from a given observer's point of view, lazily evaluated.
pub struct ModelSnapshot {
    model: *const Model,
    time: f64,
    observer: Faction,
}

impl ModelSnapshot {
    pub fn new(model: &Model, time: f64, observer: Faction) -> Self {
        let model = model as *const Model;
        Self { model, time, observer }
    }

    fn model(&mut self) -> &Model {
        unsafe { 
            &*self.model 
        }
    }
}

