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

## Tips and tricks

Using the `core` testing scenario on any appliance will test and take screenshots of Webmin.

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
