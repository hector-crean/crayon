// RouteHistoryContext.tsx
import { createContext, useContext, useState, useEffect } from 'react';
import { usePathname } from 'next/navigation';

const RouteHistoryContext = createContext<string[]>([]);

export function RouteHistoryProvider({ children }: { children: React.ReactNode }) {
  const [history, setHistory] = useState<string[]>([]);
  const pathname = usePathname();

  useEffect(() => {
    setHistory((prev) => [...prev, pathname]);
  }, [pathname]);

  return (
    <RouteHistoryContext.Provider value={history}>
      {children}
    </RouteHistoryContext.Provider>
  );
}

export const useRouteHistory = () => useContext(RouteHistoryContext);