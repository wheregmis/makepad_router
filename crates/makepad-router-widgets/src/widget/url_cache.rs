use crate::url::RouterUrl;

use super::RouterWidget;

impl RouterWidget {
    pub(super) fn parse_url_cached(&mut self, input: &str) -> RouterUrl {
        if let Some(pos) = self
            .caches
            .url_parse_cache
            .iter()
            .position(|(k, _)| k.as_str() == input)
        {
            let (k, v) = self.caches.url_parse_cache.remove(pos);
            self.caches.url_parse_cache.insert(0, (k, v.clone()));
            return v;
        }

        let parsed = RouterUrl::parse(input);
        self.caches
            .url_parse_cache
            .insert(0, (input.to_string(), parsed.clone()));
        const MAX: usize = 8;
        if self.caches.url_parse_cache.len() > MAX {
            self.caches.url_parse_cache.truncate(MAX);
        }
        parsed
    }
}
