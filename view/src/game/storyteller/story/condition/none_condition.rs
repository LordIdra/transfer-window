use crate::game::View;

use super::ConditionCheck;

pub struct NoneCondition;

impl NoneCondition {
    pub fn new() -> Box<dyn ConditionCheck> {
        Box::new(Self {})
    }
}

impl ConditionCheck for NoneCondition {
    fn met(&self, _view: &View) -> bool {
        true
    }
}

