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
  - [x] `instagram.com` (removes `?igsh=...` query parameter)
  - [x] `x.com`/Twitter (removes `?s=...&t=...` query parameters)
  - [x] `walmart.com` (removes `?from=...&sid=...` query parameters)
  - [ ] `facebook.com` (TODO)
  - [ ] `tiktok.com` (TODO)

## Installation
### [MacOS Instructions](MacOS.md#installation)
### [Windows Instructions](Windows.md#installation)