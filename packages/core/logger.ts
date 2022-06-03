import { LogConfig, setup } from "std/log/mod.ts";

export * as log from "std/log/mod.ts";

const LOG_CONFIG: LogConfig = {
  loggers: {
    default: {
      level: "DEBUG",
      handlers: ["console"],
    },
    actions: {
      level: "DEBUG",
      handlers: ["console"],
    },
    events: {
      level: "DEBUG",
      handlers: ["console"],
    },
  },
};

// Custom configuration with 2 loggers (the default and `events` loggers).
export async function setupLogger(config: LogConfig): Promise<void> {
  await setup({ handlers: config.handlers, loggers: LOG_CONFIG.loggers });
}
