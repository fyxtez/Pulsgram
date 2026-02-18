# Service Monitor Dashboard

A real-time service health monitoring dashboard built with React, TypeScript, and Vite. Polls your production and development endpoints side-by-side and displays their status with latency.

## Features

- Live polling with configurable interval (default: 5s)
- Production vs Development columns for easy comparison
- Visual status indicators with animated pulse for online services
- Latency display for healthy endpoints
- Summary pills showing overall up/down counts

## Getting Started

```bash
npm install
npm run dev
```

## Adding Services

Edit `services.ts` and add entries to the `SERVICES` array:

Each service expects its ping endpoint to return a response containing `"pong"` to be considered online.

## Configuration

| Option          | Location       | Default  |
|-----------------|---------------------------|
| `SERVICES`      | `services.ts`  | —        |
| `POLL_INTERVAL` | `services.ts`  | `5000`ms |
| Ping timeout    | `ping.ts`      | `5000`ms |

## Tech Stack

- [React](https://react.dev/) + [TypeScript](https://www.typescriptlang.org/)
- [Vite](https://vite.dev/)