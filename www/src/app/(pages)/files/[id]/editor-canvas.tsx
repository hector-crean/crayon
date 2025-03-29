"use client";

import { TooltipProvider } from "@/components/ui/tooltip";
import React, {
  useCallback,
  useEffect,
  useRef,
  useState,
  useMemo,
  useLayoutEffect,
} from "react";
import { Loading } from "@/components/ui/loading";
import { useWasm, WasmProvider } from "@/providers/use-crayon";
import { CrayonInEvent } from "@/bindings/crayon/CrayonInEvent";
import { CrayonOutEvent } from "@/bindings/crayon/CrayonOutEvent";
import { ToolState } from "@/bindings/crayon/ToolState";
import { sendCrayonEvent } from "@/pkg/crayon_app";
import { usePathname, useRouter } from "next/navigation";
import { EditorContextMenu } from "@/app/(pages)/files/[id]/editor-context-menu";
import {EditorToolbar} from "./editor-toolbar";

/* ------------------------------------------------------------------
 * 1.  XState context
 * ------------------------------------------------------------------ */

/* ------------------------------------------------------------------
 * 2.  Custom hook: poll for outgoing WASM events
 * ------------------------------------------------------------------ */
export function useCrayonEvents(onEvent?: (event: CrayonOutEvent) => void) {
  const { drainEventQueue, initialized } = useWasm();
  const [outEvents, setOutEvents] = useState<CrayonOutEvent[]>([]);

  useEffect(() => {
    if (!initialized) return;

    // Poll ~60fps with requestAnimationFrame
    let lastCheck = 0;
    const POLL_INTERVAL_MS = 16;

    const checkEvents = async (timestamp: number) => {
      if (timestamp - lastCheck >= POLL_INTERVAL_MS) {
        try {
          const newEvents = await drainEventQueue();
          if (newEvents.length > 0) {
            setOutEvents((prev) => [...prev, ...newEvents]);
            newEvents.forEach((event) => onEvent?.(event));
          }
        } catch (error) {
          console.error("Failed to drain event queue:", error);
        }
        lastCheck = timestamp;
      }
      handle = requestAnimationFrame(checkEvents);
    };

    let handle = requestAnimationFrame(checkEvents);

    return () => {
      cancelAnimationFrame(handle);
    };
  }, [initialized, drainEventQueue, onEvent]);

  return outEvents;
}

/* ------------------------------------------------------------------
 * 3.  Main CrayonCanvas component
 * ------------------------------------------------------------------ */
interface CrayonCanvasProps {}

const EditorCanvas = () => {
  const { initialized, isLoading, error, run, appRunning } = useWasm();
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const appInitializedRef = useRef(false);
  const router = useRouter();
  const pathname = usePathname();

  // Handle visibility changes at the canvas level
  useEffect(() => {
    if (!initialized || !canvasRef.current) return;

    const canvas = canvasRef.current;

    const handleVisibilityChange = () => {
      if (document.hidden) {
        // Mark canvas for restoration when visible
        canvas.style.visibility = "hidden";
      } else {
        // Small delay to ensure WebGL context is ready
        setTimeout(() => {
          canvas.style.visibility = "visible";
          // Only restart if we were previously running
          if (appRunning && !appInitializedRef.current) {
            appInitializedRef.current = true;
            run();
          }
        }, 100);
      }
    };

    document.addEventListener("visibilitychange", handleVisibilityChange);
    return () => {
      document.removeEventListener("visibilitychange", handleVisibilityChange);
    };
  }, [initialized, run, appRunning]);

  // Initial setup
  useLayoutEffect(() => {
    if (!initialized || !canvasRef.current || appInitializedRef.current) return;

    // Ensure we're visible before starting
    if (!document.hidden) {
      appInitializedRef.current = true;
      run();
    }
  }, [initialized, run]);

  /* ----------------------------------------------------------------
   * 3c. Render
   * ---------------------------------------------------------------- */
  return (
     <canvas
    id="crayon-canvas"
    style={{ pointerEvents: "all" }}
    ref={canvasRef}
    className="absolute inset-0 z-0"
  />
  );
};

export { EditorCanvas };
