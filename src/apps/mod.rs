use url::Url;
use std::path::Path;
use strum::EnumCount;
use once_cell::sync::Lazy;
use thirtyfour::prelude::*;
use async_trait::async_trait;
use strum_macros::{EnumString, EnumCount as EnumCountMacro};

mod core;
mod lamp;
mod openvpn;
mod wordpress;

pub struct State<'a> {
    pub wd:  WebDriver,
    pub act: Action,
    pub url: Url,
    pub ssp: &'a Path,
}

#[derive(EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Action {
    Test,
    Install,
}

#[derive(Clone, Copy, Debug, EnumString, EnumCountMacro)]
#[strum(serialize_all = "lowercase")]
pub enum App {
    Core,
    Lamp,
    OpenVPN,
    WordPress,
}

#[async_trait]
pub trait Runner {
    async fn exec(&self, _ : &State) -> WebDriverResult<()>;
}

pub static RUNNERS: Lazy<[Box<dyn Runner + Send + Sync>; App::COUNT]> = Lazy::new(||
    [
        Box::new(core::CoreRunner{}),
        Box::new(lamp::LampRunner{}),
        Box::new(openvpn::OpenVPNRunner{}),
        Box::new(wordpress::WordPressRunner{}),
    ]
);
