import React from "react";

export const PageLoader = () => {
  const loadingImg = "https://cdn.auth0.com/blog/hello-auth0/loader.svg";

  return (
    <div style={{ height: "90vh", display: "flex", alignItems: "center" }}>
      <div className="loader">
        <img src={loadingImg} alt="Loading..." />
      </div>
    </div>
  );
};
