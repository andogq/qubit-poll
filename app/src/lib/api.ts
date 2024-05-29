import { ws } from '@qubit-rs/client';
import type { Server } from '$lib/server';

const ws_origin = window.location.origin.replace('http', 'ws');
export default ws<Server>(new URL('/api', ws_origin).toString());
