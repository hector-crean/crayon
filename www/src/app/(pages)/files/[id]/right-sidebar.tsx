import { AvatarStack } from "./avatar-stack";
import { InboxPopover } from "./inbox-popover";
import { Threads } from "./threads";

const RightSidebar = () => {
  return (<div>
  <div className="flex flex-row gap-2 justify-end px-4"><AvatarStack /><InboxPopover/></div>
    <Threads/>
  </div>)
};

export { RightSidebar };
