import { Status } from "./types";

export async function ping(url: string): Promise<Status> {
  try {
    const controller = new AbortController();
    const timeout = setTimeout(() => controller.abort(), 5000);
    const res = await fetch(url, {
      method: "GET",
      mode: "cors",
      signal: controller.signal,
    });
    clearTimeout(timeout);
    if (!res.ok) return Status.DOWN;
    const body = await res.text();
    return body.toLowerCase().includes("pong") ? Status.UP : Status.DOWN;
  } catch {
    return Status.DOWN;
  }
}
