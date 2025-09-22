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
              rel="noopener noreferrer"
            >
              Github Repo
            </a>
          </div>
          <div className="page-footer-info__button">
            <a
              className="button button__link"
              href="https://www.moesif.com"
              target="_blank"
              rel="noopener noreferrer"
            >
              Moesif
            </a>{" "}
            <a
              className="button button__link"
              href="https://www.moesif.com/docs/developer-portal/"
              target="_blank"
              rel="noopener noreferrer"
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
