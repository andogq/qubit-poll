import type { Stream } from "@qubit-rs/client";
export type PollOverview = { id: string, name: string, description: string, options: Array<string>, };

export type Server = { get_summaries: () => Promise<Array<PollOverview>>, create: (name: string, description: string, private: boolean, options: Array<string>) => Promise<string>, get_summary: (id: string) => Promise<PollOverview | null>, vote: (poll: string, option: number) => Promise<null>, stream: { poll: (poll_id: string) => Stream<Array<number>>, poll_total: () => Stream<{ [key: string]: number }>, overview: () => Stream<{ [key: string]: PollOverview }> } };