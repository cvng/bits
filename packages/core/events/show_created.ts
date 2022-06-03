import { Show } from "bits/sdk/mod.ts";
import { Context } from "../context.ts";

export type ShowCreated = Show;

export async function showCreated(
  _cx: Context,
  _event: ShowCreated,
): Promise<void> {
  await Promise.resolve(); // TODO
}
