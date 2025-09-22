import React from "react";

import CommonTable from "../../common-table";
import { formatPrice, formatPeriod } from "../../../common/utils";

function formatNumberToHuman(input) {
  // Handle "inf" case
  if (input === "inf") {
    return "∞"; // Unicode infinity symbol
  }

  // Convert the input to a number
  const num = parseFloat(input);

  // Define the thresholds for the human-readable format
  const thresholds = [
    { value: 1000000, suffix: "M" },
    { value: 1000, suffix: "K" },
  ];

  // Find the appropriate threshold and format the number
  for (const { value, suffix } of thresholds) {
    if (num >= value) {
      return `${(num / value).toFixed(1)}${suffix}`;
    }
  }

  // If no threshold is met, return the original number
  return num.toString();
}

function TierTable(props) {
  const { tiers } = props;

  // const exampleTiers = [
  //   {
  //     up_to: 1000,
  //     unit_price_in_decimal: "0.05",
  //     flat_price_in_decimal: "0",
  //   },
  //   {
  //     up_to: "inf",
  //     unit_price_in_decimal: "0.02",
  //     flat_price_in_decimal: "0",
  //   },
  // ];

  const haveFlatFee = tiers.some((item) => !!item.flat_price_in_decimal);
  const haveUnitPrice = tiers.some((item) => !!item.unit_price_in_decimal);

  const haveBoth = haveFlatFee && haveUnitPrice;

  const data = tiers;

  let columns = [
    {
      header: "Units",
      accessor: "up_to",
      cell: ({ index, value, row }) => {
        return (
          <span>
            {data[index - 1]?.up_to
              ? formatNumberToHuman(data[index - 1]?.up_to)
              : 1}
            {" - "}
            {formatNumberToHuman(value)}
          </span>
        );
      },
      justifyContent: "flex-start",
    },
    {
      header: "",
      accessor: "id",
      cell: ({ index }) => <span className="price-operator">⟶</span>,
      width: "15px",
      justifyContent: "center",
    },
  ];

  if (haveBoth) {
    columns = [
      ...columns,
      {
        header: "/Unit",
        accessor: "unit_price_in_decimal",
        cell: ({ index, value, row }) => {
          return formatPrice(value);
        },
        justifyContent: "flex-end",
      },
      {
        header: "",
        accessor: "plus",
        cell: () => (
          <span
            className="price-operator"
            style={{
              paddingLeft: "10px",
            }}
          >
            {"+"}
          </span>
        ),
        width: "40px",
        justifyContent: "flex-end",
      },
      {
        header: <span>Flat Fee</span>,
        accessor: "flat_price_in_decimal",
        cell: ({ index, value, row }) => {
          return formatPrice(value);
        },
        justifyContent: "flex-end",
        width: "60px",
      },
    ];
  } else if (haveFlatFee) {
    columns = [
      ...columns,
      {
        header: <span>Flat Fee</span>,
        accessor: "flat_price_in_decimal",
        cell: ({ index, value, row }) => {
          return formatPrice(value);
        },
        justifyContent: "flex-end",
      },
    ];
  } else if (haveUnitPrice) {
    columns = [
      ...columns,
      {
        header: "/Unit",
        accessor: "unit_price_in_decimal",
        cell: ({ index, value, row }) => {
          return formatPrice(value);
        },
        justifyContent: "flex-end",
      },
    ];
  }

  return <CommonTable className="tier-table" data={tiers} columns={columns} />;
}

function PriceTile(props) {
  const { price, plan, actionButton, subscriptionPeriod } = props;

  return (
    <div className="price--tile">
      <div className="plan--content">
        {subscriptionPeriod && (
          <div className="plan--subscription-period">{subscriptionPeriod}</div>
        )}
        <div className="price-name">
          {price?.name || plan?.name || "Place Holder Plan"}
        </div>
        {price.tiers ? (
          <TierTable tiers={price.tiers} />
        ) : (
          <div className="single-price">
            <div>
              <span className="single-price--price">
                {formatPrice(price.price_in_decimal)}
              </span>{" "}
              <span className="single-price--unit">
                {price.pricing_model === "per_unit"
                  ? `/${plan?.unit || "unit"}`
                  : "flat fee"}
              </span>
            </div>
          </div>
        )}
      </div>
      <div className="plan--bottom">
        <div className="plan-period">
          {formatPeriod(price.period_units, price.period)}{" "}
          {plan?.name && `- ${plan?.name}`}
        </div>
        {actionButton}
      </div>
    </div>
  );
}

export default PriceTile;
