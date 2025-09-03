#!/bin/bash

# Docker Asset Testing Script for BananaBit CMS
echo "🍌 BananaBit CMS Docker Asset Testing"
echo "===================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test results
TESTS_PASSED=0
TESTS_FAILED=0

# Helper functions
log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
    ((TESTS_PASSED++))
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
    ((TESTS_FAILED++))
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    if ! command -v docker &> /dev/null; then
        log_error "Docker is not installed"
        exit 1
    fi
    
    if ! docker compose version &> /dev/null && ! command -v docker-compose &> /dev/null; then
        log_error "Docker Compose is not installed"
        exit 1
    fi
    
    if ! docker info &> /dev/null; then
        log_error "Docker daemon is not running"
        exit 1
    fi
    
    log_success "All prerequisites are met"
}

# Build the Docker image
build_image() {
    log_info "Building Docker image..."
    
    # Clean up any existing containers
    docker compose down -v &> /dev/null 2>&1 || true
    
    # Try to build with docker compose v2
    if docker compose build --no-cache 2>&1; then
        log_success "Docker image built successfully"
    else
        log_error "Failed to build Docker image"
        log_info "Checking for build errors..."
        return 1
    fi
}

# Test container filesystem structure
test_container_filesystem() {
    log_info "Testing container filesystem structure..."
    
    # Get the image name
    local image_name=$(docker compose config --services | head -n1)
    local full_image_name="bananabit-cms-${image_name}"
    
    # Create a temporary container to inspect filesystem
    local container_id=$(docker create "$full_image_name" 2>/dev/null || docker create bananabit-cms_app 2>/dev/null)
    
    if [ -z "$container_id" ]; then
        log_error "Failed to create temporary container for filesystem inspection"
        return 1
    fi
    
    # Test if main directories exist
    if docker exec "$container_id" test -d /usr/local/app 2>/dev/null || docker run --rm "$full_image_name" test -d /usr/local/app 2>/dev/null; then
        log_success "Main app directory exists: /usr/local/app"
    else
        log_error "Main app directory missing: /usr/local/app"
    fi
    
    # List contents of the app directory
    log_info "Contents of /usr/local/app:"
    if docker run --rm "$full_image_name" ls -la /usr/local/app 2>/dev/null; then
        log_success "Successfully listed app directory contents"
    else
        log_error "Failed to list app directory contents"
    fi
    
    # Test if assets directory exists and has content
    log_info "Contents of /usr/local/app/assets:"
    if docker run --rm "$full_image_name" ls -la /usr/local/app/assets 2>/dev/null; then
        log_success "Assets directory exists and has content"
    else
        log_error "Assets directory missing or empty"
    fi
    
    # Check for specific asset files
    local test_files=(
        "/usr/local/app/assets/components.css"
        "/usr/local/app/assets/blog.css"
        "/usr/local/app/assets/main.css"
        "/usr/local/app/assets/styling"
        "/usr/local/app/server"
    )
    
    for file in "${test_files[@]}"; do
        if docker run --rm "$full_image_name" test -e "$file" 2>/dev/null; then
            log_success "Found expected file/directory: $file"
        else
            log_error "Missing expected file/directory: $file"
        fi
    done
    
    # Clean up temporary container
    docker rm "$container_id" &> /dev/null || true
}

# Test asset content verification
test_asset_content() {
    log_info "Testing asset file content..."
    
    local image_name="bananabit-cms-app"
    
    # Test CSS file content
    log_info "Checking components.css content..."
    local css_content=$(docker run --rm "$image_name" cat /usr/local/app/assets/components.css 2>/dev/null | head -5)
    if [[ "$css_content" == *"comment"* ]] || [[ "$css_content" == *"CSS"* ]] || [[ "$css_content" == *"{"* ]]; then
        log_success "components.css contains valid CSS content"
    else
        log_error "components.css appears to be empty or invalid"
    fi
    
    # Test blog.css content
    log_info "Checking blog.css content..."
    local blog_content=$(docker run --rm "$image_name" cat /usr/local/app/assets/blog.css 2>/dev/null | head -5)
    if [[ "$blog_content" == *"blog"* ]] || [[ "$blog_content" == *"{"* ]] || [[ "$blog_content" == *"CSS"* ]]; then
        log_success "blog.css contains valid CSS content"
    else
        log_error "blog.css appears to be empty or invalid"
    fi
}

