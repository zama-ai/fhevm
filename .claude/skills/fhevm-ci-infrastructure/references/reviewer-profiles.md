# Reviewer Profiles - CI/Infrastructure

Understanding reviewer preferences for CI and infrastructure changes.

---

## jatZama

**Focus**: Architecture, naming, consistency

### What They Care About

- Consistent naming across all workflows
- Clear architecture for complex pipelines
- Realistic documentation and comments

### What Gets Quick Approval

- Well-organized workflow files
- Clear job and step names
- Patterns consistent with existing workflows

### What Triggers Comments

- Inconsistent naming conventions
- Unclear workflow purposes
- Missing or confusing documentation

### Interaction Style

- Asks clarifying questions about design
- Appreciates architectural explanations
- Values consistency highly

---

## rudy-6-4

**Focus**: Robustness, retry patterns

### What They Care About

- Proper retry logic for flaky operations
- Timeout configurations
- Graceful failure handling

### What Gets Quick Approval

- Retry patterns for network operations
- Clear timeout settings
- Error handling that doesn't hide failures

### What Triggers Comments

- Missing retries for network calls
- Missing or excessive timeouts
- Swallowed errors

### Interaction Style

- Fast reviewer
- Focuses on operational reliability
- Appreciates defensive programming

---

## eudelins-zama

**Focus**: Pragmatism, quick fixes

### What They Care About

- Simple, working solutions
- Documented tech debt
- Incremental improvements

### What Gets Quick Approval

- Small, focused changes
- Pragmatic implementations
- Follow-up issues for improvements

### What Triggers Comments

- Over-engineered solutions
- Missing tech debt documentation
- Unnecessary complexity

### Interaction Style

- Quick approvals for simple changes
- Prefers practical over perfect
- Values documentation of trade-offs

---

## Common Patterns Across Reviewers

### Immediate Approval Signals

1. Single-purpose PR
2. Tested in branch before review
3. Clear description of changes
4. Follows existing patterns
5. Proper error handling

### Slow Review Signals

1. Multiple unrelated changes
2. Untested workflow changes
3. Missing context for changes
4. Breaking existing patterns
