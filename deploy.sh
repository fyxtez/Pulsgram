#!/bin/bash

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
# TEST STEP
# --------------------------
echo "Running Rust tests..."
cargo test

if [ $? -ne 0 ]; then
    echo "Tests failed. Aborting deployment."
    exit 1
fi
echo "All tests passed."

# --------------------------
# BUILD STEP
# --------------------------
echo "Building Rust project with production features..."
cargo build --release --features production

if [ $? -ne 0 ]; then
    echo "Build failed. Aborting deployment."
    exit 1
fi
echo "Build succeeded."

# --------------------------
# DEPLOYMENT STEP
# --------------------------
echo "Copying binary to remote server..."
scp "$LOCAL_FILE" "$REMOTE_USER@$REMOTE_HOST:$REMOTE_PATH"
echo "Binary copied to $REMOTE_HOST:$REMOTE_PATH."

echo "Setting executable permissions on remote binary..."
ssh "$REMOTE_USER@$REMOTE_HOST" "chmod +x $REMOTE_PATH"
echo "Permissions set."

echo "Creating systemd service file..."
ssh "$REMOTE_USER@$REMOTE_HOST" "cat > /etc/systemd/system/$SERVICE_NAME.service << 'EOF'
[Unit]
Description=Pulsgram Application
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/root
ExecStart=/root/app
Restart=always
RestartSec=5
StandardOutput=append:/root/app.log
StandardError=append:/root/app.log

# Security and resource limits (optional but recommended)
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
EOF
"
echo "Systemd service file created."

echo "Reloading systemd daemon..."
ssh "$REMOTE_USER@$REMOTE_HOST" "systemctl daemon-reload"

echo "Enabling service to start on boot..."
ssh "$REMOTE_USER@$REMOTE_HOST" "systemctl enable $SERVICE_NAME"

echo "Restarting service..."
ssh "$REMOTE_USER@$REMOTE_HOST" "systemctl restart $SERVICE_NAME"
echo "Service '$SERVICE_NAME' restarted."

# --------------------------
# HEALTH CHECK
# --------------------------
echo "Waiting for service to start..."
sleep 3

echo "Checking service status..."
ssh "$REMOTE_USER@$REMOTE_HOST" "systemctl status $SERVICE_NAME --no-pager"

echo ""
echo "Pinging health endpoint..."
curl "http://$REMOTE_HOST:8181/api/v1/ping"
echo ""

echo ""
echo "Deployment complete!"
echo ""
echo "Useful commands:"
echo "   Check logs:        ssh $REMOTE_USER@$REMOTE_HOST 'journalctl -u $SERVICE_NAME -f'"
echo "   Check status:      ssh $REMOTE_USER@$REMOTE_HOST 'systemctl status $SERVICE_NAME'"
echo "   Restart service:   ssh $REMOTE_USER@$REMOTE_HOST 'systemctl restart $SERVICE_NAME'"
echo "   Stop service:      ssh $REMOTE_USER@$REMOTE_HOST 'systemctl stop $SERVICE_NAME'"