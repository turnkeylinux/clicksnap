use super::{generic::adminer, App};

pub const APP: App = App {
    pre: adminer::STEPS_PG,
    ..App::default()
};
