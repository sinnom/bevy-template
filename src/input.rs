use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

// This is the list of "things in the menu I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum MenuAction {
    GoBack,
    CursorNext,
    CursorPrevious,
    Select,
}
