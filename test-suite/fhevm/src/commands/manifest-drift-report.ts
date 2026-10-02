/** Capture the complete drift inventory before scenario cleanup resumes normal operation. */
export async function captureDriftTables(
  query: (index: number, sql: string) => Promise<string>,
) {
  const snapshots = [];
  for (const node of [0, 1, 2]) {
    try {
      const snapshot = JSON.parse(await query(node, `SELECT json_build_object(
        'total', count(*),
        'healed', count(*) FILTER (WHERE healed_at IS NOT NULL),
        'pendingHealable', count(*) FILTER (WHERE can_be_healed AND healed_at IS NULL),
        'rows', COALESCE(json_agg(d ORDER BY id), '[]'::json))::text
        FROM drifted_handle d`));
      snapshots.push({ node, ...snapshot });
      console.log(`[manifest-drift] node ${node}: ${JSON.stringify(snapshot)}`);
    } catch (error) {
      // Diagnostics must not prevent cleanup or hide the original test failure.
      snapshots.push({ node, error: String(error) });
      console.error(`[manifest-drift] node ${node}: dump failed: ${error}`);
    }
  }
  return snapshots;
}
