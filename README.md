# Clipboard Cleanse
*Automatically remove bullshit from copied URLs*

## Goals
- [x] MacOS menu bar app
- [ ] Windows system tray app
- [ ] Linux app?
- [ ] Tracking information / bullshit removal from links
  - [x] `utm_*` query parameters
  - [x] `youtu.be` links (removes `?si=...` query parameter)
  - [x] `open.spotify.com` links (removes `?si=...` query parameter)
  - [x] `amazon.com` (removes `/ref=...` from path and removes many query parameters: `crid`, `dib`, `dib_tag`, `keywords`, `qid`, `sprefix`, `sr`, `pd_rd_w`, `pf_rd_s`, `pf_rd_p`, `pf_rd_t`, `pf_rd_i`, `pf_rd_m`, `pf_rd_r`, `pd_rd_wg`, `pd_rd_r`, `linkCode`, `tag`, `linkId`, `geniuslink`, `ref`, `ref_`, `content-id`, `psc`, `th`)
  - [x] `google.com` (removes many query parameters: `gs_lcrp`, `gs_lp`, `sca_esv`, `ei`, `iflsig`, `sclient`, `rlz`, `bih`, `biw`, `dpr`, `ved`, `sa`, `fbs`, `source`, `sourceid`)
  - [ ] `facebook.com` (TODO)
  - [ ] `instagram.com` (TODO)
  - [ ] `tiktok.com` (TODO)

## Installation
- For now, follow the directions to bundle the application under the relevant section for your operating system. You will need Rust installed.

## MacOS
- Unfortunately on MacOS, we have to poll the clipboard. This polling interval defaults to 500ms but is configurable.

### Building + Bundling
- Run `./bundling/macos/bundle.sh` from the project root.
- To copy the app to the `/Applications` folder, add the `--copy-to-applications` flag
- To sign the app using `codesign`, set the `CODESIGN_IDENTIFIER` envvar

## Windows
- TODO!