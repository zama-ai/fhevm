import React from "react";
import noPriceIcon from "../../../images/icons/empty-state-price.svg";
import NoticeBox from "../../notice-box";

function NoPriceFound(props) {
  return (
    <NoticeBox
      iconSrc={noPriceIcon}
      title="No Prices Found"
      description={
        <>
          Plan pricing options will appear here when you create stripe plans
          using the{" "}
          <a
            href="https://www.moesif.com/docs/product-catalog/"
            target="_blank"
            rel="noreferrer noopener"
          >
            Product Catalogue
          </a>{" "}
          feature in{" "}
          <a
            href="https://www.moesif.com"
            target="_blank"
            rel="noreferrer noopener"
          >
            Moesif
          </a>
          . Sign in to get started.
        </>
      }
      actions={
        <>
          <a
            href="https://www.moesif.com/docs/product-catalog/"
            target="_blank"
            rel="noreferrer noopener"
          >
            <button className="button button__link">See Docs</button>
          </a>
          <a
            href="https://www.moesif.com"
            target="_blank"
            rel="noreferrer noopener"
          >
            <button className="button button--outline-secondary">
              Go to Moesif
            </button>
          </a>
        </>
      }
    />
  );
}

export default NoPriceFound;
