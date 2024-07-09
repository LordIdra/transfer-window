pub mod story_01_welcome;

#[cfg(test)]
pub(crate) mod test {
    use std::collections::HashSet;

    use crate::game::storyteller::story::Story;

    pub fn check_story_integrity(story: Story) {
        let mut unvisited: HashSet<&str> = story.states().keys().into_iter().cloned().collect();
        let mut to_visit = Vec::new();
        to_visit.push("uninitialized");
        while let Some(state_string) = to_visit.pop() {
            unvisited.remove(state_string);
            let state = story.states().get(state_string).expect(&format!("State {} does not exist", state_string));
            for transition in state.transitions().iter() {
                if unvisited.contains(transition.to()) {
                    to_visit.push(&transition.to());
                }
            }
        }
        if !unvisited.is_empty() {
            panic!("Unreachable states found: {:?}", unvisited);
        }
    }
}