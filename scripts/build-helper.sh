#!/bin/bash
# FHEVM Build Helper Script
# Provides utility functions for the build system

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
BUILD_DIR="$PROJECT_ROOT/build"
LOG_DIR="$BUILD_DIR/logs"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")

# Create necessary directories
mkdir -p "$BUILD_DIR" "$LOG_DIR"

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1" | tee -a "$LOG_DIR/build_$TIMESTAMP.log"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1" | tee -a "$LOG_DIR/build_$TIMESTAMP.log"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1" | tee -a "$LOG_DIR/build_$TIMESTAMP.log"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" | tee -a "$LOG_DIR/build_$TIMESTAMP.log"
}

# Utility functions
check_command() {
    if ! command -v "$1" &> /dev/null; then
        log_error "Command '$1' not found. Please install it and try again."
        exit 1
    fi
}

check_node_version() {
    local required_version="18.0.0"
    local current_version=$(node --version | sed 's/v//')
    
    if [ "$(printf '%s\n' "$required_version" "$current_version" | sort -V | head -n1)" != "$required_version" ]; then
        log_error "Node.js version $required_version or higher is required. Current: $current_version"
        exit 1
    fi
    
    log_success "Node.js version check passed: $current_version"
}

check_rust_version() {
    if command -v rustc &> /dev/null; then
        local rust_version=$(rustc --version | cut -d' ' -f2)
        log_success "Rust version: $rust_version"
    else
        log_warning "Rust not found. Rust projects will be skipped."
    fi
}

check_docker() {
    if command -v docker &> /dev/null; then
        local docker_version=$(docker --version | cut -d' ' -f3 | cut -d',' -f1)
        log_success "Docker version: $docker_version"
    else
        log_warning "Docker not found. Docker builds will be skipped."
    fi
}

# Project validation
validate_project() {
    local project_dir="$1"
    
    if [ ! -d "$project_dir" ]; then
        log_error "Project directory '$project_dir' does not exist"
        return 1
    fi
    
    if [ ! -f "$project_dir/package.json" ] && [ ! -f "$project_dir/Cargo.toml" ]; then
        log_error "Project directory '$project_dir' does not contain package.json or Cargo.toml"
        return 1
    fi
    
    log_success "Project '$project_dir' is valid"
    return 0
}

# Dependency checking
check_dependencies() {
    log_info "Checking system dependencies..."
    
    check_command "node"
    check_command "npm"
    check_node_version
    
    if [ -d "$PROJECT_ROOT/coprocessor" ] || [ -d "$PROJECT_ROOT/kms-connector" ]; then
        check_rust_version
    fi
    
    if [ "$1" = "docker" ]; then
        check_docker
    fi
    
    log_success "All required dependencies are available"
}

# Build functions
build_project() {
    local project_dir="$1"
    local project_name=$(basename "$project_dir")
    
    log_info "Building project: $project_name"
    
    if ! validate_project "$project_dir"; then
        return 1
    fi
    
    cd "$project_dir"
    
    # Check if it's a Node.js project
    if [ -f "package.json" ]; then
        log_info "Installing dependencies for $project_name..."
        npm ci --silent
        
        if [ -f "hardhat.config.ts" ] || [ -f "hardhat.config.js" ]; then
            log_info "Compiling contracts for $project_name..."
            npx hardhat compile
        else
            log_info "Building TypeScript for $project_name..."
            npm run build || npm run compile || log_warning "No build/compile script found"
        fi
    fi
    
    # Check if it's a Rust project
    if [ -f "Cargo.toml" ]; then
        log_info "Building Rust project: $project_name..."
        cargo build --release
    fi
    
    cd "$PROJECT_ROOT"
    log_success "Project $project_name built successfully"
}

