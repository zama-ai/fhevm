#!/bin/bash
# FHEVM Performance Monitoring Script
# Monitors build times, resource usage, and system performance

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
PERF_DIR="$BUILD_DIR/performance"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")

# Create performance directory
mkdir -p "$PERF_DIR"

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

# System information
get_system_info() {
    log_info "Collecting system information..."
    
    local info_file="$PERF_DIR/system_info_$TIMESTAMP.txt"
    
    {
        echo "=== FHEVM Performance Report ==="
        echo "Timestamp: $(date)"
        echo "Hostname: $(hostname)"
        echo "OS: $(uname -a)"
        echo ""
        echo "=== CPU Information ==="
        if command -v lscpu &> /dev/null; then
            lscpu
        elif [ -f "/proc/cpuinfo" ]; then
            cat /proc/cpuinfo | grep -E "model name|cpu cores|siblings|cache size"
        else
            sysctl -n machdep.cpu.brand_string 2>/dev/null || echo "CPU info not available"
        fi
        echo ""
        echo "=== Memory Information ==="
        if command -v free &> /dev/null; then
            free -h
        else
            vm_stat | head -10 2>/dev/null || echo "Memory info not available"
        fi
        echo ""
        echo "=== Disk Information ==="
        df -h
        echo ""
        echo "=== Node.js Information ==="
        if command -v node &> /dev/null; then
            echo "Version: $(node --version)"
            echo "NPM Version: $(npm --version)"
        else
            echo "Node.js not installed"
        fi
        echo ""
        echo "=== Rust Information ==="
        if command -v rustc &> /dev/null; then
            echo "Version: $(rustc --version)"
            echo "Cargo Version: $(cargo --version)"
        else
            echo "Rust not installed"
        fi
        echo ""
        echo "=== Docker Information ==="
        if command -v docker &> /dev/null; then
            echo "Version: $(docker --version)"
            echo "Docker Compose Version: $(docker-compose --version 2>/dev/null || echo 'Not available')"
        else
            echo "Docker not installed"
        fi
    } > "$info_file"
    
    log_success "System information saved to $info_file"
}

# Monitor build performance
monitor_build() {
    local command="$1"
    local output_file="$PERF_DIR/build_${command}_$TIMESTAMP.log"
    
    log_info "Monitoring build performance for: $command"
    
    # Start system monitoring
    local monitor_pid
    (
        while true; do
            echo "$(date): $(ps -o pid,ppid,cmd,%cpu,%mem -p $$ 2>/dev/null || echo 'Process info not available')"
            sleep 1
        done
    ) > "$PERF_DIR/monitor_$TIMESTAMP.log" &
    monitor_pid=$!
    
    # Record start time
    local start_time=$(date +%s.%N)
    local start_memory=$(ps -o rss= -p $$ 2>/dev/null || echo "0")
    
    # Run the command
    if eval "$command" > "$output_file" 2>&1; then
        local exit_code=0
    else
        local exit_code=1
    fi
    
    # Record end time
    local end_time=$(date +%s.%N)
    local end_memory=$(ps -o rss= -p $$ 2>/dev/null || echo "0")
    
    # Stop monitoring
    kill $monitor_pid 2>/dev/null || true
    
    # Calculate metrics
    local duration=$(echo "$end_time - $start_time" | bc -l 2>/dev/null || echo "0")
    local memory_diff=$((end_memory - start_memory))
    
    # Save performance metrics
    local metrics_file="$PERF_DIR/metrics_${command//[^a-zA-Z0-9]/_}_$TIMESTAMP.txt"
    {
        echo "=== Build Performance Metrics ==="
        echo "Command: $command"
        echo "Start Time: $(date -d @$start_time 2>/dev/null || date)"
        echo "End Time: $(date -d @$end_time 2>/dev/null || date)"
        echo "Duration: ${duration}s"
        echo "Start Memory: ${start_memory}KB"
        echo "End Memory: ${end_memory}KB"
        echo "Memory Difference: ${memory_diff}KB"
        echo "Exit Code: $exit_code"
        echo ""
        echo "=== Resource Usage Summary ==="
        if [ -f "$PERF_DIR/monitor_$TIMESTAMP.log" ]; then
            echo "Peak CPU Usage: $(grep -o '[0-9.]*%' "$PERF_DIR/monitor_$TIMESTAMP.log" | sort -n | tail -1 || echo 'N/A')"
            echo "Peak Memory Usage: $(grep -o '[0-9]*' "$PERF_DIR/monitor_$TIMESTAMP.log" | tail -1 || echo 'N/A')KB"
        fi
    } > "$metrics_file"
    
    if [ $exit_code -eq 0 ]; then
        log_success "Build completed successfully in ${duration}s"
    else
        log_error "Build failed after ${duration}s"
    fi
    
    return $exit_code
}

