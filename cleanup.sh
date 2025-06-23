#!/bin/bash
# Budget Tracker Project Cleanup Script (Bash version)
# This script will stop and remove all Docker resources for the Budget Tracker project only

echo "=== Budget Tracker Project Cleanup ==="
echo "This will remove all containers, images, and volumes for this project."

# Stop and remove containers from docker-compose
echo -e "\nStopping and removing containers..."
docker-compose down --volumes --remove-orphans 2>/dev/null

# Remove project-specific images
echo "Removing project images..."
docker images --format "{{.Repository}}:{{.Tag}}" | grep '^budget_tracker-app' | xargs -r docker rmi 2>/dev/null
docker images --format "{{.Repository}}:{{.Tag}}" | grep '^postgres:15' | xargs -r docker rmi 2>/dev/null
docker images --format "{{.Repository}}:{{.Tag}}" | grep '^rust:1.83' | xargs -r docker rmi 2>/dev/null
docker images --format "{{.Repository}}:{{.Tag}}" | grep '^debian:bookworm-slim' | xargs -r docker rmi 2>/dev/null

# Remove project volume
echo "Removing project volumes..."
docker volume ls --format "{{.Name}}" | grep '^budget_tracker_postgres_data$' | xargs -r docker volume rm 2>/dev/null

# Show remaining Docker resources (filtered for this project)
echo -e "\n=== Remaining Docker Resources ==="
echo "Containers:"
docker ps -a --filter "name=budget" --format "table {{.Names}}\t{{.Image}}\t{{.Status}}"

echo -e "\nImages:"
docker images --filter=reference='budget_tracker-app' --format "table {{.Repository}}\t{{.Tag}}\t{{.Size}}"

echo -e "\nVolumes:"
docker volume ls --filter name=budget

echo -e "\n=== Cleanup Complete ==="
echo "All Budget Tracker project resources have been removed."
