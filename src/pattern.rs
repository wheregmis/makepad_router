//! Route pattern matching types and logic.
//!
//! This module provides types for defining and matching URL patterns with support for:
//! - Static segments (e.g., `/user/profile`)
//! - Dynamic segments (e.g., `/user/:id`)
//! - Single-segment wildcards (e.g., `/admin/*`)
//! - Multi-segment wildcards (e.g., `/admin/**`)

use makepad_live_id::*;
use makepad_micro_serde::*;
use std::collections::HashMap;

/// Represents a route segment type in a pattern
#[derive(Clone, Debug, PartialEq, Eq, Hash, SerBin, DeBin, SerRon, DeRon)]
pub enum RouteSegment {
    /// Static segment (e.g., "user", "profile")
    Static(String),
    /// Dynamic segment with parameter name (e.g., ":id", ":postId")
    Dynamic(String),
    /// Single-segment wildcard
    WildcardSingle,
    /// Multi-segment wildcard
    WildcardMulti,
}

/// Route pattern for matching paths with dynamic segments and wildcards
#[derive(Clone, Debug, PartialEq, Eq, Hash, SerBin, DeBin, SerRon, DeRon)]
pub struct RoutePattern {
    /// Segments of the pattern
    pub segments: Vec<RouteSegment>,
}

/// Route parameters - can be extended with typed parameters in the future
#[derive(Clone, Debug, Default, PartialEq, Eq, SerBin, DeBin, SerRon, DeRon)]
pub struct RouteParams {
    /// Generic parameters stored as LiveId key-value pairs
    pub data: HashMap<LiveId, LiveId>,
}

impl RoutePattern {
    /// Parse a route pattern string (e.g., "/user/:id" or "/admin/*")
    pub fn parse(pattern: &str) -> Result<Self, String> {
        let pattern = pattern.trim();
        if pattern.is_empty() {
            return Err("Pattern cannot be empty".to_string());
        }

        // Normalize: ensure it starts with /
        let pattern = if pattern.starts_with('/') {
            &pattern[1..]
        } else {
            pattern
        };

        let mut segments = Vec::new();
        let parts: Vec<&str> = pattern.split('/').filter(|s| !s.is_empty()).collect();

        for (i, part) in parts.iter().enumerate() {
            if part == &"**" {
                // Multi-segment wildcard must be the last segment
                if i != parts.len() - 1 {
                    return Err("Multi-segment wildcard (**) must be the last segment".to_string());
                }
                segments.push(RouteSegment::WildcardMulti);
                break;
            } else if part == &"*" {
                segments.push(RouteSegment::WildcardSingle);
            } else if part.starts_with(':') {
                let param_name = &part[1..];
                if param_name.is_empty() {
                    return Err("Dynamic segment parameter name cannot be empty".to_string());
                }
                segments.push(RouteSegment::Dynamic(param_name.to_string()));
            } else {
                segments.push(RouteSegment::Static(part.to_string()));
            }
        }

        Ok(RoutePattern { segments })
    }

    /// Match a path against this pattern and extract parameters
    pub fn matches(&self, path: &str) -> Option<RouteParams> {
        let path = path.trim();
        // Normalize: ensure it starts with /
        let path = if path.starts_with('/') {
            &path[1..]
        } else {
            path
        };

        let path_segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let mut params = RouteParams::new();
        let mut pattern_idx = 0;
        let mut path_idx = 0;

        while pattern_idx < self.segments.len() && path_idx < path_segments.len() {
            match &self.segments[pattern_idx] {
                RouteSegment::Static(expected) => {
                    if path_segments[path_idx] != expected {
                        return None;
                    }
                    path_idx += 1;
                }
                RouteSegment::Dynamic(param_name) => {
                    let value = path_segments[path_idx];
                    let param_key = LiveId::from_str(param_name);
                    // Use from_str_with_intern to store the string so it can be retrieved later
                    use makepad_live_id::InternLiveId;
                    let param_value = LiveId::from_str_with_intern(value, InternLiveId::Yes);
                    params.add(param_key, param_value);
                    path_idx += 1;
                }
                RouteSegment::WildcardSingle => {
                    // Match exactly one segment
                    path_idx += 1;
                }
                RouteSegment::WildcardMulti => {
                    // Match remaining segments (zero or more)
                    // This is the last segment, so we're done
                    return Some(params);
                }
            }
            pattern_idx += 1;
        }

        // Check if we consumed all segments
        if pattern_idx < self.segments.len() {
            // Check if remaining is just a multi-segment wildcard
            if pattern_idx == self.segments.len() - 1 {
                if let RouteSegment::WildcardMulti = self.segments[pattern_idx] {
                    return Some(params);
                }
            }
            return None;
        }

        if path_idx < path_segments.len() {
            // Path has more segments than pattern
            return None;
        }

        Some(params)
    }

