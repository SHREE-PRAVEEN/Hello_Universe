import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import "./globals.css";
import { Providers } from "./providers";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "Hello Universe | Robotics & AI Platform",
  description: "Pioneering the future of robotics with AI-powered automation and blockchain technology.",
  keywords: ["robotics", "AI", "blockchain", "automation", "Web3"],
  authors: [{ name: "Hello Universe" }],
  openGraph: {
    title: "Hello Universe",
    description: "Pioneering the future of robotics with AI-powered automation and blockchain technology.",
    url: "https://hellouniverse.io",
    siteName: "Hello Universe",
    type: "website",
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body
        className={`${geistSans.variable} ${geistMono.variable} antialiased bg-white dark:bg-zinc-950`}
      >
        <Providers>{children}</Providers>
      </body>
    </html>
  );
}
