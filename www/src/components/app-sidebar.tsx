"use client";

import { Minus, Plus } from "lucide-react";

import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { SettingsDialog } from "@/components/ui/settings-dialog";
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarMenuSub,
  SidebarMenuSubButton,
  SidebarMenuSubItem,
  SidebarRail,
} from "@/components/ui/sidebar";
import { usePathname } from "next/navigation";
import { LucideIcon } from "@/components/ui/lucide-icon";
import SearchDialog from "@/components/ui/search";
import Link from "next/link";
import { useSidebar } from "@/components/ui/sidebar";
import { useEffect } from "react";
import { FileSystem} from '@/components/ui/file-system'

interface AppSidebarProps {
}

export function AppSidebar({  }: AppSidebarProps) {
  const { toggleSidebar, isMobile } = useSidebar();

  useEffect(() => {
    const handleResize = () => {
      if (window.innerWidth < 768 && !isMobile) {
        toggleSidebar();
      }
    };

    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, [isMobile, toggleSidebar]);


  return (
    <Sidebar collapsible='offcanvas'>
      <SidebarHeader>
        <SearchDialog />
      </SidebarHeader>
      <SidebarContent>
        <SidebarGroup>
          
        </SidebarGroup>
      </SidebarContent>
      <SidebarFooter>
        <SettingsDialog />
      </SidebarFooter>
      <SidebarRail />
    </Sidebar>
  );
}
