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
    {
    name: "Persistance",
    prodUrl: "http://142.93.169.63:8180/persistance/ping",
    devUrl: "http://localhost:8180/persistance/ping",
  },
];

export const POLL_INTERVAL = 5_000; // ms
