#!/bin/bash

# Quick start script for BananaBit CMS
echo "🍌 Welcome to BananaBit CMS!"
echo "============================"
echo ""
echo "This script will help you get started quickly."
echo ""

# Function to check prerequisites
check_prereqs() {
    echo "🔍 Checking prerequisites..."
    
    if ! command -v docker &> /dev/null; then
        echo "❌ Docker is not installed. Please install Docker first."
        echo "   Visit: https://docs.docker.com/get-docker/"
        exit 1
    fi
    
    if ! command -v docker-compose &> /dev/null; then
        echo "❌ docker-compose is not installed. Please install docker-compose first."
        echo "   Visit: https://docs.docker.com/compose/install/"
        exit 1
    fi
    
    if ! docker info &> /dev/null; then
        echo "❌ Docker is not running. Please start Docker first."
        exit 1
    fi
    
    echo "✅ All prerequisites met!"
}

# Main menu
show_menu() {
    echo ""
    echo "What would you like to do?"
    echo ""
    echo "1) 🧪 Development setup (with MailHog email testing)"
    echo "2) 🚀 Production setup (configure real email)"
    echo "3) 📖 View documentation"
    echo "4) 🔧 Troubleshooting guide"
    echo "5) ❌ Exit"
    echo ""
    read -p "Choose an option (1-5): " choice
}

# Development setup
dev_setup() {
    echo ""
    echo "🧪 Setting up development environment..."
    echo ""
    
    # Create data directory
    mkdir -p ./data
    echo "📁 Created data directory"
    
    # Start services
    echo "🚀 Starting services with MailHog..."
    docker-compose up -d
    
    # Wait a moment
    sleep 5
    
    # Check status
    if docker-compose ps | grep -q "Up"; then
        echo ""
        echo "✅ Development environment is ready!"
        echo ""
        echo "🌐 Application: http://localhost:8080"
        echo "📧 MailHog (email testing): http://localhost:8025"
        echo ""
        echo "📝 What to do next:"
        echo "1. Open http://localhost:8080 in your browser"
        echo "2. Register your first user (will become admin)"
        echo "3. Answer the captcha: 'a cool dude'"
        echo "4. Check http://localhost:8025 for verification email"
        echo "5. Use the verification link or token to verify your account"
        echo ""
        echo "🛑 To stop: docker-compose down"
    else
        echo "❌ Failed to start services. Check logs with: docker-compose logs"
    fi
}

# Production setup
prod_setup() {
    echo ""
    echo "🚀 Setting up production environment..."
    echo ""
    
    ./setup-production.sh
    
    echo ""
    read -p "Have you configured the .env file? (y/n): " configured
    
    if [[ $configured =~ ^[Yy]$ ]]; then
        echo "🚀 Starting production services..."
        docker-compose -f docker-compose.prod.yml up -d
        
        sleep 5
        
        if docker-compose -f docker-compose.prod.yml ps | grep -q "Up"; then
            echo "✅ Production environment is running!"
            echo "🌐 Application: http://localhost:8080"
            echo ""
            echo "🛑 To stop: docker-compose -f docker-compose.prod.yml down"
        else
            echo "❌ Failed to start services. Check logs and configuration."
        fi
    else
        echo "📝 Please configure .env first, then run this script again."
    fi
}

# Show documentation
show_docs() {
    echo ""
    echo "📖 Available documentation:"
    echo ""
    echo "1) CMS_README.md - Complete setup and email configuration guide"
    echo "2) DOCKER_README.md - Docker-specific setup instructions"
    echo "3) EMAIL_TROUBLESHOOTING.md - Email troubleshooting guide"
    echo "4) .env.example - Environment configuration template"
    echo ""
    read -p "Which document would you like to view? (1-4 or 'back'): " doc_choice
    
    case $doc_choice in
        1) less CMS_README.md ;;
        2) less DOCKER_README.md ;;
        3) less EMAIL_TROUBLESHOOTING.md ;;
        4) less .env.example ;;
        back|b) return ;;
        *) echo "Invalid choice" ;;
    esac
}

# Show troubleshooting
show_troubleshooting() {
    echo ""
    echo "🔧 Common troubleshooting steps:"
    echo ""
    echo "1) Check service status: docker-compose ps"
    echo "2) View logs: docker-compose logs"
    echo "3) View email-specific logs: docker-compose logs app | grep email"
    echo "4) Restart services: docker-compose restart"
    echo "5) Reset everything: docker-compose down && docker-compose up -d"
    echo ""
    echo "📖 For detailed troubleshooting, see EMAIL_TROUBLESHOOTING.md"
    echo ""
    read -p "Press Enter to continue..."
}

# Main script
main() {
    check_prereqs
    
    while true; do
        show_menu
        
        case $choice in
            1) dev_setup ;;
            2) prod_setup ;;
            3) show_docs ;;
            4) show_troubleshooting ;;
            5) echo "👋 Goodbye!"; exit 0 ;;
            *) echo "❌ Invalid choice. Please try again." ;;
        esac
        
        echo ""
        read -p "Press Enter to return to main menu..."
    done
}

# Run main function
main