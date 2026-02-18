interface ServiceConfig {
  name: string;
  prodUrl: string;
  devUrl: string;
}

export const SERVICES: ServiceConfig[] = [
  {
    name: "Core API",
    prodUrl: "http://142.93.169.63:8181/api/v1/ping",
    devUrl: "http://localhost:8181/api/v1/ping",
  },
  // ── add more services below ──
  // {
  //   name: "Auth Service",
  //   prodUrl: "http://142.93.169.63:8282/api/v1/ping",
  //   devUrl: "http://localhost:8282/api/v1/ping",
  // },
  // {
  //   name: "WebSocket Gateway",
  //   prodUrl: "http://142.93.169.63:8383/api/v1/ping",
  //   devUrl: "http://localhost:8383/api/v1/ping",
  // },
];

export const POLL_INTERVAL = 5_000; // ms
