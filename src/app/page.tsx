"use client";

import { useEffect } from "react";
import { useRouter } from "next/navigation";
import { Loader2 } from "lucide-react";
import { useTheme } from "@/hooks/useTheme";

export default function Home() {
  const router = useRouter();
  const { isDark } = useTheme();

  useEffect(() => {
    router.replace("/home");
  }, [router]);

  return (
    <div
      className={`glass-page min-h-screen flex items-center justify-center px-4 ${
        isDark ? "dark text-white" : "text-zinc-900"
      }`}
    >
      <div
        className={`glass-panel flex flex-col items-center justify-center px-8 py-6 ${
          isDark ? "" : "light-glass"
        }`}
      >
        <Loader2 className="w-12 h-12 animate-spin text-primary" />
        <div className="text-center">
          <h1 className="text-2xl font-bold text-foreground">載入中</h1>
        </div>
      </div>
    </div>
  );
}
