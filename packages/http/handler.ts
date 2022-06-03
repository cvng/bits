import { log } from "bits/core/logger.ts";
import { actions, Context, events } from "bits/core/mod.ts";
import { initSdk } from "bits/sdk/mod.ts";
import { camelize } from "inflector/mod.ts";
import { Handler } from "std/http/server.ts";
import { Config } from "./config.ts";
import { BadPayload } from "./errors.ts";

type Action = keyof typeof actions;

type Event = keyof typeof events;

type Result = unknown;

function isAction(name: unknown): name is Action {
  return typeof name === "string" && name in actions;
}

function isEvent(name: unknown): name is Event {
  return typeof name === "string" && name in events;
}

function isActionPayload<T>(payload: unknown): payload is ActionPayload<T> {
  return typeof payload === "object" && payload !== null && "action" in payload;
}

function isEventPayload<T>(payload: unknown): payload is EventPayload<T> {
  return typeof payload === "object" && payload !== null && "event" in payload;
}

// https://hasura.io/docs/latest/graphql/core/actions/action-handlers
export interface ActionPayload<T> {
  request_query: string;
  session_variables: Record<string, string>;
  input: { input: T };
  action: { name: string };
}

// https://hasura.io/docs/latest/graphql/core/event-triggers/payload
export interface EventPayload<T> {
  event: { data: { new: T } };
  trigger: { name: string };
}

// TODO: https://hasura.io/docs/latest/graphql/core/actions/action-handlers/#adding-an-action-secret
export function createHandler(config: Config): Handler {
  const context: Context = {
    sdk: initSdk(config.graphql),
  };

  const handler: Handler = async (req: Request): Promise<Response> => {
    try {
      const payload = await req.json();
      log.debug(payload);

      if (isActionPayload(payload)) {
        const result = await handleAction(context, payload);
        log.debug(result);
        return new Response(JSON.stringify(result));
      }

      if (isEventPayload(payload)) {
        const result = await handleEvent(context, payload);
        log.debug(result);
        return new Response(JSON.stringify(result));
      }

      throw new BadPayload();
    } catch (err) {
      return new Response(err, { status: 400 });
    }
  };

  return handler;
}

function handleAction(
  cx: Context,
  payload: ActionPayload<unknown>,
): Promise<Result> {
  const action = camelize(payload.action.name);

  if (!isAction(action)) {
    throw new BadPayload();
  }

  const handler = actions[action];

  // Safety: Payload is auto-generated.
  // deno-lint-ignore no-explicit-any
  const params = payload.input.input as any;

  return handler(cx, params);
}

function handleEvent(
  cx: Context,
  payload: EventPayload<unknown>,
): Promise<Result> {
  const event = camelize(payload.trigger.name);

  if (!isEvent(event)) {
    throw new BadPayload();
  }

  const handler = events[event];

  // Safety: Payload is auto-generated.
  // deno-lint-ignore no-explicit-any
  const params = payload.event.data.new as any;

  return handler(cx, params);
}
