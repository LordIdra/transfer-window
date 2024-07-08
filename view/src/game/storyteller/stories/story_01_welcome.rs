use crate::game::{overlay::dialogue::Dialogue, storyteller::story::{action::{CloseDialogueAction, ShowDialogueAction}, condition::ClickContinueCondition, state::State, transition::Transition, Story}};

pub fn build() -> Story {
    let mut story = Story::new("1");

    story.add(State::new("1")
        .transition(Transition::new("2", ClickContinueCondition::new()))
        .action(ShowDialogueAction::new(
            Dialogue::new("jake")
                .normal("Hello. Welcome to the Transfer Window command interface.")
            )
        )
    );

    story.add(State::new("2")
        .transition(Transition::new("3", ClickContinueCondition::new()))
        .action(ShowDialogueAction::new(
            Dialogue::new("jake")
                .normal("My name's Jake, and I'll be training you on the basics of using the interface. Once you've finished your training, you'll work to solve real-world tactical and strategic problems.")
            )
        )
    );

    story.add(State::new("3")
        .transition(Transition::new("4", ClickContinueCondition::new()))
        .action(ShowDialogueAction::new(
            Dialogue::new("jake")
                .normal("Please keep in mind that for now, this is a fully simulated environment, and your actions will have no real-world consequences. Now, let's begin.")
            )
        )
    );

    story.add(State::new("3")
        .transition(Transition::new("4", ClickContinueCondition::new()))
        .action(ShowDialogueAction::new(
            Dialogue::new("jake")
                .normal("Let's start with camera movement.")
                .normal("- ").bold("Right cick and drag ").normal("to move the camera\n")
                .normal("- ").bold("Scroll ").normal("to zoom in and out\n")
                .normal("Try it now.")
            )
        )
    );

    story.add(State::new("4")
        .action(CloseDialogueAction::new()));

    story
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