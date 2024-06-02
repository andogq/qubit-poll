import { ws } from '@qubit-rs/client';
import type { QubitServer } from '$lib/bindings';

const ws_origin = window.location.origin.replace('http', 'ws');
export default ws<QubitServer>(new URL('/_/api', ws_origin).toString());
