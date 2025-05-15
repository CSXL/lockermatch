#!/bin/bash

# Script to create a .env template file for Redis configuration

ENV_FILE=.env

if [ -f $ENV_FILE ]; then
    echo "Warning: $ENV_FILE file already exists."
    read -p "Do you want to overwrite it? (y/N): " OVERWRITE
    if [[ "$OVERWRITE" != "y" && "$OVERWRITE" != "Y" ]]; then
        echo "Operation cancelled."
        exit 1
    fi
fi

cat > $ENV_FILE << EOL
# Redis configuration
REDIS_URL=redis://127.0.0.1:6379
REDIS_USERNAME=
REDIS_PASSWORD=

# Uncomment and set these if using Redis authentication
# REDIS_USERNAME=default
# REDIS_PASSWORD=your_password_here
EOL

chmod 600 $ENV_FILE
echo "Created $ENV_FILE template file."
echo "Please edit it to set your Redis authentication credentials if needed." 