# Monitor test performance
monitor_tests() {
    local test_command="$1"
    local output_file="$PERF_DIR/test_$TIMESTAMP.log"
    
    log_info "Monitoring test performance..."
    
    local start_time=$(date +%s.%N)
    
    if eval "$test_command" > "$output_file" 2>&1; then
        local exit_code=0
    else
        local exit_code=1
    fi
    
    local end_time=$(date +%s.%N)
    local duration=$(echo "$end_time - $start_time" | bc -l 2>/dev/null || echo "0")
    
    # Parse test results
    local test_summary="$PERF_DIR/test_summary_$TIMESTAMP.txt"
    {
        echo "=== Test Performance Summary ==="
        echo "Test Command: $test_command"
        echo "Duration: ${duration}s"
        echo "Exit Code: $exit_code"
        echo ""
        echo "=== Test Results ==="
        if [ -f "$output_file" ]; then
            # Extract test statistics if available
            grep -E "(passing|failing|pending|✓|✗|PASS|FAIL)" "$output_file" | tail -10 || echo "No test statistics found"
        fi
    } > "$test_summary"
    
    if [ $exit_code -eq 0 ]; then
        log_success "Tests completed successfully in ${duration}s"
    else
        log_error "Tests failed after ${duration}s"
    fi
    
    return $exit_code
}

# Generate performance report
generate_report() {
    local report_file="$PERF_DIR/performance_report_$TIMESTAMP.md"
    
    log_info "Generating performance report..."
    
    {
        echo "# FHEVM Performance Report"
        echo ""
        echo "**Generated:** $(date)"
        echo "**Timestamp:** $TIMESTAMP"
        echo ""
        echo "## System Information"
        echo ""
        echo "\`\`\`"
        if [ -f "$PERF_DIR/system_info_$TIMESTAMP.txt" ]; then
            cat "$PERF_DIR/system_info_$TIMESTAMP.txt"
        else
            echo "System information not available"
        fi
        echo "\`\`\`"
        echo ""
        echo "## Build Performance"
        echo ""
        for metrics_file in "$PERF_DIR"/metrics_*_"$TIMESTAMP".txt; do
            if [ -f "$metrics_file" ]; then
                echo "### $(basename "$metrics_file" .txt)"
                echo ""
                echo "\`\`\`"
                cat "$metrics_file"
                echo "\`\`\`"
                echo ""
            fi
        done
        echo "## Test Performance"
        echo ""
        for test_file in "$PERF_DIR"/test_summary_*_"$TIMESTAMP".txt; do
            if [ -f "$test_file" ]; then
                echo "### $(basename "$test_file" .txt)"
                echo ""
                echo "\`\`\`"
                cat "$test_file"
                echo "\`\`\`"
                echo ""
            fi
        done
        echo "## Recommendations"
        echo ""
        echo "Based on the performance data:"
        echo ""
        echo "- Monitor build times and optimize slow builds"
        echo "- Check memory usage patterns"
        echo "- Consider parallel builds for better performance"
        echo "- Review test execution times"
        echo ""
    } > "$report_file"
    
    log_success "Performance report generated: $report_file"
}

# Clean old performance data
cleanup_performance_data() {
    local days=${1:-7}
    
    log_info "Cleaning performance data older than $days days..."
    
    find "$PERF_DIR" -type f -name "*.txt" -o -name "*.log" -o -name "*.md" | \
    while read -r file; do
        if [ $(find "$file" -mtime +$days) ]; then
            rm -f "$file"
            log_info "Removed old file: $(basename "$file")"
        fi
    done
    
    log_success "Performance data cleanup completed"
}

# Main function
main() {
    local action="${1:-help}"
    
    case "$action" in
        "system")
            get_system_info
            ;;
        "build")
            if [ $# -lt 2 ]; then
                log_error "Usage: $0 build <command>"
                exit 1
            fi
            shift
            local command="$*"
            monitor_build "$command"
            ;;
        "test")
            if [ $# -lt 2 ]; then
                log_error "Usage: $0 test <test_command>"
                exit 1
            fi
            shift
            local test_command="$*"
            monitor_tests "$test_command"
            ;;
        "report")
            generate_report
            ;;
        "cleanup")
            cleanup_performance_data "${2:-7}"
            ;;
        "full")
            get_system_info
            monitor_build "make build"
            monitor_tests "make test"
            generate_report
            ;;
        "help"|*)
            echo "Usage: $0 {system|build|test|report|cleanup|full} [args...]"
            echo ""
            echo "Commands:"
            echo "  system                    Collect system information"
            echo "  build <command>          Monitor build performance"
            echo "  test <test_command>      Monitor test performance"
            echo "  report                   Generate performance report"
            echo "  cleanup [days]           Clean old performance data (default: 7 days)"
            echo "  full                     Run complete performance analysis"
            echo "  help                     Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0 system"
            echo "  $0 build 'make build'"
            echo "  $0 test 'make test'"
            echo "  $0 full"
            ;;
    esac
}

# Run main function
main "$@"
