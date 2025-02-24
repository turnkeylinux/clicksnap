use super::{generic::GenStep, App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    skip: &[GenStep::Landing],
    test: &[
        Step {
            name: "landing",
            f: |st: &State| {
                async {
                    // wait until all images are loaded before doing landing page screenshot
                    let all_images = st.wd.find_all(By::Tag("img")).await?;

                    for img in all_images {
                        img.wait_until().has_property("complete", "true").await?;
                    }

                    // sometimes selenium is still a little too fast
                    st.sleep(5000).await;
                    let all_images = st.wd.find_all(By::Tag("img")).await?;

                    for img in all_images {
                        img.wait_until().has_property("complete", "true").await?;
                    }
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "admin-dash",
            f: |st: &State| {
                async {
                    let form = st.wd.form(By::ClassName("loginform")).await?;
                    form.set_by_name("uname", "admin").await?;
                    form.set_by_name("pass", &st.pse.app_pass).await?;
                    form.submit().await?;
                    st.goto("/admin.php").await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
