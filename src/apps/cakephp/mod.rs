use super::{generic::adminer, App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    pre: adminer::STEPS_MY,
    test: &[Step {
        name: "docs",
        desc: "shipped docs",
        f: |st: &State| {
            async {
                // hack to scroll to the bottom
                st.wait(By::XPath("//h3[contains(text(), 'Training')]"))
                    .await?
                    .scroll_into_view()
                    .await?;
                Ok(())
            }
            .boxed()
        },
        ..Step::default()
    }],
    ..App::default()
};
