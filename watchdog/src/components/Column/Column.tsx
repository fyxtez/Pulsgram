import { Status, type ServiceEntry, type ServiceStatus } from "../../types";
import ServiceRow from "../ServicesRow/ServiceRow";
import "./Column.css";

type Tag = "PROD" | "DEV";

const TAG_COLORS: Record<Tag, { bg: string; text: string; border: string }> = {
  PROD: { bg: "rgba(239,68,68,.12)", text: "#ef4444", border: "rgba(239,68,68,.25)" },
  DEV: { bg: "rgba(34,197,94,.12)", text: "#22c55e", border: "rgba(34,197,94,.25)" },
};

interface ColumnProps {
  title: string;
  tag: Tag;
  services: ServiceEntry[];
  statuses: ServiceStatus[];
}

export default function Column({ title, tag, services, statuses }: ColumnProps) {
  const t = TAG_COLORS[tag];

  return (
    <div className="column">
      <div className="column__header">
        <h2 className="column__title">{title}</h2>
        <span
          className="column__tag"
          style={{
            background: t.bg,
            color: t.text,
            borderColor: t.border,
          }}
        >
          {tag}
        </span>
      </div>

      <div className="column__list">
        {services.map((svc, i) => (
          <ServiceRow
            key={svc.name + svc.url}
            name={svc.name}
            url={svc.url}
            status={statuses[i]?.status ?? Status.PENDING}
            latency={statuses[i]?.latency ?? null}
          />
        ))}
        {services.length === 0 && (
          <div className="column__empty">No services configured.</div>
        )}
      </div>
    </div>
  );
}
