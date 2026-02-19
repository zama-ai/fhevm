#!/bin/bash
# FHEVM Development Environment Setup Script
# Automates the setup of a complete development environment

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
WHITE='\033[0;37m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Banner
show_banner() {
    echo -e "${CYAN}"
    echo "=================================="
    echo "  FHEVM Development Environment   "
    echo "       Setup Script v1.0          "
    echo "=================================="
    echo -e "${NC}"
}

# System checks
check_system() {
    log_info "Checking system requirements..."
    
    # Check OS
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        log_success "Linux detected"
        OS="linux"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        log_success "macOS detected"
        OS="macos"
    else
        log_warning "Unsupported OS: $OSTYPE"
        OS="unknown"
    fi
    
    # Check architecture
    ARCH=$(uname -m)
    log_success "Architecture: $ARCH"
}

# Install Node.js if not present
install_nodejs() {
    if command -v node &> /dev/null; then
        local version=$(node --version | sed 's/v//')
        log_success "Node.js already installed: $version"
        
        # Check if version is sufficient
        if npx semver -r ">=18.0.0" "$version" &> /dev/null; then
            return 0
        else
            log_warning "Node.js version $version is too old, need >=18.0.0"
        fi
    fi
    
    log_info "Installing Node.js..."
    
    if command -v nvm &> /dev/null; then
        log_info "Using NVM to install Node.js..."
        nvm install 20
        nvm use 20
    elif [ "$OS" = "linux" ]; then
        # Install via NodeSource repository
        curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
        sudo apt-get install -y nodejs
    elif [ "$OS" = "macos" ]; then
        if command -v brew &> /dev/null; then
            brew install node@20
        else
            log_error "Homebrew not found. Please install Node.js manually."
            exit 1
        fi
    else
        log_error "Cannot install Node.js automatically. Please install Node.js 18+ manually."
        exit 1
    fi
    
    log_success "Node.js installed successfully"
}

# Install Rust if not present
install_rust() {
    if command -v rustc &> /dev/null; then
        local version=$(rustc --version | cut -d' ' -f2)
        log_success "Rust already installed: $version"
        return 0
    fi
    
    log_info "Installing Rust..."
    
    if [ "$OS" = "linux" ] || [ "$OS" = "macos" ]; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        log_success "Rust installed successfully"
    else
        log_warning "Cannot install Rust automatically. Please install Rust manually."
    fi
}

# Install Docker if not present
install_docker() {
    if command -v docker &> /dev/null; then
        local version=$(docker --version | cut -d' ' -f3 | cut -d',' -f1)
        log_success "Docker already installed: $version"
        return 0
    fi
    
    log_info "Installing Docker..."
    
    if [ "$OS" = "linux" ]; then
        # Install Docker on Linux
        curl -fsSL https://get.docker.com -o get-docker.sh
        sudo sh get-docker.sh
        sudo usermod -aG docker $USER
        rm get-docker.sh
        log_success "Docker installed successfully"
        log_warning "Please log out and back in for Docker group changes to take effect"
    elif [ "$OS" = "macos" ]; then
        if command -v brew &> /dev/null; then
            brew install --cask docker
            log_success "Docker Desktop installed via Homebrew"
        else
            log_warning "Homebrew not found. Please install Docker Desktop manually."
        fi
    else
        log_warning "Cannot install Docker automatically. Please install Docker manually."
    fi
}

# Install additional tools
install_tools() {
    log_info "Installing additional development tools..."
    
    if [ "$OS" = "linux" ]; then
        # Essential build tools
        sudo apt-get update
        sudo apt-get install -y \
            build-essential \
            git \
            curl \
            wget \
            jq \
            python3 \
            python3-pip \
            make \
            gcc \
            g++ \
            pkg-config \
            libssl-dev
    elif [ "$OS" = "macos" ]; then
        if command -v brew &> /dev/null; then
            brew install \
                git \
                curl \
                wget \
                jq \
                python3 \
                make \
                pkg-config \
                openssl
        else
            log_warning "Homebrew not found. Please install development tools manually."
        fi
    fi
    
    log_success "Development tools installed"
}

