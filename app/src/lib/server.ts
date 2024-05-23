import type { Stream } from "@qubit-rs/client";
export type PollOverview = { id: number, name: string, description: string, options: Array<string>, };
export type Server = { get_summaries: () => Promise<Array<PollOverview>>, create: (name: string, description: string, options: Array<string>) => Promise<null>, get_summary: (id: number) => Promise<PollOverview | null>, vote: (poll: number, option: number) => Promise<null>, hello_world: () => Promise<string>, stream: { poll: (poll_id: number) => Stream<Array<number>>, poll_total: () => Stream<Array<number>>, overview: () => Stream<Array<PollOverview>> } };