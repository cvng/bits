import { Events_Show_Created } from "bits/sdk/mod.ts";
import { Context } from "./context.ts";
import * as handlers from "./events/mod.ts";

export class ShowCreated {
  type = "show_created";
  payload: Events_Show_Created;

  constructor(payload: Events_Show_Created) {
    this.payload = payload;
  }
}

export type Event = ShowCreated | ShowCreated;

export async function dispatch(
  cx: Context,
  events: Array<Event>,
): Promise<void> {
  await Promise.all(
    events.map((event) => {
      // TODO: await cx.sdk.persistEvent(event);

      // @ts-ignore: No index signature with a parameter of type 'string'.
      const handler = handlers[event.type];

      return handler(cx, event);
    }),
  );
}
