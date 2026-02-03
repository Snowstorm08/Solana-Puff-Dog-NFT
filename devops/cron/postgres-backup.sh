#!/usr/bin/env bash
set -euo pipefail

####################################
# Configuration
####################################
CURRENT_MONTH="$(date +%Y-%m)"
CURRENT_DATE="$(date +%Y-%m-%d)"
CURRENT_DATETIME="$(date +%Y-%m-%d_%H-%M-%S_%Z)"

BACKUPS_PATH="/backups"
DOCKER_SWARM_SERVICE_NAME="climate_news_stack_db"
S3_BACKUP_BUCKET="climate-news-db-backup"

# Required env vars (fail if missing)
: "${POSTGRES_DB:?POSTGRES_DB not set}"
: "${POSTGRES_USER:?POSTGRES_USER not set}"

####################################
# Paths
####################################
BACKUP_FOLDER="${BACKUPS_PATH}/${CURRENT_MONTH}/${CURRENT_DATE}"
BACKUP_FILENAME="${POSTGRES_DB}_${CURRENT_DATETIME}.sql.gz"
LOCAL_BACKUP_PATH="${BACKUP_FOLDER}/${BACKUP_FILENAME}"
S3_BACKUP_PATH="s3://${S3_BACKUP_BUCKET}${BACKUP_FOLDER}/"

####################################
# Prepare directories
####################################
mkdir -p "${BACKUP_FOLDER}"

echo "📦 Starting PostgreSQL backup"
echo "📁 Backup folder: ${BACKUP_FOLDER}"

####################################
# Get running container ID from Swarm
####################################
TASK_ID="$(docker service ps \
    --filter "desired-state=running" \
    --format '{{.ID}}' \
    "${DOCKER_SWARM_SERVICE_NAME}" | head -n1)"

if [[ -z "${TASK_ID}" ]]; then
    echo "❌ No running task found for service ${DOCKER_SWARM_SERVICE_NAME}"
    exit 1
fi

CONTAINER_ID="$(docker inspect \
    --format '{{.Status.ContainerStatus.ContainerID}}' \
    "${TASK_ID}")"

if [[ -z "${CONTAINER_ID}" ]]; then
    echo "❌ Failed to resolve container ID"
    exit 1
fi

####################################
# Dump & compress database
####################################
echo "🗄️  Dumping database: ${POSTGRES_DB}"

docker exec -t "${CONTAINER_ID}" \
    pg_dump -U "${POSTGRES_USER}" "${POSTGRES_DB}" \
    | gzip -9 > "${LOCAL_BACKUP_PATH}"

####################################
# Upload to S3
####################################
echo "☁️  Uploading backup to S3"
aws s3 cp "${LOCAL_BACKUP_PATH}" "${S3_BACKUP_PATH}"

####################################
# Cleanup
####################################
echo "🧹 Removing local backup"
rm -f "${LOCAL_BACKUP_PATH}"

echo "✅ Backup completed successfully"
