export enum LogLevel {
  DEBUG = 'debug',
  INFO = 'info',
  WARN = 'warn',
  ERROR = 'error',
}

export class Logger {
  constructor(private readonly prefix = '[Kodigo]') {}

  private format(level: LogLevel, msg: string): string {
    const color = {
      [LogLevel.DEBUG]: '\x1b[34m', // blue
      [LogLevel.INFO]: '\x1b[32m', // green
      [LogLevel.WARN]: '\x1b[33m', // yellow
      [LogLevel.ERROR]: '\x1b[31m', // red
    }[level];
    const reset = '\x1b[0m';
    return `${color}${this.prefix} ${level}: ${msg}${reset}`;
  }

  debug(msg: string) { console.log(this.format(LogLevel.DEBUG, msg)); }
  info(msg: string) { console.log(this.format(LogLevel.INFO, msg)); }
  warn(msg: string) { console.warn(this.format(LogLevel.WARN, msg)); }
  error(msg: string) { console.error(this.format(LogLevel.ERROR, msg)); }
}
