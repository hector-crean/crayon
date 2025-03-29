import { useSession } from '@clerk/nextjs'
import { auth, currentUser } from '@clerk/nextjs/server'
import { Liveblocks } from '@liveblocks/node'

export const liveblocks = new Liveblocks({
  secret: process.env.LIVEBLOCKS_SECRET_KEY as string,
})


export async function POST(req: Request) {
  // Use Clerk to get the session claims for the current user.
  // Return a 401 response if the claims are not present (the user is not logged in)
  const { sessionClaims } = await auth()
  if (!sessionClaims) {
    return new Response('Not authorized', { status: 401 })
  }

  // Use Clerk to get more details about the current user.
  const user = await currentUser()
  if (!user) {
    return new Response('Not authorized', { status: 401 })
  }

  // Parse the task ID from the request and query it from the database
  const { room } = await req.json()
//   const [task] = await getOneTask(+room)
//   if (!task) {
//     return new Response('Not authorized', { status: 401 })
//   }

  // If the task was not created by the user or within the organization,
  // send back a 401
//   if (task.owner_id !== user.id && task.owner_id !== sessionClaims?.org_id) {
//     return new Response('Not authorized', { status: 401 })
//   }

  // All security checks passed so create the session and include the name and
  // avatar of the user, which will be shown within Liveblocks components
 
  const session = liveblocks.prepareSession(user.id, {
    userInfo: {
      name: user.fullName,
      avatar: user.imageUrl,
    },
  })
  session.allow(room, session.FULL_ACCESS)
  const { body, status } = await session.authorize()

  // Return the response
  return new Response(body, { status })
}