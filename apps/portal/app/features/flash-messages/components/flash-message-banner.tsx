import { useEffect, useState } from "react";
import type { FlashMessage } from "../flash.server";

const flashStyles: Record<FlashMessage["type"], string> = {
  success: "bg-emerald-50 border-b border-emerald-200 text-emerald-900",
  info: "bg-blue-50 border-b border-blue-200 text-blue-900",
  warning: "bg-amber-50 border-b border-amber-200 text-amber-900",
  error: "bg-destructive/10 border-b border-destructive/40 text-destructive",
};

function getFlashBannerClass(type: FlashMessage["type"]) {
  return flashStyles[type] ?? flashStyles.info;
}

function bannerTitleFor(type: FlashMessage["type"]) {
  switch (type) {
    case "success":
      return "Success";
    case "warning":
      return "Attention needed";
    case "error":
      return "Something went wrong";
    default:
      return "Notice";
  }
}

export default function FlashMessageBanner({
  flash,
}: {
  flash: FlashMessage | null;
}) {
  const [show, setShow] = useState(true);
  useEffect(() => {
    if (flash) {
      setShow(true);
      const timer = setTimeout(() => {
        setShow(false);
      }, 5000);
      return () => clearTimeout(timer);
    }
  }, [flash]);

  if (!flash || !show) return null;

  return (
    <div
      role="alert"
      aria-live="assertive"
      className={getFlashBannerClass(flash.type)}
    >
      <div className="container mx-auto max-w-screen-lg py-3 text-sm">
        <p className="font-medium">
          {flash.title ?? bannerTitleFor(flash.type)}
        </p>
        <p className="mt-1 text-sm/relaxed">{flash.message}</p>
      </div>
    </div>
  );
}
