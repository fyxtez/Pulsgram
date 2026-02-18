import Indicator from "../../Indicator/Indicator";
import { Status } from "../../types";
import "./ServiceRow.css";

const STATUS_COLORS: Record<Status, string> = {
  [Status.UP]: "#22c55e",
  [Status.DOWN]: "#ef4444",
  [Status.PENDING]: "#eab308",
};

const STATUS_LABELS: Record<Status, string> = {
  [Status.UP]: "ONLINE",
  [Status.DOWN]: "OFFLINE",
  [Status.PENDING]: "CHECKING…",
};

interface ServiceRowProps {
  name: string;
  url: string;
  status: Status;
  latency: number | null;
}

export default function ServiceRow({ name, url, status, latency }: ServiceRowProps) {
  return (
    <div className="service-row">
      <Indicator status={status} />

      <div className="service-row__info">
        <div className="service-row__name">{name}</div>
        <div className="service-row__url" title={url}>
          {url}
        </div>
      </div>

      <div className="service-row__status-wrap">
        <div
          className="service-row__status-label"
          style={{ color: STATUS_COLORS[status] }}
        >
          {STATUS_LABELS[status]}
        </div>
        {latency !== null && status === Status.UP && (
          <div className="service-row__latency">{latency}ms</div>
        )}
      </div>
    </div>
  );
}
