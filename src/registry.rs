#![allow(clippy::question_mark)]
//! Route registry for pattern-based route management.
//!
//! This module provides the route registry that handles:
//! - Route registration by ID and by pattern
//! - Path resolution with pattern matching
//! - Priority-based route ordering
//! - Optimized lookups via indexing

use crate::pattern::{RouteParams, RoutePattern, RouteSegment};
use crate::route::Route;
use makepad_live_id::*;
use makepad_micro_serde::*;
use std::collections::HashMap;

/// Route registry entry
#[allow(clippy::question_mark)]
#[derive(Clone, Debug, SerBin, DeBin, SerRon, DeRon)]
struct RouteEntry {
    route_id: LiveId,
    pattern: Option<RoutePattern>,
    priority: usize,
}

/// Registry for pattern-based routes
#[derive(Clone, Debug, Default)]
pub struct RouteRegistry {
    /// Routes by LiveId (for exact matches)
    by_id: HashMap<LiveId, RouteEntry>,
    /// Routes by pattern (for path-based matching)
    by_pattern: Vec<RouteEntry>,
    /// Exact lookup for fully-static patterns (normalized path -> route_id).
    exact_static: HashMap<String, LiveId>,
    /// Candidate pattern indices keyed by first static segment.
    by_first_segment: HashMap<String, Vec<usize>>,
    /// Candidate pattern indices for patterns without a static first segment.
    fallback_first_segment: Vec<usize>,
}

