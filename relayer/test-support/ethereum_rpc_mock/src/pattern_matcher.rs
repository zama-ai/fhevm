//! Pattern matching engine for request/response pairs
//!
//! This module implements the core pattern matching logic that matches incoming
//! JSON-RPC requests against user-defined patterns and returns appropriate responses.
//!
//! Key features:
//! - First match wins: Patterns are evaluated in registration order
//! - Usage tracking: Patterns have usage limits that are decremented on matches
//! - Thread safety: Uses RwLock for concurrent access
//! - Exhausted patterns: Keep patterns in list even when usage is exhausted

use crate::mock_server::rpc_types::{CallParams, Response, TxParams};
use std::fmt;
use std::sync::Arc;
use std::sync::RwLock;

/// Response strategy for call patterns: either a fixed value or a request-dependent closure.
#[derive(Clone)]
pub enum CallResponder {
    /// Return the same response regardless of request content.
    Static(Response),
    /// Compute the response from the incoming [`CallParams`].
    Dynamic(Arc<dyn Fn(&CallParams) -> Response + Send + Sync>),
}

impl CallResponder {
    /// Produce a concrete [`Response`] for the given request.
    pub fn resolve(&self, params: &CallParams) -> Response {
        match self {
            Self::Static(r) => r.clone(),
            Self::Dynamic(f) => f(params),
        }
    }
}

impl fmt::Debug for CallResponder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Static(r) => f.debug_tuple("Static").field(r).finish(),
            Self::Dynamic(_) => f.debug_tuple("Dynamic").field(&"<closure>").finish(),
        }
    }
}

/// Type alias for predicate functions used in pattern matching
pub type PredicateFn<T> = Arc<dyn Fn(&T) -> bool + Send + Sync>;

/// Usage limits for pattern matching (simplified)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UsageLimit {
    /// Pattern can be used unlimited times
    #[default]
    Unlimited,
    /// Pattern can only be used once
    Once,
}

impl UsageLimit {
    /// Create a usage limit (simplified to once vs unlimited)
    pub fn times(count: u32) -> Self {
        if count <= 1 {
            Self::Once
        } else {
            Self::Unlimited
        }
    }

    /// Check if the pattern can still be used (internal method)
    pub(crate) fn can_use(&self) -> bool {
        true // Always available initially - pattern matcher handles removal
    }

    /// Use the pattern once (internal method)
    pub(crate) fn use_once(&mut self) -> bool {
        true // Pattern matcher handles removal for Once variants
    }

    /// Check if this is a once-only usage limit
    pub(crate) fn is_once(&self) -> bool {
        matches!(self, Self::Once)
    }
}

/// Generic pattern that matches requests of type P and returns responses of type R
#[derive(Clone)]
pub struct Pattern<P, R> {
    pub predicate: PredicateFn<P>,
    pub response: R,
    pub usage_limit: UsageLimit,
}

impl<P, R: std::fmt::Debug> std::fmt::Debug for Pattern<P, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pattern")
            .field("response", &self.response)
            .field("usage_limit", &self.usage_limit)
            .finish()
    }
}

/// Transaction pattern that matches transaction requests
pub type TransactionPattern = Pattern<TxParams, Response>;

/// Call pattern that matches call requests and resolves via [`CallResponder`].
pub type CallPattern = Pattern<CallParams, CallResponder>;

/// Pattern matcher that stores and matches patterns against requests
#[derive(Debug, Clone)]
pub struct PatternMatcher {
    transaction_patterns: Arc<RwLock<Vec<TransactionPattern>>>,
    call_patterns: Arc<RwLock<Vec<CallPattern>>>,
}

