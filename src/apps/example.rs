use super::{App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

// NOTE: this is an example app steps definition
// you can use it as a base for writing new scripts
// it should not actually be used as-is

pub const _EXAMPLE_APP: App = App {
    test: &[
        Step {
            name: "dummy-step-1",
            desc: "dummy description 1",
            f: |st: &State| {
                async {
                    st.goto("/").await?;
                    st.goto_port(12345, "/").await?;
                    // your code goes here
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "dummy-step-2",
            desc: "dummy description 2",
            screenshot: "not-dummy-step-2",
            f: |st: &State| {
                async {
                    st.wait(By::Id("foobar")).await?;
                    // your code goes here
                    Ok(())
                }
                .boxed()
            },
        },
    ],
    ..App::default()
};
