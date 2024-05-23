import { ws } from '@qubit-rs/client';
import type { Server } from '$lib/server';

export default ws<Server>('ws://localhost:3030/api');

