#!/bin/bash

# Deployment verification script for CSS fix
# Run this after deploying to Fly.io to verify CSS is loading correctly

set -e

DEPLOYMENT_URL="${1:-https://bananabit.fly.dev}"

echo "🚀 Verifying CSS loading on deployed application"
echo "Deployment URL: $DEPLOYMENT_URL"
echo "================================================"
echo ""

# Test CSS file accessibility
css_files=(
    "/assets/main.css"
    "/assets/components.css"
    "/assets/blog.css"
    "/assets/markdown.css"
    "/assets/syntax.css"
)

echo "1. 🔍 Testing CSS file accessibility..."

all_css_accessible=true

for css_file in "${css_files[@]}"; do
    url="${DEPLOYMENT_URL}${css_file}"
    echo -n "   Testing $css_file... "
    
    # Test HTTP status
    status_code=$(curl -s -o /dev/null -w "%{http_code}" "$url" || echo "000")
    
    if [ "$status_code" = "200" ]; then
        echo "✅ Accessible (HTTP $status_code)"
        
        # Check content type
        content_type=$(curl -s -I "$url" | grep -i "content-type:" | tr -d '\r' || echo "")
        if [[ "$content_type" == *"text/css"* ]]; then
            echo "     └── Content-Type: ✅ $content_type"
        else
            echo "     └── Content-Type: ⚠️  $content_type (expected text/css)"
        fi
        
        # Check if file has CSS content
        first_line=$(curl -s "$url" | head -1 | tr -d '\r\n')
        if [[ "$first_line" == *"{"* ]] || [[ "$first_line" == *"/*"* ]] || [[ "$first_line" == *"."* ]]; then
            echo "     └── Content: ✅ Contains CSS-like content"
        else
            echo "     └── Content: ⚠️  First line: '$first_line'"
        fi
        
    else
        echo "❌ Not accessible (HTTP $status_code)"
        all_css_accessible=false
    fi
    echo ""
done

echo "2. 🌐 Testing main application..."

# Test main page
echo -n "   Testing main page... "
main_status=$(curl -s -o /dev/null -w "%{http_code}" "$DEPLOYMENT_URL" || echo "000")

if [ "$main_status" = "200" ]; then
    echo "✅ Accessible (HTTP $main_status)"
    
    # Check if main page contains CSS references
    page_content=$(curl -s "$DEPLOYMENT_URL" || echo "")
    if [[ "$page_content" == *"stylesheet"* ]] || [[ "$page_content" == *".css"* ]]; then
        echo "     └── ✅ Page contains CSS references"
    else
        echo "     └── ⚠️  Page might not contain CSS references"
    fi
else
    echo "❌ Not accessible (HTTP $main_status)"
fi

echo ""
echo "3. 📊 Summary..."

if [ "$all_css_accessible" = true ] && [ "$main_status" = "200" ]; then
    echo "✅ SUCCESS: CSS fix appears to be working!"
    echo ""
    echo "📝 Manual verification steps:"
    echo "   1. Open $DEPLOYMENT_URL in a browser"
    echo "   2. Open browser Developer Tools (F12)"
    echo "   3. Go to Network tab and reload the page"
    echo "   4. Check that CSS files load successfully"
    echo "   5. Verify that styling is applied correctly"
    echo ""
    echo "🎉 CSS should now be loading properly on Fly.io!"
else
    echo "❌ ISSUES DETECTED:"
    if [ "$all_css_accessible" != true ]; then
        echo "   - Some CSS files are not accessible"
    fi
    if [ "$main_status" != "200" ]; then
        echo "   - Main application is not accessible"
    fi
    echo ""
    echo "🔧 Troubleshooting:"
    echo "   1. Verify deployment completed successfully: fly logs"
    echo "   2. Check application logs: fly logs --app bananabit"
    echo "   3. Verify Dioxus build included CSS files"
    echo "   4. Check asset directory structure in container"
fi