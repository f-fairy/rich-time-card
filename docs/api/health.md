# Health Check API

## Overview

API server health check endpoint.

This API is used to confirm that the Rich Time Card backend server is running normally.

---

## Endpoint

```http
GET /api/health
```

---

## Authentication

Not required.

---

## Request

No request body.

---

## Response

### Success Response

Status Code:

```http
200 OK
```

Response Body:

```json
{
  "status": "ok"
}
```

---

## Purpose

- Confirm backend server is running
- Health check for browser, frontend, and widget
- Deployment verification
