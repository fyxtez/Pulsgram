//@ts-ignore
export enum Status {
  UP = "up",
  DOWN = "down",
  PENDING = "pending",
}

export interface ServiceConfig {
  name: string;
  prodUrl: string;
  devUrl: string;
}

export interface ServiceEntry {
  name: string;
  url: string;
}

export interface ServiceStatus {
  status: Status;
  latency: number | null;
}