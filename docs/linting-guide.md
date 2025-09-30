# FHEVM Linting and Code Style Guide

This document provides comprehensive guidelines for code linting and formatting in the FHEVM project.

## 🎯 Overview

The FHEVM project uses a combination of ESLint and Prettier to maintain consistent code style and quality across all TypeScript, JavaScript, and Solidity files.

## 📋 Tools Used

### ESLint
- **Purpose**: Code quality and error detection
- **Languages**: TypeScript, JavaScript
- **Configuration**: `.eslintrc.yml` files

### Prettier
- **Purpose**: Code formatting and style consistency
- **Languages**: TypeScript, JavaScript, Solidity, Markdown, JSON, YAML
- **Configuration**: `.prettierrc.yml` files

### Solhint
- **Purpose**: Solidity-specific linting
- **Languages**: Solidity
- **Configuration**: Integrated with Hardhat

## 🚀 Quick Start

### Running Lints

```bash
# Run all linting checks
npm run lint

# Run specific linting tools
npm run lint:eslint      # TypeScript/JavaScript linting
npm run lint:prettier    # Code formatting check
npm run lint:sol         # Solidity linting (gateway-contracts only)
```

### Fixing Issues

```bash
# Auto-fix all fixable issues
npm run lint:fix

# Fix specific tools
npm run lint:eslint:fix     # Auto-fix ESLint issues
npm run lint:prettier:fix   # Auto-format code
```

### Formatting Code

```bash
# Format all files
npm run format

# Check formatting without fixing
npm run format:check
```

## ⚙️ Configuration Details

### ESLint Configuration

#### Root Configuration (`.eslintrc.yml`)
- Extends recommended TypeScript rules
- Integrates with Prettier to avoid conflicts
- Ignores build outputs and generated files

#### Key Rules
```yaml
rules:
  # TypeScript specific
  "@typescript-eslint/no-unused-vars": error
  "@typescript-eslint/prefer-const": error
  "@typescript-eslint/no-explicit-any": warn
  
  # General
  "no-console": warn
  "no-debugger": error
  "prefer-const": error
  "eqeqeq": ["error", "always"]
```

### Prettier Configuration

#### Global Settings
```yaml
printWidth: 120
tabWidth: 2
singleQuote: false
trailingComma: "all"
endOfLine: "lf"
```

#### File-Specific Overrides
- **Solidity**: 4-space tabs, 120 character width
- **TypeScript/JavaScript**: 2-space tabs, import sorting
- **Markdown**: 80 character width, preserve wrapping
- **JSON/YAML**: 2-space tabs, 120 character width

## 📁 Project Structure

```
fhevm/
├── .eslintrc.yml              # Root ESLint config
├── .eslintignore              # Root ESLint ignore
├── .prettierrc.yml            # Root Prettier config
├── .prettierignore            # Root Prettier ignore
├── gateway-contracts/
│   ├── .eslintrc.yml          # Gateway-specific ESLint
│   ├── .eslintignore          # Gateway-specific ignore
│   └── package.json           # Linting scripts
├── host-contracts/
│   ├── .eslintrc.yml          # Host-specific ESLint
│   ├── .eslintignore          # Host-specific ignore
│   └── package.json           # Linting scripts
└── library-solidity/
    ├── .eslintrc.yml          # Library-specific ESLint
    ├── .eslintignore          # Library-specific ignore
    └── package.json           # Linting scripts
```

## 🔧 IDE Integration

### VS Code Setup

Install these extensions:
- **ESLint** (`ms-vscode.vscode-eslint`)
- **Prettier** (`esbenp.prettier-vscode`)

Add to your `settings.json`:
```json
{
  "editor.formatOnSave": true,
  "editor.codeActionsOnSave": {
    "source.fixAll.eslint": true
  },
  "eslint.workingDirectories": [
    "gateway-contracts",
    "host-contracts", 
    "library-solidity"
  ],
  "prettier.requireConfig": true
}
```

### Other IDEs

#### IntelliJ IDEA / WebStorm
1. Install ESLint and Prettier plugins
2. Enable "Run eslint --fix on save"
3. Set Prettier as default formatter

#### Vim/Neovim
```vim
" Using vim-plug
Plug 'dense-analysis/ale'
Plug 'prettier/vim-prettier'

" Auto-fix on save
let g:ale_fixers = {
\   'javascript': ['eslint', 'prettier'],
\   'typescript': ['eslint', 'prettier'],
\}
let g:ale_fix_on_save = 1
```

## 📝 Coding Standards

### TypeScript/JavaScript

