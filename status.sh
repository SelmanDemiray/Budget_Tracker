#!/bin/bash

# Budget Tracker Status Check

echo "🔍 Budget Tracker Status Check"
echo "=============================="

# Check if containers are running
echo ""
echo "📊 Container Status:"
docker-compose --env-file .env.production ps

# Check SSL certificate status
echo ""
echo "🔒 SSL Certificate Status:"
if [ -f "certbot/conf/live/budgetfun.ca/fullchain.pem" ]; then
    echo "✅ SSL certificate exists"
    docker-compose --env-file .env.production exec certbot certbot certificates 2>/dev/null || echo "ℹ️  Run: docker-compose --env-file .env.production exec certbot certbot certificates"
else
    echo "❌ SSL certificate not found"
fi

# Test website accessibility
echo ""
echo "🌐 Website Accessibility:"
if curl -k -s -o /dev/null -w "%{http_code}" https://budgetfun.ca | grep -q "200"; then
    echo "✅ https://budgetfun.ca is accessible"
else
    echo "❌ https://budgetfun.ca is not accessible"
fi

if curl -k -s -o /dev/null -w "%{http_code}" https://www.budgetfun.ca | grep -q "200"; then
    echo "✅ https://www.budgetfun.ca is accessible"
else
    echo "❌ https://www.budgetfun.ca is not accessible"
fi

# Check if HTTP redirects to HTTPS
echo ""
echo "🔄 HTTP to HTTPS Redirect:"
HTTP_RESPONSE=$(curl -s -o /dev/null -w "%{http_code}" http://budgetfun.ca)
if [ "$HTTP_RESPONSE" = "301" ] || [ "$HTTP_RESPONSE" = "302" ]; then
    echo "✅ HTTP properly redirects to HTTPS"
else
    echo "❌ HTTP redirect not working (got $HTTP_RESPONSE)"
fi

# Show recent logs
echo ""
echo "📝 Recent Logs (last 10 lines):"
docker-compose --env-file .env.production logs --tail=10

echo ""
echo "✅ Status check complete!"
