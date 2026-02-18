import { useState, useEffect, useRef, useCallback } from "react";
import Column from "../Column/Column";
import "./Dashboard.css";
import { ping } from "../../ping";
import { POLL_INTERVAL, SERVICES } from "../../services";

//@ts-ignore
enum Status {
  UP = "up",
  DOWN = "down",
  PENDING = "pending",
}

interface ServiceEntry {
  name: string;
  url: string;
}

interface ServiceStatus {
  status: Status;
  latency: number | null;
}


export default function Dashboard() {
  const prodServices: ServiceEntry[] = SERVICES.map((s) => ({
    name: s.name,
    url: s.prodUrl,
  }));
  const devServices: ServiceEntry[] = SERVICES.map((s) => ({
    name: s.name,
    url: s.devUrl,
  }));

  const [prodStatuses, setProdStatuses] = useState<ServiceStatus[]>(() =>
    prodServices.map(() => ({ status: Status.PENDING, latency: null }))
  );
  const [devStatuses, setDevStatuses] = useState<ServiceStatus[]>(() =>
    devServices.map(() => ({ status: Status.PENDING, latency: null }))
  );
  const [lastChecked, setLastChecked] = useState<Date | null>(null);
  const [countdown, setCountdown] = useState(POLL_INTERVAL / 1000);

  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null);
  const countdownRef = useRef<ReturnType<typeof setInterval> | null>(null);

  const runChecks = useCallback(async () => {
    const check = async (list: ServiceEntry[]): Promise<ServiceStatus[]> =>
      Promise.all(
        list.map(async (svc) => {
          const start = performance.now();
          const status = await ping(svc.url);
          const latency = Math.round(performance.now() - start);
          return { status, latency };
        })
      );

    const [pResults, dResults] = await Promise.all([
      check(prodServices),
      check(devServices),
    ]);
    setProdStatuses(pResults);
    setDevStatuses(dResults);
    setLastChecked(new Date());
    setCountdown(POLL_INTERVAL / 1000);
  }, []);

  useEffect(() => {
    runChecks();
    intervalRef.current = setInterval(runChecks, POLL_INTERVAL);
    countdownRef.current = setInterval(() => {
      setCountdown((c) => (c <= 1 ? POLL_INTERVAL / 1000 : c - 1));
    }, 1000);
    return () => {
      if (intervalRef.current) clearInterval(intervalRef.current);
      if (countdownRef.current) clearInterval(countdownRef.current);
    };
  }, [runChecks]);

  const allStatuses = [...prodStatuses, ...devStatuses];
  const upCount = allStatuses.filter((s) => s.status === Status.UP).length;
  const downCount = allStatuses.filter((s) => s.status === Status.DOWN).length;
  const total = allStatuses.length;

  return (
    <div className="dashboard">
      {/* Header */}
      <div className="dashboard__header">
        <div>
          <h1 className="dashboard__title">
            <span className="dashboard__title-icon">⬡</span>
            Service Monitor
          </h1>
          <p className="dashboard__subtitle">
            Polling every {POLL_INTERVAL / 1000}s
            {lastChecked && (
              <>
                {" · "}Last checked {lastChecked.toLocaleTimeString()}
                {" · "}Next in {countdown}s
              </>
            )}
          </p>
        </div>

        {/* Summary pills */}
        <div className="dashboard__pills">
          <span className="dashboard__pill dashboard__pill--up">
            <span className="dashboard__pill-dot dashboard__pill-dot--up" />
            {upCount}/{total}
          </span>
          {downCount > 0 && (
            <span className="dashboard__pill dashboard__pill--down">
              <span className="dashboard__pill-dot dashboard__pill-dot--down" />
              {downCount} down
            </span>
          )}
        </div>
      </div>

      {/* Columns */}
      <div className="dashboard__columns">
        <Column
          title="Production"
          tag="PROD"
          services={prodServices}
          statuses={prodStatuses}
        />
        <Column
          title="Development"
          tag="DEV"
          services={devServices}
          statuses={devStatuses}
        />
      </div>

      {/* Footer */}
      <div className="dashboard__footer">
        Add services by editing the SERVICES array in src/config/services.ts
      </div>
    </div>
  );
}
