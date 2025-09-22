import React, { useState } from "react";
import { useNavigate } from "react-router-dom";

const SignUp = () => {
  const navigate = useNavigate();
  const [firstName, setFirstName] = useState("");
  const [lastName, setLastName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [errors, setErrors] = useState({});

  const handleSignup = async () => {
    const validationErrors = validateForm();
    if (Object.keys(validationErrors).length > 0) {
      setErrors(validationErrors);
      return;
    }

    try {
      const response = await fetch(
        `${import.meta.env.REACT_APP_DEV_PORTAL_API_SERVER}/okta/register`,
        {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            firstName,
            lastName,
            email: email.toLowerCase(),
            password,
          }),
        }
      );

      if (response.ok) {
        console.log("User created successfully");
        navigate("/login/oktapostcreate");
      } else {
        console.error("Failed to create user");
        // Handle error creating user
      }
    } catch (error) {
      console.error("Error creating user:", error);
      // Handle error creating user
    }
  };

  const validateForm = () => {
    const errors = {};

    if (!firstName.trim()) {
      errors.firstName = "First name is required";
    }

    if (!lastName.trim()) {
      errors.lastName = "Last name is required";
    }

    if (!email.trim()) {
      errors.email = "Email is required";
    } else if (!isValidEmail(email)) {
      errors.email = "Invalid email address";
    }

    if (!password.trim()) {
      errors.password = "Password is required";
    } else if (password.length < 6) {
      errors.password = "Password should be at least 6 characters long";
    }

    return errors;
  };

  const isValidEmail = (email) => {
    // Regular expression for email validation
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return emailRegex.test(email);
  };

  const handleKeyDown = (e) => {
    if (e.keyCode === 13) {
      handleSignup();
    }
  };

  return (
    <div className="signup-container">
      <h2 className="signup-title">Sign up</h2>
      <div>
        <label>First Name:</label>
        <input
          className="signup-form-input"
          type="text"
          value={firstName}
          onChange={(e) => setFirstName(e.target.value)}
        />
        {errors.firstName && (
          <span className="signup-error">{errors.firstName}</span>
        )}
      </div>
      <div>
        <label>Last Name:</label>
        <input
          className="signup-form-input"
          type="text"
          value={lastName}
          onChange={(e) => setLastName(e.target.value)}
        />
        {errors.lastName && (
          <span className="signup-error">{errors.lastName}</span>
        )}
      </div>
      <div>
        <label>Email:</label>
        <input
          className="signup-form-input"
          type="email"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
        />
        {errors.email && <span className="signup-error">{errors.email}</span>}
      </div>
      <div>
        <label>Password:</label>
        <input
          className="signup-form-input"
          type="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          onKeyDown={handleKeyDown}
        />
        {errors.password && (
          <span className="signup-error">{errors.password}</span>
        )}
      </div>
      <button className="signup-form-button" onClick={handleSignup}>
        Sign up
      </button>
    </div>
  );
};

export default SignUp;
