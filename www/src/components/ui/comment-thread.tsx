'use client'

import { useState } from "react"
import { useThreads } from "@liveblocks/react/suspense";
import { Composer, Thread } from "@liveblocks/react-ui";
import { ScrollArea } from "@/components/ui/scroll-area"
import { Button } from "@/components/ui/button"
import { X, MessageSquare } from "lucide-react"

export function CommentThread() {
  const { threads } = useThreads();
  const [isOpen, setIsOpen] = useState(true);

  if (!isOpen) {
    return (
      <Button 
        onClick={() => setIsOpen(true)} 
        className="fixed bottom-4 right-4 h-12 w-12 rounded-full shadow-lg bg-primary text-primary-foreground hover:bg-primary/90"
      >
        <MessageSquare className="h-6 w-6" />
        <span className="sr-only">Open comments</span>
      </Button>
    )
  }

  return (
    <div className="fixed top-4 right-4 w-80 h-[calc(100vh-32px)] bg-background rounded-lg shadow-lg flex flex-col overflow-hidden border border-border">
      <div className="p-4 flex justify-between items-center border-b border-border">
        <h2 className="text-lg font-semibold text-foreground">Comments</h2>
        <Button variant="ghost" size="icon" className="h-8 w-8 text-muted-foreground hover:text-foreground" onClick={() => setIsOpen(false)}>
          <X className="h-4 w-4" />
          <span className="sr-only">Close</span>
        </Button>
      </div>
      <ScrollArea className="grow bg-background">
        <div className="p-4 space-y-4">
          {threads.map((thread) => (
            <Thread key={thread.id} thread={thread} />
          ))}
        </div>
      </ScrollArea>
      <div className="p-4 border-t border-border bg-card">
        <Composer />
      </div>
    </div>
  );
}