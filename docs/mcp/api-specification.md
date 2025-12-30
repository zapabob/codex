# MCP Dynamic Loading API Specification

## Overview

The MCP Dynamic Loading API provides REST endpoints to manage MCP servers at runtime.

## Base URL

```
http://localhost:{port}/api/mcp
```

Where `{port}` is configured via `mcp_dynamic_loading.api_server_port` in `config.toml`.

## Endpoints

### Add Server

**POST** `/api/mcp/servers`

Add a new MCP server dynamically.

**Request Body**:
```json
{
  "name": "my-server",
  "config": {
    "command": "node",
    "args": ["server.js"],
    "env": {
      "API_KEY": "value"
    },
    "enabled": true
  }
}
```

**Response**: `200 OK`
```json
{
  "server_name": "my-server"
}
```

### Remove Server

**DELETE** `/api/mcp/servers/{name}`

Remove an MCP server.

**Response**: `200 OK`

### Reload Server

**PUT** `/api/mcp/servers/{name}/reload`

Reload a server with new configuration.

**Request Body**:
```json
{
  "config": {
    "command": "node",
    "args": ["new-server.js"],
    "enabled": true
  }
}
```

**Response**: `200 OK`

### List Servers

**GET** `/api/mcp/servers`

List all dynamically loaded servers.

**Response**: `200 OK`
```json
{
  "servers": [
    {
      "name": "my-server",
      "status": "Running",
      "last_updated": "2025-12-30T12:00:00Z"
    }
  ]
}
```

### List Server Tools

**GET** `/api/mcp/servers/{name}/tools`

List tools available from a server.

**Response**: `200 OK`
```json
{
  "tools": [
    {
      "name": "tool-name",
      "description": "Tool description"
    }
  ]
}
```

## Error Responses

All endpoints return errors in the following format:

```json
{
  "error": "Error message"
}
```

**Status Codes**:
- `400 Bad Request` - Invalid request
- `404 Not Found` - Server not found
- `500 Internal Server Error` - Server error

## Authentication

Currently, the API server only accepts connections from localhost. Future versions may support API key authentication.

## Example Usage

### Add a Server

```bash
curl -X POST http://localhost:8080/api/mcp/servers \
  -H "Content-Type: application/json" \
  -d '{
    "name": "github",
    "config": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-github"],
      "env": {
        "GITHUB_PERSONAL_ACCESS_TOKEN": "${GITHUB_TOKEN}"
      },
      "enabled": true
    }
  }'
```

### List Servers

```bash
curl http://localhost:8080/api/mcp/servers
```

### Remove a Server

```bash
curl -X DELETE http://localhost:8080/api/mcp/servers/github
```
