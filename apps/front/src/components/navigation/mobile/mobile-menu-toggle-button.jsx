import PropTypes from "prop-types";

export const MobileMenuToggleButton = ({ icon, handleClick }) => {
  return (
    <span
      className="mobile-nav-bar__toggle material-icons"
      id="mobile-menu-toggle-button"
      onClick={handleClick}
    >
      {icon}
    </span>
  );
};

MobileMenuToggleButton.propTypes = {
  icon: PropTypes.node.isRequired,
  handleClick: PropTypes.func.isRequired,
};
