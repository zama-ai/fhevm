import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import moesifBrowser from "moesif-browser-js";
import "./main.css";
import App from "./App.jsx";
import "https://js.stripe.com/v3/pricing-table.js";
import "./styles/styles.scss";

createRoot(document.getElementById("root")).render(
  <StrictMode>
    <App />
  </StrictMode>
);

if (import.meta.env.REACT_APP_MOESIF_PUBLISHABLE_APPLICATION_ID) {
  moesifBrowser.init({
    applicationId: import.meta.env.REACT_APP_MOESIF_PUBLISHABLE_APPLICATION_ID,
    // add other option here
  });
  if (window) {
    window.moesif = moesifBrowser;
  }
} else {
  console.log(
    "Please add using REACT_APP_MOESIF_PUBLISHABLE_APPLICATION_ID to .env to enable Moesif Browser JS to track actions such as sign up"
  );
}
