use crate::apps::{State, Step, Steps};
use futures::FutureExt;

// landing page present on most appliances

pub const STEPS: Steps = &[Step {
    name: "Landing page",
    desc: "Takes screenshot of the landing page of the appliance",
    screenshot: "landing",
    f: |st: &State| async { Ok(st.goto("/").await?) }.boxed(),
}];
