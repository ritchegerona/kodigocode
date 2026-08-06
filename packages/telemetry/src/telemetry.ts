export class Telemetry {
  log(event: string, data?: any) {
    // Stub – in real implementation send to analytics backend
    console.log(`[telemetry] ${event}`, data ?? {});
  }
}
