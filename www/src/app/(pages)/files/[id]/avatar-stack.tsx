import { useSelf, useOthers } from "@liveblocks/react";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";

import { User } from "@liveblocks/client";
import { ChevronRight } from "lucide-react";
import { Button } from "@/components/ui/button";
import { ComponentProps } from "react";
import { cn } from "@/lib/utils";



interface SelfAvatarProps extends ComponentProps<typeof Button> {
  self: User;
}
const SelfAvatar = ({ self, className, ...props }: SelfAvatarProps) => {
  return (
    <Popover>
      <Tooltip>
        <TooltipTrigger asChild>
          <PopoverTrigger asChild>
            <Button
              size="fit"
              className={cn(
                "pointer-events-auto flex flex-row gap-0",
                className
              )}
              {...props}
            >
              <Avatar className="size-6">
                <AvatarImage src={self.info.avatar} />
                <AvatarFallback>{self.info.name}</AvatarFallback>
              </Avatar>
              <ChevronRight className="size-3 w-4" />
            </Button>
          </PopoverTrigger>
        </TooltipTrigger>
        <TooltipContent>
          <p>{self.info.name}</p>
        </TooltipContent>
      </Tooltip>
      <PopoverContent>
        <div className="flex flex-row gap-2">
          <Avatar className="size-6">
            <AvatarImage src={self.info.avatar} />
            <AvatarFallback>{self.info.name}</AvatarFallback>
          </Avatar>
          <p>{self.info.name}</p>
        </div>
      </PopoverContent>
    </Popover>
  );
};

interface OtherAvatarProps extends ComponentProps<typeof Button> {
  user: User;
  zIndex?: number;
}
const OtherAvatar = ({
  user,
  zIndex = 0,
  className,
  ...props
}: OtherAvatarProps) => {
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <Button
          size="fit"
          className={cn("pointer-events-auto relative", className)}
          style={{ zIndex }}
          {...props}
        >
          <Avatar className="size-6">
            <AvatarImage src={user.info.avatar} />
            <AvatarFallback>{user.info.name}</AvatarFallback>
          </Avatar>
        </Button>
      </TooltipTrigger>
      <TooltipContent>
        <p>{user.info.name}</p>
      </TooltipContent>
    </Tooltip>
  );
};
const AvatarStack = () => {
  const self = useSelf();
  const others = useOthers();

  return (
    <TooltipProvider>
      <div className="flex flex-row  bg-muted rounded-full h-min w-fit pl-1 pr-3 py-1 gap-[0.5px]">
        {others.map((user, index) => (
          <OtherAvatar key={user.id} user={user} zIndex={index} />
        ))}

        {self && <SelfAvatar self={self} />}
      </div>
    </TooltipProvider>
  );
};

export { AvatarStack };
