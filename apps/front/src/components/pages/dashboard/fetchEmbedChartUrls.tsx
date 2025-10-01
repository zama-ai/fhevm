import config from "../../../config";

interface EmbedInfo {
  _id: string;
  token: string;
  [key: string]: any;
}

function customizeUrlDisplayOptions(embedInfo: EmbedInfo) {
  // see here
  // https://www.moesif.com/docs/embedded-templates/creating-and-using-templates/#display-options

  const displayOptions = {
    embed: "true",
    hide_header: "true",
    show_daterange: "true",
    primary_color: "#000",
  };

  return `https://www.moesif.com/public/em/ws/${
    embedInfo._id
  }?${new URLSearchParams(displayOptions).toString()}#${embedInfo.token}`;
}

export default async function fetchEmbedChartUrls({
  stripCustomerId: _stripCustomerId,
  authUserId,
  idToken,
  setError: _setError,
  email,
}: {
  stripCustomerId: string;
  authUserId: string;
  idToken: string;
  setError: (error: any) => void;
  email: string;
}) {
  const response = await fetch(
    `${config.devPortalApiServer}/embed-charts/` +
      encodeURIComponent(authUserId) +
      `?email=` +
      encodeURIComponent(email),
    {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${idToken}`,
      },
    }
  );

  if (!response.ok) {
    const errorBody = await response.text(); // or response.json() if the response is JSON
    console.error(
      `Failed to fetch embed charts: status=${response.status} ${response.statusText}, body=${errorBody}`
    );
    throw new Error(
      `HTTP error! status: ${response.status}, body: ${errorBody}`
    );
  }

  const embedInfoArray: EmbedInfo[] = await response.json();

  if (embedInfoArray) {
    return embedInfoArray.map((item) => {
      const customizedUrl = customizeUrlDisplayOptions(item);
      console.log("custom 1 " + customizedUrl);
      return customizedUrl;
    });
  } else {
    return [];
  }
}
