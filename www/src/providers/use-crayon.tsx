"use client";

import React, {
  createContext,
  useContext,
  useEffect,
  useState,
  ReactNode,
  useCallback,
  useMemo,
} from "react";
import init, {
  runApp,
  sendCrayonEvent,
  drainEventQueue,
  InitOutput,
  cleanup,
} from "@/pkg/crayon_app";

type WasmContextType = {
  initialized: boolean;
  isLoading: boolean;
  error: Error | null;
  sendCrayonEvent: typeof sendCrayonEvent;
  drainEventQueue: typeof drainEventQueue;
  run: typeof runApp;
  appRunning: boolean;
  setAppRunning: (value: boolean) => void;
  module: InitOutput | null;
};

const WasmContext = createContext<WasmContextType | undefined>(undefined);

export const WasmProvider = ({ children }: { children: ReactNode }) => {
  const [initialized, setInitialized] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);
  const [module, setModule] = useState<InitOutput | null>(null);
  const [appRunning, setAppRunning] = useState(false);

  const run = useCallback(() => {
    if (!appRunning && module) {
      try {
        // if (!window.crypto) {
        //   throw new Error('Crypto API not available');
        // }
        
        // Ensure any previous instance is cleaned up
        try {
          module.cleanup();
        } catch (e) {
          console.warn("Cleanup before run failed:", e);
        }
        
        module.runApp();
        setAppRunning(true);
      } catch (error) {
        console.error("Failed to run app:", error);
        setAppRunning(false);
        setError(error instanceof Error ? error : new Error(String(error)));
      }
    }
  }, [appRunning, module]);

  useEffect(() => {
    let mounted = true;

    const initializeWasm = async () => {
      if (initialized) return;

      try {
        setIsLoading(true);

        // // Wait for crypto to be available
        // if (!window.crypto) {
        //   throw new Error('Crypto API not available');
        // }

        // Initialize WASM with explicit crypto object
        const wasmModule = await init();

        if (mounted) {
          setModule(wasmModule);
          setInitialized(true);
        }
      } catch (error) {
        if (mounted) {
          console.error("Failed to initialize WebAssembly:", error);
          setError(error instanceof Error ? error : new Error(String(error)));
        }
      } finally {
        if (mounted) {
          setIsLoading(false);
        }
      }
    };

    // Add a small delay to ensure window.crypto is available
    const timer = setTimeout(initializeWasm, 100);

    return () => {
      mounted = false;
      clearTimeout(timer);
      
      // Ensure cleanup happens in the correct order
      if (appRunning && module) {
        try {
          module.sendCrayonEvent({type: 'ExitApp'});
          module.cleanup();
        } catch (e) {
          console.error("Error during cleanup:", e);
        }
        setAppRunning(false);
      }
      
      // Reset module state last
      if (module) {
        setModule(null);
        setInitialized(false);
      }
    };
  }, []);

  useEffect(() => {
    const handleVisibilityChange = () => {
      if (document.hidden && appRunning) {
        // Force cleanup of the WASM context
        if (module) {
          module.sendCrayonEvent({type: 'ExitApp'});
          module.cleanup();
          setAppRunning(false);
        }
      }
    };

    const handleBeforeUnload = () => {
      if (appRunning && module) {
        module.sendCrayonEvent({type: 'ExitApp'});
        module.cleanup();
        setAppRunning(false);
        setInitialized(false);
      }
    };

    document.addEventListener('visibilitychange', handleVisibilityChange);
    window.addEventListener('beforeunload', handleBeforeUnload);
    
    return () => {
      document.removeEventListener('visibilitychange', handleVisibilityChange);
      window.removeEventListener('beforeunload', handleBeforeUnload);
      
      // Also cleanup when unmounting
      if (appRunning && module) {
        module.sendCrayonEvent({type: 'ExitApp'});
        module.cleanup();
        setAppRunning(false);
      }
    };
  }, [appRunning, module, setAppRunning]);

  return (
    <WasmContext.Provider
      value={{
        initialized,
        isLoading,
        error,
        sendCrayonEvent,
        drainEventQueue,
        run,
        appRunning,
        setAppRunning,
        module,
      }}
    >
      {children}
    </WasmContext.Provider>
  );
};

export const useWasm = (): WasmContextType => {
  const context = useContext(WasmContext);
  if (!context) {
    throw new Error("useWasm must be used within a WasmProvider");
  }
  return context;
};
