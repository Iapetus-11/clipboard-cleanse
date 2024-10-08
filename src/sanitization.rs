use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SPOTIFY_SANITIZATION: Regex =
        Regex::new(r"^(https://open\.spotify\.com/track/[a-zA-Z0-9]+)(\?si=[a-zA-Z0-9]+)$")
            .unwrap();
}

pub fn sanitize(text: &str) -> String {
    let text = SPOTIFY_SANITIZATION.replace_all(text, "$1");
    text.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPOTIFY_URL_WITH_TRACKING: &str =
        "https://open.spotify.com/track/5WhtlIoxoZrMmuaWWEQhwV?si=4fcd3c04443548c2";
    const SPOTIFY_URL_WITHOUT_TRACKING: &str =
        "https://open.spotify.com/track/5WhtlIoxoZrMmuaWWEQhwV";

    #[test]
    fn test_spotify_with_tracking() {
        assert_eq!(
            sanitize(SPOTIFY_URL_WITH_TRACKING),
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
}
