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

An easy way to get Selenium up and running is:

```
docker run -d -p 4444:4444 -p 7900:7900 --shm-size="2g" selenium/standalone-chrome:4.7.2-20221219
```

To see what is happening in real time and interact with the browser manually, use noVNC at `localhost:7900` (password `secret`).

To run appliances in Docker alongside Selenium for quick setup, please refer to the repo for [tkldev-docker](https://github.com/turnkeylinux/tkldev-docker).

## Tips and tricks

Using the `core` testing scenario on any appliance will test and take screenshots of Shellinabox and Webmin.