#### Import Organization
```typescript
// 1. Third-party modules
import { ethers } from "ethers";
import { HardhatUserConfig } from "hardhat/types";

// 2. Local imports
import { deployContract } from "./deploy";
import { CONFIG } from "../config";
```

#### Variable Naming
```typescript
// ✅ Good
const contractAddress = "0x...";
const isDeployed = true;
const MAX_RETRIES = 3;

// ❌ Bad
const contract_address = "0x...";
const isDeployed = true;
const maxRetries = 3;
```

#### Error Handling
```typescript
// ✅ Good
try {
  await contract.deploy();
} catch (error) {
  console.error("Deployment failed:", error);
  throw error;
}

// ❌ Bad
try {
  await contract.deploy();
} catch (e) {
  console.log(e);
}
```

### Solidity

#### Contract Structure
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/...";

/**
 * @title ContractName
 * @dev Contract description
 */
contract ContractName {
    // State variables
    address private immutable _owner;
    
    // Events
    event EventName(address indexed user);
    
    // Constructor
    constructor() {
        _owner = msg.sender;
    }
    
    // Functions
    function functionName() external view returns (uint256) {
        return 0;
    }
}
```

#### Function Naming
```solidity
// ✅ Good
function transferTokens(address to, uint256 amount) external;
function getBalance(address account) external view returns (uint256);

// ❌ Bad
function transfer_tokens(address to, uint256 amount) external;
function GetBalance(address account) external view returns (uint256);
```

## 🚨 Common Issues and Solutions

### ESLint Issues

#### "Unexpected any"
```typescript
// ❌ Problem
function processData(data: any): void {}

// ✅ Solution
function processData(data: unknown): void {}
// or
interface DataType {
  id: string;
  value: number;
}
function processData(data: DataType): void {}
```

#### "Unused variable"
```typescript
// ❌ Problem
const unusedVar = "hello";
console.log("test");

// ✅ Solution
const _unusedVar = "hello"; // Prefix with underscore
console.log("test");
```

### Prettier Issues

#### Import sorting
```typescript
// ❌ Problem
import { Contract } from "./contract";
import { ethers } from "ethers";

// ✅ Solution (auto-fixed by Prettier)
import { ethers } from "ethers";

import { Contract } from "./contract";
```

## 🔄 CI/CD Integration

### GitHub Actions
The project includes automatic linting in CI:

```yaml
- name: Run ESLint
  run: npm run lint:eslint

- name: Run Prettier Check
  run: npm run lint:prettier
```

### Pre-commit Hooks
Consider adding pre-commit hooks with Husky:

```bash
npm install --save-dev husky lint-staged

# Add to package.json
{
  "husky": {
    "hooks": {
      "pre-commit": "lint-staged"
    }
  },
  "lint-staged": {
    "*.{ts,js}": ["eslint --fix", "prettier --write"],
    "*.{sol,md,json,yml,yaml}": ["prettier --write"]
  }
}
```

## 📊 Best Practices

### 1. Regular Maintenance
- Run `npm run lint` before committing
- Fix issues immediately rather than accumulating them
- Update configurations when adding new file types

### 2. Team Collaboration
- Agree on coding standards as a team
- Use consistent IDE settings
- Document any project-specific rules

### 3. Performance
- Use `.eslintignore` and `.prettierignore` effectively
- Exclude build outputs and generated files
- Consider using `--cache` flag for large projects

### 4. Gradual Adoption
- Start with basic rules and gradually add stricter ones
- Use `warn` instead of `error` for new rules initially
- Provide training and documentation for team members

## 🆘 Troubleshooting

### Common Problems

#### "Cannot find module '@typescript-eslint/parser'"
```bash
npm install --save-dev @typescript-eslint/parser @typescript-eslint/eslint-plugin
```

#### "Prettier conflicts with ESLint"
Make sure your ESLint config extends `"prettier"`:
```yaml
extends:
  - "prettier"  # This must be last
```

#### "Slow linting performance"
- Add more files to `.eslintignore`
- Use `--cache` flag
- Consider using `eslint-plugin-import` for better performance

### Getting Help

1. Check the [ESLint documentation](https://eslint.org/docs/)
2. Review [Prettier configuration options](https://prettier.io/docs/en/configuration.html)
3. Ask team members or create an issue in the repository

## 📚 Additional Resources

- [ESLint Rules](https://eslint.org/docs/rules/)
- [TypeScript ESLint Rules](https://typescript-eslint.io/rules/)
- [Prettier Options](https://prettier.io/docs/en/options.html)
- [Solhint Rules](https://protofire.github.io/solhint/docs/rules.html)
