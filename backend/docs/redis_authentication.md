# Redis Authentication

This document explains how to configure Redis authentication for the backend service.

## Environment Variables

The backend uses the following environment variables for Redis configuration:

- `REDIS_URL`: The Redis connection URL (default: `redis://127.0.0.1:6379`)
- `REDIS_USERNAME`: The Redis username for authentication (optional)
- `REDIS_PASSWORD`: The Redis password for authentication (optional)

## Setting Up Authentication

### Option 1: Environment Variables

You can set these environment variables directly in your system:

```bash
export REDIS_URL=redis://127.0.0.1:6379
export REDIS_USERNAME=default  # Optional
export REDIS_PASSWORD=your_password_here  # Optional
```

### Option 2: .env File

Alternatively, you can create a `.env` file in the project root directory:

1. Run the script `./create_env_template.sh` to create a template `.env` file
2. Edit the `.env` file to set your authentication details:

```
# Redis configuration
REDIS_URL=redis://127.0.0.1:6379
REDIS_USERNAME=default
REDIS_PASSWORD=your_password_here
```

### Authentication Flow

The authentication process works as follows:

1. If both `REDIS_USERNAME` and `REDIS_PASSWORD` are provided, the backend uses the ACL-based `AUTH username password` command format.
2. If only `REDIS_PASSWORD` is provided, the backend uses the legacy `AUTH password` command format.
3. If neither is provided, no authentication is attempted.

## Redis ACL Configuration

For Redis 6.0 and above, you can use the more secure ACL system. Here's an example of how to set up a user:

```
# Connect to Redis CLI
redis-cli

# Create a new user with a password
ACL SETUSER myuser on >mypassword ~* &* +@all

# List all users
ACL LIST
```

For older Redis versions, you can set the password in the Redis configuration file:

```
# In redis.conf
requirepass your_password_here
```

## Troubleshooting

If you encounter authentication issues:

1. Verify that the credentials in your `.env` file or environment variables match those in your Redis configuration
2. Check Redis server logs for authentication failure messages
3. Ensure your Redis server has authentication enabled
4. Make sure the Redis user has the necessary permissions to perform operations 