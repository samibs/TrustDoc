//! Resource exhaustion protection and rate limiting
//!
//! Security Fixes:
//! - Vulnerability #7: Denial of Service (DoS) via malformed handshake
//! - Vulnerability #9: State Machine Resource Exhaustion Loop
//! - Vulnerability #10: Cryptographic Power Exhaustion Attack
//!
//! This module provides utilities to prevent resource exhaustion attacks
//! through rate limiting, circuit breakers, and resource budgets.

use crate::error::{TdfError, TdfResult};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Circuit breaker state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CircuitState {
    Closed,    // Normal operation
    Open,      // Circuit open, rejecting requests
    HalfOpen, // Testing if service recovered
}

/// Circuit breaker for preventing cascade failures
///
/// Security Fix (Vuln #7, #9): Implements circuit breaker pattern to
/// prevent resource exhaustion from cascading failures.
pub struct CircuitBreaker {
    state: Arc<std::sync::Mutex<CircuitState>>,
    failure_count: Arc<AtomicU64>,
    last_failure_time: Arc<std::sync::Mutex<Option<Instant>>>,
    failure_threshold: u64,
    timeout: Duration,
    half_open_timeout: Duration,
}

impl CircuitBreaker {
    /// Create a new circuit breaker
    ///
    /// # Arguments
    /// * `failure_threshold` - Number of failures before opening circuit
    /// * `timeout` - Duration to keep circuit open
    /// * `half_open_timeout` - Duration to test in half-open state
    pub fn new(failure_threshold: u64, timeout: Duration, half_open_timeout: Duration) -> Self {
        CircuitBreaker {
            state: Arc::new(std::sync::Mutex::new(CircuitState::Closed)),
            failure_count: Arc::new(AtomicU64::new(0)),
            last_failure_time: Arc::new(std::sync::Mutex::new(None)),
            failure_threshold,
            timeout,
            half_open_timeout,
        }
    }

    /// Check if request should be allowed
    ///
    /// # Returns
    /// * `Ok(())` if request should proceed
    /// * `Err(TdfError)` if circuit is open
    pub fn check(&self) -> TdfResult<()> {
        let mut state = self.state.lock().unwrap();
        let now = Instant::now();

        match *state {
            CircuitState::Closed => {
                // Check if we've exceeded failure threshold
                if self.failure_count.load(Ordering::Relaxed) >= self.failure_threshold {
                    *state = CircuitState::Open;
                    *self.last_failure_time.lock().unwrap() = Some(now);
                    return Err(TdfError::PolicyViolation(
                        "Circuit breaker opened due to excessive failures".to_string()
                    ));
                }
                Ok(())
            }
            CircuitState::Open => {
                // Check if timeout has passed
                if let Some(last_failure) = *self.last_failure_time.lock().unwrap() {
                    if now.duration_since(last_failure) >= self.timeout {
                        *state = CircuitState::HalfOpen;
                        self.failure_count.store(0, Ordering::Relaxed);
                        return Ok(());
                    }
                }
                Err(TdfError::PolicyViolation(
                    "Circuit breaker is open".to_string()
                ))
            }
            CircuitState::HalfOpen => {
                // Allow one request to test if service recovered
                Ok(())
            }
        }
    }

    /// Record a successful operation
    pub fn record_success(&self) {
        let mut state = self.state.lock().unwrap();
        if *state == CircuitState::HalfOpen {
            *state = CircuitState::Closed;
            self.failure_count.store(0, Ordering::Relaxed);
        } else if *state == CircuitState::Closed {
            // Reset failure count on success
            self.failure_count.store(0, Ordering::Relaxed);
        }
    }

    /// Record a failed operation
    ///
    /// Security Fix: Race-safe failure recording with proper synchronization
    pub fn record_failure(&self) {
        // Atomically increment and get new count
        let count = self.failure_count.fetch_add(1, Ordering::SeqCst) + 1;

        // Update timestamp atomically
        {
            let mut last_time = self.last_failure_time.lock().unwrap();
            *last_time = Some(Instant::now());
        }

        // Check threshold with proper synchronization
        if count >= self.failure_threshold {
            let mut state = self.state.lock().unwrap();
            // Double-check pattern to prevent race conditions
            if *state == CircuitState::Closed && count >= self.failure_threshold {
                *state = CircuitState::Open;
            }
        }

        // Check for half-open state transition
        let mut state = self.state.lock().unwrap();
        if *state == CircuitState::HalfOpen {
            // If we fail in half-open, go back to open
            *state = CircuitState::Open;
        }
    }
}

/// Rate limiter to prevent resource exhaustion
///
/// Security Fix (Vuln #7, #10): Implements token bucket rate limiting
/// to prevent excessive resource consumption.
pub struct RateLimiter {
    tokens: Arc<AtomicU64>,
    capacity: u64,
    refill_rate: u64, // tokens per second
    last_refill: Arc<std::sync::Mutex<Instant>>,
}

