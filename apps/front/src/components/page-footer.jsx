import React from "react";

export const PageFooter = () => {
  return (
    <footer className="page-footer">
      <div className="page-footer-grid">
        <div className="page-footer-grid__info">
          <div className="page-footer-info__message">
            <a
              className="btn"
              href="https://github.com/Moesif/moesif-developer-portal"
              target="_blank"
            >
              Github Repo
            </a>
          </div>
          <div className="page-footer-info__button">
            <a className="button button__link" href="https://www.moesif.com" target="_blank">
              Moesif
            </a>{" "}
            <a
              className="button button__link"
              href="https://www.moesif.com/docs/developer-portal/"
              target="_blank"
              style={{ marginLeft: "40px" }}
            >
              More Docs
            </a>
          </div>
        </div>
      </div>
    </footer>
  );
};
