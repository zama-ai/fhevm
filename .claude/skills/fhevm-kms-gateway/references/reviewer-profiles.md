# Reviewer Profiles - KMS & Gateway

Understanding reviewer preferences for KMS/Gateway security-critical code.

---

## enitrat

**Focus**: Security, ACL logic, cryptographic correctness

### What They Care About

- Correctness of ACL permission checks
- Threshold signature verification
- Key material protection
- Security implications of all changes

### What Gets Quick Approval

- Explicit ACL checks with clear logic
- Tests that verify security properties
- Clear documentation of security assumptions

### What Triggers Comments

- Missing or unclear ACL verification
- Potential key material exposure
- Incomplete security testing
- Assumptions not documented

### Interaction Style

- Thorough review of security code
- Will request additional security tests
- Expects clear reasoning for security decisions
- Values explicit over implicit security

---

## jatZama

**Focus**: Architecture, naming, documentation

### What They Care About

- Architectural consistency across components
- Clear naming conventions
- Comprehensive documentation
- Integration patterns

### What Gets Quick Approval

- Well-structured code following existing patterns
- Clear NatSpec documentation
- Consistent naming with codebase

### What Triggers Comments

- Deviation from established architecture
- Missing or unclear documentation
- Inconsistent naming

### Interaction Style

- Asks clarifying questions about design
- Appreciates architectural explanations
- Values consistency highly

---

## rudy-6-4

**Focus**: Robustness, retry patterns

### What They Care About

- Proper retry logic for KMS calls
- Circuit breaker patterns
- Graceful degradation
- Timeout handling

### What Gets Quick Approval

- Exponential backoff with jitter
- Circuit breakers for external calls
- Clear timeout configurations

### What Triggers Comments

- Missing retry logic for KMS operations
- Unbounded retries
- Poor timeout handling

### Interaction Style

- Fast reviewer
- Focuses on operational reliability
- Appreciates defensive programming

---

## Cross-Reviewer Expectations

### Security-First Mindset

All reviewers expect:

1. ACL checks before any sensitive operation
2. Fail-secure behavior (deny on error)
3. Audit logging without sensitive data
4. Clear security documentation

### Testing Requirements

| Reviewer   | Test Focus                          |
| ---------- | ----------------------------------- |
| enitrat    | Security tests, ACL rejection paths |
| jatZama    | Integration tests, documentation    |
| rudy-6-4   | Retry/timeout behavior tests        |

### Documentation Requirements

| Reviewer   | Documentation Focus                 |
| ---------- | ----------------------------------- |
| enitrat    | Security assumptions, threat model  |
| jatZama    | Architecture, NatSpec               |
| rudy-6-4   | Operational behavior, failure modes |
