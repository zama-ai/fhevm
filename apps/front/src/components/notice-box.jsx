import React from "react";
import SVG from "react-inlinesvg";

import { iconFillColor } from "../common/constants";

function NoticeBox({ iconSrc, title, description, actions}) {
  return (
    <div className="notice-box">
      <SVG
        src={iconSrc}
        style={{ width: "100px", height: "100px", fill: iconFillColor }}
      />
      <h4 className="box-title">{title}</h4>
      <p className="box-description">
        {description}
      </p>
      <div className="box-actions">
        {actions}
      </div>
    </div>
  );
}

export default NoticeBox;
