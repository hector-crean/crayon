
import "@/app/globals.css";
import { ThemeProvider } from "@/components/theme-provider";
import { Metadata } from "next";
import { notFound } from "next/navigation";
import { Toaster } from "@/components/ui/toaster";
import { TanstackQueryProvider } from "@/providers/tanstack-query";
import { TooltipProvider } from "@/components/ui/tooltip";
import {
  ColorThemeController,
  ColorThemeDropdownController,
} from "@/components/ui/color-theme-control";
import {
  ClerkProvider,
  SignInButton,
  SignedIn,
  SignedOut,
  UserButton,
} from "@clerk/nextjs";

export const metadata: Metadata = {
  title: "crayon",
  description: "crayon",
};

interface Props {
  children: React.ReactNode;
  params: {
    locale: string;
  };
}

export default async function RootLayout({
  children,
}: Props) {
  return (
    <ClerkProvider
    appearance={{
      elements: {
        userButtonPopoverFooter: 'hidden',
        footer: 'hidden',
      },
    }}
    >
      <html lang="en">
        <meta
          name="viewport"
          content="width=device-width, initial-scale=1"
        ></meta>
        <body className={`antialiased`}>
          <ThemeProvider
            attribute="class"
            defaultTheme="system"
            enableSystem={true}
            disableTransitionOnChange
            themes={["light", "dark", "system"]}
          >
            <TanstackQueryProvider>
              <TooltipProvider>
                {children}
                <Toaster />
              </TooltipProvider>
            </TanstackQueryProvider>
          </ThemeProvider>
        </body>
      </html>
    </ClerkProvider>
  );
}
