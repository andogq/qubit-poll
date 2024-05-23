import type { Stream } from "@qubit-rs/client";
export type Poll = { id: number, name: string, description: string, options: Array<string>, };
export type Server = { hello_world: () => Promise<string>, polls: { list: () => Promise<Array<Poll>>, create: (name: string, description: string, options: Array<string>) => Promise<null>, get: (id: number) => Promise<Poll | null> } };