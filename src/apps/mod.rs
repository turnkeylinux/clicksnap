use self::generic::{GenStep, GEN_STEPS};
use color_eyre::eyre::{eyre, Result};
use futures::{future::BoxFuture, FutureExt};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use thirtyfour::prelude::*;
use url::Url;

mod ansible;
mod asp_net_core;
mod avideo;
mod b2evolution;
mod bagisto;
mod bookstack;
mod bugzilla;
mod cakephp;
mod canvas;
mod codeigniter;
mod concrete_cms;
mod core;
mod couchdb;
mod django;
mod dokuwiki;
mod drupal10;
mod drupal7;
mod e107;
mod elgg;
mod espocrm;
mod etherpad;
mod example;
mod ezplatform;
mod faveo_helpdesk;
mod fileserver;
mod foswiki;
mod gallery;
mod generic;
mod gitea;
mod gitlab;
mod icescrum;
mod invoice_ninja;
mod joomla4;
mod lamp;
mod lapp;
mod laravel;
mod lighttpd_php_fastcgi;
mod limesurvey;
mod mahara;
mod mantis;
mod matomo;
mod mattermost;
mod mediawiki;
mod mibew;
mod moodle;
mod nextcloud;
mod nginx_php_fastcgi;
mod nodejs;
mod observium;
mod odoo;
mod omeka;
mod opencart;
mod openldap;
mod openvpn;
mod orangehrm;
mod oscommerce;
mod otrs;
mod owncloud;
mod phpbb;
mod phplist;
mod postgresql;
mod prestashop;
mod processwire;
mod rails;
mod redis;
mod redmine;
mod roundup;
mod silverstripe;
mod simplemachines;
mod suitecrm;
mod syncthing;
mod tomcat;
mod tracks;
mod typo3;
mod ushahidi;
mod web2py;
mod wordpress;
mod xoops;
mod yiiframework;
mod zencart;

/// Custom values to use for the appliance inithook variables
pub struct Preseeds {
    pub root_pass: String,
    pub db_pass: String,
    pub app_pass: String,
    pub app_email: String,
    pub app_domain: String,
}

/// Default values for the inithook variables, overridable with envvars
impl Default for Preseeds {
    fn default() -> Self {
        Self {
            root_pass: env::var("ROOT_PASS").unwrap_or("turnkey".to_owned()),
            db_pass: env::var("DB_PASS").unwrap_or("turnkey".to_owned()),
            app_pass: env::var("APP_PASS").unwrap_or("turnkey".to_owned()),
            app_email: env::var("APP_EMAIL").unwrap_or("admin@example.com".to_owned()),
            app_domain: env::var("APP_DOMAIN").unwrap_or("example.com".to_owned()),
        }
    }
}

/// Action to perform on the named appliance
pub enum Action {
    Test,
    Install,
}

/// Running state used by `App` `Step`s
pub struct State {
    pub name: String,
    pub wd: WebDriver,
    pub act: Action,
    pub url: Url,
    pub ssp: PathBuf,
    pub pse: Preseeds,
}

impl State {
    /// Wait for the first selector-shaped element to appear
    async fn wait(&self, sel: By) -> WebDriverResult<WebElement> {
        let elt = self.wd.query(sel.clone()).first().await?;

        elt.wait_until().displayed().await.map_err(|e| {
            WebDriverError::CustomError(format!("waiting for {sel:?} to be displayed: {e}"))
        })?;

        Ok(elt)
    }

    /// Sleep for `m` milliseconds (for use when `wait()` doesn't work)
    async fn sleep(&self, m: u64) {
        thirtyfour::support::sleep(std::time::Duration::from_millis(m)).await
    }

    /// Convenient goto for path under current URL root
    async fn goto(&self, path: &str) -> WebDriverResult<()> {
        self.wd.goto(self.url.join(path)?).await
    }

    /// Convenient goto for path under current URL root but with a different port
    async fn goto_port(&self, port: u16, path: &str) -> WebDriverResult<()> {
        let mut u = self.url.clone();

        u.set_port(Some(port))
            .map_err(|()| WebDriverError::CustomError("Failed setting port".to_string()))?;

        self.wd.goto(u.join(path)?).await
    }
}

