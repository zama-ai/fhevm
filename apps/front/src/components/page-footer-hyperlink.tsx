import { ReactNode } from "react";

type Props = {
  children: ReactNode;
  path: string;
};

export const PageFooterHyperlink = ({ children, path }: Props) => {
  return (
    <a
      className="page-footer__hyperlink"
      href={path}
      target="_blank"
      rel="noopener noreferrer"
    >
      {children}
    </a>
  );
};
