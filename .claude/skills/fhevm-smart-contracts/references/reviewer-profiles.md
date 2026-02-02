# Reviewer Profiles - Smart Contracts

Understanding reviewer preferences helps get PRs merged faster.

---

## jatZama

**Focus**: Architecture, documentation, examples

### What They Care About

- Contract architecture matching established patterns
- Comprehensive NatSpec with realistic examples
- Consistent naming across the codebase
- Clear separation between interface and implementation

### What Gets Quick Approval

- Well-documented functions with usage examples
- Patterns that mirror existing contracts
- Clean interface definitions

### What Triggers Comments

- Missing or incomplete NatSpec
- Examples that don't reflect real usage
- Inconsistent naming with existing codebase
- Architectural decisions without explanation

### Interaction Style

- Asks clarifying questions about design decisions
- Appreciates when PRs explain why a pattern was chosen
- Values documentation as much as code

---

## enitrat

**Focus**: Security, ACL logic, access control

### What They Care About

- Correctness of ACL/permission logic
- Security implications of all changes
- Proper access control patterns
- Edge cases in permission handling

### What Gets Quick Approval

- Explicit tests for ACL paths
- Security considerations documented
- Access control following OpenZeppelin patterns

### What Triggers Comments

- ACL logic bugs or edge cases
- Missing permission checks
- Potential security vulnerabilities
- Access control inconsistencies

### Interaction Style

- Thorough review of security-sensitive code
- May request additional edge case tests
- Values explicit documentation of access patterns

---

## eudelins-zama

**Focus**: Pragmatic solutions, quick fixes

### What They Care About

- Clean, simple implementations
- Pragmatic over perfect
- Tech debt tracked in issues

### What Gets Quick Approval

- Small, focused fixes
- Straightforward implementations
- Issues created for known improvements

### What Triggers Comments

- Over-engineered solutions
- Complexity without justification
- Missing tech debt documentation

### Interaction Style

- Fast reviewer
- Prefers simple solutions
- Appreciates follow-up issue creation

---

## Common Patterns Across Reviewers

### Immediate Approval Signals

1. PR description explains "why" not just "what"
2. Tests cover the happy path AND edge cases
3. NatSpec present for all public functions
4. Custom errors used consistently
5. ACL permissions explicitly handled

### Slow Review Signals

1. Large PRs without context
2. Multiple unrelated changes
3. Missing test coverage
4. Breaking changes without migration plan

---

## Review Response Guidelines

### When Asked for Changes

1. Address ALL comments before re-requesting review
2. Explain your reasoning if you disagree
3. Create follow-up issues for out-of-scope suggestions
4. Mark conversations resolved when addressed

### When Approved with Comments

1. Address non-blocking comments if trivial
2. Create issues for larger suggestions
3. Don't block merge for stylistic preferences
