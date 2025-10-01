import safeGet from "lodash/get";
import type { ReactNode, Ref, CSSProperties } from "react";

function convertToPx(x: CSSProperties["width"] | CSSProperties["height"]) {
  if (x === undefined || x === null) return x;

  return typeof x === "number" ? x.toString() + "px" : x.toString();
}

type TableHeaderProps = {
  columnField: string;
  header: ReactNode | ((props: { data: any[] }) => string);
  data: any[]; // PropTypes.array,
  alignRight?: boolean;
  contentType?: string;
  justify?: string;
  minRowHeight?: CSSProperties["minHeight"];
};

function TableHeader({
  columnField,
  header,
  data,
  alignRight,
  contentType,
  justify,
}: TableHeaderProps) {
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
}

export type CommonTableProps = {
  data: any[];
  columns: Column[]; // PropTypes.array.isRequired,
  overlayTrigger?: boolean;
  defaultSortColumnAccessor?: string;
  minRows?: number;
  minRowHeight?: CSSProperties["minHeight"];
  emptyState?: ReactNode;
  children?: ReactNode;
  style?: CSSProperties;
  className?: string;
  withBorder?: boolean;
  tableRef?: Ref<HTMLTableElement>;
  rowRef?: Ref<HTMLTableRowElement>;
};

type Column = {
  header: ReactNode;
  accessor: string | ((props: { row: any; index: number }) => string);
  cell: (props: {
    index: number;
    value: any;
    row: number;
    data: any;
  }) => ReactNode;
  justifyContent?: CSSProperties["justifyContent"];
  width?: CSSProperties["width"];
  minWidth?: CSSProperties["width"];
  alignRight?: boolean;
  contentType?: string;
};

function CommonTable({
  data,
  columns,
  overlayTrigger,
  children,
  style,
  minRows = 3,
  minRowHeight,
  emptyState,
  className = "",
  withBorder,
  tableRef,
  rowRef,
}: CommonTableProps) {
  let displayData: any[] = [];

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
            {columns.map((col, idx) => (
              <TableHeader
                key={
                  typeof col.accessor === "string"
                    ? col.accessor
                    : `column-${idx}`
                }
                columnField={
                  typeof col.accessor === "string"
                    ? col.accessor
                    : `column-${idx}`
                }
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

                let colKey = idx.toString();

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
