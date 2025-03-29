"use client";

import { AppCommandPalette } from "@/components/app-command-palette";


const Layout = ({ children }: { children: React.ReactNode }) => {

 

  return (
    <>
      {children}
      <AppCommandPalette />
    </>
  );
};

export default Layout;
