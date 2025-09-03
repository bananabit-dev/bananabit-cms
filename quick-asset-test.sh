#!/bin/bash

# Quick Docker Asset Test for BananaBit CMS
echo "🍌 BananaBit CMS Quick Asset Test"
echo "================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

# Check if we can use an existing image or need to build
check_existing_image() {
    log_info "Checking for existing BananaBit CMS images..."
    
    local images=$(docker images | grep bananabit-cms)
    if [ -n "$images" ]; then
        log_success "Found existing BananaBit CMS images:"
        echo "$images"
        return 0
    else
        log_warning "No existing BananaBit CMS images found"
        return 1
    fi
}

# Test source asset files exist before building
test_source_assets() {
    log_info "Testing source asset files..."
    
    local source_files=(
        "./ba-server/assets/components.css"
        "./ba-server/assets/main.css"
        "./ui/assets/blog.css"
        "./web/assets/components.css"
        "./ba-server/assets/styling"
        "./ui/assets/styling"
    )
    
    local found_files=0
    local total_files=${#source_files[@]}
    
    for file in "${source_files[@]}"; do
        if [ -e "$file" ]; then
            log_success "Found source asset: $file"
            ((found_files++))
        else
            log_warning "Missing source asset: $file"
        fi
    done
    
    log_info "Found $found_files/$total_files source asset files"
    
    if [ $found_files -gt 0 ]; then
        log_success "Source assets are present for testing"
        return 0
    else
        log_error "No source assets found"
        return 1
    fi
}

# Show asset file contents
show_asset_contents() {
    log_info "Showing sample asset file contents..."
    
    if [ -f "./ba-server/assets/components.css" ]; then
        log_info "First 10 lines of ba-server/assets/components.css:"
        head -10 "./ba-server/assets/components.css"
        echo ""
    fi
    
    if [ -f "./ui/assets/blog.css" ]; then
        log_info "First 10 lines of ui/assets/blog.css:"
        head -10 "./ui/assets/blog.css"
        echo ""
    fi
}

# Test Dockerfile copy commands
test_dockerfile_commands() {
    log_info "Analyzing Dockerfile copy commands..."
    
    if [ -f "./Dockerfile" ]; then
        log_info "Asset copy commands in Dockerfile:"
        grep -n "COPY.*assets" ./Dockerfile || log_warning "No asset copy commands found"
        echo ""
        
        log_info "All COPY commands in Dockerfile:"
        grep -n "COPY" ./Dockerfile
        echo ""
    else
        log_error "Dockerfile not found"
    fi
}

# Try a quick build test (with timeout)
quick_build_test() {
    log_info "Attempting quick Docker build test..."
    
    # Try to build just the first few steps
    timeout 300 docker compose build --no-cache > /tmp/build.log 2>&1 &
    local build_pid=$!
    
    log_info "Build started (PID: $build_pid), waiting up to 5 minutes..."
    
    # Monitor the build
    local elapsed=0
    while [ $elapsed -lt 300 ]; do
        if ! kill -0 $build_pid 2>/dev/null; then
            wait $build_pid
            local exit_code=$?
            if [ $exit_code -eq 0 ]; then
                log_success "Build completed successfully!"
                return 0
            else
                log_error "Build failed with exit code $exit_code"
                log_info "Last 20 lines of build log:"
                tail -20 /tmp/build.log
                return 1
            fi
        fi
        
        sleep 10
        ((elapsed+=10))
        
        if [ $((elapsed % 60)) -eq 0 ]; then
            log_info "Build still running... (${elapsed}s elapsed)"
            # Show some progress from the log
            if [ -f /tmp/build.log ]; then
                local recent_lines=$(tail -5 /tmp/build.log | grep -E "(Step|Compiling|Downloaded)" | tail -2)
                if [ -n "$recent_lines" ]; then
                    echo "$recent_lines"
                fi
            fi
        fi
    done
    
    # Build took too long
    log_warning "Build timed out after 5 minutes"
    kill $build_pid 2>/dev/null || true
    
    log_info "Build log (last 30 lines):"
    tail -30 /tmp/build.log
    
    return 1
}

# Test with existing image if available
test_existing_image() {
    log_info "Testing with existing image..."
    
    local image_name="bananabit-cms-app"
    
    # Check if the image exists
    if ! docker images | grep -q "$image_name"; then
        # Try alternative names
        local alt_names=("bananabit-cms_app" "bananabit-cms:latest")
        for name in "${alt_names[@]}"; do
            if docker images | grep -q "$name"; then
                image_name="$name"
                break
            fi
        done
    fi
    
    if ! docker images | grep -q "$image_name"; then
        log_error "No suitable Docker image found for testing"
        return 1
    fi
    
    log_success "Using image: $image_name"
    
    # Test image contents
    log_info "Testing image filesystem structure..."
    
    if docker run --rm "$image_name" ls -la /usr/local/app 2>/dev/null; then
        log_success "Successfully accessed /usr/local/app in container"
    else
        log_error "Failed to access /usr/local/app in container"
        return 1
    fi
    
    # Test asset directory
    log_info "Testing assets directory..."
    if docker run --rm "$image_name" ls -la /usr/local/app/assets 2>/dev/null; then
        log_success "Assets directory exists and is accessible"
    else
        log_error "Assets directory not found or not accessible"
        return 1
    fi
    
    # Test specific asset files
    local asset_files=(
        "/usr/local/app/assets/components.css"
        "/usr/local/app/assets/blog.css"
        "/usr/local/app/assets/main.css"
    )
    
    for asset in "${asset_files[@]}"; do
        if docker run --rm "$image_name" test -f "$asset" 2>/dev/null; then
            log_success "Found asset file: $asset"
            
            # Check file size
            local size=$(docker run --rm "$image_name" stat -c%s "$asset" 2>/dev/null)
            if [ "$size" -gt 0 ]; then
                log_success "Asset file has content (${size} bytes): $asset"
            else
                log_warning "Asset file is empty: $asset"
            fi
        else
            log_warning "Asset file not found: $asset"
        fi
    done
    
    return 0
}

# Main execution
main() {
    echo ""
    log_info "Starting quick Docker asset test..."
    echo ""
    
    # Test 1: Check source assets
    test_source_assets
    echo ""
    
    # Test 2: Show sample contents
    show_asset_contents
    echo ""
    
    # Test 3: Analyze Dockerfile
    test_dockerfile_commands
    echo ""
    
    # Test 4: Check for existing images
    if check_existing_image; then
        echo ""
        test_existing_image
    else
        echo ""
        log_info "No existing image found, attempting quick build..."
        quick_build_test
    fi
    
    echo ""
    log_info "Quick test completed!"
    
    echo ""
    echo "📋 Summary:"
    echo "- Source asset files checked"
    echo "- Dockerfile copy commands analyzed"
    echo "- Docker image testing attempted"
    echo ""
    echo "For a complete test including HTTP accessibility,"
    echo "run the full test-docker-assets.sh after the build completes."
}

# Run main function
main "$@"