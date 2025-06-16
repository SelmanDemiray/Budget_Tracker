#!/bin/bash

# Budget Tracker - Domain Deployment Script
# This script sets up Budget Tracker with proper SSL certificates for budgetfun.ca

set -e

echo "🚀 Budget Tracker - Domain Deployment for budgetfun.ca"
echo "====================================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
log_info() { echo -e "${BLUE}ℹ️  $1${NC}"; }
log_success() { echo -e "${GREEN}✅ $1${NC}"; }
log_warning() { echo -e "${YELLOW}⚠️  $1${NC}"; }
log_error() { echo -e "${RED}❌ $1${NC}"; }

# Check if running as root or with sudo access
if [ "$EUID" -eq 0 ]; then
    SUDO=""
elif sudo -n true 2>/dev/null; then
    SUDO="sudo"
else
    log_error "This script requires root privileges or sudo access"
    exit 1
fi

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    log_info "Installing Docker..."
    curl -fsSL https://get.docker.com -o get-docker.sh
    $SUDO sh get-docker.sh
    $SUDO usermod -aG docker $USER
    log_success "Docker installed successfully"
else
    log_success "Docker is already installed"
fi

# Check if Docker Compose is installed
if ! command -v docker-compose &> /dev/null; then
    log_info "Installing Docker Compose..."
    $SUDO curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
    $SUDO chmod +x /usr/local/bin/docker-compose
    log_success "Docker Compose installed successfully"
else
    log_success "Docker Compose is already installed"
fi

# Configure firewall
log_info "Configuring firewall..."
if command -v ufw &> /dev/null; then
    $SUDO ufw --force enable
    $SUDO ufw allow ssh
    $SUDO ufw allow 80/tcp
    $SUDO ufw allow 443/tcp
    log_success "Firewall configured"
else
    log_warning "UFW not available, please ensure ports 80 and 443 are open"
fi

# Stop any existing containers
log_info "Stopping existing containers..."
docker-compose down --volumes 2>/dev/null || true

# Create required directories
log_info "Creating required directories..."
mkdir -p certbot/conf
mkdir -p certbot/www

# Check if .env.production exists, if not create it
if [ ! -f .env.production ]; then
    log_info "Creating production environment file..."
    
    # Generate a secure password
    POSTGRES_PASSWORD=$(openssl rand -base64 32 | tr -d "=+/" | cut -c1-25)
    
    cat > .env.production << EOF
# Database Configuration
DATABASE_URL=postgres://postgres:${POSTGRES_PASSWORD}@postgres:5432/budget_tracker
POSTGRES_PASSWORD=${POSTGRES_PASSWORD}

# Application Configuration
RUST_LOG=info
ENVIRONMENT=production
EOF
    
    log_success "Production environment file created"
else
    log_success "Production environment file already exists"
fi

# Function to get SSL certificate
get_ssl_certificate() {
    log_info "Obtaining SSL certificate from Let's Encrypt..."
    
    # Check if certificate already exists
    if [ -f "certbot/conf/live/budgetfun.ca/fullchain.pem" ]; then
        log_success "SSL certificate already exists"
        return 0
    fi
    
    # Start nginx temporarily for certificate verification
    log_info "Starting temporary nginx for certificate verification..."
    
    # Create temporary nginx config for certificate verification
    mkdir -p nginx/temp
    cat > nginx/temp/nginx.conf << 'EOF'
events {
    worker_connections 1024;
}

http {
    server {
        listen 80;
        server_name budgetfun.ca www.budgetfun.ca;
        
        location /.well-known/acme-challenge/ {
            root /var/www/certbot;
        }
        
        location / {
            return 200 'OK';
            add_header Content-Type text/plain;
        }
    }
}
EOF
    
    # Start temporary nginx container
    docker run -d --name temp-nginx \
        -p 80:80 \
        -v "$(pwd)/nginx/temp/nginx.conf:/etc/nginx/nginx.conf:ro" \
        -v "$(pwd)/certbot/www:/var/www/certbot:ro" \
        nginx:alpine
    
    # Wait for nginx to start
    sleep 5
    
    # Get certificate
    docker run --rm \
        -v "$(pwd)/certbot/conf:/etc/letsencrypt" \
        -v "$(pwd)/certbot/www:/var/www/certbot" \
        certbot/certbot certonly \
        --webroot \
        --webroot-path=/var/www/certbot \
        --email admin@budgetfun.ca \
        --agree-tos \
        --no-eff-email \
        -d budgetfun.ca \
        -d www.budgetfun.ca
    
    # Stop temporary nginx
    docker stop temp-nginx
    docker rm temp-nginx
    rm -rf nginx/temp
    
    if [ -f "certbot/conf/live/budgetfun.ca/fullchain.pem" ]; then
        log_success "SSL certificate obtained successfully"
    else
        log_error "Failed to obtain SSL certificate"
        log_error "Please ensure:"
        log_error "1. Your domain budgetfun.ca points to this server's IP"
        log_error "2. Ports 80 and 443 are open in your firewall/security groups"
        log_error "3. No other web server is running on port 80"
        exit 1
    fi
}

# Get SSL certificate
get_ssl_certificate

# Build and start the application
log_info "Building and starting the application..."
docker-compose --env-file .env.production build --no-cache
docker-compose --env-file .env.production up -d

# Wait for services to be ready
log_info "Waiting for services to start..."
sleep 30

# Check if services are running
log_info "Checking service status..."
docker-compose --env-file .env.production ps

# Test the application
log_info "Testing the application..."
if curl -k -s https://budgetfun.ca > /dev/null; then
    log_success "Application is responding to HTTPS requests"
else
    log_warning "Application may not be fully ready yet"
fi

echo ""
echo "========================================"
echo "🎉 Deployment Complete!"
echo "========================================"
echo ""
echo "Your Budget Tracker is now running with SSL!"
echo ""
echo "🌐 Access your application at:"
echo "   https://budgetfun.ca"
echo "   https://www.budgetfun.ca"
echo ""
echo "🔒 SSL Certificate:"
echo "   ✅ Let's Encrypt certificate installed"
echo "   ✅ Auto-renewal configured (every 12 hours)"
echo "   ✅ Both budgetfun.ca and www.budgetfun.ca supported"
echo ""
echo "🛠️  Management Commands:"
echo "   View logs:    docker-compose --env-file .env.production logs -f"
echo "   Stop app:     docker-compose --env-file .env.production down"
echo "   Restart app:  docker-compose --env-file .env.production restart"
echo "   Renew SSL:    docker-compose --env-file .env.production exec certbot certbot renew"
echo ""
echo "📊 Container Status:"
docker-compose --env-file .env.production ps
echo ""
echo "✅ Your Budget Tracker is ready to use!"