    /// Match a path prefix against this pattern and return both extracted params and a "tail" path.
    ///
    /// This is used for nested routing: if a parent route pattern matches the beginning of a path,
    /// the remaining part (or captured wildcard part) can be delegated to a child router.
    ///
    /// Tail rules:
    /// - If the pattern ends before the path, the tail is the remaining unmatched segments.
    /// - If the pattern ends with `*` or `**`, the tail is the segment(s) matched by that wildcard.
    /// - The returned tail is `""` if there is nothing to delegate, otherwise it starts with `/`.
    pub fn matches_prefix_with_tail(&self, path: &str) -> Option<(RouteParams, String)> {
        let path = path.trim();
        let path = if path.starts_with('/') { &path[1..] } else { path };
        let path_segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        let mut params = RouteParams::new();
        let mut pattern_idx = 0usize;
        let mut path_idx = 0usize;
        let mut tail_start_at: Option<usize> = None;

        while pattern_idx < self.segments.len() && path_idx < path_segments.len() {
            match &self.segments[pattern_idx] {
                RouteSegment::Static(expected) => {
                    if path_segments[path_idx] != expected {
                        return None;
                    }
                    path_idx += 1;
                }
                RouteSegment::Dynamic(param_name) => {
                    let value = path_segments[path_idx];
                    let param_key = LiveId::from_str(param_name);
                    use makepad_live_id::InternLiveId;
                    let param_value = LiveId::from_str_with_intern(value, InternLiveId::Yes);
                    params.add(param_key, param_value);
                    path_idx += 1;
                }
                RouteSegment::WildcardSingle => {
                    // For nested routing we only "capture" a trailing wildcard, otherwise it's just a matcher.
                    if pattern_idx == self.segments.len().saturating_sub(1) {
                        tail_start_at = Some(path_idx);
                    }
                    path_idx += 1;
                }
                RouteSegment::WildcardMulti => {
                    // Must be last (enforced by parser). Capture the rest (could be empty).
                    tail_start_at = Some(path_idx);
                    path_idx = path_segments.len();
                    pattern_idx += 1;
                    break;
                }
            }
            pattern_idx += 1;
        }

        // If we did not consume the whole pattern, only a trailing `**` can match an empty remainder.
        if pattern_idx < self.segments.len() {
            if pattern_idx == self.segments.len() - 1
                && matches!(self.segments[pattern_idx], RouteSegment::WildcardMulti)
            {
                tail_start_at = Some(path_idx);
            } else {
                return None;
            }
        }

        // If the pattern is fully matched but the path has more segments, this is prefix-match tail.
        if tail_start_at.is_none() && path_idx < path_segments.len() {
            tail_start_at = Some(path_idx);
        }

        let tail = if let Some(start) = tail_start_at {
            if start >= path_segments.len() {
                String::new()
            } else {
                format!("/{}", path_segments[start..].join("/"))
            }
        } else {
            String::new()
        };

        Some((params, tail))
    }

    /// Get the priority for route matching (lower = higher priority)
    pub fn priority(&self) -> usize {
        let mut priority = 0;
        for segment in &self.segments {
            match segment {
                RouteSegment::Static(_) => priority += 1,
                RouteSegment::Dynamic(_) => priority += 100,
                RouteSegment::WildcardSingle => priority += 10000,
                RouteSegment::WildcardMulti => priority += 100000,
            }
        }
        priority
    }

