# Quick Start - budgetfun.ca Deployment

## 🚀 One-Command Setup

**Prerequisites:**
- Domain `budgetfun.ca` and `www.budgetfun.ca` must point to your server's IP
- Ubuntu/Debian server with ports 80/443 available

**Deploy:**
```bash
chmod +x deploy-domain.sh
./deploy-domain.sh
```

**Access:**
- https://budgetfun.ca ✅ (No SSL warnings!)
- https://www.budgetfun.ca ✅ (No SSL warnings!)

## 🔧 Management

**Standard Docker Compose commands work after deployment:**

```bash
# Start application
docker-compose --env-file .env.production up -d

# Stop application
docker-compose --env-file .env.production down

# View logs
docker-compose --env-file .env.production logs -f

# Check status
./status.sh
```

## 🔒 SSL Certificate

- ✅ Let's Encrypt certificates (trusted by all browsers)
- ✅ Auto-renewal every 12 hours
- ✅ No browser warnings or security alerts

## 🛠️ What Gets Set Up

- Docker & Docker Compose (auto-installed if needed)
- Firewall configuration (UFW)
- Let's Encrypt SSL certificates
- Nginx reverse proxy with security headers
- PostgreSQL database
- Budget Tracker application

The deployment script handles everything automatically!
