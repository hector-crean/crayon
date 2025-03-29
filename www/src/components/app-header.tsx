import { PanelLeft } from "lucide-react";
import { Button } from "./ui/button";
import { SidebarTrigger } from "./ui/sidebar";
import {  SignInButton, SignedIn, SignedOut, UserButton } from '@clerk/nextjs'
import { ColorThemeDropdownController } from "./ui/color-theme-control";

export const Header = () => {
  return (
    <div className="h-16 flex flex-row items-center gap-2 bg-transparent opacity-90 backdrop-blur-xs px-4 py-2">
      <SidebarTrigger>
        <Button variant="ghost" size="icon" asChild>
          <PanelLeft />
        </Button>
      </SidebarTrigger>

      <div className="flex-1" />
      <ColorThemeDropdownController />

      <SignedOut>
          <SignInButton />
        </SignedOut>
        <SignedIn>
          <UserButton />
        </SignedIn>
    </div>
  );
};
