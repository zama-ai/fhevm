# Reviewer Profiles - Rust Coprocessor

Understanding reviewer preferences helps get PRs merged faster.

---

## jatZama

**Focus**: Architecture, naming, documentation

### What They Care About

- Clean architecture that follows established patterns
- Descriptive naming (no abbreviations without context)
- Realistic examples in documentation and comments
- Consistency with existing codebase conventions

### What Gets Quick Approval

- Well-structured code that mirrors existing patterns
- Clear separation of concerns
- Comprehensive inline comments for complex logic

### What Triggers Comments

- Abbreviated names without explanation
- Deviation from established architectural patterns
- Missing or unclear documentation
- Examples that don't reflect real use cases

### Interaction Style

- Asks clarifying questions before requesting changes
- Appreciates when PRs explain architectural decisions
- Prefers smaller, focused changes over large refactors

---

## enitrat

**Focus**: Security, ACL logic, parallelization

### What They Care About

- Correctness of ACL/permission logic
- Thread safety and parallelization patterns
- Security implications of changes
- Lock ordering and deadlock prevention

### What Gets Quick Approval

- Clear reasoning about security implications
- Proper use of synchronization primitives
- Tests that cover edge cases in concurrent code

### What Triggers Comments

- ACL logic bugs or inconsistencies
- Missing synchronization in concurrent code
- Potential security vulnerabilities
- Race conditions in tests

### Interaction Style

- Thorough review of security-sensitive code
- May request additional tests for edge cases
- Values explicit documentation of thread safety guarantees

---

## rudy-6-4

**Focus**: Retry/robustness, quick approvals

### What They Care About

- Robust error handling and retry patterns
- Graceful degradation under failure
- Clean, straightforward implementations

### What Gets Quick Approval

- Proper exponential backoff with jitter
- Circuit breaker patterns where appropriate
- Clear error categorization (Recoverable vs Irrecoverable)

### What Triggers Comments

- Missing retry logic for network calls
- Unbounded retries without circuit breakers
- Silent failures without logging

### Interaction Style

- Fast reviewer - often responds within hours
- Appreciates PRs that document tech debt for follow-up issues
- Quick to approve if patterns are followed correctly

---

## eudelins-zama

**Focus**: Quick fixes, tech debt documentation

### What They Care About

- Clean, simple fixes
- Documentation of tech debt for future work
- Pragmatic solutions over perfect ones

### What Gets Quick Approval

- Small, focused bug fixes
- PRs that create issues for known tech debt
- Incremental improvements

### What Triggers Comments

- Over-engineered solutions
- Missing tech debt documentation
- Changes that increase complexity unnecessarily

### Interaction Style

- Prefers pragmatic solutions
- Appreciates when follow-up issues are created
- Values clarity over cleverness
