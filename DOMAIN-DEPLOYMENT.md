# Budget Tracker - Domain Deployment Guide

This guide helps you deploy Budget Tracker to be accessible at `budgetfun.ca` with proper HTTPS certificates.

## Prerequisites

1. **Domain Setup**: Ensure your domain DNS is configured:
   ```
   A record: budgetfun.ca → YOUR_SERVER_IP
   A record: www.budgetfun.ca → YOUR_SERVER_IP
   ```

2. **Server Requirements**:
   - Ubuntu/Debian server with public IP
   - At least 2GB RAM and 20GB storage
   - Ports 80 and 443 available (no other web servers running)

## One-Command Deployment

1. **Upload your project to the server**

2. **Run the deployment script**:
   ```bash
   chmod +x deploy-domain.sh
   ./deploy-domain.sh
   ```

3. **Access your application**:
   - HTTPS: `https://budgetfun.ca`
   - HTTPS: `https://www.budgetfun.ca`
   - HTTP: `http://budgetfun.ca` (redirects to HTTPS)

## What Gets Deployed

The deployment automatically:

- **Installs Docker and Docker Compose** (if not already installed)
- **Configures Firewall** (UFW) to allow ports 80 and 443
- **Obtains SSL Certificate** from Let's Encrypt for both `budgetfun.ca` and `www.budgetfun.ca`
- **Sets up Nginx** as reverse proxy with SSL termination
- **Starts PostgreSQL** database
- **Builds and runs** your Budget Tracker application
- **Configures Auto-renewal** for SSL certificates

## Using Docker Compose Only

After the initial deployment, you can manage your app with standard Docker Compose commands:

```bash
# Start the application
docker-compose --env-file .env.production up -d

# Stop the application  
docker-compose --env-file .env.production down

# View logs
docker-compose --env-file .env.production logs -f

# Restart services
docker-compose --env-file .env.production restart
```

## SSL Certificates

- ✅ **Let's Encrypt certificates** - Trusted by all browsers, no warnings
- ✅ **Auto-renewal** every 12 hours via Certbot
- ✅ **Both domains supported** - `budgetfun.ca` and `www.budgetfun.ca`
- ✅ **Modern TLS** - TLS 1.2 and 1.3 support with secure ciphers

## Status Check

Check your deployment status anytime:

```bash
chmod +x status.sh
./status.sh
```

This will show:
- Container status
- SSL certificate validity
- Website accessibility  
- Recent logs

## Management Commands

```bash
# View all container status
docker-compose --env-file .env.production ps

# View application logs
docker-compose --env-file .env.production logs -f

# Restart the application
docker-compose --env-file .env.production restart

# Stop everything
docker-compose --env-file .env.production down

# Manually renew SSL certificates
docker-compose --env-file .env.production exec certbot certbot renew
```

## Security Features

- ✅ SSL/TLS encryption
- ✅ HTTP to HTTPS redirect
- ✅ Security headers (HSTS, X-Frame-Options, etc.)
- ✅ Rate limiting (API: 10 req/s, Login: 5 req/min)
- ✅ Gzip compression
- ✅ Static file caching

## Troubleshooting

### SSL Certificate Issues
```bash
# Check certificate status
docker-compose --env-file .env.production exec certbot certbot certificates

# Test certificate renewal
docker-compose --env-file .env.production exec certbot certbot renew --dry-run
```

### Domain Not Accessible
1. Verify DNS: `nslookup budgetfun.ca`
2. Check firewall: `sudo ufw status`
3. Test ports: `telnet budgetfun.ca 80` and `telnet budgetfun.ca 443`

### Application Issues
```bash
# Check app logs
docker-compose --env-file .env.production logs app

# Check nginx logs
docker-compose --env-file .env.production logs nginx

# Test internal connectivity
docker-compose --env-file .env.production exec nginx curl http://app:3000
```

## File Structure

After deployment, your project will have:

```
Budget_Tracker/
├── nginx/
│   └── nginx.conf          # Nginx configuration
├── certbot/
│   ├── conf/              # SSL certificates
│   └── www/               # ACME challenge files
├── deploy-domain.sh       # Main deployment script
├── setup-ssl.sh          # SSL setup script
├── docker-compose.yml     # Updated with nginx and certbot
└── .env.production       # Production environment variables
```

## Support

For issues:
1. Check container logs
2. Verify DNS configuration
3. Ensure firewall allows ports 80/443
4. Check Let's Encrypt rate limits if SSL fails
