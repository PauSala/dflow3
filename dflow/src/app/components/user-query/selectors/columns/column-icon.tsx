'use-client'
import { Calendar, Sigma, Type } from "lucide-react";
import React from "react";

export default function ColumnIcon({
  type,
}: {
  type: "Integer" | "Float" | "Text" | "Date";
}) {
  switch (type) {
    case "Integer":
      return <Sigma className="w-4 h-4 mr-2 text-zinc-500" />;
    case "Float":
      return <Sigma className="w-4 h-4 mr-2 text-zinc-500" />;
    case "Text":
      return <Type className="w-4 h-4 mr-2 text-zinc-500" />;
    case "Date":
      return <Calendar className="w-4 h-4 mr-2 text-zinc-500" />;
  }
}
