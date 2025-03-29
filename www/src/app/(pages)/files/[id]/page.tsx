"use client";

import { Room } from "@/app/room";
import { CommentThread } from "@/components/ui/comment-thread";
// import { WasmProvider } from "@/providers/use-crayon";
import {
  SidebarInput,
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
  useSidebar,
} from "@/components/ui/sidebar";
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
import { Button } from "@/components/ui/button";
import {
  ArrowLeftRight,
  Blocks,
  ChartAreaIcon,
  Command,
  History,
  Home,
  Layers,
  Menu,
  PanelLeft,
  Plus,
} from "lucide-react";
import { ColorThemeDropdownController } from "@/components/ui/color-theme-control";
import { SignedIn, SignedOut, SignInButton, UserButton } from "@clerk/nextjs";
import { useEffect, useState, use } from "react";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import Link from "next/link";
import { WasmProvider } from "@/providers/use-crayon";
import { useRouter, usePathname } from "next/navigation";
import { useOthers, useSelf } from "@liveblocks/react";
import {TopPanel} from "./top-panel";
import { EditorShell } from "./editor-shell";
import { RightPanel } from "./right-panel";

const views = [
  { title: "layers", icon: Layers },
  { title: "blocks", icon: Blocks },
  { title: "history", icon: History },
  { title: "comments", icon: ChartAreaIcon },
];

const commands = [{ title: "toggle-sidebar", icon: ArrowLeftRight }];

type RouteParams = { id: string };

interface PageProps {
  params: Promise<RouteParams>;
}

const Page = (props: PageProps) => {
  const params = use(props.params);
  const { id } = params;


  


  return (
    <Room roomId={id}>
      <EditorShell
        topPanel={
          <TopPanel />
        }
        toolbar={
          <div className="flex justify-center items-center">toolbar</div>
        }
        rightPanel={
          <RightPanel/>
        }
        leftPanel={
          <div className="flex justify-center items-center">leftPanel</div>
        }
      />
    </Room>
  );
};

export default Page;

export function SubSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
  // Note: I'm using state to show active item.
  // IRL you should use the url/router.
  const [activeItem, setActiveItem] = useState(views[0]);
  const { setOpen, open } = useSidebar();
  return (
    <Sidebar
      collapsible="icon"
      className="overflow-hidden *:data-[sidebar=sidebar]:flex-row bg-transparent"
      {...props}
    >
      {/* This is the first sidebar */}
      {/* We disable collapsible and adjust width to icon. */}
      {/* This will make the sidebar appear as icons. */}
      <Sidebar
        collapsible="none"
        className="w-[calc(var(--sidebar-width-icon)+1px)]! border-r rounded-l-sm bg-destructive"
      >
        <SidebarHeader>
          <SidebarMenu>
            <SidebarMenuItem>
              <Link href="/" passHref>
                <SidebarMenuButton
                  tooltip={{
                    children: "Crayon",
                    hidden: false,
                  }}
                  className="px-2.5 md:px-2"
                >
                  <Home />
                </SidebarMenuButton>
              </Link>
            </SidebarMenuItem>
          </SidebarMenu>
        </SidebarHeader>
        <SidebarContent>
          <SidebarGroup>
            <SidebarGroupContent className="px-1.5 md:px-0">
              <SidebarMenu>
                {views.map((item) => (
                  <SidebarMenuItem key={item.title}>
                    <SidebarMenuButton
                      tooltip={{
                        children: item.title,
                        hidden: false,
                      }}
                      onClick={() => {
                        setActiveItem(item);
                        setOpen(true);
                      }}
                      isActive={activeItem.title === item.title}
                      className="px-2.5 md:px-2"
                    >
                      <item.icon />
                      <span>{item.title}</span>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                ))}
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>

          <SidebarGroup>
            <SidebarGroupContent className="px-1.5 md:px-0">
              <SidebarMenu>
                <SidebarMenuItem>
                  <SidebarMenuButton
                    tooltip={{
                      children: "Toggle Sidebar",
                      hidden: false,
                    }}
                    onClick={() => {
                      setOpen(!open);
                    }}
                    className="px-2.5 md:px-2"
                  >
                    <ArrowLeftRight />
                  </SidebarMenuButton>
                </SidebarMenuItem>
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        </SidebarContent>
        <SidebarFooter>
          <SignedOut>
            <SignInButton />
          </SignedOut>
          <SignedIn>
            <UserButton />
          </SignedIn>
        </SidebarFooter>
      </Sidebar>
      {/* This is the second sidebar */}
      {/* We disable collapsible and let it fill remaining space */}
      <Sidebar collapsible="none" className="hidden flex-1 md:flex">
        <SidebarHeader className="gap-3.5 border-b p-4">
          <div className="flex w-full items-center justify-between">
            <div className="text-base font-medium text-foreground">
              {activeItem.title}
            </div>
           
          </div>
        </SidebarHeader>
        <SidebarContent>
          <SidebarGroup className="px-0">
            <SidebarGroupContent></SidebarGroupContent>
          </SidebarGroup>
        </SidebarContent>
      </Sidebar>
    </Sidebar>
  );
}
