use std::{env, path::PathBuf};
use thirtyfour::prelude::*;
use url::Url;

mod apps;
use apps::*;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // parse args
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(color_eyre::Report::msg(
            "usage: tkl-webtest <action: test|install> <appliance name> <root URL>
usage: tkl-webtest list",
        ));
    }

    if args[1] == "list" {
        Runners::new().list();
        return Ok(());
    } else if args.len() != 4 {
        return Err(color_eyre::Report::msg(
            "usage: tkl-webtest <action: test|install> <appliance name> <root URL>",
        ));
    }

    let act = match args[1].as_str() {
        "test" => Action::Test,
        "install" => Action::Install,
        x => panic!("unknown action: {}", x),
    };

    let name = args[2].to_string();
    let url = Url::parse(&args[3])?;

    // setup webdriver
    let mut caps = DesiredCapabilities::chrome();
    caps.accept_insecure_certs(true)?;

    let wdurl = env::var("TKL_WEBDRIVER_URL").unwrap_or("http://localhost:4444/".to_owned());

    let wd: WebDriverResult<WebDriver> = WebDriver::new(&wdurl, caps).await;

    let wd_connect_attempts: i32 = env::var("WEBDRIVER_CONNECT_ATTEMPTS")
        .unwrap_or("1".to_owned())
        .parse()?;
    let wd_connect_timeout: f32 = env::var("WEBDRIVER_CONNECT_TIMEOUT")
        .unwrap_or("1".to_owned())
        .parse()?;

    for _ in 0..wd_connect_attempts {
        if wd.is_ok() {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_secs_f32(wd_connect_timeout)).await;
        println!("attempting to connect to selenium: ...");
    }
    let wd = wd?;

    // x + 8, y + 126 to account for window decorations/borders
    wd.set_window_rect(0, 0, 1366 + 8, 768 + 126).await?;
    let scrpath = env::var("TKL_SCREENSHOT_PATH").unwrap_or("/tmp".to_owned());

    Runners::new()
        .run(State {
            name,
            wd,
            act,
            url,
            ssp: PathBuf::from(&scrpath),
            pse: Default::default(),
        })
        .await
}
