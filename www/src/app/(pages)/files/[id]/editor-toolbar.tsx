"use client";

import React, { useState, useCallback, useRef } from "react";
import { ToolState } from "@/bindings/crayon/ToolState";
import { CrayonInEvent } from "@/bindings/crayon/CrayonInEvent";
import { sendCrayonEvent } from "@/pkg/crayon_app";
import { useCrayonEvents } from "./editor-canvas";
import { Button } from "@/components/ui/button";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import {
  BoxSelect,
  HandIcon,
  ZoomIn,
  PencilIcon,
  ShapesIcon,
  TypeIcon,
  UndoIcon,
  RedoIcon,
  ImageIcon,
  LayersIcon,
  EyeIcon,
  DownloadIcon,
  MessageSquare,
  ChevronDown,
} from "lucide-react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuPortal,
  DropdownMenuSeparator,
  DropdownMenuShortcut,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";

interface EditorToolbarProps {}

const EditorToolbar: React.FC<EditorToolbarProps> = ({}) => {
  const [activeTool, setActiveTool] = useState<ToolState>("Comment");
  const [pendingTool, setPendingTool] = useState<ToolState | null>(null);
  const pendingTimeoutRef = useRef<NodeJS.Timeout>(undefined);

  const handleToolChange = useCallback(
    (tool: ToolState) => {
      setPendingTool(tool);
      if (pendingTimeoutRef.current) {
        clearTimeout(pendingTimeoutRef.current);
      }
      pendingTimeoutRef.current = setTimeout(() => {
        console.warn("Tool change not confirmed by WASM, reverting...");
        setPendingTool(null);
      }, 1000);

      const changeToolEvent: CrayonInEvent = {
        type: "ChangeTool",
        data: tool,
      };
      sendCrayonEvent(changeToolEvent);
    },
    [sendCrayonEvent]
  );

  /* ----------------------------------------------------------------
   * 3b. Listen for events from WASM
   * ---------------------------------------------------------------- */
  useCrayonEvents((event) => {
    switch (event.type) {
      case "ToolChanged": {
        // e.g. { type: 'ToolStateChanged', data: { type: 'Move' } }
        console.log("ToolChanged event received:", event);

        const newTool = event.data; // or event.data
        if (pendingTimeoutRef.current) {
          clearTimeout(pendingTimeoutRef.current);
        }
        setActiveTool(newTool);
        setPendingTool(null);
        break;
      }
      default:
        // If you plan to handle more event types in the future, add them here
        console.warn("Unhandled event from WASM:", event);
    }
  });

  const displayTool = pendingTool || activeTool;

  return (
    <Toolbar
      activeTool={displayTool}
      setActiveTool={handleToolChange}
      sendCrayonEvent={sendCrayonEvent}
    />
  );
};

export { EditorToolbar };

interface Tool {
  id: ToolState;
  icon: React.ReactNode;
  label: string;
}

interface ToolWithSubmenu extends Tool {
  submenu?: Array<Tool>;
}

const tools: Array<ToolWithSubmenu> = [
  {
    id: "Comment",
    icon: <MessageSquare className="h-4 w-4" />,
    label: "Comment",
  },
  {
    id: "Transform",
    icon: <HandIcon className="h-4 w-4" />,
    label: "Transform",
  },
  {
    id: "Markup",
    icon: <PencilIcon className="h-4 w-4" />,
    label: "Markup",
    submenu: [
      {
        id: "Block",
        icon: <MessageSquare className="h-4 w-4" />,
        label: "Block",
      },
    ],
  },
  // { id: "draw", icon: <PencilIcon className="h-4 w-4" />, label: "Draw" },
  // { id: "shape", icon: <ShapesIcon className="h-4 w-4" />, label: "Shape" },
  // { id: "image", icon: <ImageIcon className="h-4 w-4" />, label: "Image" },
];

interface ToolbarProps {
  activeTool: ToolState;
  setActiveTool: (tool: ToolState) => void;
  sendCrayonEvent: (event: CrayonInEvent) => void;
}

export function Toolbar({
  activeTool,
  setActiveTool,
  sendCrayonEvent,
}: ToolbarProps) {
  return (
    <div className="relative flex items-center space-x-2  rounded-2xl shadow-md p-1.5 px-2 pointer-events-auto">
      {tools.map((tool) => (
        <div className="flex flex-row items-center justify-center bg-primary rounded-sm p-0.5">
          <Tooltip key={tool.id}>
            <TooltipTrigger asChild>
              <Button
                variant={activeTool === tool.id ? "default" : "secondary"}
                size="icon"
                className="flex items-center justify-center"
                onClick={() => {
                  setActiveTool(tool.id);
                  sendCrayonEvent({ type: "ChangeTool", data: tool.id });
                }}
                aria-label={tool.label}
              >
                {tool.icon}
              </Button>
            </TooltipTrigger>
            <TooltipContent>
              <p>{tool.label}</p>
            </TooltipContent>
          </Tooltip>
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button
                variant="secondary"
                size="fit"
                className="flex items-center justify-center"
              >
                <ChevronDown />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent className="w-56">
              <DropdownMenuLabel>My Account</DropdownMenuLabel>
              <DropdownMenuSeparator />
              <DropdownMenuGroup>
                <DropdownMenuItem>
                  Profile
                  <DropdownMenuShortcut>⇧⌘P</DropdownMenuShortcut>
                </DropdownMenuItem>
                <DropdownMenuItem>
                  Billing
                  <DropdownMenuShortcut>⌘B</DropdownMenuShortcut>
                </DropdownMenuItem>
                <DropdownMenuItem>
                  Settings
                  <DropdownMenuShortcut>⌘S</DropdownMenuShortcut>
                </DropdownMenuItem>
                <DropdownMenuItem>
                  Keyboard shortcuts
                  <DropdownMenuShortcut>⌘K</DropdownMenuShortcut>
                </DropdownMenuItem>
              </DropdownMenuGroup>
              <DropdownMenuSeparator />
              <DropdownMenuGroup>
                <DropdownMenuItem>Team</DropdownMenuItem>
                <DropdownMenuSub>
                  <DropdownMenuSubTrigger>Invite users</DropdownMenuSubTrigger>
                  <DropdownMenuPortal>
                    <DropdownMenuSubContent>
                      <DropdownMenuItem>Email</DropdownMenuItem>
                      <DropdownMenuItem>Message</DropdownMenuItem>
                      <DropdownMenuSeparator />
                      <DropdownMenuItem>More...</DropdownMenuItem>
                    </DropdownMenuSubContent>
                  </DropdownMenuPortal>
                </DropdownMenuSub>
                <DropdownMenuItem>
                  New Team
                  <DropdownMenuShortcut>⌘+T</DropdownMenuShortcut>
                </DropdownMenuItem>
              </DropdownMenuGroup>
              <DropdownMenuSeparator />
              <DropdownMenuItem>GitHub</DropdownMenuItem>
              <DropdownMenuItem>Support</DropdownMenuItem>
              <DropdownMenuItem disabled>API</DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem>
                Log out
                <DropdownMenuShortcut>⇧⌘Q</DropdownMenuShortcut>
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      ))}
    </div>
  );
}
