import { CreateShowInput, CreateShowPayload } from "bits/sdk/sdk.ts";
import { Context } from "../context.ts";

export async function createShow(
  cx: Context,
  input: CreateShowInput,
): Promise<CreateShowPayload> {
  const { show } = await cx.sdk.createShow(input);

  if (!show) {
    throw new Error(); // TODO: specify error
  }

  const payload: CreateShowPayload = {
    id: show.id,
  };

  return payload;
}
