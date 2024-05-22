import type { Stream } from "@qubit-rs/client";

export type Server = { hello_world: () => Promise<string>, polls: { list: () => Promise<Array<string>>, create: (name: string) => Promise<null> } };