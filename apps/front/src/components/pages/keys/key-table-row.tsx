import copy from "copy-to-clipboard";
import { useCallback, useState } from "react";

import type { ApiKey } from "../../../types/api-key";
import { useDeleteApiKey } from "../../../hooks/useDeleteApiKey";
import TrashIcon from "../../../images/icons/trash";
import Eye from "../../../images/icons/eye";
import Hidden from "../../../images/icons/hidden";
import Copy from "../../../images/icons/copy";
import SuccessIcon from "../../../images/icons/success";
import CheckIcon from "../../../images/icons/check";
import CrossIcon from "../../../images/icons/cross";

interface Props {
  apiKey: ApiKey;
}

const MASKED = "********************************";

function formatDateFromSeconds(seconds: number): string {
  const locale = navigator?.language ?? "en-US";
  const dt = new Date(seconds * 1000);
  const formatter = new Intl.DateTimeFormat(locale, {
    year: "numeric",
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
    timeZoneName: "short",
  });
  return formatter.format(dt);
}

export function KeyTableRow({ apiKey }: Props) {
  const { deleteApiKey } = useDeleteApiKey();
  const [masked, setMasked] = useState(true);
  const [isCopied, setIsCopied] = useState(false);
  const [showConfirm, setShowConfirm] = useState(false);

  const copyApiKey = useCallback(() => {
    const successfulCopy = copy(apiKey.key);
    if (successfulCopy) {
      setIsCopied(true);
      setTimeout(() => setIsCopied(false), 3_000);
    }
  }, [apiKey, setIsCopied]);

  return (
    <tr key={apiKey.id}>
      <td>
        <div className="flex border border-solid border-black rounded-md w-full align-center">
          <button
            className="bg-black text-zama-yellow border-none"
            onClick={() => {
              setMasked(() => !masked);
            }}
          >
            {masked ? (
              <Eye width="15px" height="13.5px" />
            ) : (
              <Hidden width="15px" height="13.5px" />
            )}
          </button>
          <div className="flex-1 border-x border-x-solid">
            <pre className="text-center">{masked ? MASKED : apiKey.key}</pre>
          </div>
          <button
            className="bg-black text-zama-yellow border-none"
            onClick={copyApiKey}
          >
            {isCopied ? (
              <SuccessIcon height="1.5rem" width="1.5rem" />
            ) : (
              <Copy height="1.5rem" width="1.5rem" />
            )}
          </button>
        </div>
      </td>
      <td className="text-align">{formatDateFromSeconds(apiKey.created_at)}</td>
      <td className="text-align">
        {apiKey.ttl
          ? formatDateFromSeconds(apiKey.ttl + apiKey.created_at)
          : "Never"}
      </td>
      <td>
        {showConfirm ? (
          <form
            style={{
              display: "flex",
              flexDirection: "row",
              gap: "5px",
              alignItems: "center",
            }}
          >
            Sure?
            <button
              className="button button__link"
              onClick={() => deleteApiKey(apiKey.id)}
            >
              <CheckIcon width="1.5rem" height="1.5rem" />
            </button>
            <button
              className="button button__link"
              onClick={() => setShowConfirm(false)}
            >
              <CrossIcon width="1.5rem" height="1.5rem" />
            </button>
          </form>
        ) : (
          <button
            className="button button--primary"
            // onClick={() => deleteApiKey(apiKey.id)}
            onClick={() => setShowConfirm(true)}
          >
            Delete <TrashIcon height="1.5rem" width="1.5rem" />
          </button>
        )}
      </td>
    </tr>
  );
}
