use std::{collections::HashSet, hash::RandomState, sync::LazyLock};

use regex::Regex;
use url::Url;

static URL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"([a-zA-Z0-9]+:\/\/)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)"
    )
    .unwrap()
});

fn remove_query_params(url: Url, query_param_keys: &HashSet<&str, RandomState>) -> Url {
    let mut new_url = url.clone();

    {
        let mut new_url_query_param_pairs = new_url.query_pairs_mut();
        let mut new_url_builder = new_url_query_param_pairs.clear();

        for (key, value) in url
            .query_pairs()
            .filter(|(key, _)| !query_param_keys.contains(&key.clone().into_owned().as_str()))
        {
            new_url_builder = new_url_builder.append_pair(&key, &value);
        }

        new_url_builder.finish();
    }

    if new_url.query_pairs().count() == 0 {
        new_url.set_query(None);
    }

    new_url
}

pub fn sanitize(text: &str) -> String {
    let url_parser = Url::options();
    let matched_parts: Vec<(&str, Url)> = URL_REGEX
        .find_iter(text)
        .map(|p| p.as_str())
        .map(|p| (p, url_parser.parse(p)))
        .filter_map(|(p, u)| match u {
            Ok(u) => Some((p, u)),
            Err(_) => None,
        })
        .collect();

    let mut output = text.to_string();
    let mut query_params_to_remove = HashSet::from([
        "utm_source",
        "utm_medium",
        "utm_name",
        "utm_term",
        "utm_content",
        "utm_campaign",
    ]);

    for (split_part, url) in matched_parts {
        let mut url = url;

        match url.domain().unwrap_or("") {
            "youtu.be" | "www.youtube.com" | "youtube.com" => {
                query_params_to_remove.insert("si");
            }
            "open.spotify.com" => {
                query_params_to_remove.insert("si");
            }
            "www.amazon.com" | "amazon.com" => {
                query_params_to_remove.extend([
                    "crid",
                    "dib",
                    "dib_tag",
                    "keywords",
                    "qid",
                    "sprefix",
                    "sr",
                    "pd_rd_w",
                    "pf_rd_s",
                    "pf_rd_p",
                    "pf_rd_t",
                    "pf_rd_i",
                    "pf_rd_m",
                    "pf_rd_r",
                    "pd_rd_wg",
                    "pd_rd_r",
                    "linkCode",
                    "tag",
                    "linkId",
                    "geniuslink",
                    "ref",
                    "ref_",
                    "content-id",
                    "psc",
                    "th",
                ]);

                if let Some(path_segments) = url.path_segments() {
                    let path_segments: Vec<&str> = path_segments.collect();

                    let last_path_segment = path_segments.last().unwrap_or(&"");

                    if last_path_segment.starts_with("ref=") {
                        url.set_path(&path_segments[0..path_segments.len() - 1].join("/"));
                    }
                }
            }
            "www.google.com" | "google.com" => {
                query_params_to_remove.extend([
                    "gs_lcrp", "gs_lp", "sca_esv", "ei", "iflsig", "sclient", "rlz", "bih", "biw",
                    "dpr", "ved", "sa", "fbs", "source", "sourceid",
                ]);
            }
            "www.instagram.com" | "instagram.com" => {
                query_params_to_remove.insert("igsh");
            }
            "www.x.com" | "x.com" | "www.twitter.com" | "twitter.com" => {
                query_params_to_remove.extend(["t", "s"]);
            }
            "www.ebay.com" | "ebay.com" => {
                query_params_to_remove.extend([
                    "_trksid",
                    "mkcid",
                    "mkevt",
                    "mkrid",
                    "ssspo",
                    "sssrc",
                    "ssuid",
                    "widget_ver",
                    "media",
                ]);
            }
            "www.walmart.com" | "walmart.com" => {
                query_params_to_remove.extend(["sid", "from"]);
            }
            _ => {}
        }

        let url = remove_query_params(url, &query_params_to_remove);

        output = output.replace(split_part, url.as_str());
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";

    #[test]
    fn test_ignores_normal_stuff() {
        for case in [
            "",
            "abracadabra",
            "https://iapetus11.me/",
            "http://iapetus11.me/",
            "test one two three !@#$%^&*()_+[]{}|\\/~`,.<>-='\"",
            LOREM_IPSUM,
        ] {
            assert_eq!(sanitize(case), case);
        }
    }

    #[test]
    fn test_spotify() {
        const WITH_BS: &str =
            "https://open.spotify.com/track/2bDJK04GaUdkBAuZoPt2ch?si=1efd90192fb242df";
        const NO_BS: &str = "https://open.spotify.com/track/2bDJK04GaUdkBAuZoPt2ch";

        let cases = [
            (sanitize(WITH_BS), NO_BS),
            (
                sanitize(&format!("{LOREM_IPSUM} {WITH_BS} {LOREM_IPSUM}")),
                &format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}"),
            ),
            (sanitize(NO_BS), NO_BS),
            (
                sanitize(&format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}")),
                &format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}"),
            ),
        ];

        for (test, expected) in cases {
            assert_eq!(test, expected);
        }
    }

    #[test]
    fn test_youtu_dot_be() {
        const WITH_BS: &str = "https://youtu.be/dQw4w9WgXcQ?si=LPnCW-jopJtkzMRx";
        const NO_BS: &str = "https://youtu.be/dQw4w9WgXcQ";
        const WITH_BS_WITH_TIMESTAMP: &str =
            "https://youtu.be/dQw4w9WgXcQ?si=LPnCW-jopJtkzMRx&t=47";
        const NO_BS_WITH_TIMESTAMP: &str = "https://youtu.be/dQw4w9WgXcQ?t=47";

        let cases = [
            (sanitize(WITH_BS), NO_BS),
            (
                sanitize(&format!("{LOREM_IPSUM} {WITH_BS} {LOREM_IPSUM}")),
                &format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}"),
            ),
            (sanitize(NO_BS), NO_BS),
            (
                sanitize(&format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}")),
                &format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}"),
            ),
            (sanitize(WITH_BS_WITH_TIMESTAMP), NO_BS_WITH_TIMESTAMP),
            (
                sanitize(&format!(
                    "{LOREM_IPSUM} {WITH_BS_WITH_TIMESTAMP} {LOREM_IPSUM}"
                )),
                &format!("{LOREM_IPSUM} {NO_BS_WITH_TIMESTAMP} {LOREM_IPSUM}"),
            ),
            (sanitize(NO_BS_WITH_TIMESTAMP), NO_BS_WITH_TIMESTAMP),
            (
                sanitize(&format!(
                    "{LOREM_IPSUM} {NO_BS_WITH_TIMESTAMP} {LOREM_IPSUM}"
                )),
                &format!("{LOREM_IPSUM} {NO_BS_WITH_TIMESTAMP} {LOREM_IPSUM}"),
            ),
        ];

        for (test, expected) in cases {
            assert_eq!(test, expected);
        }
    }

    #[test]
    fn test_reddit() {
        const WITH_BS: &str =
        "https://www.reddit.com/r/ShittySysadmin/comments/1foocp8/my_boss_found_out_what_the_ai_deep_learning/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button";
        const NO_BS: &str = "https://www.reddit.com/r/ShittySysadmin/comments/1foocp8/my_boss_found_out_what_the_ai_deep_learning/";

        let cases = [
            (sanitize(WITH_BS), NO_BS),
            (
                sanitize(&format!("{LOREM_IPSUM} {WITH_BS} {LOREM_IPSUM}")),
                &format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}"),
            ),
            (sanitize(NO_BS), NO_BS),
            (
                sanitize(&format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}")),
                &format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}"),
            ),
        ];

        for (test, expected) in cases {
            assert_eq!(test, expected);
        }
    }

    #[test]
    fn test_amazon_search() {
        const WITH_BS: &str =
        "https://www.amazon.com/s?k=among+us&crid=3LTHOHS5L9240&sprefix=among+us%2Caps%2C182&ref=nb_sb_noss_1";
        const NO_BS: &str = "https://www.amazon.com/s?k=among+us";

        let cases = [
            (sanitize(WITH_BS), NO_BS),
            (
                sanitize(&format!("{LOREM_IPSUM} {WITH_BS} {LOREM_IPSUM}")),
                &format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}"),
            ),
            (sanitize(NO_BS), NO_BS),
            (
                sanitize(&format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}")),
                &format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}"),
            ),
        ];

        for (test, expected) in cases {
            assert_eq!(test, expected);
        }
    }

    #[test]
    fn test_amazon_product() {
        const WITH_BS: &str =
        "https://www.amazon.com/Inflatable-Costume-Halloween-Spacesuit-Astronaut/dp/B09BJS9BBJ/ref=sr_1_40?crid=2Z3A2U3LQAC8Z&dib=eyJ2IjoiMSJ9.QD-XGq5wyJMbhSkgs3XeTqbz8N9tRCkGBrM-kKlN8Xfk3jE6NS3EA_QhY5mkLjnSW1ah7_UCLuMP9zoRSZSTpa1oK1Y3iTMbZNi48GU0NLVAgUothoMaCiku4AnqBKsnUH0RP7qVzz5irWJ3YDguCNwQxM6m_XzUvKJeeEGllJVPM31VJ0t0JcUD9PaXs6MFCm1unguEF1iaLr6Cp9zg59RazntwyXblgSEiXopgshSCyc4GmumipJqTodXxh8riuHxDYQ-dtIUEZuQrtcqpsWEtrargU88sKW6iqmsXB84.msLyoWCuM8cBB-gYfdt-nZwDszc7wfcap6eUvMlqABc&dib_tag=se&keywords=among+us&qid=1728832966&sprefix=among+u%2Caps%2C115&sr=8-40";
        const NO_BS: &str =
            "https://www.amazon.com/Inflatable-Costume-Halloween-Spacesuit-Astronaut/dp/B09BJS9BBJ";

        let cases = [
            (sanitize(WITH_BS), NO_BS),
            (
                sanitize(&format!("{LOREM_IPSUM} {WITH_BS} {LOREM_IPSUM}")),
                &format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}"),
            ),
            (sanitize(NO_BS), NO_BS),
            (
                sanitize(&format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}")),
                &format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}"),
            ),
        ];

        for (test, expected) in cases {
            assert_eq!(test, expected);
        }
    }

    #[test]
    fn test_google() {
        const WITH_BS: &str =
        "https://www.google.com/search?q=let+me+google+that+for+you&udm=14&rlz=1C5CHFA_enUS1022US1022&oq=let+me+google+that+for+you&gs_lcrp=EgZjaHJvbWUyBggAEEUYOTIGCAEQRRhAMgYIAhAAGEAyBggDEAAYQDIGCAQQABhA0gEIMjM0MWowajeoAgCwAgA&sourceid=chrome&ie=UTF-8";
        const NO_BS: &str =
            "https://www.google.com/search?q=let+me+google+that+for+you&udm=14&oq=let+me+google+that+for+you&ie=UTF-8";

        const WITH_BS_BAND: &str = "https://www.google.com/search?sca_esv=eab902f88a49963b&rlz=1C5CHFA_enUS1022US1022&q=microwave+(band)&source=lnms&fbs=AEQNm0Aa4sjWe7Rqy32pFwRj0UkWd8nbOJfsBGGB5IQQO6L3J_86uWOeqwdnV0yaSF-x2jogM63VUdBhAMVqo6r6ESHk5gYCycVYeSiTstipcfTqmIhRyNTkvcUNlFNBFo1Ct8djYRwYkoYYVQCjXdCMx_QpPNuVSAotPifJ1VZwOnoSbLVxcdVtmtRchwzdBXA8SbTftA_onVzsK5maxZvT9OLVuyPkOw&sa=X&ved=2ahUKEwj2hOa0iIyJAxWaTDABHaNRCpAQ0pQJegQIEhAB&biw=1512&bih=793&dpr=2";
        const NO_BS_BAND: &str = "https://www.google.com/search?q=microwave+%28band%29";

        const WITH_BS_MOVIE: &str = "https://www.google.com/search?sca_esv=eab902f88a49963b&rlz=1C5CHFA_enUS1022US1022&q=iron+man&source=lnms&fbs=AEQNm0D8w290mrrxEB5tt05ZGXVzey53Ax5hYPfVm2hPmC54fDCmp6uNdUTjug5J6iXC9R56JST0dqljAGrQuD17_gPphSdptBqrpONac59aEg0atuDCge5YKBwQJ8eEtI4mHmFziRkzFQkWBan59H2WVJl5UuGam0MykN5C1gPIIzI_eTXdB2q5r30l2wS3XYg4VirYrzJ_ZDXOEm__K690lE80L8L41w&sa=X&ved=2ahUKEwi38O_tiIyJAxVp5MkDHangI4gQ0pQJegQIEhAB&biw=1512&bih=793&dpr=2";
        const NO_BS_MOVIE: &str = "https://www.google.com/search?q=iron+man";

        const WITH_BS_ACTOR: &str = "https://www.google.com/search?sa=X&sca_esv=eab902f88a49963b&rlz=1C5CHFA_enUS1022US1022&q=Robert+Downey+Jr.&stick=H4sIAAAAAAAAAONgFuLUz9U3SKnKSapQAjMNzaqMsrSEspOt9NMyc3LBhFVyYnHJIlbBoPyk1KISBZf88rzUSgWvIr0drIwAiCXEPEMAAAA&ved=2ahUKEwjHp-uciYyJAxUJM9AFHb4YOl8QgOQBegQIRhAG&biw=1512&bih=793&dpr=2";
        const NO_BS_ACTOR: &str = "https://www.google.com/search?q=Robert+Downey+Jr.&stick=H4sIAAAAAAAAAONgFuLUz9U3SKnKSapQAjMNzaqMsrSEspOt9NMyc3LBhFVyYnHJIlbBoPyk1KISBZf88rzUSgWvIr0drIwAiCXEPEMAAAA";

        let cases = [
            (sanitize(WITH_BS), NO_BS),
            (
                sanitize(&format!("{LOREM_IPSUM} {WITH_BS} {LOREM_IPSUM}")),
                &format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}"),
            ),
            (sanitize(NO_BS), NO_BS),
            (
                sanitize(&format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}")),
                &format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}"),
            ),
            (sanitize(WITH_BS_BAND), NO_BS_BAND),
            (
                sanitize(&format!("{LOREM_IPSUM} {WITH_BS_BAND} {LOREM_IPSUM}")),
                &format!("{LOREM_IPSUM} {NO_BS_BAND} {LOREM_IPSUM}"),
            ),
            (sanitize(NO_BS_BAND), NO_BS_BAND),
            (sanitize(WITH_BS_MOVIE), NO_BS_MOVIE),
            (
                sanitize(&format!("{LOREM_IPSUM} {WITH_BS_MOVIE} {LOREM_IPSUM}")),
                &format!("{LOREM_IPSUM} {NO_BS_MOVIE} {LOREM_IPSUM}"),
            ),
            (sanitize(NO_BS_MOVIE), NO_BS_MOVIE),
            (sanitize(WITH_BS_ACTOR), NO_BS_ACTOR),
            (
                sanitize(&format!("{LOREM_IPSUM} {WITH_BS_ACTOR} {LOREM_IPSUM}")),
                &format!("{LOREM_IPSUM} {NO_BS_ACTOR} {LOREM_IPSUM}"),
            ),
            (sanitize(NO_BS_ACTOR), NO_BS_ACTOR),
        ];

        for (test, expected) in cases {
            assert_eq!(test, expected);
        }
    }

    #[test]
    fn test_instagram() {
        const WITH_BS: &str =
        "https://www.instagram.com/p/DA3VayjOVSM/?utm_source=ig_web_button_share_sheet&igsh=ZDNlZDc0MzIxNw==";
        const NO_BS: &str = "https://www.instagram.com/p/DA3VayjOVSM/";

        let cases = [
            (sanitize(WITH_BS), NO_BS),
            (
                sanitize(&format!("{LOREM_IPSUM} {WITH_BS} {LOREM_IPSUM}")),
                &format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}"),
            ),
            (sanitize(NO_BS), NO_BS),
            (
                sanitize(&format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}")),
                &format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}"),
            ),
        ];

        for (test, expected) in cases {
            assert_eq!(test, expected);
        }
    }

    #[test]
    fn test_x_formerly_known_as_twitter() {
        const WITH_BS: &str =
            "https://x.com/kirawontmiss/status/1843681066282017177?s=46&t=dmXz8VbTtezubBw4-OTfRw";
        const NO_BS: &str = "https://x.com/kirawontmiss/status/1843681066282017177";

        let cases = [
            (sanitize(WITH_BS), NO_BS),
            (
                sanitize(&format!("{LOREM_IPSUM} {WITH_BS} {LOREM_IPSUM}")),
                &format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}"),
            ),
            (sanitize(NO_BS), NO_BS),
            (
                sanitize(&format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}")),
                &format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}"),
            ),
        ];

        for (test, expected) in cases {
            assert_eq!(test, expected);
        }
    }

    #[test]
    fn test_ebay() {
        const WITH_BS: &str =
            "https://www.ebay.com/itm/365052429033?mkcid=16&mkevt=1&mkrid=711-127632-2357-0&ssspo=EdIxCRi_Twa&sssrc=2047675&ssuid=14aqx1bysd2&widget_ver=artemis&media=COPY";
        const NO_BS: &str = "https://www.ebay.com/itm/365052429033";

        let cases = [
            (sanitize(WITH_BS), NO_BS),
            (
                sanitize(&format!("{LOREM_IPSUM} {WITH_BS} {LOREM_IPSUM}")),
                &format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}"),
            ),
            (sanitize(NO_BS), NO_BS),
            (
                sanitize(&format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}")),
                &format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}"),
            ),
        ];

        for (test, expected) in cases {
            assert_eq!(test, expected);
        }
    }

    #[test]
    fn test_walmart() {
        const WITH_BS: &str =
            "https://www.walmart.com/ip/Halloween-Costumes-Woman-Plus-Size-Women-Deluxe-Sexy-Women-s-Clothin-Horrific-Fancy-Dress-Costume-Distinctive-Scary-Fun-World-Fashion/8773851353?classType=VARIANT&athbdg=L1600&selectedSellerId=101685259&from=%2Fsearch&sid=21aed447-b41b-42b1-a1cd-add60fd1c945";
        const NO_BS: &str = "https://www.walmart.com/ip/Halloween-Costumes-Woman-Plus-Size-Women-Deluxe-Sexy-Women-s-Clothin-Horrific-Fancy-Dress-Costume-Distinctive-Scary-Fun-World-Fashion/8773851353?classType=VARIANT&athbdg=L1600&selectedSellerId=101685259";

        let cases = [
            (sanitize(WITH_BS), NO_BS),
            (
                sanitize(&format!("{LOREM_IPSUM} {WITH_BS} {LOREM_IPSUM}")),
                &format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}"),
            ),
            (sanitize(NO_BS), NO_BS),
            (
                sanitize(&format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}")),
                &format!("{LOREM_IPSUM} {NO_BS} {LOREM_IPSUM}"),
            ),
        ];

        for (test, expected) in cases {
            assert_eq!(test, expected);
        }
    }
}
