export function formatTimeForResolution(timestamp: Date | string, resolution: string): string {
  const d = typeof timestamp === "string" ? new Date(timestamp) : timestamp;
  switch (resolution) {
    case "1m":
      return d.toISOString();
    case "5m":
      return d.toISOString();
    case "1h":
      return d.toISOString();
    case "1d":
      return d.toISOString().split("T")[0] ?? "";
    default:
      return d.toISOString();
  }
}
