#!/bin/bash
set -euo pipefail
trap 'echo ""; echo "❌ Deployment failed at line $LINENO"; exit 1' ERR

# --------------------------
# LOAD CONFIG FROM .env
# --------------------------
if [ ! -f .env ]; then
    echo "Error: .env file not found!"
    echo "Please create a .env file with the required configuration."
    exit 1
fi

# Load environment variables from .env file
set -a
source .env
set +a

# --------------------------
# VALIDATE REQUIRED VARS
# --------------------------
: "${REMOTE_USER:?Missing REMOTE_USER}"
: "${REMOTE_HOST:?Missing REMOTE_HOST}"
: "${SERVICE_NAME:?Missing SERVICE_NAME}"
: "${LOCAL_FILE:?Missing LOCAL_FILE}"
: "${REMOTE_PATH:?Missing REMOTE_PATH}"

echo "Deploying to $REMOTE_HOST"
read -p "Continue? (y/N): " CONFIRM

if [[ "$CONFIRM" != "y" ]]; then
    echo "Aborted."
    exit 0
fi

# --------------------------
# TEST STEP
# --------------------------
echo "Running Rust tests..."
cargo test -- --nocapture --include-ignored # Can do this but only localy for now.

echo "All tests passed."

# --------------------------
# BUILD STEP
# --------------------------
echo "Preparing build version..."
# Get current git commit short hash.
# This identifies exactly what source code is being deployed.
BUILD_VERSION=$(git rev-parse --short HEAD)
echo "Build version: $BUILD_VERSION"

echo "Building Rust project with production features..."
# Inject BUILD_VERSION only for this cargo build command.
#
# This makes BUILD_VERSION available at *compile time*.
# Rust's `option_env!("BUILD_VERSION")` reads it during compilation
# and embeds it directly into the binary.
#
# After this command finishes, BUILD_VERSION is NOT kept in the shell.
# Used for get_build_version() function when bootstraping.
BUILD_VERSION="$BUILD_VERSION" cargo build --release --features production
echo "Build succeeded."

# --------------------------
# BINARY SIZE INFO
# --------------------------
if [ -f "$LOCAL_FILE" ]; then
    echo "Calculating binary size..."

    # Human readable size (MB/GB automatically)
    HUMAN_SIZE=$(du -h "$LOCAL_FILE" | cut -f1)

    # Exact size in bytes
    BYTES_SIZE=$(stat -c%s "$LOCAL_FILE")

    # Size in MB with decimals
    MB_SIZE=$(awk "BEGIN {printf \"%.2f\", $BYTES_SIZE/1024/1024}")

    echo "Binary size:"
    echo "   • Human readable: $HUMAN_SIZE"
    echo "   • Exact bytes:    $BYTES_SIZE bytes"
    echo "   • In MB:          $MB_SIZE MB"
else
    echo "Warning: Built binary not found at $LOCAL_FILE"
fi

DEPLOY_START_TIME=$(date +"%Y-%m-%d %H:%M:%S")

# --------------------------
# DEPLOYMENT STEP
# --------------------------
echo "Stopping service before deploy..."
ssh "$REMOTE_USER@$REMOTE_HOST" "systemctl stop $SERVICE_NAME 2>/dev/null || true"

echo "Copying binary to remote server..."
scp "$LOCAL_FILE" "$REMOTE_USER@$REMOTE_HOST:$REMOTE_PATH"

echo "Binary copied to $REMOTE_HOST:$REMOTE_PATH."

echo "Setting executable permissions on remote binary..."
ssh "$REMOTE_USER@$REMOTE_HOST" "chmod +x $REMOTE_PATH"
echo "Permissions set."

echo "Ensuring systemd service exists..."

ssh "$REMOTE_USER@$REMOTE_HOST" "
if [ ! -f /etc/systemd/system/$SERVICE_NAME.service ]; then
    echo 'Creating systemd service...'

    cat > /etc/systemd/system/$SERVICE_NAME.service << EOF
[Unit]
Description=Pulsgram Application
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/root
ExecStart=$REMOTE_PATH
Restart=always
RestartSec=5

# Logging via journal (recommended)
StandardOutput=journal
StandardError=journal

# Resource limits
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
EOF

    systemctl daemon-reload
    systemctl enable $SERVICE_NAME
    echo 'Service created and enabled.'
else
    echo 'Service already exists. Skipping creation.'
fi
"

echo "Restarting service..."
ssh "$REMOTE_USER@$REMOTE_HOST" "systemctl restart $SERVICE_NAME"
echo "Service '$SERVICE_NAME' restarted."

# --------------------------
# HEALTH CHECK
# --------------------------

echo "Waiting for service to become active..."

if ! ssh "$REMOTE_USER@$REMOTE_HOST" \
    "systemctl is-active --quiet $SERVICE_NAME"; then
    echo "Service is not active!"
    ssh "$REMOTE_USER@$REMOTE_HOST" \
        "systemctl status $SERVICE_NAME --no-pager"
    exit 1
fi

echo "Service is active."


echo "Verifying deployed build version in app.log..."

EXPECTED_VERSION="$BUILD_VERSION"

LOG_OUTPUT=$(ssh "$REMOTE_USER@$REMOTE_HOST" \
    "tail -n 100 /root/app.log")

if echo "$LOG_OUTPUT" | grep -Fq "Build Version: $EXPECTED_VERSION"; then    echo "Build version $EXPECTED_VERSION confirmed in logs ✅"
else
    echo "ERROR: Build version $EXPECTED_VERSION NOT found in logs ❌"
    echo ""
    echo "Last 100 log lines:"
    echo "$LOG_OUTPUT"
    exit 1
fi

echo "Checking HTTP health endpoint..."

HEALTH_OK=false

for i in {1..10}; do
    if curl -fs "http://$REMOTE_HOST:8181/api/v1/ping" >/dev/null 2>&1; then
        HEALTH_OK=true
        break
    fi
    sleep 2
done

if [ "$HEALTH_OK" != true ]; then
    echo "Health check FAILED!"
    echo ""
    echo "Service status:"
    ssh "$REMOTE_USER@$REMOTE_HOST" \
        "systemctl status $SERVICE_NAME --no-pager"

    echo ""
    echo "Last 30 journal logs:"
    ssh "$REMOTE_USER@$REMOTE_HOST" \
        "journalctl -u $SERVICE_NAME -n 30 --no-pager"

    exit 1
fi

# --------------------------
# FINISH TIME
# --------------------------

DEPLOY_END_TIME=$(date +"%Y-%m-%d %H:%M:%S")

echo "Health check passed! (HTTP 200)"
echo ""
echo "Deployment complete!"
echo ""
echo "Started at : $DEPLOY_START_TIME"
echo "Finished at: $DEPLOY_END_TIME"
echo ""
echo "Useful commands:"
echo "   Check logs:        ssh $REMOTE_USER@$REMOTE_HOST 'tail -f $REMOTE_PATH.log'"
echo "   Last 100 lines:    ssh $REMOTE_USER@$REMOTE_HOST 'tail -n 100 $REMOTE_PATH.log'"
echo "   Check status:      ssh $REMOTE_USER@$REMOTE_HOST 'systemctl status $SERVICE_NAME'"
echo "   Restart service:   ssh $REMOTE_USER@$REMOTE_HOST 'systemctl restart $SERVICE_NAME'"
echo "   Stop service:      ssh $REMOTE_USER@$REMOTE_HOST 'systemctl stop $SERVICE_NAME'"