impl RateLimiter {
    /// Create a new rate limiter
    ///
    /// # Arguments
    /// * `capacity` - Maximum number of tokens
    /// * `refill_rate` - Tokens added per second
    pub fn new(capacity: u64, refill_rate: u64) -> Self {
        RateLimiter {
            tokens: Arc::new(AtomicU64::new(capacity)),
            capacity,
            refill_rate,
            last_refill: Arc::new(std::sync::Mutex::new(Instant::now())),
        }
    }

    /// Try to consume a token
    ///
    /// # Returns
    /// * `Ok(())` if token was consumed
    /// * `Err(TdfError)` if rate limit exceeded
    pub fn try_acquire(&self) -> TdfResult<()> {
        self.refill_tokens();

        let tokens = self.tokens.load(Ordering::Relaxed);
        if tokens > 0 {
            self.tokens.fetch_sub(1, Ordering::Relaxed);
            Ok(())
        } else {
            Err(TdfError::PolicyViolation(
                "Rate limit exceeded".to_string()
            ))
        }
    }

    /// Refill tokens based on elapsed time
    fn refill_tokens(&self) {
        let mut last_refill = self.last_refill.lock().unwrap();
        let now = Instant::now();
        let elapsed = now.duration_since(*last_refill);

        if elapsed.as_secs() > 0 {
            let tokens_to_add = (elapsed.as_secs() as u64)
                .saturating_mul(self.refill_rate)
                .min(self.capacity);

            let current = self.tokens.load(Ordering::Relaxed);
            let new_tokens = (current + tokens_to_add).min(self.capacity);
            self.tokens.store(new_tokens, Ordering::Relaxed);
            *last_refill = now;
        }
    }
}

/// Resource budget tracker
///
/// Security Fix (Vuln #10): Tracks resource consumption to prevent
/// power exhaustion attacks on field devices.
pub struct ResourceBudget {
    cpu_time: Arc<AtomicU64>, // milliseconds
    memory_used: Arc<AtomicU64>, // bytes
    operations: Arc<AtomicU64>,
    max_cpu_time: u64,
    max_memory: u64,
    max_operations: u64,
}

impl ResourceBudget {
    /// Create a new resource budget
    ///
    /// # Arguments
    /// * `max_cpu_time` - Maximum CPU time in milliseconds
    /// * `max_memory` - Maximum memory in bytes
    /// * `max_operations` - Maximum number of operations
    pub fn new(max_cpu_time: u64, max_memory: u64, max_operations: u64) -> Self {
        ResourceBudget {
            cpu_time: Arc::new(AtomicU64::new(0)),
            memory_used: Arc::new(AtomicU64::new(0)),
            operations: Arc::new(AtomicU64::new(0)),
            max_cpu_time,
            max_memory,
            max_operations,
        }
    }

    /// Check if operation is allowed within budget
    ///
    /// # Returns
    /// * `Ok(())` if within budget
    /// * `Err(TdfError)` if budget exceeded
    pub fn check_budget(&self) -> TdfResult<()> {
        if self.cpu_time.load(Ordering::Relaxed) > self.max_cpu_time {
            return Err(TdfError::PolicyViolation(
                "CPU time budget exceeded".to_string()
            ));
        }

        if self.memory_used.load(Ordering::Relaxed) > self.max_memory {
            return Err(TdfError::PolicyViolation(
                "Memory budget exceeded".to_string()
            ));
        }

        if self.operations.load(Ordering::Relaxed) > self.max_operations {
            return Err(TdfError::PolicyViolation(
                "Operation count budget exceeded".to_string()
            ));
        }

        Ok(())
    }

    /// Record CPU time usage
    pub fn record_cpu_time(&self, ms: u64) {
        self.cpu_time.fetch_add(ms, Ordering::Relaxed);
    }

    /// Record memory usage
    pub fn record_memory(&self, bytes: u64) {
        self.memory_used.fetch_add(bytes, Ordering::Relaxed);
    }

    /// Record an operation
    pub fn record_operation(&self) {
        self.operations.fetch_add(1, Ordering::Relaxed);
    }

    /// Reset the budget
    pub fn reset(&self) {
        self.cpu_time.store(0, Ordering::Relaxed);
        self.memory_used.store(0, Ordering::Relaxed);
        self.operations.store(0, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_breaker() {
        let breaker = CircuitBreaker::new(
            3,
            Duration::from_secs(1),
            Duration::from_millis(500)
        );

        // Should allow requests initially
        assert!(breaker.check().is_ok());

        // Record failures
        breaker.record_failure();
        breaker.record_failure();
        breaker.record_failure();

        // Should now reject requests
        assert!(breaker.check().is_err());
    }

    #[test]
    fn test_rate_limiter() {
        let limiter = RateLimiter::new(10, 5);

        // Should allow initial requests
        for _ in 0..10 {
            assert!(limiter.try_acquire().is_ok());
        }

        // Should reject when tokens exhausted
        assert!(limiter.try_acquire().is_err());
    }

    #[test]
    fn test_resource_budget() {
        let budget = ResourceBudget::new(1000, 1024 * 1024, 100);

        // Should allow operations within budget
        assert!(budget.check_budget().is_ok());

        // Exceed CPU budget
        budget.record_cpu_time(2000);
        assert!(budget.check_budget().is_err());

        budget.reset();
        assert!(budget.check_budget().is_ok());
    }
}
