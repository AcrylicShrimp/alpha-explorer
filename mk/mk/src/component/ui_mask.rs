use specs::{prelude::*, Component};

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct UIMask {
    pub render_itself: bool,
}
