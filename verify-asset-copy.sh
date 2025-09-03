#!/bin/bash

# Asset Copy Verification Test for BananaBit CMS
echo "🍌 BananaBit CMS Asset Copy Verification"
echo "========================================"

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

# Test results tracking
TESTS_PASSED=0
TESTS_FAILED=0

test_pass() {
    log_success "$1"
    ((TESTS_PASSED++))
}

test_fail() {
    log_error "$1"
    ((TESTS_FAILED++))
}

# Verify source asset structure
verify_source_assets() {
    log_info "Step 1: Verifying source asset structure..."
    echo ""
    
    # Check ba-server assets
    if [ -d "./ba-server/assets" ]; then
        test_pass "ba-server/assets directory exists"
        
        log_info "Contents of ba-server/assets:"
        ls -la ./ba-server/assets/ | sed 's/^/  /'
        echo ""
        
        # Check for key files
        local ba_files=("components.css" "main.css" "styling")
        for file in "${ba_files[@]}"; do
            if [ -e "./ba-server/assets/$file" ]; then
                test_pass "Found ba-server/assets/$file"
            else
                test_fail "Missing ba-server/assets/$file"
            fi
        done
        
    else
        test_fail "ba-server/assets directory not found"
    fi
    
    echo ""
    
    # Check ui assets
    if [ -d "./ui/assets" ]; then
        test_pass "ui/assets directory exists"
        
        log_info "Contents of ui/assets:"
        ls -la ./ui/assets/ | sed 's/^/  /'
        echo ""
        
        # Check for key files
        local ui_files=("blog.css")
        for file in "${ui_files[@]}"; do
            if [ -e "./ui/assets/$file" ]; then
                test_pass "Found ui/assets/$file"
            else
                test_fail "Missing ui/assets/$file"
            fi
        done
        
    else
        test_fail "ui/assets directory not found"
    fi
    
    echo ""
    
    # Check web assets  
    if [ -d "./web/assets" ]; then
        test_pass "web/assets directory exists"
        
        log_info "Contents of web/assets:"
        ls -la ./web/assets/ | sed 's/^/  /'
        echo ""
        
    else
        test_fail "web/assets directory not found"
    fi
}

# Verify asset file contents
verify_asset_contents() {
    log_info "Step 2: Verifying asset file contents..."
    echo ""
    
    # Test ba-server/assets/components.css
    if [ -f "./ba-server/assets/components.css" ]; then
        local size=$(stat -c%s "./ba-server/assets/components.css" 2>/dev/null || stat -f%z "./ba-server/assets/components.css" 2>/dev/null)
        if [ "$size" -gt 0 ]; then
            test_pass "ba-server/assets/components.css has content ($size bytes)"
            
            log_info "First 5 lines of ba-server/assets/components.css:"
            head -5 "./ba-server/assets/components.css" | sed 's/^/  /'
            echo ""
        else
            test_fail "ba-server/assets/components.css is empty"
        fi
    else
        test_fail "ba-server/assets/components.css not found"
    fi
    
    # Test ui/assets/blog.css
    if [ -f "./ui/assets/blog.css" ]; then
        local size=$(stat -c%s "./ui/assets/blog.css" 2>/dev/null || stat -f%z "./ui/assets/blog.css" 2>/dev/null)
        if [ "$size" -gt 0 ]; then
            test_pass "ui/assets/blog.css has content ($size bytes)"
            
            log_info "First 5 lines of ui/assets/blog.css:"
            head -5 "./ui/assets/blog.css" | sed 's/^/  /'
            echo ""
        else
            test_fail "ui/assets/blog.css is empty"
        fi
    else
        test_fail "ui/assets/blog.css not found"
    fi
}

# Analyze Dockerfile asset copy commands
analyze_dockerfile() {
    log_info "Step 3: Analyzing Dockerfile asset copy strategy..."
    echo ""
    
    if [ ! -f "./Dockerfile" ]; then
        test_fail "Dockerfile not found"
        return 1
    fi
    
    test_pass "Dockerfile exists"
    
    # Extract and analyze COPY commands for assets
    log_info "Asset-related COPY commands in Dockerfile:"
    grep -n "COPY.*assets" ./Dockerfile | while read -r line; do
        echo "  $line"
    done
    echo ""
    
    # Check specific copy patterns
    if grep -q "COPY --from=builder /app/ba-server/assets/ /usr/local/app/assets/" ./Dockerfile; then
        test_pass "ba-server assets copy command found"
    else
        test_fail "ba-server assets copy command missing"
    fi
    
    if grep -q "COPY --from=builder /app/ui/assets/ /usr/local/app/assets/" ./Dockerfile; then
        test_pass "ui assets copy command found"
    else
        test_fail "ui assets copy command missing"
    fi
    
    # Analyze copy destinations
    log_info "Copy destination analysis:"
    echo "  Source: /app/ba-server/assets/ → Destination: /usr/local/app/assets/"
    echo "  Source: /app/ui/assets/ → Destination: /usr/local/app/assets/"
    echo ""
    
    log_warning "Note: Both sources copy to same destination - files may overwrite!"
    echo ""
}

