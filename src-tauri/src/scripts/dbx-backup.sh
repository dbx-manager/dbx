#!/bin/bash

# Podman Container Backup Script
# Usage: ./main.sh [config_file]
# 
# This script reads a JSON config file containing container IDs,
# commits each container to an image, and exports it to a tar file
# in the specified export directory.
#
# Designed for cron scheduling - no built-in scheduling logic.

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_FILE="${1:-${SCRIPT_DIR}/config.json}"
LOG_FILE="${SCRIPT_DIR}/backup.log"
TIMESTAMP=$(date '+%Y%m%d_%H%M%S')

# Distrobox compatibility
USE_DISTROBOX=false

# Check if distrobox-host-exec is available and set up wrapper function
check_distrobox_host_exec() {
    if command -v distrobox-host-exec &> /dev/null; then
        log_message "INFO" "distrobox-host-exec detected - will use host Podman commands"
        USE_DISTROBOX=true
    else
        log_message "INFO" "distrobox-host-exec not found - using local Podman"
        USE_DISTROBOX=false
    fi
}

# Wrapper function for Podman commands to support distrobox-host-exec
podman_cmd() {
    if [ "$USE_DISTROBOX" = true ]; then
        distrobox-host-exec podman "$@"
    else
        podman "$@"
    fi
}

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging function
log_message() {
    local level="$1"
    local message="$2"
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    
    case "$level" in
        "INFO")
            echo -e "${BLUE}[INFO]${NC} $message" | tee -a "$LOG_FILE"
            ;;
        "SUCCESS")
            echo -e "${GREEN}[SUCCESS]${NC} $message" | tee -a "$LOG_FILE"
            ;;
        "WARNING")
            echo -e "${YELLOW}[WARNING]${NC} $message" | tee -a "$LOG_FILE"
            ;;
        "ERROR")
            echo -e "${RED}[ERROR]${NC} $message" | tee -a "$LOG_FILE"
            ;;
    esac
}

# Error handling function
handle_error() {
    local exit_code=$1
    local message="$2"
    log_message "ERROR" "$message"
    cleanup_temp_images
    exit "$exit_code"
}

# Cleanup function for temporary images
cleanup_temp_images() {
    log_message "INFO" "Cleaning up temporary images..."
    
    # Find and remove temporary images created by this script
    local temp_images=$(podman_cmd images --format "{{.Repository}}:{{.Tag}}" 2>/dev/null | grep -E "^backup-temp-" 2>/dev/null || true)
    
    if [ -n "$temp_images" ]; then
        echo "$temp_images" | while read -r image; do
            if podman_cmd rmi "$image" 2>/dev/null; then
                log_message "INFO" "Removed temporary image: $image"
            else
                log_message "WARNING" "Could not remove temporary image: $image"
            fi
        done
    else
        log_message "INFO" "No temporary images to clean up"
    fi
}

# Check prerequisites
check_prerequisites() {
    log_message "INFO" "Checking prerequisites..."
    
    # Check if distrobox-host-exec is available
    check_distrobox_host_exec
    
    # Check if Podman is installed (on host if using distrobox-host-exec, local otherwise)
    if [ "$USE_DISTROBOX" = true ]; then
        if ! distrobox-host-exec podman --version &> /dev/null; then
            handle_error 1 "Podman is not installed on the host system"
        fi
    else
        if ! command -v podman &> /dev/null; then
            handle_error 1 "Podman is not installed or not in PATH"
        fi
    fi
    
    # Check if Podman is accessible (using wrapper function)
    if ! podman_cmd info &> /dev/null; then
        handle_error 1 "Podman is not running or accessible (via $(if [ "$USE_DISTROBOX" = true ]; then echo "distrobox-host-exec"; else echo "local"; fi))"
    fi
    
    # Check if jq is installed (on host if using distrobox-host-exec, local otherwise)
    if [ "$USE_DISTROBOX" = true ]; then
        if ! distrobox-host-exec jq -V &> /dev/null; then
            handle_error 1 "jq is not installed on the host system. Please install jq to parse JSON config."
        fi
    else
        if ! command -v jq &> /dev/null; then
            handle_error 1 "jq is not installed or not in PATH. Please install jq to parse JSON config."
        fi
    fi
    
    # Check if config file exists
    if [ ! -f "$CONFIG_FILE" ]; then
        handle_error 1 "Config file not found: $CONFIG_FILE"
    fi
    
    log_message "SUCCESS" "Prerequisites check passed"
}

# Parse JSON config file
parse_config() {
    log_message "INFO" "Parsing config file: $CONFIG_FILE"
    
    # Use jq from host if using distrobox-host-exec, otherwise use local
    if [ "$USE_DISTROBOX" = true ]; then
        JQ_CMD="distrobox-host-exec jq"
    else
        JQ_CMD="jq"
    fi
    
    # Validate JSON syntax
    if ! $JQ_CMD empty "$CONFIG_FILE" 2>/dev/null; then
        handle_error 1 "Invalid JSON syntax in config file: $CONFIG_FILE"
    fi
    
    # Extract export path
    export_path=$($JQ_CMD -r '.export_path' "$CONFIG_FILE" 2>/dev/null)
    
    if [ "$export_path" = "null" ] || [ -z "$export_path" ]; then
        handle_error 1 "export_path not found in config file"
    fi
    
    # Extract container IDs
    container_ids=$($JQ_CMD -r '.containers[]' "$CONFIG_FILE" 2>/dev/null)
    
    if [ "$container_ids" = "null" ] || [ -z "$container_ids" ]; then
        handle_error 1 "containers array not found or empty in config file"
    fi
    
    log_message "SUCCESS" "Config parsed successfully. Export path: $export_path"
    log_message "INFO" "Found $(echo "$container_ids" | wc -l) containers to backup"
}

