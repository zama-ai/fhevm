import React from "react";
import PropTypes from "prop-types";
import safeGet from "lodash/get";

function convertToPx(x) {
  if (x === undefined || x === null) return x;

  return typeof x === "number" ? x.toString() + "px" : x.toString();
}

const TableHeader = (props) => {
  const {
    columnField,
    header,
    data,
    alignRight,
    contentType,
    justify,
  } = props;

  const justifyContent =
    justify ||
    (contentType === "number" || contentType === "date" ? "flex-end" : "");

  let displayHeader;
  if (header === undefined) {
    displayHeader = columnField && columnField.toString();
  } else if (typeof header === "function") {
    displayHeader = header({ data });
  } else {
    displayHeader = header;
  }

  if (alignRight) {
    return (
      <th
        style={{
          textAlign: "right",
          fontWeight: 500,
          color: "#444",
          justifyContent,
        }}
      >
        {displayHeader}
      </th>
    );
  }

  return <th style={{ display: "flex", justifyContent }}>{displayHeader}</th>;
};

const FullRowComponent = ({ children, className }) => (
  <tr className={className}>
    <td className="d-flex">{children}</td>
  </tr>
);

FullRowComponent.propTypes = {
  children: PropTypes.any,
  className: PropTypes.string,
};

function CommonTable(props) {
  const {
    data,
    columns,
    overlayTrigger,
    children,
    style,
    minRows,
    minRowHeight,
    emptyState,
    className = "",
    withBorder,
    tableRef,
    rowRef,
  } = props;
  let displayData = [];

  if (data) {
    displayData = [...data];
  }

  while (displayData.length < minRows) {
    displayData.push(null);
  }

  const widths = columns
    .map((col) => {
      const widthPx = convertToPx(col.width);
      const minWidthPx = convertToPx(col.minWidth);
      switch (true) {
        case !!(widthPx && minWidthPx):
          return `minmax(${minWidthPx}, ${widthPx})`;
        case !!widthPx:
          return widthPx;
        case !!minWidthPx:
          return `minmax(${minWidthPx}, 1fr)`;
        default:
          return "minmax(0, 1fr)";
      }
    })
    .join(" ");

  const borderClass = withBorder ? "common-table--with-border" : "";
  return (
    <>
      <table
        ref={tableRef}
        className={`common-table ${className} ${borderClass}`}
        style={style}
      >
        {emptyState}
        <tbody>
          <tr
            className="common-table__header"
            style={{ gridTemplateColumns: widths }}
          >
            {columns.map((col) => (
              <TableHeader
                key={col.accessor}
                columnField={col.accessor}
                header={col.header}
                data={displayData}
                alignRight={col.alignRight}
                justify={col.justifyContent}
                contentType={col.contentType}
                minRowHeight={minRowHeight}
              />
            ))}
          </tr>

          {displayData.map((row, index) => (
            <tr
              key={row?.id || index}
              ref={index === 0 ? rowRef : null}
              style={{ gridTemplateColumns: widths }}
              className={overlayTrigger ? "overlay-button-trigger" : ""}
            >
              {columns.map((col, idx) => {
                const justifyContent =
                  col.justifyContent ||
                  (col.contentType === "number" || col.contentType === "date"
                    ? "flex-end"
                    : "");
                if (row === null) {
                  return (
                    <td
                      key={
                        typeof col.accessor === "string" ? col.accessor : idx
                      }
                      style={{
                        minHeight: minRowHeight,
                        justifyContent,
                        display: "flex",
                      }}
                    />
                  );
                }

                let content = null;

                let colKey = idx;

                if (col.accessor instanceof Function) {
                  content = safeGet(row, col.accessor({ row, index }));
                } else if (col.accessor) {
                  colKey = col.accessor;
                  content = safeGet(row, col.accessor);
                }

                if (col.cell) {
                  content = (
                    <td
                      key={colKey}
                      style={{
                        minHeight: minRowHeight,
                        justifyContent,
                        display: "flex",
                      }}
                    >
                      {col.cell({
                        value: content,
                        row,
                        index,
                        data: displayData,
                      })}
                    </td>
                  );
                } else {
                  content = (
                    <td
                      key={colKey}
                      style={{
                        minHeight: minRowHeight,
                        justifyContent,
                        display: "flex",
                      }}
                    >
                      {content}
                    </td>
                  );
                }

                return content;
              })}
            </tr>
          ))}

          {children}
        </tbody>
      </table>
    </>
  );
}

export default CommonTable;

CommonTable.propTypes = {
  data: PropTypes.array.isRequired,
  columns: PropTypes.array.isRequired,
  overlayTrigger: PropTypes.bool,
  defaultSortColumnAccessor: PropTypes.string,
  minRows: PropTypes.number,
  minRowHeight: PropTypes.string,
  emptyState: PropTypes.element,
  children: PropTypes.object,
  style: PropTypes.object,
  className: PropTypes.string,
  withBorder: PropTypes.bool,
};