impl PatternMatcher {
    /// Create new empty pattern matcher
    pub fn new() -> Self {
        Self {
            transaction_patterns: Arc::new(RwLock::new(Vec::new())),
            call_patterns: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Add pattern to pattern collection
    fn add_pattern<P, R>(
        patterns: &Arc<RwLock<Vec<Pattern<P, R>>>>,
        predicate: PredicateFn<P>,
        response: R,
        usage: UsageLimit,
    ) {
        let pattern = Pattern {
            predicate,
            response,
            usage_limit: usage,
        };
        patterns.write().unwrap().push(pattern);
    }

    /// Find matching pattern in pattern collection
    fn find_match<P, R: Clone>(
        patterns: &Arc<RwLock<Vec<Pattern<P, R>>>>,
        params: &P,
    ) -> Option<R> {
        let mut patterns_guard = patterns.write().unwrap();

        for (i, pattern) in patterns_guard.iter_mut().enumerate() {
            // Check if predicate matches AND usage limit allows use
            if (pattern.predicate)(params) && pattern.usage_limit.can_use() {
                let response = pattern.response.clone();
                // Use the pattern
                if pattern.usage_limit.use_once() {
                    // Remove once-only patterns after use for simplified boolean approach
                    if pattern.usage_limit.is_once() {
                        patterns_guard.remove(i);
                    }
                    return Some(response);
                }
            }
        }

        None
    }

    /// Add transaction pattern that matches requests with predicate
    pub fn add_transaction_pattern(
        &self,
        predicate: PredicateFn<TxParams>,
        response: Response,
        usage: UsageLimit,
    ) {
        Self::add_pattern(&self.transaction_patterns, predicate, response, usage);
    }

    /// Add call pattern with a static response.
    pub fn add_call_pattern(
        &self,
        predicate: PredicateFn<CallParams>,
        response: Response,
        usage: UsageLimit,
    ) {
        Self::add_pattern(
            &self.call_patterns,
            predicate,
            CallResponder::Static(response),
            usage,
        );
    }

    /// Add call pattern with a dynamic (request-dependent) response.
    pub fn add_call_pattern_dynamic(
        &self,
        predicate: PredicateFn<CallParams>,
        responder: Arc<dyn Fn(&CallParams) -> Response + Send + Sync>,
        usage: UsageLimit,
    ) {
        Self::add_pattern(
            &self.call_patterns,
            predicate,
            CallResponder::Dynamic(responder),
            usage,
        );
    }

    /// Find matching transaction pattern for given parameters
    pub fn find_transaction_match(&self, tx_params: &TxParams) -> Option<Response> {
        Self::find_match(&self.transaction_patterns, tx_params)
    }

    /// Find matching call pattern for given parameters.
    ///
    /// Uses [`CallResponder::resolve`] so dynamic patterns can inspect the request.
    pub fn find_call_match(&self, call_params: &CallParams) -> Option<Response> {
        let mut patterns_guard = self.call_patterns.write().unwrap();

        for (i, pattern) in patterns_guard.iter_mut().enumerate() {
            if (pattern.predicate)(call_params) && pattern.usage_limit.can_use() {
                let response = pattern.response.resolve(call_params);
                if pattern.usage_limit.use_once() {
                    if pattern.usage_limit.is_once() {
                        patterns_guard.remove(i);
                    }
                    return Some(response);
                }
            }
        }

        None
    }

    /// Clear all patterns from both collections
    pub fn clear_all_patterns(&self) {
        self.transaction_patterns.write().unwrap().clear();
        self.call_patterns.write().unwrap().clear();
    }
}

impl Default for PatternMatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock_server::rpc_types::{CallParams, Response, TxParams};
    use alloy::primitives::{Address, Bytes, U256};
    use std::sync::Arc;

    #[test]
    fn test_pattern_matching_core_behavior() {
        let patterns = Arc::new(RwLock::new(Vec::new()));

        PatternMatcher::add_pattern(
            &patterns,
            Arc::new(|&x: &u32| x > 5),
            "big".to_string(),
            UsageLimit::Once,
        );
        PatternMatcher::add_pattern(
            &patterns,
            Arc::new(|&x: &u32| x > 0),
            "small".to_string(),
            UsageLimit::Unlimited,
        );

        assert_eq!(
            PatternMatcher::find_match(&patterns, &10),
            Some("big".to_string()),
            "First pattern should match when value > 5 (first-match-wins)"
        );

        assert_eq!(
            PatternMatcher::find_match(&patterns, &10),
            Some("small".to_string()),
            "Second pattern should match after first is exhausted (Once usage limit)"
        );

        assert_eq!(
            PatternMatcher::find_match(&patterns, &3),
            Some("small".to_string()),
            "Second pattern should continue working (Unlimited usage limit)"
        );

        assert_eq!(
            PatternMatcher::find_match(&patterns, &0),
            None,
            "No pattern should match when all predicates fail (value <= 0)"
        );
    }

    #[test]
    fn test_dual_api_independence() {
        let matcher = PatternMatcher::new();

        matcher.add_transaction_pattern(
            Arc::new(|_| true),
            Response::transaction_success(),
            UsageLimit::Once,
        );
        matcher.add_call_pattern(
            Arc::new(|_| true),
            Response::call_success(Bytes::new()),
            UsageLimit::Unlimited,
        );

        let tx_params = TxParams::new(Some(Address::ZERO), U256::ZERO, Bytes::new());
        let call_params = CallParams::new(Address::ZERO, Bytes::new());

        assert!(
            matcher.find_transaction_match(&tx_params).is_some(),
            "Transaction patterns should work independently of call patterns"
        );

        assert!(
            matcher.find_call_match(&call_params).is_some(),
            "Call patterns should work independently of transaction patterns"
        );

        assert!(
            matcher.find_transaction_match(&tx_params).is_none(),
            "Transaction pattern should be exhausted after Once usage"
        );

        assert!(
            matcher.find_call_match(&call_params).is_some(),
            "Call pattern should still work (Unlimited usage)"
        );

        matcher.clear_all_patterns();

        assert!(
            matcher.find_transaction_match(&tx_params).is_none(),
            "Clear should remove all transaction patterns"
        );
        assert!(
            matcher.find_call_match(&call_params).is_none(),
            "Clear should remove all call patterns"
        );
    }
}
