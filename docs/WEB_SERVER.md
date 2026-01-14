# JobWeaver Web Server

## Overview

JobWeaver includes a built-in web server that provides an enterprise-grade web interface for exploring and analyzing Control-M jobs stored in SQLite database.

## Features

### 🔐 Authentication
- **Local Authentication**: Username and password login
- **Entra ID (Azure AD)**: Microsoft enterprise authentication support
- **JWT Tokens**: Secure session management with JSON Web Tokens
- **Session Management**: Persistent login sessions

### 🔍 Advanced Search & Filtering
- **Multi-criteria Search**: Filter by job name, folder, application, task type, and critical status
- **Dropdown Filters**: Pre-populated filter options from database
- **Real-time Search**: Instant results as you type
- **Smart Filtering**: Combine multiple filters for precise results

### 📊 Enterprise-Grade Table
- **Sortable Columns**: Click column headers to sort ascending/descending
- **Pagination**: Configurable records per page (25, 50, 100, 200)
- **Drill-down Details**: Click any job to view complete details
- **Responsive Design**: Works on desktop, tablet, and mobile
- **Fast Performance**: Optimized queries with proper indexing

### 📈 Interactive Dashboard
- **Key Metrics**: Total jobs, folders, critical jobs, cyclic jobs
- **Job Type Analysis**: File transfer jobs, CLI jobs breakdown
- **Visual Charts**: Bar charts for applications, folders, task types
- **Complexity Distribution**: Low, medium, high complexity visualization
- **Real-time Updates**: Fresh data on every page load

### 🎨 Modern UI/UX
- **Enterprise-Grade Design**: Professional, clean interface
- **Responsive Layout**: Adapts to any screen size
- **Smooth Animations**: Polished transitions and interactions
- **Intuitive Navigation**: Easy-to-use sidebar navigation
- **Accessibility**: WCAG compliant design

## Getting Started

### Prerequisites

1. **SQLite Database**: Export your Control-M XML to SQLite first
   ```bash
   jobweaver export-sqlite -i controlm.xml -o controlm.db
   ```

2. **Database File**: Ensure the SQLite database file exists and contains data

### Starting the Web Server

#### Basic Usage
```bash
jobweaver serve -d controlm.db
```

#### Custom Port
```bash
jobweaver serve -d controlm.db -p 3000
```

#### Custom Host
```bash
jobweaver serve -d controlm.db --host 0.0.0.0 -p 8080
```

#### All Options
```bash
jobweaver serve \
  --database controlm.db \
  --port 8080 \
  --host 127.0.0.1
```

### Accessing the Web Interface

Once started, open your browser and navigate to:
```
http://localhost:8080
```

Default credentials:
- **Username**: `admin`
- **Password**: `admin`

⚠️ **Security Note**: Change the default password in production!

## Configuration

### Environment Variables

You can configure the web server using environment variables:

```bash
export JOBWEAVER_JWT_SECRET="your-secret-key-here"
export JOBWEAVER_SESSION_KEY="your-session-key-here"
export JOBWEAVER_DB_PATH="controlm.db"
export JOBWEAVER_PORT="8080"
export JOBWEAVER_HOST="127.0.0.1"
```

### Entra ID Configuration

To enable Microsoft Entra ID authentication:

1. Register an application in Azure AD
2. Configure redirect URI: `http://localhost:8080/api/auth/entra-callback`
3. Set environment variables:

```bash
export JOBWEAVER_ENABLE_ENTRA_ID="true"
export JOBWEAVER_ENTRA_CLIENT_ID="your-client-id"
export JOBWEAVER_ENTRA_CLIENT_SECRET="your-client-secret"
export JOBWEAVER_ENTRA_TENANT_ID="your-tenant-id"
```

## API Endpoints

### Authentication

#### POST `/api/auth/login`
Login with username and password

**Request:**
```json
{
  "username": "admin",
  "password": "admin"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIs...",
    "user": {
      "username": "admin",
      "display_name": "admin",
      "auth_type": "local"
    }
  }
}
```

#### POST `/api/auth/entra-callback`
Complete Entra ID authentication

**Request:**
```json
{
  "code": "authorization-code-from-microsoft"
}
```

