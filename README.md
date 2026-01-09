# 🎮 Tetris API

Classic NES Tetris as a REST API, written in Rust.

## Features

- **Classic NES Mechanics**: Authentic 10×20 board, original piece rotations
- **Original Scoring**: 40/100/300/1200 point system with level multiplier
- **RESTful API**: Create games, perform actions, check state
- **JWT Authentication**: Secure session-based access
- **Async Rust**: Built with Axum and Tokio

## Quick Start

```bash
# Clone and build
cargo build --release

# Run the server
cargo run

# Or with custom config
PORT=8080 JWT_SECRET=my-secret cargo run
```

## API Endpoints

| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| `POST` | `/api/v1/games` | ❌ | Create new game |
| `GET` | `/api/v1/games/{id}` | ✅ | Get game state |
| `POST` | `/api/v1/games/{id}/action` | ✅ | Perform action |
| `POST` | `/api/v1/games/{id}/tick` | ✅ | Gravity tick |
| `DELETE` | `/api/v1/games/{id}` | ✅ | End game |
| `GET` | `/api/v1/health` | ❌ | Health check |

## Example Usage

### Create a Game

```bash
curl -X POST http://localhost:3000/api/v1/games \
  -H "Content-Type: application/json" \
  -d '{"start_level": 0}'
```

Response:
```json
{
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "token": "eyJhbGciOiJIUzI1NiIs...",
  "state": {
    "board": [[0,0,0,...], ...],
    "current_piece": {"type": "T", "x": 3, "y": 0, "rotation": 0},
    "next_piece": "L",
    "score": 0,
    "level": 0,
    "lines": 0,
    "status": "playing"
  }
}
```

### Perform Actions

```bash
# Move left
curl -X POST http://localhost:3000/api/v1/games/{id}/action \
  -H "Authorization: Bearer {token}" \
  -H "Content-Type: application/json" \
  -d '{"action": "left"}'

# Rotate
curl -X POST http://localhost:3000/api/v1/games/{id}/action \
  -H "Authorization: Bearer {token}" \
  -H "Content-Type: application/json" \
  -d '{"action": "rotate"}'

# Hard drop
curl -X POST http://localhost:3000/api/v1/games/{id}/action \
  -H "Authorization: Bearer {token}" \
  -H "Content-Type: application/json" \
  -d '{"action": "drop"}'
```

### Gravity Tick

Call this periodically based on level speed:

```bash
curl -X POST http://localhost:3000/api/v1/games/{id}/tick \
  -H "Authorization: Bearer {token}"
```

## Available Actions

| Action | Description |
|--------|-------------|
| `left` | Move piece left |
| `right` | Move piece right |
| `down` | Soft drop (+1 point) |
| `rotate` | Rotate clockwise |
| `drop` | Hard drop & lock |

## Scoring (NES Original)

| Lines | Base Points |
|-------|-------------|
| Single (1) | 40 × (level + 1) |
| Double (2) | 100 × (level + 1) |
| Triple (3) | 300 × (level + 1) |
| Tetris (4) | 1200 × (level + 1) |

Soft drop: +1 point per cell

## Configuration

Environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `HOST` | `0.0.0.0` | Server host |
| `PORT` | `3000` | Server port |
| `JWT_SECRET` | (default) | JWT signing key |
| `TOKEN_DURATION_HOURS` | `24` | Token validity |
| `LOG_LEVEL` | `info` | Log verbosity |

## License

MIT
