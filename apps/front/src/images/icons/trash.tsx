import type { SVGProps } from "react";

export default function TrashIcon(props: SVGProps<SVGSVGElement>) {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 25 25" {...props}>
      <g id="trash">
        <path
          fill="currentColor"
          d="M20.5,4H16.86l-.69-2.06A1.37,1.37,0,0,0,14.87,1H10.13a1.37,1.37,0,0,0-1.3.94L8.14,4H4.5a.5.5,0,0,0,0,1h.34l1,17.59A1.45,1.45,0,0,0,7.2,24H17.8a1.45,1.45,0,0,0,1.41-1.41L20.16,5h.34a.5.5,0,0,0,0-1ZM9.77,2.26A.38.38,0,0,1,10.13,2h4.74a.38.38,0,0,1,.36.26L15.81,4H9.19Zm8.44,20.27a.45.45,0,0,1-.41.47H7.2a.45.45,0,0,1-.41-.47L5.84,5H19.16Z"
        />
        <path
          fill="currentColor"
          d="M9.5,10a.5.5,0,0,0-.5.5v7a.5.5,0,0,0,1,0v-7A.5.5,0,0,0,9.5,10Z"
        />
        <path
          fill="currentColor"
          d="M12.5,9a.5.5,0,0,0-.5.5v9a.5.5,0,0,0,1,0v-9A.5.5,0,0,0,12.5,9Z"
        />
        <path
          fill="currentColor"
          d="M15.5,10a.5.5,0,0,0-.5.5v7a.5.5,0,0,0,1,0v-7A.5.5,0,0,0,15.5,10Z"
        />
      </g>
    </svg>
  );
}
