import PropTypes from "prop-types";
import { NavLink } from "react-router-dom";

export const MobileNavBarTab = ({ path, label, handleClick }) => {
  return (
    <NavLink
      onClick={handleClick}
      to={path}
      end
      className={({ isActive }) =>
        "mobile-nav-bar__tab " + (isActive ? "mobile-nav-bar__tab--active" : "")
      }
    >
      {label}
    </NavLink>
  );
};

MobileNavBarTab.propTypes = {
  path: PropTypes.string.isRequired,
  label: PropTypes.string.isRequired,
  handleClick: PropTypes.func.isRequired,
};
