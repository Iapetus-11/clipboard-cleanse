use url::Url;

fn remove_query_param(url: Url, query_param_name: &str) -> Url {
    let mut new_url = url.clone();

    {
        let mut new_url_query_param_pairs = new_url.query_pairs_mut();
        let mut new_url_builder = new_url_query_param_pairs.clear();

        for (key, value) in url.query_pairs().filter(|(key, _)| key != query_param_name) {
            new_url_builder = new_url_builder.append_pair(&key, &value);
        }

        new_url_builder.finish();
    }

    new_url
}

pub fn sanitize(text: &str) -> String {
    let url_parser = Url::options();
    let matched_parts: Vec<(&str, Url)> = text
        .split_ascii_whitespace()
        .map(|p| (p, url_parser.parse(p)))
        .filter_map(|(p, u)| match u {
            Ok(u) => Some((p, u)),
            Err(_) => None,
        })
        .collect();

    let mut output = text.to_string();

    for (split_part, url) in matched_parts {
        match url.domain().unwrap_or("") {
            "youtu.be" => {
                let new_url = remove_query_param(url, "si");
                output = output.replace(split_part, new_url.as_str());
            }
            "open.spotify.com" => {
                let new_url = remove_query_param(url, "si");
                output = output.replace(split_part, new_url.as_str());
            }
            _ => {}
        }
    }

    output.strip_suffix('?').unwrap_or_else(|| &output).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ignores_normal_stuff() {
        for case in [
            "",
            "abracadabra",
            "https://iapetus11.me",
            "http://iapetus11.me",
            "test one two three !@#$%^&*()_+[]{}|\\/~`,.<>-='\"",
        ] {
            assert_eq!(sanitize(case), case);
        }
    }

    const SPOTIFY_URL_WITHOUT_TRACKING: &str =
        "https://open.spotify.com/track/5WhtlIoxoZrMmuaWWEQhwV";

    #[test]
    fn test_spotify_with_tracking() {
        assert_eq!(
            sanitize("https://open.spotify.com/track/5WhtlIoxoZrMmuaWWEQhwV?si=4fcd3c04443548c2"),
            SPOTIFY_URL_WITHOUT_TRACKING,
        );
    }

    #[test]
    fn test_spotify_without_tracking() {
        assert_eq!(
            sanitize(SPOTIFY_URL_WITHOUT_TRACKING),
            SPOTIFY_URL_WITHOUT_TRACKING,
        );
    }

    const YOUTU_DOT_BE_URL_WITHOUT_TRACKING: &str = "https://youtu.be/dQw4w9WgXcQ";
    const YOUTU_DOT_BE_URL_WITH_TIMESTAMP_WITHOUT_TRACKING: &str =
        "https://youtu.be/dQw4w9WgXcQ?t=47";

    #[test]
    fn test_youtu_dot_be_with_tracking() {
        assert_eq!(
            sanitize("https://youtu.be/dQw4w9WgXcQ?si=LPnCW-jopJtkzMRx"),
            YOUTU_DOT_BE_URL_WITHOUT_TRACKING,
        );
    }

    #[test]
    fn test_youtu_dot_be_without_tracking() {
        assert_eq!(
            sanitize(YOUTU_DOT_BE_URL_WITHOUT_TRACKING),
            YOUTU_DOT_BE_URL_WITHOUT_TRACKING,
        );
    }

    #[test]
    fn test_youtu_dot_be_with_timestamp_with_tracking() {
        assert_eq!(
            sanitize("https://youtu.be/dQw4w9WgXcQ?si=LPnCW-jopJtkzMRx&t=47"),
            YOUTU_DOT_BE_URL_WITH_TIMESTAMP_WITHOUT_TRACKING,
        );

        assert_eq!(
            sanitize("https://youtu.be/dQw4w9WgXcQ?si=LPnCW-jopJtkzMRx&t=47"),
            YOUTU_DOT_BE_URL_WITH_TIMESTAMP_WITHOUT_TRACKING,
        );
    }
}
