# Authentication System Upgrade

## Overview
Enhanced the login system with environment variable-based authentication and login attempt rate limiting with automatic account lockout.

## Features Implemented

### 1. Environment Variable Configuration
- **Username/Password from .env**: Authentication credentials are now loaded from `.env` file
- **Configurable Security Settings**: Max login attempts and lockout duration are configurable

### 2. Login Attempt Tracking
- **Failed Attempt Counting**: System tracks failed login attempts per username
- **Automatic Lockout**: Account is locked after exceeding max attempts
- **Time-based Unlock**: Accounts automatically unlock after the configured duration
- **Attempt Reset**: Successful login resets the attempt counter

### 3. Security Features
- **Rate Limiting**: Prevents brute force attacks
- **Informative Messages**: Users are informed of remaining attempts
- **Lockout Notifications**: Clear messages about lockout status and remaining time

## Configuration

### Environment Variables (.env)

```bash
# Authentication credentials
AUTH_USERNAME=admin
AUTH_PASSWORD=your-secure-password

# Security settings
MAX_LOGIN_ATTEMPTS=3              # Number of failed attempts before lockout
LOCKOUT_DURATION_MINUTES=30       # Lockout duration in minutes
```

### Default Values
If not specified in `.env`, the system uses these defaults:
- `AUTH_USERNAME`: "admin"
- `AUTH_PASSWORD`: "admin"
- `MAX_LOGIN_ATTEMPTS`: 3
- `LOCKOUT_DURATION_MINUTES`: 30

## How It Works

### Login Flow

1. **Lockout Check**: System first checks if the account is currently locked
   - If locked: Returns HTTP 429 with remaining lockout time
   
2. **Credential Validation**: Verifies username and password against .env configuration
   - If invalid: Records failed attempt and returns remaining attempts
   - If max attempts exceeded: Locks account for configured duration
   
3. **Success**: On valid credentials, resets attempt counter and issues JWT token

### Response Codes

- **200 OK**: Successful login with JWT token
- **401 Unauthorized**: Invalid credentials (with remaining attempts count)
- **429 Too Many Requests**: Account locked due to too many failed attempts

### Example Responses

**Failed Login (attempts remaining):**
```json
{
  "success": false,
  "error": "Invalid username or password. 2 attempts remaining."
}
```

**Account Locked:**
```json
{
  "success": false,
  "error": "Account locked due to too many failed login attempts. Please try again in 28 minutes."
}
```

**Successful Login:**
```json
{
  "success": true,
  "data": {
    "token": "eyJhbGc...",
    "user": {
      "username": "admin",
      "display_name": "admin",
      "auth_type": "Local"
    }
  }
}
```

## Files Modified

### Core Authentication
- `src/web/auth.rs`: Added `LoginAttemptTracker` struct with lockout logic
- `src/web/config.rs`: Added auth configuration fields
- `src/web/handlers.rs`: Updated login handler with attempt tracking
- `src/web/server.rs`: Initialize tracker and user store with config

### Configuration
- `src/main.rs`: Load .env file and parse environment variables
- `Cargo.toml`: Added `dotenv` dependency
- `.env.example`: Created example configuration file

## Setup Instructions

1. **Copy the example configuration:**
   ```bash
   cp .env.example .env
   ```

2. **Edit .env with your credentials:**
   ```bash
   # Change these values!
   AUTH_USERNAME=your-username
   AUTH_PASSWORD=your-secure-password
   MAX_LOGIN_ATTEMPTS=5
   LOCKOUT_DURATION_MINUTES=15
   ```

3. **Build and run:**
   ```bash
   cargo build --release
   cargo run -- serve
   ```

## Security Considerations

### Best Practices
✅ **Change default credentials** in production
✅ **Use strong passwords** (minimum 12 characters recommended)
✅ **Keep .env file secure** (never commit to version control)
✅ **Use HTTPS** in production to protect credentials in transit
✅ **Monitor failed login attempts** for security incidents

### Recommendations
- Set `MAX_LOGIN_ATTEMPTS` to 3-5 for good security/usability balance
- Set `LOCKOUT_DURATION_MINUTES` to 15-30 minutes
- Consider implementing email notifications for lockouts
- Regularly rotate passwords
- Use environment-specific .env files

## Testing

### Test Scenarios

1. **Valid Login:**
   ```bash
   curl -X POST http://localhost:8080/api/auth/login \
     -H "Content-Type: application/json" \
     -d '{"username":"admin","password":"admin"}'
   ```

2. **Invalid Login (test attempts):**
   ```bash
   # Try 3 times with wrong password
   curl -X POST http://localhost:8080/api/auth/login \
     -H "Content-Type: application/json" \
     -d '{"username":"admin","password":"wrong"}'
   ```

3. **Locked Account:**
   ```bash
   # After 3 failed attempts, should return 429
   curl -X POST http://localhost:8080/api/auth/login \
     -H "Content-Type: application/json" \
     -d '{"username":"admin","password":"admin"}'
   ```

## Implementation Details

### LoginAttemptTracker
- Thread-safe using `Mutex<HashMap>`
- Tracks attempts and lockout timestamps per username
- Automatically cleans up expired lockouts
- Configurable max attempts and lockout duration

### UserStore
- Validates against single configured user from .env
- Uses bcrypt for password hashing
- Constant-time comparison for security

### Lockout Logic
- Incremental attempt counting
- Timestamp-based lockout expiration
- Automatic reset on successful login
- Remaining time calculation for user feedback

## Future Enhancements

Potential improvements for future versions:
- [ ] Multiple user support with database backend
- [ ] Email notifications on lockout
- [ ] IP-based rate limiting
- [ ] Audit logging of login attempts
- [ ] Admin interface to unlock accounts
- [ ] Progressive lockout (increasing duration)
- [ ] CAPTCHA after failed attempts

## Troubleshooting

### Issue: Can't login with correct credentials
- Check if account is locked (wait for lockout duration)
- Verify .env file has correct credentials
- Ensure .env file is in the project root
- Check server logs for authentication errors

### Issue: .env not loading
- Verify file is named exactly `.env` (not `.env.txt`)
- Ensure file is in the same directory as the binary
- Check file permissions (should be readable)

### Issue: Lockout not expiring
- Verify system clock is correct
- Check `LOCKOUT_DURATION_MINUTES` value
- Restart server if configuration changed

## Support

For issues or questions:
1. Check the logs for detailed error messages
2. Verify .env configuration
3. Review this documentation
4. Check the code comments in `src/web/auth.rs`