/// Convenience alias for the actual work future a single `Step` describes
type StepFn = fn(&State) -> BoxFuture<'_, Result<()>>;

/// Single incremental step of an `App` scenario
#[derive(Clone)]
pub struct Step {
    name: &'static str,
    desc: &'static str,
    screenshot: &'static str,

    f: StepFn,
}

impl Step {
    pub const fn default() -> Self {
        Self {
            name: "dummy",
            desc: "No description provided",
            screenshot: "",
            f: |_| async { Ok(()) }.boxed(),
        }
    }

    /// Run the step while taking screenshots
    pub async fn run(&self, st: &State) -> Result<()> {
        let res = (self.f)(st).await;

        let screenshot = if res.is_err() {
            // browser error screenshot
            "screenshot-error.png".to_string()
        } else if !self.screenshot.is_empty() {
            format!("screenshot-{}.png", self.screenshot)
        } else {
            format!("screenshot-{}.png", self.name)
        };

        let scr_res = st.wd.screenshot(&st.ssp.join(screenshot)).await;
        // see if step failed
        res?;
        // only if it didn't, see if screenshotting it failed
        scr_res?;

        Ok(())
    }
}

impl Default for Step {
    fn default() -> Self {
        Self::default()
    }
}

/// Convenience alias for a sequence of `Step`s
type Steps = &'static [Step];

/// Single app-specific scenario with steps split by stages
pub struct App {
    pre: Steps,
    test: Steps,
    install: Steps,
    post: Steps,
    skip: &'static [GenStep],
}

impl App {
    pub const fn default() -> Self {
        Self {
            pre: &[],
            test: &[],
            install: &[],
            post: &[],
            skip: &[],
        }
    }
}

impl Default for App {
    fn default() -> Self {
        App::default()
    }
}

/// The map of appliance names to appliance scenario definitions
pub struct Runners(HashMap<&'static str, &'static App>);