impl RouteRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            by_id: HashMap::new(),
            by_pattern: Vec::new(),
            exact_static: HashMap::new(),
            by_first_segment: HashMap::new(),
            fallback_first_segment: Vec::new(),
        }
    }

    /// Register a route by LiveId
    pub fn register_by_id(&mut self, route_id: LiveId) {
        let entry = RouteEntry {
            route_id,
            pattern: None,
            priority: 0, // Highest priority
        };
        self.by_id.insert(route_id, entry);
    }

    /// Register a route pattern
    pub fn register_pattern(&mut self, pattern: &str, route_id: LiveId) -> Result<(), String> {
        let route_pattern = RoutePattern::parse(pattern)?;
        let priority = route_pattern.priority();
        let entry = RouteEntry {
            route_id,
            pattern: Some(route_pattern),
            priority,
        };

        self.by_id.insert(route_id, entry.clone());

        // Insert in sorted order by priority (lower priority value = higher priority)
        // Find insertion point
        let pos = self.by_pattern.iter()
            .position(|e| e.priority > priority)
            .unwrap_or(self.by_pattern.len());
        self.by_pattern.insert(pos, entry);
        self.rebuild_indices();
        Ok(())
    }

    /// Resolve a path to a route (exact static match first, then pattern match).
    pub fn resolve_path(&self, path: &str) -> Option<Route> {
        let normalized = Self::normalize_path(path);

        if let Some(route_id) = self.exact_static.get(&normalized).copied() {
            let pattern = self.by_id.get(&route_id).and_then(|e| e.pattern.clone());
            return Some(Route {
                id: route_id,
                params: Default::default(),
                query: Default::default(),
                hash: String::new(),
                pattern,
            });
        }

        let first = normalized
            .trim_start_matches('/')
            .split('/')
            .next()
            .unwrap_or("")
            .to_string();

        if let Some(candidates) = self.by_first_segment.get(&first) {
            for &idx in candidates {
                let entry = self.by_pattern.get(idx)?;
                let Some(ref pattern) = entry.pattern else { continue };
                if let Some(params) = pattern.matches(&normalized) {
                    return Some(Route {
                        id: entry.route_id,
                        params,
                        query: Default::default(),
                        hash: String::new(),
                        pattern: Some(pattern.clone()),
                    });
                }
            }
        }

        for &idx in &self.fallback_first_segment {
            let entry = self.by_pattern.get(idx)?;
            let Some(ref pattern) = entry.pattern else { continue };
            if let Some(params) = pattern.matches(&normalized) {
                return Some(Route {
                    id: entry.route_id,
                    params,
                    query: Default::default(),
                    hash: String::new(),
                    pattern: Some(pattern.clone()),
                });
            }
        }

        None
    }

    /// Check if a route ID is registered
    pub fn has_route(&self, route_id: LiveId) -> bool {
        self.by_id.contains_key(&route_id)
    }

    /// Get pattern for a route ID (if registered with a pattern).
    pub fn get_pattern(&self, route_id: LiveId) -> Option<&RoutePattern> {
        self.by_id.get(&route_id).and_then(|e| e.pattern.as_ref())
    }

    fn normalize_path(path: &str) -> String {
        let mut p = path.trim().to_string();
        if p.is_empty() {
            return "/".to_string();
        }
        // Accept full URLs too (same behavior as RouterUrl::parse).
        if let Some((_, after_scheme)) = p.split_once("://") {
            let mut rest = after_scheme;
            if let Some((_, after_host_slash)) = rest.split_once('/') {
                rest = after_host_slash;
                p = format!("/{}", rest);
            } else {
                p = "/".to_string();
            }
        }
        if !p.starts_with('/') {
            p.insert(0, '/');
        }
        // Strip query/hash for matching.
        if let Some((before_hash, _)) = p.split_once('#') {
            p = before_hash.to_string();
        }
        if let Some((before_q, _)) = p.split_once('?') {
            p = before_q.to_string();
        }
        // Collapse trailing slashes.
        while p.len() > 1 && p.ends_with('/') {
            p.pop();
        }
        p
    }

    fn rebuild_indices(&mut self) {
        self.exact_static.clear();
        self.by_first_segment.clear();
        self.fallback_first_segment.clear();

        for (idx, entry) in self.by_pattern.iter().enumerate() {
            let Some(pattern) = entry.pattern.as_ref() else {
                continue;
            };

            // Build exact static lookup.
            if pattern
                .segments
                .iter()
                .all(|s| matches!(s, RouteSegment::Static(_)))
            {
                let path = format!(
                    "/{}",
                    pattern
                        .segments
                        .iter()
                        .filter_map(|s| match s {
                            RouteSegment::Static(v) => Some(v.as_str()),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                        .join("/")
                );
                self.exact_static.entry(path).or_insert(entry.route_id);
            }

            // Index by first segment (if static).
            match pattern.segments.first() {
                Some(RouteSegment::Static(first)) => {
                    self.by_first_segment
                        .entry(first.clone())
                        .or_default()
                        .push(idx);
                }
                _ => self.fallback_first_segment.push(idx),
            }
        }
    }
}

impl SerBin for RouteRegistry {
    fn ser_bin(&self, s: &mut Vec<u8>) {
        self.by_id.ser_bin(s);
        self.by_pattern.ser_bin(s);
    }
}

impl DeBin for RouteRegistry {
    fn de_bin(o: &mut usize, d: &[u8]) -> Result<Self, DeBinErr> {
        let by_id = <HashMap<LiveId, RouteEntry>>::de_bin(o, d)?;
        let by_pattern = <Vec<RouteEntry>>::de_bin(o, d)?;
        let mut out = Self {
            by_id,
            by_pattern,
            exact_static: HashMap::new(),
            by_first_segment: HashMap::new(),
            fallback_first_segment: Vec::new(),
        };
        out.rebuild_indices();
        Ok(out)
    }
}

impl SerRon for RouteRegistry {
    fn ser_ron(&self, d: usize, s: &mut SerRonState) {
        s.st_pre();
        s.field(d + 1, "by_id");
        self.by_id.ser_ron(d + 1, s);
        s.conl();
        s.field(d + 1, "by_pattern");
        self.by_pattern.ser_ron(d + 1, s);
        s.out.push('\n');
        s.st_post(d);
    }
}

impl DeRon for RouteRegistry {
    fn de_ron(s: &mut DeRonState, i: &mut std::str::Chars) -> Result<Self, DeRonErr> {
        s.paren_open(i)?;
        let mut by_id: Option<HashMap<LiveId, RouteEntry>> = None;
        let mut by_pattern: Option<Vec<RouteEntry>> = None;
        loop {
            match s.tok {
                DeRonTok::ParenClose => {
                    s.paren_close(i)?;
                    break;
                }
                DeRonTok::Ident => {
                    let key = s.identbuf.clone();
                    s.ident(i)?;
                    s.colon(i)?;
                    match key.as_str() {
                        "by_id" => by_id = Some(HashMap::<LiveId, RouteEntry>::de_ron(s, i)?),
                        "by_pattern" => by_pattern = Some(Vec::<RouteEntry>::de_ron(s, i)?),
                        _ => return Err(s.err_token("by_id or by_pattern")),
                    }
                    s.eat_comma_paren(i)?;
                }
                _ => return Err(s.err_token("Identifier or )")),
            }
        }
        let mut out = Self {
            by_id: by_id.unwrap_or_default(),
            by_pattern: by_pattern.unwrap_or_default(),
            exact_static: HashMap::new(),
            by_first_segment: HashMap::new(),
            fallback_first_segment: Vec::new(),
        };
        out.rebuild_indices();
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_registry_register_pattern() {
        let mut registry = RouteRegistry::new();
        registry.register_pattern("/user/:id", live_id!(user_profile)).unwrap();
        assert!(registry.has_route(live_id!(user_profile)));
    }

    #[test]
    fn test_route_registry_resolve_path() {
        let mut registry = RouteRegistry::new();
        registry.register_pattern("/user/:id", live_id!(user_profile)).unwrap();

        let route = registry.resolve_path("/user/123").unwrap();
        assert_eq!(route.id, live_id!(user_profile));
        assert_eq!(route.get_param(LiveId::from_str("id")), Some(LiveId::from_str("123")));
    }

    #[test]
    fn test_route_registry_priority() {
        let mut registry = RouteRegistry::new();
        // Register in reverse priority order
        registry.register_pattern("/user/**", live_id!(user_wildcard)).unwrap();
        registry.register_pattern("/user/*", live_id!(user_single)).unwrap();
        registry.register_pattern("/user/:id", live_id!(user_dynamic)).unwrap();
        registry.register_pattern("/user/profile", live_id!(user_static)).unwrap();

        // Most specific should match first
        let route = registry.resolve_path("/user/profile").unwrap();
        assert_eq!(route.id, live_id!(user_static));

        let route = registry.resolve_path("/user/123").unwrap();
        assert_eq!(route.id, live_id!(user_dynamic));

        let route = registry.resolve_path("/user/other").unwrap();
        assert_eq!(route.id, live_id!(user_dynamic));

        let route = registry.resolve_path("/user/123/posts").unwrap();
        assert_eq!(route.id, live_id!(user_wildcard));
    }
}
