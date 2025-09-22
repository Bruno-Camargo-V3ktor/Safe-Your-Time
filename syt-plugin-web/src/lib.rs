fn get_blocked_sites() -> Vec<&'static str> {
    vec!["facebook.com", "twitter.com", "instagram.com", "tiktok.com"]
}

fn is_blocked(url: &str, blocked_list: &[&str]) -> bool {
    for site in blocked_list {
        if url.contains(site) {
            return true;
        }
    }
    false
}
