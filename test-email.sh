#!/bin/bash

# Test script for BananaBit CMS email functionality
echo "🍌 BananaBit CMS Email Test Script"
echo "================================="

# Check if docker-compose is available
if ! command -v docker-compose &> /dev/null; then
    echo "❌ docker-compose is not installed. Please install docker-compose to run this test."
    exit 1
fi

# Check if Docker is running
if ! docker info &> /dev/null; then
    echo "❌ Docker is not running. Please start Docker to run this test."
    exit 1
fi

echo "✅ Docker and docker-compose are available"

# Create data directory if it doesn't exist
mkdir -p ./data

echo "📁 Created data directory for database persistence"

# Start the services
echo "🚀 Starting BananaBit CMS with MailHog..."
docker-compose up -d

# Wait for services to start
echo "⏳ Waiting for services to start..."
sleep 10

# Check if services are running
if docker-compose ps | grep -q "Up"; then
    echo "✅ Services are running!"
    echo ""
    echo "🌐 Application URL: http://localhost:8080"
    echo "📧 MailHog Web UI: http://localhost:8025"
    echo ""
    echo "To test email functionality:"
    echo "1. Open http://localhost:8080 in your browser"
    echo "2. Register a new user (first user will become admin)"
    echo "3. Check http://localhost:8025 for the verification email"
    echo "4. Use the verification token or link to verify your account"
    echo ""
    echo "To stop the services, run: docker-compose down"
else
    echo "❌ Services failed to start. Check the logs with: docker-compose logs"
    exit 1
fi