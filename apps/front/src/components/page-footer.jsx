import config from "../config";

export const PageFooter = () => {
  return (
    <footer className="page-footer">
      <div className="page-footer-grid">
        <div className="page-footer-grid__info">
          <div className="page-footer-info__message">
            <a
              className="btn"
              href={config.links.zama}
              target="_blank"
              rel="noopener noreferrer"
            >
              © 2025 Zama SAS
            </a>
          </div>
          <div className="page-footer-info__button">
            <a
              className="button button__link"
              href={config.links.termsAndConditions}
              target="_blank"
              rel="noopener noreferrer"
            >
              Terms & Conditions
            </a>{" "}
            <a
              className="button button__link"
              href={config.links.privacyPolicy}
              target="_blank"
              rel="noopener noreferrer"
              style={{ marginLeft: "40px" }}
            >
              Privacy Policy
            </a>
          </div>
        </div>
      </div>
    </footer>
  );
};
