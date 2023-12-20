// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// This is proxy configuration example.
// The `proxy` can be used in `WindowBuilder` code directly as the
// following:
//
// ```
//   WindowBuilder::new(...)
//     .proxy_config(proxy)
//     .build()
//     .unwrap()
// ```
//
// or just write in `tauri.conf.json` likes the file
// example-tauri.conf.json in its example:
//
// ```
//   {
//     "tauri": {
//       "windows": [
//         "proxy": proxy,
//         ...
//       ]
//     },
//     ...
// ```
//
// The supporting proxy type is
// - HTTP
// - SOCKSv5
//
// More details please refer: https://github.com/tauri-apps/wry/pull/1006

use tauri::{WindowBuilder, WindowUrl};

const TITLE: &str = "Proxy";
const URL: &str = "https://example.com";
const PROXY: &str = "socks5://127.0.0.1:1080";

fn main() {
  let url = url::Url::parse(PROXY).unwrap();

  tauri::Builder::default()
    .setup(|app| {
      WindowBuilder::new(app, TITLE, WindowUrl::App(URL.into()))
        .proxy_url(url)
        .build()
        .unwrap();

      Ok(())
    })
    .run(tauri::generate_context!(
      "../../examples/proxy/tauri.conf.json"
    ))
    .expect("error while running tauri application");
}
