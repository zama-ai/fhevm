import { NavLink } from "react-router-dom";
import PropTypes from "prop-types";

export function NavBarTab({ path, label }) {
  return (
    <NavLink
      to={path}
      end
      className={({ isActive }) =>
        "nav-bar__tab " + (isActive ? "nav-bar__tab--active" : "")
      }
    >
      {label}
    </NavLink>
  );
}

NavBarTab.propTypes = {
  path: PropTypes.string.isRequired,
  label: PropTypes.string.isRequired,
};
