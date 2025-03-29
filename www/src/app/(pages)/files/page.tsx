"use server"

import Link from "next/link"
import { Button } from "@/components/ui/button"
import { useQuery } from '@tanstack/react-query'
import { liveblocks } from '@/app/api/liveblocks-auth/route'



const FilesPage = async () => {

  const { data: rooms, nextCursor, nextPage } = await liveblocks.getRooms();

  return (
    <div className="px-4 pt-16">
      <h1 className="text-2xl font-bold mb-6">Files</h1>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {rooms.map((room) => (
          <Link href={`/files/${room.id}`} key={room.id}>
            <Button variant="outline" className="w-full justify-start">
              <span className="mr-2">{room.id}</span>
            </Button>
          </Link>
        ))}
      </div>
    </div>
  )
}



export default FilesPage;