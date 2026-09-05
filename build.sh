#!/bin/bash
# ─── Platform Made Easy — Resource-Safe Build Script ──────────────────────────
# Compiles without turning your laptop into a space heater.
# Usage:
#   ./build.sh              → dev build (infra-job-killer frontend)
#   ./build.sh release      → release build
#   ./build.sh import       → build infra-import only
#   ./build.sh check        → cargo check (fastest, no codegen)
#   ./build.sh watch        → cargo leptos watch with resource limits

set -e

# ─── Config ──────────────────────────────────────────────────────────────────
MAX_JOBS=4
# Memory high-water mark (KB) — kill build if RSS goes past this
# 6.5GB leaves ~1.5GB for OS when WSL has 8GB
MEM_LIMIT_KB=6815744

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
DIM='\033[2m'
NC='\033[0m'

# ─── Helpers ─────────────────────────────────────────────────────────────────

check_memory() {
    local available_kb
    available_kb=$(grep MemAvailable /proc/meminfo | awk '{print $2}')
    local available_mb=$((available_kb / 1024))
    
    if [ "$available_kb" -lt 512000 ]; then
        echo -e "${RED}WARNING: Only ${available_mb}MB RAM available!${NC}"
        echo -e "${YELLOW}Dropping to 2 parallel jobs to survive...${NC}"
        export CARGO_BUILD_JOBS=2
    elif [ "$available_kb" -lt 1048576 ]; then
        echo -e "${YELLOW}RAM getting tight (${available_mb}MB free). Keeping jobs at 3.${NC}"
        export CARGO_BUILD_JOBS=3
    else
        echo -e "${GREEN}RAM OK (${available_mb}MB free). Using ${MAX_JOBS} jobs.${NC}"
        export CARGO_BUILD_JOBS=$MAX_JOBS
    fi
}

print_header() {
    echo -e "${DIM}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "  Platform Made Easy — ${GREEN}$1${NC}"
    echo -e "${DIM}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

monitor_build() {
    # Background memory watchdog — warns if getting close to OOM
    local build_pid=$1
    while kill -0 "$build_pid" 2>/dev/null; do
        local available_kb
        available_kb=$(grep MemAvailable /proc/meminfo | awk '{print $2}')
        if [ "$available_kb" -lt 256000 ]; then
            echo -e "\n${RED}CRITICAL: <256MB RAM left! Consider killing heavy apps on Windows.${NC}"
        fi
        sleep 5
    done
}

time_build() {
    local start_time=$SECONDS
    "$@"
    local exit_code=$?
    local elapsed=$(( SECONDS - start_time ))
    local mins=$(( elapsed / 60 ))
    local secs=$(( elapsed % 60 ))
    echo ""
    if [ $exit_code -eq 0 ]; then
        echo -e "${GREEN}Build completed in ${mins}m ${secs}s${NC}"
    else
        echo -e "${RED}Build FAILED after ${mins}m ${secs}s${NC}"
    fi
    return $exit_code
}

# ─── Commands ────────────────────────────────────────────────────────────────

cmd_dev() {
    print_header "Dev Build (infra-job-killer)"
    check_memory
    echo ""
    time_build cargo leptos build &
    local pid=$!
    monitor_build $pid &
    local monitor_pid=$!
    wait $pid
    local result=$?
    kill $monitor_pid 2>/dev/null
    return $result
}

cmd_release() {
    print_header "Release Build"
    check_memory
    echo -e "${YELLOW}Release builds are RAM-hungry. Using 2 jobs for safety.${NC}"
    export CARGO_BUILD_JOBS=2
    echo ""
    time_build cargo leptos build --release &
    local pid=$!
    monitor_build $pid &
    local monitor_pid=$!
    wait $pid
    local result=$?
    kill $monitor_pid 2>/dev/null
    return $result
}

cmd_import() {
    print_header "Build infra-import"
    check_memory
    echo ""
    time_build cargo build -p infra-import
}

cmd_check() {
    print_header "Cargo Check (no codegen)"
    echo -e "${DIM}This is the fastest way to verify your code compiles.${NC}"
    echo ""
    time_build cargo check --workspace
}

cmd_watch() {
    print_header "Leptos Watch (dev server)"
    check_memory
    echo ""
    echo -e "${YELLOW}Starting cargo leptos watch...${NC}"
    echo -e "${DIM}Memory watchdog active — will warn if RAM gets low.${NC}"
    echo ""
    cargo leptos watch &
    local pid=$!
    monitor_build $pid &
    wait $pid
}

# ─── Entrypoint ──────────────────────────────────────────────────────────────

case "${1:-dev}" in
    dev)     cmd_dev ;;
    release) cmd_release ;;
    import)  cmd_import ;;
    check)   cmd_check ;;
    watch)   cmd_watch ;;
    *)
        echo "Usage: ./build.sh [dev|release|import|check|watch]"
        echo ""
        echo "  dev      Build infra-job-killer for development (default)"
        echo "  release  Release build (slower, optimized)"
        echo "  import   Build infra-import CLI only"
        echo "  check    Fast type-check, no binary output"
        echo "  watch    Dev server with hot reload"
        exit 1
        ;;
esac
