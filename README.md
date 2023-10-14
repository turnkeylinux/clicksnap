# clicksnap

Usage:

```
cargo run -- test core https://1.2.3.4/
```

or

```
cargo build && ./target/debug/clicksnap test core https://1.2.3.4/
```

Expected result: success or failure based on whether the pages required for the supplied appliance can be loaded. Screenshots are saved in `/tmp` by default, can be overridden by setting `TKL_SCREENSHOT_PATH`. See source for per-appliance screenshot filenames, subject to change.

Expects Selenium Webdriver to be listening somewhere accessible (default is `localhost:4444`, can be overridden by setting `TKL_WEBDRIVER_URL`).

To see what is happening in real time and interact with the browser manually, use the container-provided noVNC at `localhost:7900` (password `secret`).

To run appliances in Docker alongside Selenium for quick setup, please refer to the repo for [tkldev-docker](https://github.com/turnkeylinux/tkldev-docker).

## Setup resources

Install tools (to install on TKLDev/Debian)

Rust: https://www.rust-lang.org/tools/install

Docker: https://docs.docker.com/engine/install/debian/

Selenium Webdriver:

```
docker run -d -p 4444:4444 -p 7900:7900 --shm-size="2g" selenium/standalone-chrome:4.7.2-20221219
```

Then from your local browser, browse to:

```
http://xxx.xxx.xxx.xxx:7900/vnc.html?host=yyy.yyy.yyy.yyy&port=7900
```
Where:
    - `xxx.xxx.xxx.xxx` is the remote TKLDev IP
    - `yyy.yyy.yyy.yyy` is the LAN IP of the Selenium container (setup as per above)
    - above IPs can be the same (if they are running on the same host)
    - `7900` is the default port and should not need adjustment

E.g. I am running Selenium (via Docker) on my TKLDev (ip: 192.168.1.157). To connect:

```
http://192.168.1.157:7900/vnc.html?host=192.168.1.157&port=7900
```
## Writing new app scripts

An example script is located in `src/apps/example.rs`. It showcases the general structure of an `App` struct and a few functions which can be used when writing new app scripts.

If you do not need a special name for your screenshot, the name of the step will be used for the screenshot name. It is therefore a good idea to name your steps like valid/convenient filenames right away.

All fields of the `Step` struct can have default values filled in via `.. Step::default()`. *Note:* use `..Step::default()` and `..App::default()`, not the generic `Default::default()` since `const fn` in traits is not stable yet.

If you need some DOM/async content to load before taking the screenshot, just `st.sleep()` before returning `Ok(())` from the step. The screenshots gets taken immediately after the step returns.

The sequentially next step starts off where the previous one ended. It receives the same state struct and the webdriver state persists across steps as well.