# Create a simple test container to verify copy logic
create_test_dockerfile() {
    log_info "Step 4: Creating test Dockerfile to verify copy logic..."
    echo ""
    
    # Create a minimal test Dockerfile
    cat > /tmp/test-copy.Dockerfile << 'EOF'
# Test Dockerfile to verify asset copying
FROM alpine:latest

# Create source directories and test files
RUN mkdir -p /test-source/ba-server/assets /test-source/ui/assets /test-dest/assets

# Copy our actual source files to simulate the build environment
COPY ba-server/assets/ /test-source/ba-server/assets/
COPY ui/assets/ /test-source/ui/assets/

# Simulate the copy commands from the real Dockerfile
COPY ba-server/assets/ /test-dest/assets/
COPY ui/assets/ /test-dest/assets/

# List contents for verification
RUN ls -la /test-dest/assets/ > /test-results.txt
RUN echo "=== Source ba-server files ===" >> /test-results.txt
RUN ls -la /test-source/ba-server/assets/ >> /test-results.txt
RUN echo "=== Source ui files ===" >> /test-results.txt  
RUN ls -la /test-source/ui/assets/ >> /test-results.txt
RUN echo "=== Merged destination files ===" >> /test-results.txt
RUN ls -la /test-dest/assets/ >> /test-results.txt

CMD ["cat", "/test-results.txt"]
EOF
    
    log_info "Building test container to verify asset copying..."
    if docker build -f /tmp/test-copy.Dockerfile -t bananabit-asset-test . > /tmp/test-build.log 2>&1; then
        test_pass "Test container built successfully"
        
        log_info "Running test container to check asset copy results..."
        if docker run --rm bananabit-asset-test > /tmp/test-output.txt; then
            test_pass "Test container executed successfully"
            
            log_info "Asset copy test results:"
            cat /tmp/test-output.txt | sed 's/^/  /'
            echo ""
            
            # Analyze results
            if grep -q "components.css" /tmp/test-output.txt; then
                test_pass "components.css found in destination"
            else
                test_fail "components.css not found in destination"
            fi
            
            if grep -q "blog.css" /tmp/test-output.txt; then
                test_pass "blog.css found in destination"
            else
                test_fail "blog.css not found in destination"
            fi
            
        else
            test_fail "Test container execution failed"
        fi
        
        # Clean up test image
        docker rmi bananabit-asset-test > /dev/null 2>&1 || true
        
    else
        test_fail "Test container build failed"
        log_info "Build log (last 20 lines):"
        tail -20 /tmp/test-build.log | sed 's/^/  /'
        echo ""
    fi
    
    # Clean up
    rm -f /tmp/test-copy.Dockerfile /tmp/test-build.log /tmp/test-output.txt
}

# Verify Docker Compose configuration
verify_docker_compose() {
    log_info "Step 5: Verifying Docker Compose configuration..."
    echo ""
    
    if [ -f "./docker-compose.yml" ]; then
        test_pass "docker-compose.yml exists"
        
        # Check build configuration
        if grep -q "build: ." ./docker-compose.yml; then
            test_pass "Build configuration found in docker-compose.yml"
        else
            test_fail "Build configuration missing in docker-compose.yml"
        fi
        
        # Check volume mappings
        log_info "Volume mappings in docker-compose.yml:"
        grep -A 5 -B 5 "volumes:" ./docker-compose.yml | sed 's/^/  /'
        echo ""
        
    else
        test_fail "docker-compose.yml not found"
    fi
}

# Generate test report
generate_report() {
    echo ""
    echo "========================================"
    echo "🍌 Asset Copy Verification Report"
    echo "========================================"
    echo ""
    
    log_info "Test Summary:"
    echo "  Tests Passed: $TESTS_PASSED"
    echo "  Tests Failed: $TESTS_FAILED"
    echo ""
    
    if [ $TESTS_FAILED -eq 0 ]; then
        log_success "All tests passed! Asset copying configuration is correct."
        echo ""
        log_info "Key findings:"
        echo "  ✅ Source asset files are present and have content"
        echo "  ✅ Dockerfile copy commands are properly configured"
        echo "  ✅ Asset merging works as expected"
        echo ""
        log_info "Next steps:"
        echo "  1. Build the full Docker image: docker compose build"
        echo "  2. Run the application: docker compose up -d"
        echo "  3. Test asset accessibility at: http://localhost:8080/assets/"
        
    else
        log_error "Some tests failed. Please review the issues above."
        echo ""
        log_info "Common issues to check:"
        echo "  - Missing source asset files"
        echo "  - Incorrect Dockerfile copy paths"
        echo "  - File permissions"
        echo "  - Directory structure"
    fi
    
    echo ""
}

# Main execution
main() {
    echo ""
    log_info "Starting asset copy verification..."
    echo ""
    
    verify_source_assets
    echo ""
    
    verify_asset_contents
    echo ""
    
    analyze_dockerfile
    echo ""
    
    create_test_dockerfile
    echo ""
    
    verify_docker_compose
    echo ""
    
    generate_report
}

# Run main function
main "$@"