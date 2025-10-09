import { SVGProps } from "react";

export default function Copy(props: SVGProps<SVGSVGElement>) {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" {...props}>
      <g id="Symbol" fill="currentcolor">
        <path d="M20,6H17V3a1,1,0,0,0-1-1H4A1,1,0,0,0,3,3V17a1,1,0,0,0,1,1H7v3a1,1,0,0,0,1,1H20a1,1,0,0,0,1-1V7A1,1,0,0,0,20,6ZM5,16V4H15V16Zm14,4H9V18h7a1,1,0,0,0,1-1V8h2Z" />
        <rect x="7" y="6" width="6" height="2" />
        <rect x="7" y="9" width="6" height="2" />
        <rect x="7" y="12" width="6" height="2" />
      </g>
    </svg>
  );
}
