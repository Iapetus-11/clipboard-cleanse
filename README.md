# Clipboard Cleanse
*Automatically remove bullshit from copied URLs*

## Why?
Ever wonder what that extra stuff on the end of a URL is for? Well, there's a good chance it's useless tracking data to help corporations spy on you and your friends... take this Amazon link for example:
```
https://www.amazon.com/Inflatable-Costume-Halloween-Spacesuit-Astronaut/dp/B09BJS9BBJ?crid=11KHII13JFHSH&dib=eyJ2IjoiMSJ9.nqrP3iAjR9VsVRLmEpoJxBYxL2H53Zh_vi7fD3WLEc6FHQzxqQmlxkz-wrjBRDODNMVohaEB4LRuIWfih8XwD3xp7c-BS_ee-3hftiEMQprYJqYXYRkymkKjje28V8EDwjnQMNmDHIN3tFJUy8udIy9gWy5khyOXi_uMKVoBjqj4V5cnXAMwny9OTVr5BK2_msWm915igcvUfms6fgoQIMaIfRZlFsAX_ATpjjLUeNGLnEktxuXqogPryHsIo5o_jsbIKSyF38lJ1iDiSB13XvvAusjmkXbC0EDNT7m9n7k.WfS1pc4D_HZbaEYBi9kLOLyhRDWMoInlulOcI-kxUMc&dib_tag=se&keywords=among%2Bus&qid=1730694134&sprefix=among%2Bu%2Caps%2C118&sr=8-40&th=1
```
With Clipboard Cleanse, all that junk would be removed, leaving you with this:
```
https://www.amazon.com/Inflatable-Costume-Halloween-Spacesuit-Astronaut/dp/B09BJS9BBJ
```

## Installation
### [MacOS Instructions](MacOS.md#installation)
### [Windows Instructions](Windows.md#installation)

## Project Goals
- [x] MacOS menu bar app
- [x] Windows system tray app
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