# Budget Tracker Project Cleanup Script
# This script will automatically stop and remove all Docker resources

Write-Host "=== Budget Tracker Project Cleanup ===" -ForegroundColor Yellow
Write-Host "This will remove all containers, images, and volumes for this project." -ForegroundColor Red

# Stop and remove containers from docker-compose
Write-Host "`nStopping and removing containers..." -ForegroundColor Green
docker-compose down --volumes --remove-orphans 2>$null

# Remove project-specific images
Write-Host "Removing project images..." -ForegroundColor Green
docker rmi budget_tracker-app 2>$null
docker rmi postgres:15 2>$null
docker rmi rust:1.83 2>$null
docker rmi debian:bookworm-slim 2>$null

# Remove any dangling images
Write-Host "Removing dangling images..." -ForegroundColor Green
docker image prune -f 2>$null

# Remove project volumes
Write-Host "Removing project volumes..." -ForegroundColor Green
docker volume rm budget_tracker_postgres_data 2>$null

# Remove any dangling volumes
Write-Host "Removing dangling volumes..." -ForegroundColor Green
docker volume prune -f 2>$null

# Remove any stopped containers
Write-Host "Removing stopped containers..." -ForegroundColor Green
docker container prune -f 2>$null

# Remove any unused networks
Write-Host "Removing unused networks..." -ForegroundColor Green
docker network prune -f 2>$null

# Show remaining Docker resources
Write-Host "`n=== Remaining Docker Resources ===" -ForegroundColor Yellow
Write-Host "Containers:" -ForegroundColor Cyan
docker ps -a --format "table {{.Names}}\t{{.Image}}\t{{.Status}}"

Write-Host "`nImages:" -ForegroundColor Cyan
docker images --format "table {{.Repository}}\t{{.Tag}}\t{{.Size}}"

Write-Host "`nVolumes:" -ForegroundColor Cyan
docker volume ls

Write-Host "`n=== Cleanup Complete ===" -ForegroundColor Green
Write-Host "All Budget Tracker project resources have been removed." -ForegroundColor Green
