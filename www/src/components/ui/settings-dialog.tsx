"use client";

import * as React from "react";
import { useEffect } from "react";
import { useSearchParams } from "next/navigation";
import {
  Bell,
  Check,
  Globe,
  Home,
  Keyboard,
  Lock,
  LucideIcon,
  Menu,
  MessageCircle,
  Paintbrush,
  Settings,
  SettingsIcon,
  Video,
} from "lucide-react";

import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "@/components/ui/breadcrumb";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
  SidebarRail,
} from "@/components/ui/sidebar";
import { TypographyController } from "./typography-controller";
import { ColorThemeController } from "./color-theme-control";
import { SettingsDialogQueryParams } from "@/query-params";

interface NavItem {
  id: SettingsDialogQueryParams["state"];
  name: string;
  icon: LucideIcon;
  component: React.ReactNode;
}

const data: NavItem[] = [
  {
    id: "appearance",
    name: "Appearance",
    icon: Paintbrush,
    component: <TypographyController />,
  },
  
  {
    id: "accessibility",
    name: "Accessibility",
    icon: Keyboard,
    component: <ColorThemeController />,
  },
];

function useSettingsDialog() {
  const [open, setOpen] = React.useState(false);
  const [selectedItem, setSelectedItem] =
    React.useState<SettingsDialogQueryParams["state"]>("appearance");
  const searchParams = useSearchParams();

  useEffect(() => {
    const settings = searchParams.get(
      "settings"
    ) as SettingsDialogQueryParams["state"];
    if (settings) {
      setSelectedItem(settings);
      setOpen(true);
    }
  }, [searchParams]);

  useEffect(() => {
    const url = new URL(window.location.href);
    if (open) {
      url.searchParams.set("settings", selectedItem);
    } else {
      url.searchParams.delete("settings");
    }
    window.history.pushState({}, "", url);
  }, [selectedItem, open]);

  return { open, setOpen, selectedItem, setSelectedItem };
}

export function SettingsDialog() {
  const { open, setOpen, selectedItem, setSelectedItem } = useSettingsDialog();

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button size="sm" className="aspect-square" variant="ghost">
          <SettingsIcon />
        </Button>
      </DialogTrigger>
      <DialogContent className="overflow-hidden p-0 h-screen w-screen md:h-auto md:max-h-[500px] md:max-w-[700px] lg:max-w-[800px]">
        <DialogTitle className="sr-only">Settings</DialogTitle>
        <DialogDescription className="sr-only">
          Customize your settings here.
        </DialogDescription>
        <SidebarProvider className="items-start @container w-full">
          <Sidebar collapsible="none" className="flex">
            <SidebarContent>
              <SidebarGroup>
                <SidebarGroupContent>
                  <SidebarMenu>
                    {data.map((item) => (
                      <SidebarMenuItem key={item.name}>
                        <SidebarMenuButton
                          asChild
                          isActive={item.id === selectedItem}
                          onPointerDown={() => setSelectedItem(item.id)}
                        >
                          <div className="flex flex-row items-center gap-2">
                            <item.icon className="h-4 w-4" />
                            <span className="hidden @md:inline">
                              {item.name}
                            </span>
                          </div>
                        </SidebarMenuButton>
                      </SidebarMenuItem>
                    ))}
                  </SidebarMenu>
                </SidebarGroupContent>
              </SidebarGroup>
            </SidebarContent>
            <SidebarRail />
          </Sidebar>
          <main className="flex flex-1 flex-col overflow-y-scroll">
            <header className="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-data-[collapsible=icon]/sidebar-wrapper:h-12">
              <div className="flex items-center gap-2 px-4">
                <Breadcrumb>
                  <BreadcrumbList>
                    <BreadcrumbItem className="hidden md:block">
                      <BreadcrumbLink href="#">Settings</BreadcrumbLink>
                    </BreadcrumbItem>
                    <BreadcrumbSeparator className="hidden md:block" />
                    <BreadcrumbItem>
                      <BreadcrumbPage>{selectedItem}</BreadcrumbPage>
                    </BreadcrumbItem>
                  </BreadcrumbList>
                </Breadcrumb>
              </div>
            </header>
            <div className="flex flex-1 flex-col gap-4 overflow-y-auto p-4 pt-0">
              {data.find((item) => item.id === selectedItem)?.component}
            </div>
          </main>
        </SidebarProvider>
      </DialogContent>
    </Dialog>
  );
}