# Create export directory
create_export_directory() {
    log_message "INFO" "Creating export directory: $export_path"
    
    if ! mkdir -p "$export_path" 2>/dev/null; then
        handle_error 1 "Failed to create export directory: $export_path"
    fi
    
    if ! [ -w "$export_path" ]; then
        handle_error 1 "No write permission for export directory: $export_path"
    fi
    
    log_message "SUCCESS" "Export directory ready: $export_path"
}

# Commit container to temporary image
commit_container() {
    local container_id="$1"
    local temp_image_name="backup-temp-${container_id:0:8}-${TIMESTAMP}"
    # log_message "INFO" "Committing container $container_id to temporary image..."
    
    if ! podman_cmd commit "$container_id" "$temp_image_name" &> /dev/null; then
        # log_message "ERROR" "Failed to commit container $container_id"
        return 1
    fi
    
    # log_message "SUCCESS" "Container $container_id committed to image: $temp_image_name"
    echo "$temp_image_name"

}

# Export image to tar file
export_image() {
    local image_name="$1"
    local container_id="$2"
    local export_file="${export_path}/container_${container_id:0:12}_${TIMESTAMP}.tar"
    log_message "INFO" "Exporting image $image_name to $export_file..."
    
    if ! podman_cmd save "$image_name" -o "$export_file" &> /dev/null; then
        log_message "ERROR" "Failed to export image $image_name"
        return 1
    fi
    
    # Check if file was created and has size > 0
    if [ ! -f "$export_file" ] || [ ! -s "$export_file" ]; then
        log_message "ERROR" "Export file was not created or is empty: $export_file"
        return 1
    fi
    
    local file_size=$(du -h "$export_file" | cut -f1)
    log_message "SUCCESS" "Image exported successfully: $export_file ($file_size)"
    echo "$export_file"
}

# Remove temporary image
remove_temp_image() {
    local image_name="$1"
    
    log_message "INFO" "Removing temporary image: $image_name"
    
    if ! podman_cmd rmi "$image_name" &> /dev/null; then
        log_message "WARNING" "Failed to remove temporary image: $image_name"
        return 1
    fi
    
    log_message "SUCCESS" "Temporary image removed: $image_name"
}

# Main backup function
backup_container() {
    local container_id="$1"
    local backup_success=true
    
    log_message "INFO" "Starting backup for container: $container_id"
    
    # Check if container exists and is running
    if ! podman_cmd ps -q --filter "id=$container_id" &> /dev/null; then
        if ! podman_cmd ps -aq --filter "id=$container_id" &> /dev/null; then
            log_message "ERROR" "Container $container_id not found"
            return 1
        else
            log_message "WARNING" "Container $container_id exists but is not running"
        fi
    fi
    
    # Commit container
    local temp_image_name
    temp_image_name=$(commit_container "$container_id")
    
    if [ $? -ne 0 ]; then
        log_message "ERROR" "Failed to commit container $container_id"
        return 1
    fi
    
    # Export image
    local export_file
        echo "klodiea $temp_image_name end sceen"
        $(sleep 2)
    export_file=$(export_image "$temp_image_name" "$container_id")
    if [ $? -ne 0 ]; then
        log_message "ERROR" "Failed to export image for container $container_id"
        # Try to clean up temp image even if export failed
        remove_temp_image "$temp_image_name" &> /dev/null
        return 1
    fi
    
    # Remove temporary image
    if ! remove_temp_image "$temp_image_name"; then
        log_message "WARNING" "Failed to clean up temporary image for container $container_id"
        backup_success=false
    fi
    
    if [ "$backup_success" = true ]; then
        log_message "SUCCESS" "Backup completed for container: $container_id"
        return 0
    else
        log_message "WARNING" "Backup completed with warnings for container: $container_id"
        return 0  # Don't fail the entire script for cleanup issues
    fi
}

# Main function
main() {
    log_message "INFO" "Starting Podman container backup script"
    log_message "INFO" "Timestamp: $TIMESTAMP"
    log_message "INFO" "Config file: $CONFIG_FILE"
    
    # Trap to ensure cleanup on exit
    trap cleanup_temp_images EXIT
    
    # Check prerequisites
    check_prerequisites
    
    # Parse config
    parse_config
    
    # Create export directory
    create_export_directory
    
    # Initialize counters
    total_containers=0
    successful_backups=0
    failed_backups=0
    
    # Process each container
    while IFS= read -r container_id; do
        # Skip empty lines
        if [ -z "$container_id" ] || [ "$container_id" = "null" ]; then
            continue
        fi
        
        total_containers=$((total_containers + 1))
        
        log_message "INFO" "========================================"
        log_message "INFO" "Processing container $total_containers"
        
        if backup_container "$container_id"; then
            successful_backups=$((successful_backups + 1))
        else
            failed_backups=$((failed_backups + 1))
        fi
    done <<< "$container_ids"
    
    # Summary
    log_message "INFO" "========================================"
    log_message "INFO" "Backup Summary:"
    log_message "INFO" "  Total containers: $total_containers"
    log_message "SUCCESS" "  Successful backups: $successful_backups"
    if [ $failed_backups -gt 0 ]; then
        log_message "ERROR" "  Failed backups: $failed_backups"
    fi
    
    if [ $failed_backups -eq 0 ]; then
        log_message "SUCCESS" "All container backups completed successfully!"
        exit 0
    else
        log_message "ERROR" "Some container backups failed. Check log for details."
        exit 1
    fi
}

# Run main function
main "$@"