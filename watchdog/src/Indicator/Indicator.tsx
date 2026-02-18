import { Status } from "../types";
import "./Indicator.css";

const STATUS_COLORS: Record<Status, string> = {
  [Status.UP]: "#22c55e",
  [Status.DOWN]: "#ef4444",
  [Status.PENDING]: "#eab308",
};

const STATUS_GLOWS: Record<Status, string> = {
  [Status.UP]: "0 0 8px 2px rgba(34,197,94,.45)",
  [Status.DOWN]: "0 0 8px 2px rgba(239,68,68,.45)",
  [Status.PENDING]: "0 0 8px 2px rgba(234,179,8,.35)",
};

interface IndicatorProps {
  status: Status;
}

export default function Indicator({ status }: IndicatorProps) {
  const color = STATUS_COLORS[status];

  return (
    <span
      className="indicator"
      style={{ backgroundColor: color, boxShadow: STATUS_GLOWS[status] }}
    >
      {status === Status.UP && (
        <span
          className="indicator__ring"
          style={{ borderColor: color }}
        />
      )}
    </span>
  );
}