impl Runners {
    pub fn new() -> Runners {
        let mut h = HashMap::new();
        h.insert("ansible", &ansible::APP);
        h.insert("asp-net-core", &asp_net_core::APP);
        h.insert("avideo", &avideo::APP);
        h.insert("b2evolution", &b2evolution::APP);
        h.insert("bagisto", &bagisto::APP);
        h.insert("bookstack", &bookstack::APP);
        h.insert("bugzilla", &bugzilla::APP);
        h.insert("cakephp", &cakephp::APP);
        h.insert("canvas", &canvas::APP);
        h.insert("codeigniter", &codeigniter::APP);
        h.insert("concrete-cms", &concrete_cms::APP);
        h.insert("core", &core::APP);
        h.insert("couchdb", &couchdb::APP);
        h.insert("django", &django::APP);
        h.insert("dokuwiki", &dokuwiki::APP);
        h.insert("drupal10", &drupal10::APP);
        h.insert("drupal7", &drupal7::APP);
        h.insert("e107", &e107::APP);
        h.insert("elgg", &elgg::APP);
        h.insert("espocrm", &espocrm::APP);
        h.insert("etherpad", &etherpad::APP);
        h.insert("ezplatform", &ezplatform::APP);
        h.insert("faveo_helpdesk", &faveo_helpdesk::APP);
        h.insert("fileserver", &fileserver::APP);
        h.insert("foswiki", &foswiki::APP);
        h.insert("gallery", &gallery::APP);
        h.insert("gitea", &gitea::APP);
        h.insert("gitlab", &gitlab::APP);
        h.insert("icescrum", &icescrum::APP);
        h.insert("invoice-ninja", &invoice_ninja::APP);
        h.insert("joomla4", &joomla4::APP);
        h.insert("lamp", &lamp::APP);
        h.insert("lapp", &lapp::APP);
        h.insert("laravel", &laravel::APP);
        h.insert("lighttpd-php-fastcgi", &lighttpd_php_fastcgi::APP);
        h.insert("limesurvey", &limesurvey::APP);
        h.insert("mahara", &mahara::APP);
        h.insert("mantis", &mantis::APP);
        h.insert("matomo", &matomo::APP);
        h.insert("mattermost", &mattermost::APP);
        h.insert("mediawiki", &mediawiki::APP);
        h.insert("mibew", &mibew::APP);
        h.insert("moodle", &moodle::APP);
        h.insert("mysql", &lamp::APP);
        h.insert("nextcloud", &nextcloud::APP);
        h.insert("nginx-php-fastcgi", &nginx_php_fastcgi::APP);
        h.insert("nodejs", &nodejs::APP);
        h.insert("observium", &observium::APP);
        h.insert("odoo", &odoo::APP);
        h.insert("omeka", &omeka::APP);
        h.insert("opencart", &opencart::APP);
        h.insert("openldap", &openldap::APP);
        h.insert("openvpn", &openvpn::APP);
        h.insert("orangehrm", &orangehrm::APP);
        h.insert("oscommerce", &oscommerce::APP);
        h.insert("otrs", &otrs::APP);
        h.insert("owncloud", &owncloud::APP);
        h.insert("phpbb", &phpbb::APP);
        h.insert("phplist", &phplist::APP);
        h.insert("postgresql", &postgresql::APP);
        h.insert("prestashop", &prestashop::APP);
        h.insert("processwire", &processwire::APP);
        h.insert("rails", &rails::APP);
        h.insert("redis", &redis::APP);
        h.insert("redmine", &redmine::APP);
        h.insert("roundup", &roundup::APP);
        h.insert("silverstripe", &silverstripe::APP);
        h.insert("simplemachines", &simplemachines::APP);
        h.insert("suitecrm", &suitecrm::APP);
        h.insert("syncthing", &syncthing::APP);
        h.insert("tomcat", &tomcat::APP);
        h.insert("tracks", &tracks::APP);
        h.insert("typo3", &typo3::APP);
        h.insert("ushahidi", &ushahidi::APP);
        h.insert("web2py", &web2py::APP);
        h.insert("wordpress", &wordpress::APP);
        h.insert("xoops", &xoops::APP);
        h.insert("yiiframework", &yiiframework::APP);
        h.insert("zencart", &zencart::APP);
        Self(h)
    }

    async fn try_run(&self, st: &State) -> Result<()> {
        let app = self
            .0
            .get(st.name.as_str())
            .ok_or(eyre!("Unknown app: '{:?}'!", st.name))?;

        let (act_name, act_steps) = match st.act {
            Action::Test => ("test", app.test),
            Action::Install => ("install", app.install),
        };

        println!("Running steps for {} (action: {})...", st.name, act_name);
        println!("  Default steps used:");

        let def_steps: &[Step] = &GEN_STEPS
            .iter()
            .filter_map(|(n, s)| {
                if app.skip.contains(n) {
                    println!("    - (skipped by this app) {:?}", n);
                    None
                } else {
                    println!("    - {:?}", n);
                    Some(*s)
                }
            })
            .flatten()
            .cloned()
            .collect::<Vec<Step>>();

        let stages = &[
            ("Default generic steps", def_steps),
            ("Appliance 'pre' steps", app.pre),
            (&format!("Appliance {act_name} steps"), act_steps),
            ("Appliance 'post' steps", app.post),
        ];

        for i in 0..stages.len() {
            let (name, steps) = stages[i];

            if steps.is_empty() {
                println!("  Skipping stage {i} ({name}): no steps defined for it");
                continue;
            } else {
                println!("  Running stage {i} ({name}) steps:");
            }

            if i > 0 {
                // always start non-empty non-default step sequence at root url for convenience
                st.goto("/").await?;
            }

            for j in 0..steps.len() {
                let step = &steps[j];
                println!(
                    "    Running step {}.{}: {} {}",
                    i,
                    j + 1,
                    step.name,
                    step.desc
                );
                step.run(st).await?
            }
        }

        Ok(())
    }

    /// Wrapper function to always quit WebDriver cleanly
    pub async fn run(&self, st: State) -> Result<()> {
        let o = self.try_run(&st).await;
        let _ = st.wd.clone().quit().await;
        o
    }
}
