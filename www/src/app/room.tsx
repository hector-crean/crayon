"use client";

import { ReactNode } from "react";
import {
  LiveblocksProvider,
  RoomProvider,
  ClientSideSuspense,
} from "@liveblocks/react/suspense";
import { Loading } from "@/components/ui/loading";

interface RoomProps {
  roomId: string;
  children: ReactNode;
}
export function Room({ roomId, children }: RoomProps) {
  return (
    <LiveblocksProvider
      authEndpoint={'/api/liveblocks-auth'}
      // Throttle time (ms) between WebSocket updates
      throttle={100}
      // ---
      // Prevent browser tab from closing while local changes aren’t synchronized yet
      preventUnsavedChanges={false}
      // ---
      // Throw lost-connection event after 5 seconds offline
      lostConnectionTimeout={5000}
      // ---
      // Disconnect users after X (ms) of inactivity, disabled by default
      backgroundKeepAliveTimeout={undefined}
      // ---
      // Resolve user info for Comments and Notifications
      // resolveUsers={async ({ userIds }) => {
      //   const usersData = await __getUsersFromDB__(userIds);

      //   return usersData.map((userData) => ({
      //     name: userData.name,
      //     avatar: userData.avatar.src,
      //   }));
      // }}
      // ---
      // Resolve room info for Notifications
      // resolveRoomsInfo={async ({ roomIds }) => {
      //   const documentsData = await __getDocumentsFromDB__(roomIds);

      //   return documentsData.map((documentData) => ({
      //     name: documentData.name,
      //     // url: documentData.url,
      //   }));
      // }}
      // ---
      // Resolve mention suggestions for Comments
      // resolveMentionSuggestions={async ({ text, roomId }) => {
      //   const workspaceUsers = await __getWorkspaceUsersFromDB__(roomId);

      //   if (!text) {
      //     // Show all workspace users by default
      //     return __getUserIds__(workspaceUsers);
      //   } else {
      //     const matchingUsers = __findUsers__(workspaceUsers, text);
      //     return __getUserIds__(matchingUsers);
      //   }
      // }}
    >
      <RoomProvider id={roomId}>
        <ClientSideSuspense fallback={<Loading isLoading={true} />}>
          {children}
        </ClientSideSuspense>
      </RoomProvider>
    </LiveblocksProvider>
  );
}

//publicApiKey={"pk_dev_36Ju4MKoLiwdy5CCDPgcw8jx5aAtRtVyvn0UD0znQ-2vWDc_X4dVNZw7RyfgymYb"}