# Setup project dependencies
setup_project() {
    log_info "Setting up FHEVM project..."
    
    cd "$PROJECT_ROOT"
    
    # Install root dependencies
    log_info "Installing root dependencies..."
    npm install
    
    # Install project dependencies
    local projects=("gateway-contracts" "host-contracts" "library-solidity" "protocol-contracts")
    
    for project in "${projects[@]}"; do
        if [ -d "$project" ]; then
            log_info "Installing dependencies for $project..."
            cd "$project"
            npm install
            cd "$PROJECT_ROOT"
        fi
    done
    
    # Setup Rust projects
    local rust_projects=("coprocessor" "kms-connector")
    
    for project in "${rust_projects[@]}"; do
        if [ -d "$project" ]; then
            log_info "Setting up Rust project: $project..."
            cd "$project"
            cargo build
            cd "$PROJECT_ROOT"
        fi
    done
    
    log_success "Project setup completed"
}

# Create development configuration
create_dev_config() {
    log_info "Creating development configuration..."
    
    # Create .env files if they don't exist
    local env_files=(
        "gateway-contracts/.env"
        "host-contracts/.env"
        "test-suite/e2e/.env"
    )
    
    for env_file in "${env_files[@]}"; do
        if [ ! -f "$env_file" ] && [ -f "${env_file}.example" ]; then
            log_info "Creating $env_file from example..."
            cp "${env_file}.example" "$env_file"
        fi
    done
    
    # Create build directory
    mkdir -p build logs
    
    # Create VS Code settings if VS Code is installed
    if command -v code &> /dev/null; then
        mkdir -p .vscode
        cat > .vscode/settings.json << EOF
{
    "editor.formatOnSave": true,
    "editor.codeActionsOnSave": {
        "source.fixAll.eslint": true
    },
    "eslint.workingDirectories": [
        "gateway-contracts",
        "host-contracts",
        "library-solidity",
        "protocol-contracts"
    ],
    "prettier.requireConfig": true,
    "files.exclude": {
        "**/node_modules": true,
        "**/target": true,
        "**/artifacts": true,
        "**/cache": true
    }
}
EOF
        log_success "VS Code settings created"
    fi
    
    log_success "Development configuration created"
}

# Run validation tests
run_validation() {
    log_info "Running validation tests..."
    
    cd "$PROJECT_ROOT"
    
    # Test Node.js projects
    local projects=("gateway-contracts" "host-contracts" "library-solidity")
    
    for project in "${projects[@]}"; do
        if [ -d "$project" ]; then
            log_info "Testing $project..."
            cd "$project"
            
            if npm run test --silent 2>/dev/null; then
                log_success "$project tests passed"
            else
                log_warning "$project tests failed or no tests found"
            fi
            
            cd "$PROJECT_ROOT"
        fi
    done
    
    # Test Rust projects
    local rust_projects=("coprocessor" "kms-connector")
    
    for project in "${rust_projects[@]}"; do
        if [ -d "$project" ]; then
            log_info "Testing Rust project: $project..."
            cd "$project"
            
            if cargo test --quiet; then
                log_success "$project Rust tests passed"
            else
                log_warning "$project Rust tests failed"
            fi
            
            cd "$PROJECT_ROOT"
        fi
    done
    
    log_success "Validation completed"
}

# Main setup function
main() {
    local install_deps=true
    local install_rust_flag=true
    local install_docker_flag=true
    local run_tests=false
    
    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --no-deps)
                install_deps=false
                shift
                ;;
            --no-rust)
                install_rust_flag=false
                shift
                ;;
            --no-docker)
                install_docker_flag=false
                shift
                ;;
            --test)
                run_tests=true
                shift
                ;;
            --help)
                echo "Usage: $0 [OPTIONS]"
                echo ""
                echo "Options:"
                echo "  --no-deps      Skip dependency installation"
                echo "  --no-rust      Skip Rust installation"
                echo "  --no-docker    Skip Docker installation"
                echo "  --test         Run validation tests"
                echo "  --help         Show this help message"
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                exit 1
                ;;
        esac
    done
    
    show_banner
    
    check_system
    
    if [ "$install_deps" = true ]; then
        install_tools
        install_nodejs
    fi
    
    if [ "$install_rust_flag" = true ]; then
        install_rust
    fi
    
    if [ "$install_docker_flag" = true ]; then
        install_docker
    fi
    
    setup_project
    create_dev_config
    
    if [ "$run_tests" = true ]; then
        run_validation
    fi
    
    echo -e "${GREEN}"
    echo "=================================="
    echo "  Development Environment Ready!  "
    echo "=================================="
    echo -e "${NC}"
    echo ""
    echo "Next steps:"
    echo "  1. Run 'make help' to see available commands"
    echo "  2. Run 'make build' to build all projects"
    echo "  3. Run 'make test' to run all tests"
    echo "  4. Run 'make docker' to build Docker images"
    echo ""
    echo "Happy coding! 🚀"
}

# Run main function
main "$@"
