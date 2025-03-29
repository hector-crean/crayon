import {
    Calendar,
    Search,
    Settings
} from "lucide-react"

import { Button } from "@/components/ui/button"
import {
    CommandDialog,
    CommandEmpty,
    CommandGroup,
    CommandInput,
    CommandItem,
    CommandList,
    CommandSeparator,
    CommandShortcut
} from "@/components/ui/command"

import { useEffect, useState, useCallback } from "react"
import { useRouter, useSearchParams } from 'next/navigation'
import { SearchDialogQueryParams } from "@/query-params"

export default function SearchDialog() {
    const router = useRouter()
    const searchParams = useSearchParams()
    const isOpen = searchParams.get('search') === 'true'
    const [open, setOpen] = useState(isOpen)

    const toggleOpen = useCallback(() => {
        setOpen((prevOpen) => {
            const newOpen = !prevOpen
            updateQueryParam(newOpen)
            return newOpen
        })
    }, [])
    
    useEffect(() => {
        const down = (e: KeyboardEvent) => {
            if (e.key === "k" && (e.metaKey || e.ctrlKey)) {
                e.preventDefault()
                toggleOpen()
            }
        }

        document.addEventListener("keydown", down)
        return () => document.removeEventListener("keydown", down)
    }, [toggleOpen])

  

    const updateQueryParam = (newOpen: boolean) => {
        const url = new URL(window.location.href)
        const searchParams: SearchDialogQueryParams = { type: 'search', open: newOpen }
        
        if (newOpen) {
            url.searchParams.set('search', searchParams.open.toString())
        } else {
            url.searchParams.delete('search')
        }
        window.history.pushState({}, "", url.toString())
    }

    const handleOpenChange = (newOpen: boolean) => {
        setOpen(newOpen)
        updateQueryParam(newOpen)
    }

    return (
        <>
            <Button variant='ghost' size="sm" onClick={() => handleOpenChange(true)}>
                <Search className="w-full h-full aspect-square" />
            </Button>

            <CommandDialog open={open} onOpenChange={handleOpenChange} >
                <CommandInput placeholder="Type a command or search..." />
                <CommandList>
                    <CommandEmpty>No results found.</CommandEmpty>
                    <CommandGroup heading="Suggestions">
                        <CommandItem>
                            <Calendar className="w-4 h-4 mr-2" />
                            <span>Search Calendar</span>
                        </CommandItem>

                    </CommandGroup>
                    <CommandSeparator />
                    <CommandGroup heading="Settings">

                        <CommandItem>
                            <Settings className="w-4 h-4 mr-2" />
                            <span>Settings</span>
                            <CommandShortcut>⌘S</CommandShortcut>
                        </CommandItem>
                    </CommandGroup>
                </CommandList>
            </CommandDialog>
        </>
    )
}

