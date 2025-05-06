import { useSearchParams } from "next/navigation";
import { RoomProvider, useThreads } from "@liveblocks/react/suspense";
import Loading from "./loading";
import { Composer, Thread } from "@liveblocks/react-ui";
import { ClientSideSuspense } from "@liveblocks/react";
import { ErrorBoundary } from "react-error-boundary";
import { useUser } from "@clerk/nextjs";

/**
 * Displays a list of threads, along with a composer for creating
 * new threads.
 */

const Threads = () => {
    const { threads } = useThreads();
    const { isSignedIn, user, isLoaded } = useUser();
  
    if (!isSignedIn || !isLoaded) {
        return <div>Loading...</div>;
    }
    console.log(user.firstName)
  
    return (
      <main className="h-tablet overflow-y-auto">
        {threads.map((thread) => (
          <Thread 
            key={thread.id} 
            thread={thread} 
            className="thread"
          />
        ))}
        <Composer 
          className="composer" 
          // Optionally pass user info directly if needed
          metadata={{
            name: user?.firstName || "Anonymous From Composer",
            avatar: user?.imageUrl
          }}
        />
      </main>
    );
}
  
export { Threads };