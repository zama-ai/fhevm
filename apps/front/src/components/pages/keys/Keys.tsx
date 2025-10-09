import { useState } from "react";
import Modal from "react-modal";
import SVG from "react-inlinesvg";
import copy from "copy-to-clipboard";

import { PageLayout } from "../../page-layout";
import CopyIcon from "../../../images/icons/copy";
import SuccessIcon from "../../../images/icons/success";
import apiKeyIcon from "../../../images/icons/api-key.svg";
import { useCreateApiKey } from "../../../hooks/useCreateApiKey";
import { KeyTable } from "./key-table";

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
  const { createApiKey } = useCreateApiKey();

  const [APIKey, setAPIKey] = useState("");
  const [modalIsOpen, setIsOpen] = useState(false);
  const [isCopied, setIsCopied] = useState(false);

  Modal.setAppElement("#root");

  async function createKey() {
    try {
      const apiKey = await createApiKey({});

      setAPIKey(apiKey.key);
    } catch (e: any) {
      setAPIKey(e?.message ?? "Failed to create API Key");
    } finally {
      openModal();
    }
  }

  function openModal() {
    setIsOpen(true);
  }

  function closeModal() {
    setIsOpen(false);
  }

  return (
    <PageLayout>
      <div className="keys-description">
        <h1>My API Keys</h1>
        <div className="flex align-stretch">
          <div className="description flex-1">
            <p>
              On this page, you can generate an API key to access the APIs
              you&apos;re subscribed to.
            </p>
            <p>
              To create an API key, you must be subscribed to an API plan first.
            </p>
          </div>
          <div className="page-action flex-1">
            <button className="button__purp" onClick={createKey}>
              Create Key
            </button>
          </div>
        </div>
      </div>

      <KeyTable />

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
              {isCopied ? (
                <SuccessIcon width="15px" height="13.5px" />
              ) : (
                <CopyIcon width="15px" height="13.5px" />
              )}
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
                <li>
                  If provision was triggered, was provisioning successful?
                </li>
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
