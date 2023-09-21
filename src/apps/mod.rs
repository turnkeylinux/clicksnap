use async_trait::async_trait;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::PathBuf;
use thirtyfour::prelude::*;
use url::Url;

mod asp_net_core;
mod avideo;
mod b2evolution;
mod bagisto;
mod bugzilla;
mod cakephp;
mod canvas;
mod codeigniter;
mod concrete_cms;
mod core;
mod couchdb;
mod django;
mod dokuwiki;
mod fileserver;
mod lamp;
mod lapp;
mod nginx_php_fastcgi;
mod nodejs;
mod odoo;
mod openvpn;
mod owncloud;
mod nextcloud;
mod rails;
mod redmine;
mod wordpress;
mod gitea;
mod drupal7;
mod drupal10;
mod silverstripe;
mod orangehrm;

pub struct Preseeds {
    pub root_pass: String,
    pub db_pass: String,
    pub app_pass: String,
    pub app_email: String,
    pub app_domain: String,
}

pub struct State {
    pub wd: WebDriver,
    pub act: Action,
    pub url: Url,
    pub ssp: PathBuf,
    pub pse: Preseeds,
}

impl State {
    async fn wait(&self, sel: By) -> WebDriverResult<WebElement> {
        let elt = self.wd.query(sel).first().await?;
        elt.wait_until().displayed().await?;
        Ok(elt)
    }

    async fn sleep(&self, m: u64) -> () {
        thirtyfour::support::sleep(std::time::Duration::from_millis(m)).await
    }
}

pub enum Action {
    Test,
    Install,
}

#[async_trait]
pub trait AsyncFn {
    async fn exec(&self, _: State) -> WebDriverResult<()>;
}

#[async_trait]
impl<T, F> AsyncFn for T
where
    T: Fn(State) -> F + Sync,
    F: std::future::Future<Output = WebDriverResult<()>> + Send + 'static,
{
    async fn exec(&self, st: State) -> WebDriverResult<()> {
        let wd = st.wd.clone();
        let r = (self(st)).await;
        let _ = wd.quit().await;
        r
    }
}

type BoxAsyncFn = Box<dyn AsyncFn + Send + Sync>;
type BoxAsyncFnMap = HashMap<&'static str, BoxAsyncFn>;

pub static RUNNERS: Lazy<BoxAsyncFnMap> = Lazy::new(|| {
    let mut h: BoxAsyncFnMap = HashMap::new();
    h.insert("asp-net-core", Box::new(asp_net_core::exec));
    h.insert("avideo", Box::new(avideo::exec));
    h.insert("b2evolution", Box::new(b2evolution::exec));
    h.insert("bagisto", Box::new(bagisto::exec));
    h.insert("bugzilla", Box::new(bugzilla::exec));
    h.insert("cakephp", Box::new(cakephp::exec));
    h.insert("canvas", Box::new(canvas::exec));
    h.insert("codeigniter", Box::new(codeigniter::exec));
    h.insert("concrete-cms", Box::new(concrete_cms::exec));
    h.insert("core", Box::new(core::exec));
    h.insert("couchdb", Box::new(couchdb::exec));
    h.insert("django", Box::new(django::exec));
    h.insert("dokuwiki", Box::new(dokuwiki::exec));
    h.insert("fileserver", Box::new(fileserver::exec));
    h.insert("lamp", Box::new(lamp::exec));
    h.insert("lapp", Box::new(lapp::exec));
    h.insert("mysql", Box::new(lamp::exec));
    h.insert("nginx-php-fastcgi", Box::new(nginx_php_fastcgi::exec));
    h.insert("nodejs", Box::new(nodejs::exec));
    h.insert("odoo", Box::new(odoo::exec));
    h.insert("openvpn", Box::new(openvpn::exec));
    h.insert("owncloud", Box::new(owncloud::exec));
    h.insert("postgresql", Box::new(lapp::exec));
    h.insert("nextcloud", Box::new(nextcloud::exec));
    h.insert("rails", Box::new(rails::exec));
    h.insert("redmine", Box::new(redmine::exec));
    h.insert("wordpress", Box::new(wordpress::exec));
    h.insert("gitea", Box::new(gitea::exec));
    h.insert("drupal7", Box::new(drupal7::exec));
    h.insert("drupal10", Box::new(drupal10::exec));
    h.insert("silverstripe", Box::new(silverstripe::exec));
    h.insert("orangehrm", Box::new(orangehrm::exec));
    h
});
