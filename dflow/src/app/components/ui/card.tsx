import React from "react";

type Props = {
  children: JSX.Element;
};

export default function Card({ children }: Props) {
  return <div className="p-2 rounded shadow-sm">{children}</div>;
}