    /// Format a concrete path (no wildcards) from this pattern and params.
    pub fn format_path(&self, params: &RouteParams) -> Option<String> {
        let mut out: Vec<String> = Vec::with_capacity(self.segments.len());
        for segment in &self.segments {
            match segment {
                RouteSegment::Static(s) => out.push(s.clone()),
                RouteSegment::Dynamic(param_name) => {
                    let key = LiveId::from_str(param_name);
                    let value = params.get(key)?;
                    out.push(value.to_string());
                }
                RouteSegment::WildcardSingle | RouteSegment::WildcardMulti => return None,
            }
        }
        Some(format!("/{}", out.join("/")))
    }

    /// Format the "base" part of a pattern, stopping before wildcards.
    ///
    /// This is useful for nested routing patterns like `/admin/**`, where the base is `/admin`.
    pub fn format_base_path(&self, params: &RouteParams) -> String {
        let mut out: Vec<String> = Vec::with_capacity(self.segments.len());
        for segment in &self.segments {
            match segment {
                RouteSegment::Static(s) => out.push(s.clone()),
                RouteSegment::Dynamic(param_name) => {
                    let key = LiveId::from_str(param_name);
                    if let Some(value) = params.get(key) {
                        out.push(value.to_string());
                    } else {
                        break;
                    }
                }
                RouteSegment::WildcardSingle | RouteSegment::WildcardMulti => break,
            }
        }
        if out.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", out.join("/"))
        }
    }
}

impl RouteParams {
    /// Create empty route parameters
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a parameter
    pub fn add(&mut self, key: LiveId, value: LiveId) {
        self.data.insert(key, value);
    }

