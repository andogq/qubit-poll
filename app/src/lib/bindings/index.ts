/*    @@@@@@@@@@@@@ & ###############
   @@@@@@@@@@@@@@ &&& ###############
 @@@@@@@@@@@@@@ &&&&& ###############
############### &&&&& ###############
######## Generated by Qubit! ########
############### &&&&& ###############
############### &&&&& @@@@@@@@@@@@@@
############### && @@@@@@@@@@@@@@
############### & @@@@@@@@@@@@@    */

import type { PollOverview } from "./PollOverview.ts";
import type { Query } from "@qubit-rs/client";
import type { Mutation } from "@qubit-rs/client";
import type { Subscription } from "@qubit-rs/client";
import type { StreamHandler } from "@qubit-rs/client";
import type { StreamUnsubscribe } from "@qubit-rs/client";

export type QubitServer = { get_summaries: Query<() => Promise<Array<PollOverview>>>, create: Mutation<(name: string, description: string, is_private: boolean, options: Array<string>, ) => Promise<string>>, get_summary: Query<(id: string, ) => Promise<PollOverview | null>>, vote: Mutation<(poll: string, option: number, ) => Promise<null>>, stream: { poll: Subscription<(poll_id: string,  handler: StreamHandler<Array<number>>) => StreamUnsubscribe>, poll_total: Subscription<( handler: StreamHandler<{ [key: string]: number }>) => StreamUnsubscribe>, overview: Subscription<( handler: StreamHandler<{ [key: string]: PollOverview }>) => StreamUnsubscribe> } };