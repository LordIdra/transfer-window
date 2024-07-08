use std::collections::HashMap;

use crate::game::storyteller::story::{action::{CloseDialogueAction, ShowDialogueAction}, condition::ClickContinueCondition, state::State, transition::Transition, Story};

pub fn build() -> Story {
    let mut states = HashMap::new();
    states.insert("start", State::default()
        .with_transition(Transition::new("end", ClickContinueCondition::new()))
        .with_action(ShowDialogueAction::new("Jake", "CNI is a disaster")));
    states.insert("end", State::default()
        .with_action(CloseDialogueAction::new()));
    Story::new(states, "start")
}

#[cfg(test)]
mod test {
    use crate::game::storyteller::stories::test::check_story_integrity;

    use super::build;

    #[test]
    pub fn test_story_01_welcome() {
        check_story_integrity(build());
    }
}