    /// Get a parameter value by key
    pub fn get(&self, key: LiveId) -> Option<LiveId> {
        self.data.get(&key).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_parse_static() {
        let pattern = RoutePattern::parse("/user/profile").unwrap();
        assert_eq!(pattern.segments.len(), 2);
        assert!(matches!(pattern.segments[0], RouteSegment::Static(ref s) if s == "user"));
        assert!(matches!(pattern.segments[1], RouteSegment::Static(ref s) if s == "profile"));
    }

    #[test]
    fn test_pattern_parse_dynamic() {
        let pattern = RoutePattern::parse("/user/:id").unwrap();
        assert_eq!(pattern.segments.len(), 2);
        assert!(matches!(pattern.segments[0], RouteSegment::Static(ref s) if s == "user"));
        assert!(matches!(pattern.segments[1], RouteSegment::Dynamic(ref s) if s == "id"));
    }

    #[test]
    fn test_pattern_parse_wildcard_single() {
        let pattern = RoutePattern::parse("/admin/*").unwrap();
        assert_eq!(pattern.segments.len(), 2);
        assert!(matches!(pattern.segments[0], RouteSegment::Static(ref s) if s == "admin"));
        assert!(matches!(pattern.segments[1], RouteSegment::WildcardSingle));
    }

    #[test]
    fn test_pattern_parse_wildcard_multi() {
        let pattern = RoutePattern::parse("/admin/**").unwrap();
        assert_eq!(pattern.segments.len(), 2);
        assert!(matches!(pattern.segments[0], RouteSegment::Static(ref s) if s == "admin"));
        assert!(matches!(pattern.segments[1], RouteSegment::WildcardMulti));
    }

    #[test]
    fn test_pattern_parse_mixed() {
        let pattern = RoutePattern::parse("/user/:id/posts/*").unwrap();
        assert_eq!(pattern.segments.len(), 4);
        assert!(matches!(pattern.segments[0], RouteSegment::Static(ref s) if s == "user"));
        assert!(matches!(pattern.segments[1], RouteSegment::Dynamic(ref s) if s == "id"));
        assert!(matches!(pattern.segments[2], RouteSegment::Static(ref s) if s == "posts"));
        assert!(matches!(pattern.segments[3], RouteSegment::WildcardSingle));
    }

    #[test]
    fn test_pattern_match_static() {
        let pattern = RoutePattern::parse("/user/profile").unwrap();
        assert!(pattern.matches("/user/profile").is_some());
        assert!(pattern.matches("/user/settings").is_none());
    }

    #[test]
    fn test_pattern_match_dynamic() {
        let pattern = RoutePattern::parse("/user/:id").unwrap();
        let params = pattern.matches("/user/123").unwrap();
        assert_eq!(params.get(LiveId::from_str("id")), Some(LiveId::from_str("123")));

        let params = pattern.matches("/user/john").unwrap();
        assert_eq!(params.get(LiveId::from_str("id")), Some(LiveId::from_str("john")));
    }

    #[test]
    fn test_pattern_match_multiple_dynamic() {
        let pattern = RoutePattern::parse("/post/:postId/:slug").unwrap();
        let params = pattern.matches("/post/123/my-post").unwrap();
        assert_eq!(params.get(LiveId::from_str("postId")), Some(LiveId::from_str("123")));
        assert_eq!(params.get(LiveId::from_str("slug")), Some(LiveId::from_str("my-post")));
    }

    #[test]
    fn test_pattern_match_wildcard_single() {
        let pattern = RoutePattern::parse("/admin/*").unwrap();
        assert!(pattern.matches("/admin/users").is_some());
        assert!(pattern.matches("/admin/settings").is_some());
        assert!(pattern.matches("/admin/users/123").is_none()); // Should not match multiple segments
    }

    #[test]
    fn test_pattern_match_wildcard_multi() {
        let pattern = RoutePattern::parse("/admin/**").unwrap();
        assert!(pattern.matches("/admin/users").is_some());
        assert!(pattern.matches("/admin/users/123").is_some());
        assert!(pattern.matches("/admin/users/123/edit").is_some());
        assert!(pattern.matches("/admin").is_some()); // Should match zero segments too
    }

    #[test]
    fn test_pattern_prefix_tail_static() {
        let pattern = RoutePattern::parse("/admin").unwrap();
        let (params, tail) = pattern.matches_prefix_with_tail("/admin/dashboard").unwrap();
        assert_eq!(params.data.len(), 0);
        assert_eq!(tail, "/dashboard");
    }

    #[test]
    fn test_pattern_prefix_tail_wildcard_single() {
        let pattern = RoutePattern::parse("/admin/*").unwrap();
        let (_params, tail) = pattern.matches_prefix_with_tail("/admin/dashboard").unwrap();
        assert_eq!(tail, "/dashboard");
    }

    #[test]
    fn test_pattern_prefix_tail_wildcard_multi() {
        let pattern = RoutePattern::parse("/admin/**").unwrap();
        let (_params, tail) = pattern.matches_prefix_with_tail("/admin/a/b").unwrap();
        assert_eq!(tail, "/a/b");
    }

    #[test]
    fn test_pattern_prefix_tail_dynamic() {
        let pattern = RoutePattern::parse("/user/:id/**").unwrap();
        let (params, tail) = pattern
            .matches_prefix_with_tail("/user/42/profile/settings")
            .unwrap();
        assert_eq!(
            params.get(LiveId::from_str("id")),
            Some(LiveId::from_str("42"))
        );
        assert_eq!(tail, "/profile/settings");
    }

    #[test]
    fn test_pattern_priority() {
        let static_pattern = RoutePattern::parse("/user/profile").unwrap();
        let dynamic_pattern = RoutePattern::parse("/user/:id").unwrap();
        let wildcard_single = RoutePattern::parse("/user/*").unwrap();
        let wildcard_multi = RoutePattern::parse("/user/**").unwrap();

        assert!(static_pattern.priority() < dynamic_pattern.priority());
        assert!(dynamic_pattern.priority() < wildcard_single.priority());
        assert!(wildcard_single.priority() < wildcard_multi.priority());
    }

    #[test]
    fn test_pattern_types_exist() {
        // Verify that RoutePattern and RouteSegment can be constructed
        let _pattern = RoutePattern::parse("/user/:id").unwrap();
    }
}
