#!/bin/bash

# Domain Setup Verification Script

echo "🔍 Budget Tracker Domain Setup Check"
echo "===================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() { echo -e "${BLUE}ℹ️  $1${NC}"; }
log_success() { echo -e "${GREEN}✅ $1${NC}"; }
log_warning() { echo -e "${YELLOW}⚠️  $1${NC}"; }
log_error() { echo -e "${RED}❌ $1${NC}"; }

echo ""
log_info "Checking DNS records for budgetfun.ca..."

# Check A record
echo ""
echo "A Records:"
A_RECORD=$(dig +short budgetfun.ca A)
if [ -n "$A_RECORD" ]; then
    log_success "budgetfun.ca resolves to: $A_RECORD"
else
    log_error "budgetfun.ca does not resolve to an IP address"
fi

# Check WWW CNAME
echo ""
echo "CNAME Records:"
WWW_RECORD=$(dig +short www.budgetfun.ca)
if [ -n "$WWW_RECORD" ]; then
    log_success "www.budgetfun.ca resolves to: $WWW_RECORD"
else
    log_error "www.budgetfun.ca does not resolve"
fi

# Check if IP matches server
echo ""
log_info "Checking if domain points to this server..."
SERVER_IP=$(curl -s http://checkip.amazonaws.com/ 2>/dev/null || curl -s http://ipecho.net/plain 2>/dev/null)
if [ "$A_RECORD" = "$SERVER_IP" ]; then
    log_success "Domain points to this server ($SERVER_IP)"
else
    log_warning "Domain IP ($A_RECORD) doesn't match server IP ($SERVER_IP)"
    log_warning "This may be expected if you haven't pointed your domain yet"
fi

# Check port accessibility
echo ""
log_info "Checking port accessibility..."

# Check if port 80 is open
if nc -zv localhost 80 2>/dev/null; then
    log_success "Port 80 is accessible"
else
    log_warning "Port 80 is not accessible"
fi

# Check if port 443 is open
if nc -zv localhost 443 2>/dev/null; then
    log_success "Port 443 is accessible"
else
    log_warning "Port 443 is not accessible"
fi

# Check if Docker is running
echo ""
log_info "Checking Docker status..."
if docker info > /dev/null 2>&1; then
    log_success "Docker is running"
else
    log_error "Docker is not running"
fi

# Check if containers are running
echo ""
log_info "Checking container status..."
if [ -f docker-compose.yml ]; then
    CONTAINERS=$(docker-compose --env-file .env.production ps --services --filter "status=running" 2>/dev/null | wc -l)
    if [ "$CONTAINERS" -gt 0 ]; then
        log_success "$CONTAINERS containers are running"
        docker-compose --env-file .env.production ps
    else
        log_warning "No containers are currently running"
    fi
else
    log_warning "docker-compose.yml not found"
fi

# Check SSL certificate
echo ""
log_info "Checking SSL certificate..."
if [ -f "certbot/conf/live/budgetfun.ca/fullchain.pem" ]; then
    log_success "SSL certificate exists"
    CERT_EXPIRY=$(openssl x509 -noout -dates -in certbot/conf/live/budgetfun.ca/fullchain.pem | grep notAfter | cut -d= -f2)
    log_info "Certificate expires: $CERT_EXPIRY"
else
    log_warning "SSL certificate not found"
fi

echo ""
echo "========================================"
echo "🚀 Next Steps:"
echo ""
if [ "$A_RECORD" != "$SERVER_IP" ]; then
    echo "1. ⚠️  Update your DNS A record to point to: $SERVER_IP"
    echo "2. Wait 5-10 minutes for DNS propagation"
    echo "3. Run: chmod +x deploy-domain.sh && ./deploy-domain.sh"
else
    echo "1. ✅ Your DNS is properly configured"
    echo "2. Run: chmod +x deploy-domain.sh && ./deploy-domain.sh"
fi
echo ""
echo "After deployment, your app will be available at:"
echo "🌐 https://budgetfun.ca"
echo "🌐 https://www.budgetfun.ca"
echo ""
