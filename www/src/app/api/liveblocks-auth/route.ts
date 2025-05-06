import { useSession } from '@clerk/nextjs'
import { auth, currentUser } from '@clerk/nextjs/server'
import { Liveblocks } from '@liveblocks/node'
import { z } from 'zod'
import { ClerkJsUser } from '../../../../../crates/crayon_core/bindings/ClerkJsUser'
import { CreateUserRequest } from '../../../../../server/bindings/CreateUserRequest'

export const liveblocks = new Liveblocks({
  secret: process.env.LIVEBLOCKS_SECRET_KEY as string,
})
console.log('liveblocks',  process.env.LIVEBLOCKS_SECRET_KEY)

// Define a schema for request validation
const LiveblocksAuthRequestSchema = z.object({
  room: z.string().optional(),
})

export async function POST(req: Request) {
  // Use Clerk to get the session claims for the current user.
  // Return a 401 response if the claims are not present (the user is not logged in)
  const { sessionClaims } = await auth()
  console.log(sessionClaims)
  if (!sessionClaims) {
    return new Response('Not authorized', { status: 401 })
  }

  // Use Clerk to get more details about the current user.
  const user = await currentUser()
  if (!user) {
    return new Response('Not authorized', { status: 401 })
  }

  const createUserRequest: CreateUserRequest = {
    user: user,
  }

  console.log('createUserRequest', createUserRequest)

  const response = await fetch('http://0.0.0.0:8080/user', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(createUserRequest),
  })
  
  // Enhanced logging for the response
  console.log('===== USER CREATION RESPONSE =====')
  console.log('Status:', response.status, response.statusText)
  console.log('Headers:', Object.fromEntries(response.headers.entries()))
  
  // Log the response body if possible
  try {
    const responseClone = response.clone() // Clone to avoid consuming the original
    const responseBody = await responseClone.text()
    console.log('Body:', responseBody)
  } catch (err) {
    console.log('Could not read response body:', err)
  }
  console.log('===================================')

  // Parse and validate the request body
  try {
    const body = await req.json()
    console.log('body', body)
    
    if (!body || typeof body !== 'object') {
      return new Response('Invalid request body format', { status: 400 })
    }
    
    const { room } = LiveblocksAuthRequestSchema.parse(body)
    
    // All security checks passed so create the session and include the name and
    // avatar of the user, which will be shown within Liveblocks components
    const session = liveblocks.prepareSession(user.id, {
      userInfo: {
        name: user.firstName || "Anomyoise from Liveblocks Auth",
        avatar: user.imageUrl,
      },
    })

    if(room) {
      session.allow('room_1', session.FULL_ACCESS)
      const { body: responseBody, status } = await session.authorize()

      // Return the response
      return new Response(responseBody, { status })
    } else {
      return new Response('Invalid request body format', { status: 400 })
    }

  
  } catch (error) {
    console.error('Error processing request:', error)
    if (error instanceof z.ZodError) {
      return new Response(`Validation error: ${error.message}`, { status: 400 })
    }
    return new Response('Invalid request format', { status: 400 })
  }
}
