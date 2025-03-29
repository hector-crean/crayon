"use client";

import React, { useState } from "react";
import {
  ChevronRight as ChevronRightIcon,
  File as FileIcon,
  Folder as FolderIcon,
} from "lucide-react";
import { AppRouterEntry } from "@/bindings/AppRouterEntry";
import { File } from "@/bindings/File";
import { Directory } from "@/bindings/Directory";
import { cn } from "@/lib/utils";
import { AnimatePresence } from "motion/react";
import { motion } from "motion/react";
import Link from "next/link";

interface FileSystemProps {
  entries: AppRouterEntry[];
}

export const FileSystem: React.FC<FileSystemProps> = ({ entries }) => {
  return (
    <div className="p-4">
      {entries.map((entry, index) => (
        <FileSystemNode key={index} entry={entry} level={0} />
      ))}
    </div>
  );
};

type FileNodeProps = {
  file: File;
  level: number;
};

const FileNode = ({ file, level }: FileNodeProps) => {
  // Check if the path contains dynamic segments
  const isDynamicRoute = /\[.*?\]/.test(file.relative_path);

  return (
    <div className={cn("ml-4", level === 0 && "ml-0")}>
      <div className="flex items-center py-1 cursor-pointer hover:bg-gray-100 rounded">
        {!isDynamicRoute ? (
          <Link href={file.relative_path.replace("page.tsx", "")}>
            <span>{file.path_segment}</span>
          </Link>
        ) : (
          <span>{file.path_segment}</span> // Render as plain text if dynamic
        )}
      </div>
    </div>
  );
};

type DirectoryNodeProps = {
  directory: Directory;
  level: number;
};

const DirectoryNode = ({ directory, level }: DirectoryNodeProps) => {
  const [isExpanded, setIsExpanded] = useState(false);

  const toggleExpand = () => {
    setIsExpanded(!isExpanded);
  };

  return (
    <div className={cn("ml-4", level === 0 && "ml-0")}>
      <div
        className="flex items-center py-1 cursor-pointer hover:bg-gray-100 rounded"
        onClick={toggleExpand}
      >
        <ChevronRightIcon
          className={cn(
            "w-4 h-4 mr-1 transition-transform",
            isExpanded && "transform rotate-90"
          )}
        />
        <FolderIcon className="w-4 h-4 mr-2 text-blue-500" />
        <span>{directory.path_segment}</span>
      </div>
      <AnimatePresence>
        {isExpanded && (
          <motion.div className="ml-4">
            {directory.children.map((child, index) => (
              <FileSystemNode key={index} entry={child} level={level + 1} />
            ))}
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
};

interface FileSystemNodeProps {
  entry: AppRouterEntry;
  level: number;
}
const FileSystemNode: React.FC<FileSystemNodeProps> = ({ entry, level }) => {
  switch (entry.type) {
    case "Directory":
      return <DirectoryNode directory={entry} level={level} />;
    case "File":
      return <FileNode file={entry} level={level} />;
  }
};
