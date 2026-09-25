# FHEVM Codebase Documentation (mdBook)

This directory contains comprehensive technical documentation for the FHEVM codebase, structured as an mdBook.

## Quick Start

### Install mdBook

```bash
cargo install mdbook
```

### Build and Serve Locally

```bash
# From this directory
mdbook serve

# Or from repo root
mdbook serve docs/codebase
```

Then open http://localhost:3000 in your browser.

### Build Static Site

```bash
mdbook build
```

Output will be in `book/` directory (gitignored).

## Structure

```
docs/codebase/
├── book.toml              # mdBook configuration
├── src/
│   ├── SUMMARY.md         # Table of contents (navigation)
│   ├── README.md          # Landing page
│   │
│   ├── executive-summary.md
│   ├── key-concepts.md
│   ├── architecture.md
│   ├── component-health.md
│   │
│   ├── components/        # Core component docs
│   │   ├── README.md
│   │   ├── gateway-contracts.md
│   │   ├── host-contracts.md
│   │   ├── library-solidity.md
│   │   ├── coprocessor.md
│   │   ├── kms-connector.md
│   │   ├── protocol-contracts.md
│   │   └── infrastructure.md
│   │
│   ├── workflows/         # Key workflow docs
│   │   ├── README.md
│   │   ├── symbolic-execution.md
│   │   ├── decryption-pipeline.md
│   │   └── input-verification.md
│   │
│   └── reference/         # Reference materials
│       ├── tech-stack.md
│       ├── roadmap.md
│       ├── quick-reference.md
│       └── glossary.md
```

## Current Status

✅ **Level 0 Complete**: High-level overview with all major sections populated
🚧 **Level 1 In Progress**: Detailed documentation of each component (see TODOs)

Each component file includes `[TODO]` markers indicating areas for deeper documentation. See `src/reference/roadmap.md` for the complete documentation plan.

## Contributing

When adding or updating documentation:

1. Edit markdown files in `src/`
2. Update `SUMMARY.md` if adding new pages
3. Run `mdbook serve` to preview changes
4. Remove `[TODO]` markers when documentation is complete
5. Update `roadmap.md` to reflect progress

## Publishing

To publish to GitHub Pages:

```bash
mdbook build
# Copy book/ contents to gh-pages branch or docs/ directory
```

Or use the provided GitHub Action (if configured).

## Links

- mdBook Documentation: https://rust-lang.github.io/mdBook/
- Original Overview: `/CODEBASE_OVERVIEW.md` (consolidated into this mdBook structure)
