import PropTypes from "prop-types";

export const PageFooterHyperlink = ({ children, path }) => {
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

PageFooterHyperlink.propTypes = {
  children: PropTypes.node.isRequired,
  path: PropTypes.string.isRequired,
};
