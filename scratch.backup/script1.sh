#!/bin/bash

# إنشاء مجلد لتجميع البيانات
mkdir -p dataset_raw/docker
mkdir -p dataset_raw/system

echo "--- [1/3] Collecting Docker Container Logs ---"

# 1. Traefik (Proxy Logs - JSON/Text mixed)
docker logs traefik-proxy > dataset_raw/docker/traefik_proxy.log 2>&1
echo "✅ Traefik logs collected."

# 2. RabbitMQ (Message Queue Logs)
docker logs rabbitmq > dataset_raw/docker/rabbitmq.log 2>&1
echo "✅ RabbitMQ logs collected."

# 3. Redis (Cache Logs)
docker logs redis > dataset_raw/docker/redis.log 2>&1
echo "✅ Redis logs collected."

# 4. Apache HTTPD (Web Server) - Already running container
docker logs httpd > dataset_raw/docker/apache_httpd.log 2>&1
echo "✅ Apache HTTPD logs collected."

# 5. Backend App (Node.js/App Logs)
docker logs backend-dev > dataset_raw/docker/nodejs_backend.log 2>&1
echo "✅ Backend Node.js logs collected."

# 6. LMS Frontend (Nginx inside container)
docker logs lms-frontend-container-v227 > dataset_raw/docker/nginx_frontend_container.log 2>&1
echo "✅ Frontend Container logs collected."

# 7. Grafana & Loki (Monitoring Logs - Good for internal errors)
docker logs monitoring-stack-grafana-1 > dataset_raw/docker/grafana.log 2>&1
docker logs monitoring-stack-loki-1 > dataset_raw/docker/loki.log 2>&1
echo "✅ Monitoring stack logs collected."


echo "--- [2/3] Collecting System Service Logs (Requires Sudo) ---"

# 1. PostgreSQL 17 (Main DB) - Copying latest log file
# عادة اللوجز بتكون في المسار ده، لو متغير عندك قولي
if [ -d "/var/log/postgresql" ]; then
    sudo cp /var/log/postgresql/postgresql-17-main.log dataset_raw/system/postgresql_17_host.log
    sudo chmod 644 dataset_raw/system/postgresql_17_host.log
    echo "✅ Postgres Host logs collected."
else
    echo "⚠️  Postgres logs not found in default path."
fi

# 2. Nginx (Host Reverse Proxy)
if [ -d "/var/log/nginx" ]; then
    sudo cp /var/log/nginx/access.log dataset_raw/system/nginx_host_access.log
    sudo cp /var/log/nginx/error.log dataset_raw/system/nginx_host_error.log
    sudo chmod 644 dataset_raw/system/nginx_host_*.log
    echo "✅ Nginx Host logs collected."
else
    echo "⚠️  Nginx logs not found."
fi

# 3. Syslog / Auth (Linux System Logs)
sudo cp /var/log/auth.log dataset_raw/system/linux_auth.log
sudo cp /var/log/syslog dataset_raw/system/linux_syslog.log
sudo chmod 644 dataset_raw/system/linux_*.log
echo "✅ System Auth & Syslog collected."

# 4. Fail2Ban
if [ -f "/var/log/fail2ban.log" ]; then
    sudo cp /var/log/fail2ban.log dataset_raw/system/fail2ban.log
    sudo chmod 644 dataset_raw/system/fail2ban.log
    echo "✅ Fail2Ban logs collected."
fi

echo "--- [3/3] Collection Complete! Check 'dataset_raw' folder. ---"
ls -lh dataset_raw/*