#### GET `/api/auth/me`
Get current user information (requires authentication)

**Headers:**
```
Authorization: Bearer <token>
```

### Jobs

#### GET `/api/jobs/search`
Search jobs with filters

**Query Parameters:**
- `job_name` (optional): Filter by job name (partial match)
- `folder_name` (optional): Filter by folder name
- `application` (optional): Filter by application
- `task_type` (optional): Filter by task type
- `critical` (optional): Filter by critical status (true/false)
- `page` (optional): Page number (default: 1)
- `per_page` (optional): Records per page (default: 50)
- `sort_by` (optional): Sort column (default: job_name)
- `sort_order` (optional): Sort order (asc/desc, default: asc)

**Example:**
```
GET /api/jobs/search?job_name=BATCH&critical=true&page=1&per_page=50&sort_by=job_name&sort_order=asc
```

**Response:**
```json
{
  "success": true,
  "data": {
    "jobs": [
      {
        "id": 1,
        "job_name": "BATCH_001",
        "folder_name": "PROD",
        "application": "Finance",
        "task_type": "Command",
        "critical": true,
        "in_conditions_count": 5,
        "out_conditions_count": 3,
        "control_resources_count": 2,
        "variables_count": 10
      }
    ],
    "total": 150,
    "page": 1,
    "per_page": 50,
    "total_pages": 3
  }
}
```

#### GET `/api/jobs/{id}`
Get detailed job information

**Response:**
```json
{
  "success": true,
  "data": {
    "job": {
      "id": 1,
      "job_name": "BATCH_001",
      "folder_name": "PROD",
      "application": "Finance",
      "description": "Daily batch processing",
      "cmdline": "/scripts/batch.sh",
      "critical": true
    },
    "scheduling": {
      "time_from": "00:00",
      "time_to": "06:00"
    },
    "in_conditions": [...],
    "out_conditions": [...],
    "variables": [...],
    "control_resources": [...]
  }
}
```

### Dashboard

#### GET `/api/dashboard/stats`
Get dashboard statistics

**Response:**
```json
{
  "success": true,
  "data": {
    "total_jobs": 1500,
    "total_folders": 50,
    "critical_jobs": 200,
    "cyclic_jobs": 100,
    "file_transfer_jobs": 300,
    "cli_jobs": 800,
    "jobs_by_application": [
      {"application": "Finance", "count": 500},
      {"application": "HR", "count": 300}
    ],
    "jobs_by_folder": [...],
    "jobs_by_task_type": [...],
    "complexity_distribution": {
      "low": 500,
      "medium": 700,
      "high": 300
    }
  }
}
```

### Filters

#### GET `/api/filters`
Get available filter options

**Response:**
```json
{
  "success": true,
  "data": {
    "applications": ["Finance", "HR", "IT"],
    "folders": ["PROD", "DEV", "TEST"],
    "task_types": ["Command", "Script", "FileTransfer"],
    "owners": ["admin", "user1", "user2"]
  }
}
```

## Architecture

### Backend (Rust)

```
src/web/
├── mod.rs              # Module exports
├── config.rs           # Configuration structures
├── auth.rs             # Authentication & JWT
├── models.rs           # Request/Response models
├── repository.rs       # Database queries
├── handlers.rs         # API endpoint handlers
└── server.rs           # Actix-web server setup
```

### Frontend (SPA)

```
static/
├── index.html          # Main HTML file
├── css/
│   └── styles.css      # Enterprise-grade styling
└── js/
    └── app.js          # Single-page application logic
```

### Technology Stack

**Backend:**
- **actix-web**: High-performance web framework
- **rusqlite**: SQLite database access
- **jsonwebtoken**: JWT authentication
- **bcrypt**: Password hashing
- **oauth2**: Entra ID integration

**Frontend:**
- **Vanilla JavaScript**: No framework dependencies
- **Modern CSS**: Flexbox, Grid, CSS Variables
- **Font Awesome**: Professional icons
- **Inter Font**: Clean, readable typography

## Performance

### Database Optimization
- **Indexed Queries**: All search columns are indexed
- **Connection Pooling**: Efficient database connections
- **Prepared Statements**: Cached query plans
- **Pagination**: Efficient large dataset handling

