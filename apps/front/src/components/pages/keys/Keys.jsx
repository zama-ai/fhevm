import React, { useState } from "react";
import Modal from "react-modal";
import SVG from "react-inlinesvg";
import copy from "copy-to-clipboard";

import { PageLayout } from "../../page-layout";
import { PageLoader } from "../../page-loader";
import copyIcon from "../../../images/icons/copy.svg";
import successIcon from "../../../images/icons/success.svg";
import apiKeyIcon from "../../../images/icons/api-key.svg";
import useAuthCombined from '../../../hooks/useAuthCombined';

const customStyles = {
  content: {
    top: "50%",
    left: "50%",
    right: "auto",
    bottom: "auto",
    marginRight: "-50%",
    transform: "translate(-50%, -50%)",
  },
};

const Keys = () => {
  const {
    user,
    isLoading,
    userEmail,
    idToken,
  } = useAuthCombined();

  const [APIKey, setAPIKey] = useState("");
  const [modalIsOpen, setIsOpen] = useState(false);
  const [isCopied, setIsCopied] = useState(false);


  let resolvedEmail = user?.email || userEmail;

  Modal.setAppElement("#root");

  async function createKey() {
    fetch(`${import.meta.env.REACT_APP_DEV_PORTAL_API_SERVER}/create-key`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${idToken}`,
      },
      body: JSON.stringify({
        email: resolvedEmail,
      }),
    })
      .then((res) => {
        if (!res.ok) {
          throw new Error("Failed to create key");
        }
        return res.json();
      })
      .then((result) => {
        console.log(result);
        setAPIKey(result.apikey);
        openModal(setIsOpen);
      })
      .catch((error) => {
        setAPIKey(
          `Error creating key: ${
            error.response?.body?.message || error.toString()
          }`
        );
        openModal(setIsOpen);
      });
  }

  function openModal() {
    setIsOpen(true);
  }

  function closeModal() {
    setIsOpen(false);
  }

  if (isLoading) {
    return <PageLoader />;
  }

  return (
    <PageLayout>
      <div className="keys-description">
        <h1>My API Keys</h1>
        <p className="description">
          On this page, you can generate an API key to access{"\n"}the APIs
          you're subscribed to.
        </p>
        <div>
          <p>
            To create an API key, you must be subscribed to an API plan first.
          </p>
        </div>
        <div className="page-action">
        <button className="button__purp" onClick={createKey}>
          Create Key
        </button>
        </div>
        <p>
          <strong>Note: </strong>
          Make sure to store the key somewhere safe as you will not be{"\n"}
          able to retrieve it once you close the modal.
        </p>
      </div>

      <Modal
        isOpen={modalIsOpen}
        onRequestClose={closeModal}
        style={customStyles}
        contentLabel="API Key"
      >
        <h3 className="modal-title">Get API Key</h3>
        <div className="modal-body">
          <label>Your API Credentials</label>
          <div className="api-key-container">
            <span className="api-key-presentation">
              <SVG src={apiKeyIcon} />
              <pre className="api-key">{APIKey}</pre>
            </span>
            <button
              className="copy-button"
              onClick={() => {
                const successfulCopy = copy(APIKey);
                if (successfulCopy) {
                  setIsCopied(true);
                  setTimeout(() => setIsCopied(false), 3000);
                }
              }}
            >
              <SVG
                className="icon"
                style={{ width: "15px", height: "13.5px" }}
                fill="currentcolor"
                src={isCopied ? successIcon : copyIcon}
              />
            </button>
          </div>
          {APIKey?.includes("Error") && (
            <div style={{ color: "black", fontWeight: 300, maxWidth: 600 }}>
              <h6>Trouble shooting API Key Generation Error</h6>
              <ul>
                <li>
                  If using pre-supported API Gateway, did you set up already and
                  configured it?
                </li>
                <li>
                  If using custom API gateway, did you implement the code for
                  generating key?
                </li>
                <li>
                  Did you already purchase a plan? In default implementation,
                  API Gateway provisioning is triggered upon successful checkout
                  call back from Stripe. See API route{" "}
                  <code>/register/stripe/:checkout_session_id</code>.
                </li>
                <li>If provision was triggered, was provisioning successful?</li>
              </ul>
            </div>
          )}
        </div>
        <div className="modal-footer">
          <button
            className="button button--outline-secondary"
            onClick={closeModal}
          >
            Close
          </button>
        </div>
      </Modal>
    </PageLayout>
  );
};

export default Keys;
