import { createClient } from "@liveblocks/client";

const client = createClient({
    authEndpoint: "/api/liveblocks-auth"
  });
  
  
  export { client };