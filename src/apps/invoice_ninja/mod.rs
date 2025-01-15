use super::{generic::GenStep, App, State, Step};
use futures::FutureExt;
use thirtyfour::prelude::*;

pub const APP: App = App {
    skip: &[GenStep::Landing],
    test: &[
        Step {
            name: "landing",
            desc: "invoice ninja landing (special case, waiting for login)",
            f: |st: &State| {
                async {
                    st.wait(By::Tag("form")).await?;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
        Step {
            name: "invoiceninja-dash",
            desc: "invoice ninja dashboard (post login)",
            f: |st: &State| {
                async {
                    let form = st.wd.form(By::Tag("form")).await?;

                    form.set_by_name("email", &st.pse.app_email).await?;
                    form.set_by_name("password", &st.pse.app_pass).await?;
                    form.submit().await?;
                    st.wait(By::Css("svg.cursor-pointer"))
                        .await?
                        .wait_until()
                        .clickable()
                        .await?;

                    // working around invoice_ninja being difficult.
                    //
                    // this just removes 3 elements that obscure clicking the button to close an
                    // overlay
                    st.wd
                        .handle
                        .execute(
                            r#"
let el = document.getElementById("headlessui-dialog-overlay-:r7:");
el.parentNode.removeChild(el);

el = document.querySelector("div.flex.items-end.justify-center.min-h-screen.pt-4.px-4.pb-20.text-center");
el.parentNode.removeChild(el);

el = document.getElementById("headlessui-dialog-:r6:");
el.parentNode.removeChild(el);
                    "#, Vec::default())
                        .await?;

                    st.wait(By::Css("svg.cursor-pointer"))
                        .await?
                        .click()
                        .await?;
                    st.sleep(10_000).await;
                    Ok(())
                }
                .boxed()
            },
            ..Step::default()
        },
    ],
    ..App::default()
};