# Test functions
test_project() {
    local project_dir="$1"
    local project_name=$(basename "$project_dir")
    
    log_info "Testing project: $project_name"
    
    if ! validate_project "$project_dir"; then
        return 1
    fi
    
    cd "$project_dir"
    
    # Check if it's a Node.js project
    if [ -f "package.json" ]; then
        if npm run test --silent 2>/dev/null; then
            log_success "Tests passed for $project_name"
        else
            log_warning "No tests found or tests failed for $project_name"
        fi
    fi
    
    # Check if it's a Rust project
    if [ -f "Cargo.toml" ]; then
        if cargo test --quiet; then
            log_success "Rust tests passed for $project_name"
        else
            log_warning "Rust tests failed for $project_name"
        fi
    fi
    
    cd "$PROJECT_ROOT"
}

# Clean functions
clean_project() {
    local project_dir="$1"
    local project_name=$(basename "$project_dir")
    
    log_info "Cleaning project: $project_name"
    
    if ! validate_project "$project_dir"; then
        return 1
    fi
    
    cd "$project_dir"
    
    # Check if it's a Node.js project
    if [ -f "package.json" ]; then
        if [ -f "hardhat.config.ts" ] || [ -f "hardhat.config.js" ]; then
            npx hardhat clean
        fi
        rm -rf node_modules/.cache
        rm -rf dist build
    fi
    
    # Check if it's a Rust project
    if [ -f "Cargo.toml" ]; then
        cargo clean
    fi
    
    cd "$PROJECT_ROOT"
    log_success "Project $project_name cleaned"
}

# Docker functions
docker_build_project() {
    local project_dir="$1"
    local project_name=$(basename "$project_dir")
    local version=${2:-"latest"}
    
    log_info "Building Docker image for: $project_name"
    
    if [ ! -f "$project_dir/Dockerfile" ]; then
        log_warning "No Dockerfile found for $project_name, skipping..."
        return 0
    fi
    
    cd "$project_dir"
    docker build -t "fhevm-$project_name:$version" .
    cd "$PROJECT_ROOT"
    
    log_success "Docker image built for $project_name"
}

# Parallel execution
run_parallel() {
    local command="$1"
    local max_jobs="${2:-4}"
    local pids=()
    
    shift 2
    
    for arg in "$@"; do
        while [ ${#pids[@]} -ge $max_jobs ]; do
            for i in "${!pids[@]}"; do
                if ! kill -0 "${pids[$i]}" 2>/dev/null; then
                    unset "pids[$i]"
                fi
            done
            pids=("${pids[@]}")
            sleep 0.1
        done
        
        $command "$arg" &
        pids+=($!)
    done
    
    # Wait for all jobs to complete
    for pid in "${pids[@]}"; do
        wait "$pid"
    done
}

# Main execution
main() {
    local action="$1"
    shift
    
    case "$action" in
        "check-deps")
            check_dependencies "${1:-}"
            ;;
        "build")
            check_dependencies
            for project in "$@"; do
                build_project "$PROJECT_ROOT/$project"
            done
            ;;
        "test")
            check_dependencies
            for project in "$@"; do
                test_project "$PROJECT_ROOT/$project"
            done
            ;;
        "clean")
            for project in "$@"; do
                clean_project "$PROJECT_ROOT/$project"
            done
            ;;
        "docker")
            check_dependencies "docker"
            for project in "$@"; do
                docker_build_project "$PROJECT_ROOT/$project"
            done
            ;;
        "parallel-build")
            check_dependencies
            local max_jobs="${1:-4}"
            shift
            run_parallel build_project "$max_jobs" "$@"
            ;;
        *)
            echo "Usage: $0 {check-deps|build|test|clean|docker|parallel-build} [args...]"
            echo ""
            echo "Commands:"
            echo "  check-deps [docker]     Check system dependencies"
            echo "  build <projects...>     Build specified projects"
            echo "  test <projects...>      Test specified projects"
            echo "  clean <projects...>     Clean specified projects"
            echo "  docker <projects...>    Build Docker images for projects"
            echo "  parallel-build <jobs> <projects...>  Build projects in parallel"
            exit 1
            ;;
    esac
}

# Run main function with all arguments
main "$@"
