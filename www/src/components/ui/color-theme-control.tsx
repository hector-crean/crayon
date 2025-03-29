"use client"

import * as React from "react"
import { useTheme } from "next-themes"
import { Button } from "@/components/ui/button"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import { themes } from "@/lib/color-theme.config"
import { Palette } from 'lucide-react'
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip"
import { cn } from "@/lib/utils"

export function ColorThemeDropdownController() {
  const { theme, setTheme } = useTheme()

  return (
    <DropdownMenu>
        <Tooltip>
          <TooltipTrigger asChild>
            <DropdownMenuTrigger asChild>
              <Button variant="ghost" size="icon" id="color-scheme-control">
                <Palette className="h-[1.2rem] w-[1.2rem]" />
                <span className="sr-only">Toggle theme</span>
              </Button>
            </DropdownMenuTrigger>
          </TooltipTrigger>
          <TooltipContent>
            <p>Change color theme</p>
          </TooltipContent>
        </Tooltip>
      <DropdownMenuContent align="end" className="w-56">
        <DropdownMenuLabel>Color Theme</DropdownMenuLabel>
        <DropdownMenuSeparator />
        {themes.map(({ name, icon: Icon, description }) => (
          <DropdownMenuItem
            key={name}
            onClick={() => setTheme(name)}
            className={cn(
              "flex items-center justify-between",
              theme === name && "bg-accent"
            )}
          >
            <div className="flex items-center">
              <Icon className="mr-2 h-4 w-4" />
              <span className="capitalize">{name}</span>
            </div>
            {theme === name && (
              <svg className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
              </svg>
            )}
              <Tooltip>
                <TooltipTrigger asChild>
                  <Button variant="ghost" size="icon" className="ml-auto">
                    <svg className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                  </Button>
                </TooltipTrigger>
                <TooltipContent side="left">
                  <p>{description}</p>
                </TooltipContent>
              </Tooltip>
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  )
}



export function ColorThemeController() {
  const { theme, setTheme } = useTheme()

  return (
   
    <>
       {themes.map(({ name, icon: Icon, description }) => (
          <Button
            key={name}
            onClick={() => setTheme(name)}
            className={cn(
              "flex items-center justify-between",
              theme === name && "bg-accent"
            )}
          >
            <div className="flex items-center">
              <Icon className="mr-2 h-4 w-4" />
              <span className="capitalize">{name}</span>
            </div>
          
           
          </Button>
        ))}
        </>
     
  )
}
