#!/bin/bash

set -e  # Exit on any error

echo "=== Budget Tracker Cloud VM Deployment Script ===" 
echo "Starting deployment..."

# Check if we're running on a cloud VM
if [ -f /sys/hypervisor/uuid ] && [ "$(head -c 3 /sys/hypervisor/uuid)" = "ec2" ]; then
    echo "Detected AWS EC2 instance"
elif [ -f /sys/class/dmi/id/product_name ] && grep -q "Google" /sys/class/dmi/id/product_name; then
    echo "Detected Google Cloud instance"
else
    echo "Deploying on cloud VM"
fi

# Check if production env file exists
if [ ! -f .env.production ]; then
    echo "ERROR: .env.production file not found!"
    echo "Please create .env.production with your production settings."
    echo "Example:"
    echo "DATABASE_URL=postgres://postgres:your_secure_password@postgres:5432/budget_tracker"
    echo "POSTGRES_PASSWORD=your_secure_password"
    echo "RUST_LOG=info"
    echo "ENVIRONMENT=production"
    exit 1
fi

# Validate environment variables
source .env.production
if [ -z "$POSTGRES_PASSWORD" ] || [ "$POSTGRES_PASSWORD" = "your_secure_password_here" ]; then
    echo "ERROR: Please set a secure POSTGRES_PASSWORD in .env.production"
    exit 1
fi

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "ERROR: Docker is not running. Please start Docker first."
    exit 1
fi

# Stop existing containers
echo "Stopping existing containers..."
docker-compose --env-file .env.production down --volumes 2>/dev/null || true

# Clean up old images to save space
echo "Cleaning up old Docker images..."
docker system prune -f

# Pull latest base images
echo "Pulling latest base images..."
docker pull postgres:15
docker pull rust:1.83
docker pull debian:bookworm-slim

# Build application
echo "Building application..."
docker-compose --env-file .env.production build --no-cache

# Start containers
echo "Starting containers..."
docker-compose --env-file .env.production up -d

# Wait for services to be ready
echo "Waiting for services to start..."
for i in {1..30}; do
    if docker-compose --env-file .env.production exec -T app curl -f http://localhost:3000/ > /dev/null 2>&1; then
        echo "Application is ready!"
        break
    fi
    if [ $i -eq 30 ]; then
        echo "WARNING: Application may not be ready yet. Check logs below."
    fi
    sleep 2
done

# Get VM's public IP (works on most cloud providers)
PUBLIC_IP=$(curl -s http://checkip.amazonaws.com/ 2>/dev/null || curl -s http://ipecho.net/plain 2>/dev/null || echo "UNKNOWN")

# Show container status
echo ""
echo "=== Container Status ==="
docker-compose --env-file .env.production ps

# Show logs
echo ""
echo "=== Recent Logs ==="
docker-compose --env-file .env.production logs --tail=20

echo ""
echo "=== Deployment Complete ==="
echo "Your Budget Tracker is now running!"
echo ""
if [ "$PUBLIC_IP" != "UNKNOWN" ]; then
    echo "🌐 Access your application at: http://$PUBLIC_IP:3000"
else
    echo "🌐 Access your application at: http://YOUR_VM_PUBLIC_IP:3000"
fi
echo ""
echo "📋 Management Commands:"
echo "  Monitor logs: docker-compose --env-file .env.production logs -f"
echo "  Stop app:     docker-compose --env-file .env.production down"
echo "  Restart app:  docker-compose --env-file .env.production restart"
echo "  View status:  docker-compose --env-file .env.production ps"
echo ""
echo "🔒 Security Reminders:"
echo "  - Ensure firewall allows port 3000"
echo "  - Consider setting up SSL/TLS for production use"
echo "  - Regularly backup your database"
echo "  - Keep your VM and Docker images updated"
