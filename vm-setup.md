# Cloud VM Setup Instructions for Budget Tracker

## Prerequisites
- Ubuntu/Debian cloud VM with public IP (AWS EC2, Google Cloud, DigitalOcean, etc.)
- SSH access to the VM
- At least 2GB RAM and 20GB storage
- VM security group/firewall allows inbound traffic on port 3000

## 1. Initial VM Setup

### Connect to your VM:
```bash
ssh user@YOUR_VM_PUBLIC_IP
# For AWS EC2: ssh -i your-key.pem ubuntu@YOUR_VM_PUBLIC_IP
# For Google Cloud: gcloud compute ssh your-instance-name
```

### Update system:
```bash
sudo apt update && sudo apt upgrade -y
sudo apt install -y curl wget git ufw
```

## 2. Install Docker and Docker Compose

### Install Docker:
```bash
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
sudo usermod -aG docker $USER
```

### Install Docker Compose:
```bash
sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
```

### Logout and login again for Docker group changes:
```bash
exit
ssh user@YOUR_VM_PUBLIC_IP
```

### Verify Docker installation:
```bash
docker --version
docker-compose --version
```

## 3. Configure Firewall

```bash
# Allow SSH (if not already configured)
sudo ufw allow ssh

# Allow HTTP traffic on port 3000
sudo ufw allow 3000/tcp

# Enable firewall
sudo ufw --force enable

# Check firewall status
sudo ufw status
```

## 4. Cloud Provider Security Groups

### AWS EC2:
- Go to EC2 Dashboard → Security Groups
- Select your instance's security group
- Add inbound rule: Type: Custom TCP, Port: 3000, Source: 0.0.0.0/0

### Google Cloud:
```bash
gcloud compute firewall-rules create allow-budget-tracker \
    --allow tcp:3000 \
    --source-ranges 0.0.0.0/0 \
    --description "Allow Budget Tracker on port 3000"
```

### DigitalOcean:
- Go to Networking → Firewalls
- Create or edit firewall rule
- Add inbound rule: TCP, Port 3000, All IPv4

## 5. Deploy Application

### Upload your project files:

#### Option 1: Using scp from your local machine
```bash
# From your local machine (not VM)
scp -r Budget_Tracker/ user@YOUR_VM_IP:/home/user/
```

#### Option 2: Using git (recommended)
```bash
# On your VM
git clone https://github.com/yourusername/budget-tracker.git Budget_Tracker
cd Budget_Tracker
```

### Configure production environment:
```bash
# Copy and edit the production environment file
cp .env.production .env.production.backup
nano .env.production

# Set a strong password (example):
# DATABASE_URL=postgres://postgres:MyStr0ngP@ssw0rd!@postgres:5432/budget_tracker
# POSTGRES_PASSWORD=MyStr0ngP@ssw0rd!
# RUST_LOG=info
# ENVIRONMENT=production
```

### Make deploy script executable and run:
```bash
chmod +x deploy.sh
./deploy.sh
```

## 6. Verify Deployment

### Check if application is running:
```bash
# Check container status
docker-compose --env-file .env.production ps

# Check application health
curl http://localhost:3000/

# Get your public IP
curl http://checkip.amazonaws.com/
```

### Access your application:
Open your browser and go to: `http://YOUR_VM_PUBLIC_IP:3000`

## 7. Monitoring and Maintenance

### View logs:
```bash
# All logs
docker-compose --env-file .env.production logs -f

# Just app logs
docker-compose --env-file .env.production logs -f app

# Just database logs
docker-compose --env-file .env.production logs -f postgres
```

### Common maintenance commands:
```bash
# Restart application
docker-compose --env-file .env.production restart

# Stop application
docker-compose --env-file .env.production down

# Update and redeploy
git pull  # if using git
./deploy.sh

# Check disk usage
df -h
docker system df
```

### Backup database:
```bash
# Create backup
docker-compose --env-file .env.production exec postgres pg_dump -U postgres budget_tracker > backup_$(date +%Y%m%d).sql

# Restore backup (if needed)
docker-compose --env-file .env.production exec -T postgres psql -U postgres budget_tracker < backup_20241201.sql
```

## 8. Troubleshooting

### Application won't start:
```bash
# Check logs
docker-compose --env-file .env.production logs

# Check if port is in use
sudo netstat -tlnp | grep :3000

# Rebuild from scratch
docker-compose --env-file .env.production down --volumes
docker system prune -a
./deploy.sh
```

### Can't access from browser:
1. Check VM firewall: `sudo ufw status`
2. Check cloud provider security groups
3. Verify container is running: `docker-compose --env-file .env.production ps`
4. Test locally on VM: `curl http://localhost:3000/`

### Database issues:
```bash
# Check database container
docker-compose --env-file .env.production logs postgres

# Connect to database directly
docker-compose --env-file .env.production exec postgres psql -U postgres budget_tracker
```

## Security Best Practices

1. **Strong Passwords**: Use complex passwords in `.env.production`
2. **Regular Updates**: Keep VM and Docker images updated
3. **SSL/TLS**: Consider using nginx with Let's Encrypt for HTTPS
4. **Backup Strategy**: Set up automated database backups
5. **Log Monitoring**: Regularly check application logs
6. **Access Control**: Consider restricting access to specific IP ranges

## Performance Optimization

1. **Resource Monitoring**: Monitor CPU and memory usage
2. **Database Optimization**: Regular database maintenance
3. **Log Rotation**: Set up log rotation to prevent disk space issues
4. **Image Cleanup**: Regularly clean up unused Docker images

```bash
# Monitor resources
htop
docker stats

# Clean up Docker
docker system prune -a

# Set up log rotation (optional)
sudo nano /etc/logrotate.d/docker-compose
```

Your Budget Tracker should now be fully deployed and accessible from anywhere via your VM's public IP address!
