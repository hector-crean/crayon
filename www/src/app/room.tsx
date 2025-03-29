"use client";

import { ReactNode } from "react";
import {
  LiveblocksProvider,
  RoomProvider,
  ClientSideSuspense,
} from "@liveblocks/react/suspense";
import { Loading } from "@/components/ui/loading";


interface RoomProps { 
  roomId: string,
  children: ReactNode;
}
export function Room({ roomId, children }: RoomProps) {
  return (
    <LiveblocksProvider authEndpoint="/api/liveblocks-auth" >
      <RoomProvider id={roomId}>
        <ClientSideSuspense fallback={<Loading isLoading={true} />}>
          {children}
        </ClientSideSuspense>
      </RoomProvider>
    </LiveblocksProvider>
  );
}

//publicApiKey={"pk_dev_36Ju4MKoLiwdy5CCDPgcw8jx5aAtRtVyvn0UD0znQ-2vWDc_X4dVNZw7RyfgymYb"}