# Start the application and test HTTP access to assets
test_running_container() {
    log_info "Testing running container asset access..."
    
    # Start the application
    log_info "Starting the application..."
    
    # Create data directory
    mkdir -p ./data
    
    if docker compose up -d; then
        log_success "Application started successfully"
    elif docker-compose up -d; then
        log_success "Application started successfully (using docker-compose)"
    else
        log_error "Failed to start application"
        return 1
    fi
    
    # Wait for the application to start
    log_info "Waiting for application to start..."
    sleep 15
    
    # Test if the main application is accessible
    local retries=5
    local app_accessible=false
    
    for ((i=1; i<=retries; i++)); do
        if curl -s http://localhost:8080 > /dev/null 2>&1; then
            log_success "Application is accessible at http://localhost:8080"
            app_accessible=true
            break
        else
            log_warning "Attempt $i/$retries: Application not yet accessible, waiting..."
            sleep 5
        fi
    done
    
    if [ "$app_accessible" = false ]; then
        log_error "Application failed to become accessible after $retries attempts"
        log_info "Checking application logs..."
        docker compose logs app 2>/dev/null || docker-compose logs app 2>/dev/null || true
        return 1
    fi
    
    # Test asset accessibility via HTTP
    local asset_urls=(
        "http://localhost:8080/assets/components.css"
        "http://localhost:8080/assets/blog.css"
        "http://localhost:8080/assets/main.css"
    )
    
    for url in "${asset_urls[@]}"; do
        log_info "Testing asset URL: $url"
        local response=$(curl -s -o /dev/null -w "%{http_code}" "$url")
        if [ "$response" = "200" ]; then
            log_success "Asset accessible: $url (HTTP $response)"
            
            # Check content-type header
            local content_type=$(curl -s -I "$url" | grep -i content-type || true)
            if [[ "$content_type" == *"text/css"* ]]; then
                log_success "Correct content-type for CSS: $url"
            else
                log_warning "Unexpected content-type for CSS: $url ($content_type)"
            fi
        else
            log_error "Asset not accessible: $url (HTTP $response)"
        fi
    done
    
    # Test asset content via HTTP
    log_info "Testing asset content via HTTP..."
    local css_via_http=$(curl -s "http://localhost:8080/assets/components.css" | head -5)
    if [[ "$css_via_http" == *"{"* ]] || [[ "$css_via_http" == *"css"* ]] || [[ "$css_via_http" == *"comment"* ]]; then
        log_success "CSS file content is valid when served via HTTP"
    else
        log_error "CSS file content appears invalid when served via HTTP"
        log_info "First 5 lines of CSS via HTTP:"
        echo "$css_via_http"
    fi
}

# Cleanup function
cleanup() {
    log_info "Cleaning up..."
    docker compose down -v &> /dev/null || docker-compose down -v &> /dev/null || true
    log_success "Cleanup completed"
}

# Main test execution
main() {
    echo ""
    log_info "Starting Docker asset testing..."
    echo ""
    
    # Set trap to cleanup on exit
    trap cleanup EXIT
    
    # Run tests
    check_prerequisites
    echo ""
    
    build_image
    echo ""
    
    test_container_filesystem
    echo ""
    
    test_asset_content
    echo ""
    
    test_running_container
    echo ""
    
    # Summary
    echo "===================================="
    echo "🍌 BananaBit CMS Asset Test Results"
    echo "===================================="
    echo ""
    log_success "Tests passed: $TESTS_PASSED"
    if [ $TESTS_FAILED -gt 0 ]; then
        log_error "Tests failed: $TESTS_FAILED"
        echo ""
        log_error "Some tests failed. Please review the output above."
        exit 1
    else
        echo ""
        log_success "All tests passed! Docker asset copying and loading is working correctly."
        echo ""
        log_info "The application is running at: http://localhost:8080"
        log_info "To stop the application: docker compose down"
    fi
}

# Run main function
main "$@"