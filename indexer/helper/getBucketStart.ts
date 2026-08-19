export function getBucketStart(date: Date, resolution: string): Date {
  const d = new Date(date);
  switch (resolution) {
    case "1m":
      d.setSeconds(0, 0);
      break;
    case "5m":
      d.setMinutes(Math.floor(d.getMinutes() / 5) * 5, 0, 0);
      break;
    case "1h":
      d.setMinutes(0, 0, 0);
      break;
    case "1d":
      d.setHours(0, 0, 0, 0);
      break;
    default:
      d.setMinutes(0, 0, 0);
  }
  return d;
}
