import {
  redirect,
  useOutletContext,
  type LoaderFunctionArgs,
} from "react-router";
import type { Auth0User } from "~/types/auth";
import type { Route } from "../../../routes/+types/dashboard";
import { requireSubscription } from "~/lib/logic.server";

export async function loader({ request }: LoaderFunctionArgs) {
  const { getInfoForEmbeddedWorkspaces } = await import(
    "~/features/dashboard/moesif.server"
  );
  const {
    customer: { id },
  } = await requireSubscription(request);

  if (!id) {
    return { embeddedInfo: [] };
  }
  const embeddedInfo = await getInfoForEmbeddedWorkspaces(id);

  return { embeddedInfo };
}

export default function Dashboard({ loaderData }: Route.ComponentProps) {
  const { embeddedInfo } = loaderData;
  const { user } = useOutletContext<{ user: Auth0User }>();

  return (
    <div className="bg-gray-50 rounded-lg p-4">
      <h3 className="text-lg font-semibold mb-3">User Information</h3>
      <div className="space-y-2">
        {user.picture && (
          <div className="mb-4">
            <img
              src={user.picture}
              alt={user.name || "User"}
              className="w-16 h-16 rounded-full"
            />
          </div>
        )}
        <div>
          <span className="font-medium text-gray-700">Name:</span>{" "}
          <span className="text-gray-900">{user.name || "N/A"}</span>
        </div>
        <div>
          <span className="font-medium text-gray-700">Email:</span>{" "}
          <span className="text-gray-900">{user.email || "N/A"}</span>
        </div>
        <div>
          <span className="font-medium text-gray-700">User ID:</span>{" "}
          <span className="text-gray-900 text-sm">{user.sub || "N/A"}</span>
        </div>
      </div>
      <div className="grid gap-4 grid-cols-1 lg:grid-cols-2 mt-6">
        {embeddedInfo.map((info, idx) => (
          <iframe
            key={info._id}
            title={`Embedded Workspace ${idx + 1}`}
            src={info.url}
            id={info._id}
            name="preview-frame"
            className="w-full aspect-16/9 border-0"
          />
        ))}
      </div>
    </div>
  );
}
