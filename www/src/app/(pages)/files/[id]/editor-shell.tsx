"use client";

import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import {
  SidebarInput,
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
  useSidebar,
} from "@/components/ui/sidebar";
import { EditorContextMenu } from "./editor-context-menu";
import { WasmProvider } from "@/providers/use-crayon";
import { EditorToolbar } from "./editor-toolbar";
import { EditorCanvas } from "./editor-canvas";
import { SubSidebar } from "./page";
import { RightSidebar } from "./right-sidebar";

interface ShellProps {
  topPanel?: React.ReactNode;
  leftPanel?: React.ReactNode;
  rightPanel?: React.ReactNode;
  toolbar?: React.ReactNode;
  centrePanel?: React.ReactNode;
}

const EditorShell = ({
  topPanel,
  leftPanel,
  rightPanel,
  toolbar,
  centrePanel,
}: ShellProps) => {
  return (
    <WasmProvider>
      <SidebarProvider
        defaultOpen={false}
        style={
          {
            "--sidebar-width": "350px",
          } as React.CSSProperties
        }
      >
        <SubSidebar />
        <SidebarInset className="relative overflow-hidden isolate">
          <div id="ui-shell" className="isolate absolute inset-0 z-10">
            <ResizablePanelGroup
              direction="horizontal"
              autoSaveId={"conditional"}
              className="w-full h-full"
            >
           
              <ResizablePanel className="hud-shell isolate" order={2}>
                <div className="left-pane z-10">{leftPanel}</div>
                <div className="right-pane z-10">{rightPanel}</div>
                <div className="top-pane z-10">{topPanel}</div>
                <div className="bottom-pane z-10">
                  <EditorToolbar />
                </div>
                <EditorCanvas />
              </ResizablePanel>
              <ResizableHandle withHandle />
              {true && (
                <ResizablePanel
                  id="right-ui-pane"
                  className="pointer-events-auto bg-background min-w-60"
                  order={3}
                  defaultSize={20}
                  maxSize={50}
                >
                  <RightSidebar/>
                </ResizablePanel>
              )}
            </ResizablePanelGroup>
          </div>
        </SidebarInset>
      </SidebarProvider>
    </WasmProvider>
  );
};

export { EditorShell };
