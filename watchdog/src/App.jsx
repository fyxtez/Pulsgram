import { useState, useEffect, useRef, useCallback } from "react";

/*
 * ── CONFIG ──────────────────────────────────────────────────────
 * Add new services here. Each entry gets a row in both columns.
 * `prodUrl` → left column, `devUrl` → right column.
 */
const SERVICES = [
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

const POLL_INTERVAL = 5_000; // ms

/* ── status helpers ── */
const STATUS = {
  UP: "up",
  DOWN: "down",
  PENDING: "pending",
};

const statusColor = {
  [STATUS.UP]: "#22c55e",
  [STATUS.DOWN]: "#ef4444",
  [STATUS.PENDING]: "#eab308",
};

const statusGlow = {
  [STATUS.UP]: "0 0 8px 2px rgba(34,197,94,.45)",
  [STATUS.DOWN]: "0 0 8px 2px rgba(239,68,68,.45)",
  [STATUS.PENDING]: "0 0 8px 2px rgba(234,179,8,.35)",
};

const statusLabel = {
  [STATUS.UP]: "ONLINE",
  [STATUS.DOWN]: "OFFLINE",
  [STATUS.PENDING]: "CHECKING…",
};

/* ── ping logic ── */
async function ping(url) {
  try {
    const controller = new AbortController();
    const timeout = setTimeout(() => controller.abort(), 5000);
    const res = await fetch(url, {
      method: "GET",
      mode: "cors",
      signal: controller.signal,
    });
    clearTimeout(timeout);
    if (!res.ok) return STATUS.DOWN;
    const body = await res.text();
    return body.toLowerCase().includes("pong") ? STATUS.UP : STATUS.DOWN;
  } catch {
    return STATUS.DOWN;
  }
}

function Indicator({ status }) {
  const color = statusColor[status];
  const glow = statusGlow[status];
  const pulse = status === STATUS.UP;

  return (
    <span
      style={{
        position: "relative",
        display: "inline-block",
        width: 12,
        height: 12,
        borderRadius: "50%",
        backgroundColor: color,
        boxShadow: glow,
        flexShrink: 0,
      }}
    >
      {pulse && (
        <span
          style={{
            position: "absolute",
            inset: -3,
            borderRadius: "50%",
            border: `2px solid ${color}`,
            animation: "ping-ring 2s cubic-bezier(0,0,.2,1) infinite",
          }}
        />
      )}
    </span>
  );
}

function ServiceRow({ name, url, status, latency }) {
  return (
    <div
      style={{
        display: "flex",
        alignItems: "center",
        gap: 14,
        padding: "14px 18px",
        background: "#1a1a1a",
        borderRadius: 8,
        border: "1px solid #2a2a2a",
        transition: "border-color .2s",
      }}
      onMouseEnter={(e) => (e.currentTarget.style.borderColor = "#3a3a3a")}
      onMouseLeave={(e) => (e.currentTarget.style.borderColor = "#2a2a2a")}
    >
      <Indicator status={status} />
      <div style={{ flex: 1, minWidth: 0 }}>
        <div
          style={{
            fontSize: 14,
            fontWeight: 600,
            color: "#e5e5e5",
            letterSpacing: ".02em",
          }}
        >
          {name}
        </div>
        <div
          style={{
            fontSize: 11,
            color: "#666",
            marginTop: 2,
            overflow: "hidden",
            textOverflow: "ellipsis",
            whiteSpace: "nowrap",
            fontFamily: "'JetBrains Mono', 'Fira Code', monospace",
          }}
          title={url}
        >
          {url}
        </div>
      </div>
      <div style={{ textAlign: "right", flexShrink: 0 }}>
        <div
          style={{
            fontSize: 11,
            fontWeight: 700,
            color: statusColor[status],
            letterSpacing: ".08em",
            fontFamily: "'JetBrains Mono', 'Fira Code', monospace",
          }}
        >
          {statusLabel[status]}
        </div>
        {latency !== null && status === STATUS.UP && (
          <div
            style={{
              fontSize: 10,
              color: "#555",
              marginTop: 2,
              fontFamily: "'JetBrains Mono', 'Fira Code', monospace",
            }}
          >
            {latency}ms
          </div>
        )}
      </div>
    </div>
  );
}

function Column({ title, tag, services, statuses }) {
  const tagColors = {
    PROD: { bg: "rgba(239,68,68,.12)", text: "#ef4444", border: "rgba(239,68,68,.25)" },
    DEV: { bg: "rgba(34,197,94,.12)", text: "#22c55e", border: "rgba(34,197,94,.25)" },
  };
  const t = tagColors[tag] || tagColors.DEV;

  return (
    <div style={{ flex: 1, minWidth: 280 }}>
      <div
        style={{
          display: "flex",
          alignItems: "center",
          gap: 10,
          marginBottom: 20,
          paddingBottom: 14,
          borderBottom: "1px solid #222",
        }}
      >
        <h2
          style={{
            margin: 0,
            fontSize: 16,
            fontWeight: 700,
            color: "#ccc",
            letterSpacing: ".04em",
            textTransform: "uppercase",
          }}
        >
          {title}
        </h2>
        <span
          style={{
            fontSize: 10,
            fontWeight: 700,
            padding: "3px 8px",
            borderRadius: 4,
            background: t.bg,
            color: t.text,
            border: `1px solid ${t.border}`,
            letterSpacing: ".06em",
          }}
        >
          {tag}
        </span>
      </div>
      <div style={{ display: "flex", flexDirection: "column", gap: 10 }}>
        {services.map((svc, i) => (
          <ServiceRow
            key={svc.name + svc.url}
            name={svc.name}
            url={svc.url}
            status={statuses[i]?.status ?? STATUS.PENDING}
            latency={statuses[i]?.latency ?? null}
          />
        ))}
        {services.length === 0 && (
          <div style={{ color: "#444", fontSize: 13, padding: 18 }}>
            No services configured.
          </div>
        )}
      </div>
    </div>
  );
}

/* ── main dashboard ── */
export default function Dashboard() {
  const prodServices = SERVICES.map((s) => ({ name: s.name, url: s.prodUrl }));
  const devServices = SERVICES.map((s) => ({ name: s.name, url: s.devUrl }));

  const [prodStatuses, setProdStatuses] = useState(
    () => prodServices.map(() => ({ status: STATUS.PENDING, latency: null }))
  );
  const [devStatuses, setDevStatuses] = useState(
    () => devServices.map(() => ({ status: STATUS.PENDING, latency: null }))
  );
  const [lastChecked, setLastChecked] = useState(null);
  const [countdown, setCountdown] = useState(POLL_INTERVAL / 1000);
  const intervalRef = useRef(null);
  const countdownRef = useRef(null);

  const runChecks = useCallback(async () => {
    const check = async (list) =>
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
      clearInterval(intervalRef.current);
      clearInterval(countdownRef.current);
    };
  }, [runChecks]);

  const allStatuses = [...prodStatuses, ...devStatuses];
  const upCount = allStatuses.filter((s) => s.status === STATUS.UP).length;
  const downCount = allStatuses.filter((s) => s.status === STATUS.DOWN).length;
  const total = allStatuses.length;

  return (
    <div
      style={{
        minHeight: "100vh",
        background: "#111",
        color: "#e5e5e5",
        fontFamily:
          "'IBM Plex Sans', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif",
        padding: "32px 24px",
      }}
    >
      <style>{`
        @import url('https://fonts.googleapis.com/css2?family=IBM+Plex+Sans:wght@400;600;700&family=JetBrains+Mono:wght@400;700&display=swap');
        @keyframes ping-ring {
          0% { transform: scale(1); opacity: .6; }
          75%, 100% { transform: scale(1.8); opacity: 0; }
        }
        * { box-sizing: border-box; }
      `}</style>

      {/* Header */}
      <div
        style={{
          maxWidth: 960,
          margin: "0 auto 32px",
          display: "flex",
          alignItems: "flex-end",
          justifyContent: "space-between",
          flexWrap: "wrap",
          gap: 16,
        }}
      >
        <div>
          <h1
            style={{
              margin: 0,
              fontSize: 22,
              fontWeight: 700,
              color: "#f5f5f5",
              letterSpacing: ".03em",
            }}
          >
            <span style={{ color: "#555", marginRight: 8 }}>⬡</span>
            Service Monitor
          </h1>
          <p style={{ margin: "6px 0 0", fontSize: 12, color: "#555" }}>
            Polling every {POLL_INTERVAL / 1000}s
            {lastChecked && (
              <>
                {" · "}Last checked{" "}
                {lastChecked.toLocaleTimeString()}
                {" · "}Next in {countdown}s
              </>
            )}
          </p>
        </div>

        {/* Summary pills */}
        <div style={{ display: "flex", gap: 8 }}>
          <span
            style={{
              display: "flex",
              alignItems: "center",
              gap: 6,
              padding: "6px 12px",
              borderRadius: 6,
              fontSize: 12,
              fontWeight: 700,
              fontFamily: "'JetBrains Mono', monospace",
              background: "rgba(34,197,94,.08)",
              color: "#22c55e",
              border: "1px solid rgba(34,197,94,.2)",
            }}
          >
            <span
              style={{
                width: 7,
                height: 7,
                borderRadius: "50%",
                background: "#22c55e",
              }}
            />
            {upCount}/{total}
          </span>
          {downCount > 0 && (
            <span
              style={{
                display: "flex",
                alignItems: "center",
                gap: 6,
                padding: "6px 12px",
                borderRadius: 6,
                fontSize: 12,
                fontWeight: 700,
                fontFamily: "'JetBrains Mono', monospace",
                background: "rgba(239,68,68,.08)",
                color: "#ef4444",
                border: "1px solid rgba(239,68,68,.2)",
              }}
            >
              <span
                style={{
                  width: 7,
                  height: 7,
                  borderRadius: "50%",
                  background: "#ef4444",
                }}
              />
              {downCount} down
            </span>
          )}
        </div>
      </div>

      {/* Columns */}
      <div
        style={{
          maxWidth: 960,
          margin: "0 auto",
          display: "flex",
          gap: 32,
          flexWrap: "wrap",
        }}
      >
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

      {/* Footer hint */}
      <div
        style={{
          maxWidth: 960,
          margin: "40px auto 0",
          padding: "16px 0",
          borderTop: "1px solid #1a1a1a",
          fontSize: 11,
          color: "#333",
          fontFamily: "'JetBrains Mono', monospace",
        }}
      >
        Add services by editing the SERVICES array at the top of the file.
      </div>
    </div>
  );
}