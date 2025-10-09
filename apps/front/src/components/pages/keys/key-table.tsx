import { useApiKeys } from "../../../hooks/useApiKeys";
import NoticeBox from "../../notice-box";
import apiKeyIcon from "../../../images/icons/api-key.svg";
import { KeyTableRow } from "./key-table-row";
import { PageLoader } from "../../page-loader";

export function KeyTable() {
  const { apiKeys, isLoading, error } = useApiKeys();

  if (isLoading) {
    return <PageLoader />;
  }

  if (error) {
    return (
      <NoticeBox
        iconSrc={apiKeyIcon}
        title="Something went wrong"
        description={error.message}
      />
    );
  }
  return (
    <table className="table">
      <thead>
        <tr>
          <th style={{ width: "50%" }}>API Key</th>
          <th>Created At</th>
          <th>Expires At</th>
          <th style={{ width: "110px", maxWidth: "110px" }} />
        </tr>
      </thead>
      <tbody>
        {apiKeys?.length ? (
          apiKeys.map((apiKey) => (
            <KeyTableRow key={apiKey.id} apiKey={apiKey} />
          ))
        ) : (
          <tr>
            <td colSpan={4} style={{ textAlign: "center" }}>
              <i>No API Key found</i>
            </td>
          </tr>
        )}
      </tbody>
    </table>
  );
}
