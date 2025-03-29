// Import your event types if they're defined elsewhere
//@ts-ignore
import { CrayonInEvent } from '@/bindings/crayon/CrayonInEvent';
//@ts-ignore
import { CrayonOutEvent } from '@/bindings/crayon/CrayonOutEvent';

export function sendCrayonEvent(message: CrayonInEvent): Promise<void>;
export function drainEventQueue(): Promise<CrayonOutEvent[]>;
export function cleanup(): void;
export function runApp(): void;