### Frontend Optimization
- **Lazy Loading**: Load data on demand
- **Debounced Search**: Reduce API calls
- **Cached Filters**: Minimize database queries
- **Optimized Rendering**: Fast DOM updates

### Expected Performance
- **Search Response**: < 100ms for 10,000 jobs
- **Dashboard Load**: < 200ms
- **Job Detail**: < 50ms
- **Concurrent Users**: 100+ simultaneous users

## Security

### Authentication
- **JWT Tokens**: Secure, stateless authentication
- **Password Hashing**: bcrypt with salt
- **Token Expiration**: 24-hour token lifetime
- **HTTPS Ready**: Production-ready SSL support

### Authorization
- **Bearer Token**: All API endpoints require authentication
- **Token Validation**: Every request validates JWT
- **Session Management**: Secure cookie handling

### Best Practices
1. **Change Default Credentials**: Update admin password
2. **Use HTTPS**: Enable SSL in production
3. **Rotate JWT Secret**: Change secret keys regularly
4. **Enable CORS**: Configure allowed origins
5. **Rate Limiting**: Add rate limiting middleware (future)

## Troubleshooting

### Server Won't Start

**Problem**: Port already in use
```
Error: Address already in use
```

**Solution**: Use a different port
```bash
jobweaver serve -d controlm.db -p 3000
```

### Database Not Found

**Problem**: SQLite file doesn't exist
```
Error: Failed to open database
```

**Solution**: Export XML to SQLite first
```bash
jobweaver export-sqlite -i controlm.xml -o controlm.db
```

### Authentication Failed

**Problem**: Invalid credentials
```
Error: Invalid username or password
```

**Solution**: Use default credentials (admin/admin) or check user store

### No Data in Dashboard

**Problem**: Empty database
```
Dashboard shows 0 jobs
```

**Solution**: Ensure database has data
```bash
sqlite3 controlm.db "SELECT COUNT(*) FROM jobs;"
```

## Deployment

### Development
```bash
# Start with default settings
jobweaver serve -d controlm.db

# Access at http://localhost:8080
```

### Production

#### Using systemd (Linux)

Create `/etc/systemd/system/jobweaver.service`:
```ini
[Unit]
Description=JobWeaver Web Server
After=network.target

[Service]
Type=simple
User=jobweaver
WorkingDirectory=/opt/jobweaver
ExecStart=/opt/jobweaver/jobweaver serve -d /data/controlm.db -p 8080 --host 0.0.0.0
Restart=always
Environment="JOBWEAVER_JWT_SECRET=your-production-secret"

[Install]
WantedBy=multi-user.target
```

Start service:
```bash
sudo systemctl enable jobweaver
sudo systemctl start jobweaver
```

#### Using Docker

Create `Dockerfile`:
```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libsqlite3-0
COPY --from=builder /app/target/release/jobweaver /usr/local/bin/
COPY --from=builder /app/static /app/static
WORKDIR /app
EXPOSE 8080
CMD ["jobweaver", "serve", "-d", "/data/controlm.db", "-p", "8080", "--host", "0.0.0.0"]
```

Build and run:
```bash
docker build -t jobweaver .
docker run -p 8080:8080 -v /path/to/data:/data jobweaver
```

#### Reverse Proxy (nginx)

```nginx
server {
    listen 80;
    server_name jobweaver.example.com;

    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

## Future Enhancements

### Planned Features
- [ ] User management UI
- [ ] Role-based access control
- [ ] Export search results to CSV
- [ ] Job execution history
- [ ] Real-time job monitoring
- [ ] Dependency graph visualization
- [ ] Advanced analytics and reports
- [ ] Email notifications
- [ ] API rate limiting
- [ ] Audit logging

### Community Contributions
We welcome contributions! See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

## Support

### Documentation
- [SQLite Schema](SQLITE_SCHEMA.md)
- [Performance Guide](PERFORMANCE.md)
- [API Reference](WEB_SERVER.md#api-endpoints)

### Issues
Report bugs and feature requests on GitHub Issues.

### Community
Join our discussions on GitHub Discussions.

---

**Version:** 1.0.0  
**Last Updated:** 2026-01-13  
**License:** MIT
