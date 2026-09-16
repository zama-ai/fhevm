/**
 * Scoped KMS node control for the `kms-context-qa-tests` profile.
 *
 * QA cases withhold quorums by taking KMS components offline: stopping one committee tx-sender
 * blocks an on-chain confirmation, stopping a whole party removes it from the MPC group. The
 * existing profiles do this with a hand-written `try`/`finally` at each call site
 * (`src/commands/kms-context-switch.ts:530-591`, `src/commands/kms-generation.ts:180-200`). That
 * works, but it puts the restore guarantee in the hands of every future case author, and a missed
 * `finally` leaves containers down for every subsequent case and for the developer's next run.
 *
 * {@link NodeSupervisor} centralizes the guarantee. It owns the scope helpers, and it also tracks
 * everything it has stopped so the profile runner can force a restore from its own outermost
 * `finally` even when a case failed in a way that skipped its scope exit.
 *
 * Party naming is never spelled inline — it comes from `src/kms-party.ts`, which encodes the rule
 * that party 1 keeps the bare container names and parties 2..N get a suffix.
 */
import { PreflightError } from "../errors";
import { dockerInspect } from "../flow/readiness";
import {
  partyContainers,
  setRunning,
  waitForContainersStopped,
  waitForPartiesRunning,
  waitForPartiesStopped,
} from "../commands/kms-generation";
import { kmsTxSenderName } from "../kms-party";
import type { CaseEvidence } from "./evidence";

/**
 * Fails fast when any listed party's tx-sender is not running.
 *
 * A context switch reaches its creation quorum only when **every** node of the new context has
 * confirmed on chain (`_hasContextCreationQuorum` in ProtocolConfig requires
 * `newTxSenderConfirmationCount == kmsNodesForContext.length`). A party whose tx-sender is down can
 * never confirm, so the context stays Pending forever and the activation wait burns its whole
 * budget before reporting a timeout that says nothing about the cause.
 *
 * This happens in practice: `kms-context-switch`'s node-swap step stops the dropped party's
 * tx-sender and deliberately leaves it down, so any stack that has run that profile is in exactly
 * this state.
 *
 * @throws PreflightError naming the stopped containers and what they block.
 */
export const assertTxSendersRunning = async (
  parties: readonly number[],
  evidence: CaseEvidence,
  why: string,
): Promise<void> => {
  await evidence.step(
    "node",
    "every party's tx-sender is running",
    { parties: parties.join(",") },
    async () => {
      const stopped: string[] = [];
      for (const party of parties) {
        const container = kmsTxSenderName(party);
        const [inspected] = await dockerInspect(container);
        if (inspected?.State.Status !== "running") {
          stopped.push(`${container} (${inspected?.State.Status ?? "missing"})`);
        }
      }
      if (stopped.length) {
        throw new PreflightError(
          `kms-context-qa: ${stopped.length} tx-sender(s) are not running: ${stopped.join(", ")}. ${why} ` +
            `A party that cannot transact never confirms, so the operation would stay Pending until the activation ` +
            `wait times out. Note that kms-context-switch's node-swap step stops the dropped party's tx-sender and ` +
            `leaves it down — start it with \`docker start <container>\`, or re-up the stack.`,
        );
      }
    },
  );
};

/**
 * Tracks and restores every container this profile takes offline.
 *
 * One instance per profile run, shared by all cases, so {@link NodeSupervisor.restoreAll} can act
 * as the final safety net.
 */
export class NodeSupervisor {
  /** Containers currently believed to be stopped by this profile, in the order they were stopped. */
  readonly #stopped = new Set<string>();

  /** Containers this supervisor stopped and has not yet restored. */
  get stoppedContainers(): readonly string[] {
    return [...this.#stopped];
  }

  /**
   * Stops `containers`, runs `task`, and restores them — even when `task` throws.
   *
   * The restore is best-effort and never masks the original error: a failure while restarting is
   * reported and swallowed, because the error that caused the scope to unwind is the one worth
   * surfacing. `setRunning` is already idempotent, so a double restore is harmless.
   */
  async withContainersStopped<T>(
    containers: readonly string[],
    label: string,
    evidence: CaseEvidence,
    task: () => Promise<T>,
  ): Promise<T> {
    await evidence.step("node", `stop ${label}`, { containers: containers.join(",") }, async () => {
      await setRunning([...containers], "stop");
      await waitForContainersStopped([...containers]);
      for (const container of containers) this.#stopped.add(container);
    });
    try {
      return await task();
    } finally {
      await this.#restoreContainers(containers, label, evidence);
    }
  }

  /**
   * Stops the tx-senders of `parties`, runs `task`, and restarts them.
   *
   * Stopping only the tx-sender leaves the party's core running, so the KMS still reshares while
   * the party cannot land its confirmation on chain — the lever for withholding an on-chain quorum
   * without disturbing the MPC group.
   */
  async withTxSendersStopped<T>(
    parties: readonly number[],
    label: string,
    evidence: CaseEvidence,
    task: () => Promise<T>,
  ): Promise<T> {
    const containers = parties.map((party) => kmsTxSenderName(party));
    await evidence.step(
      "node",
      `stop tx-sender(s) for ${label}`,
      { parties: parties.join(","), containers: containers.join(",") },
      async () => {
        await setRunning(containers, "stop");
        await waitForContainersStopped(containers);
        for (const container of containers) this.#stopped.add(container);
      },
    );
    try {
      return await task();
    } finally {
      await evidence
        .step(
          "node",
          `restart tx-sender(s) after ${label}`,
          { parties: parties.join(",") },
          async () => {
            await setRunning(containers, "start");
            await waitForPartiesRunning([...parties]);
            for (const container of containers) this.#stopped.delete(container);
          },
        )
        .catch((error: unknown) => {
          console.log(
            `[kms-context-qa][warn] could not restart tx-sender(s) for parties ${parties.join(",")}: ${String(error)}`,
          );
        });
    }
  }

  /**
   * Stops every container of `parties`, runs `task`, and restarts them.
   *
   * Use when a party must leave the MPC group entirely — for example to drive the live committee
   * below the 2t+1 reconstruction threshold.
   */
  async withPartiesStopped<T>(
    parties: readonly number[],
    label: string,
    evidence: CaseEvidence,
    task: () => Promise<T>,
  ): Promise<T> {
    const containers = parties.flatMap((party) => partyContainers(party));
    await evidence.step(
      "node",
      `stop part(y/ies) for ${label}`,
      { parties: parties.join(","), containerCount: String(containers.length) },
      async () => {
        await setRunning(containers, "stop");
        await waitForPartiesStopped([...parties]);
        for (const container of containers) this.#stopped.add(container);
      },
    );
    try {
      return await task();
    } finally {
      await evidence
        .step("node", `restart part(y/ies) after ${label}`, { parties: parties.join(",") }, async () => {
          // Reverse order so the first party stopped is the last restarted, mirroring the
          // unwinding order the existing profiles use.
          await setRunning([...containers].reverse(), "start");
          await waitForPartiesRunning([...parties]);
          for (const container of containers) this.#stopped.delete(container);
        })
        .catch((error: unknown) => {
          console.log(
            `[kms-context-qa][warn] could not restart part(y/ies) ${parties.join(",")}: ${String(error)}`,
          );
        });
    }
  }

  /** Restores `containers`, reporting but not rethrowing a failure. */
  async #restoreContainers(
    containers: readonly string[],
    label: string,
    evidence: CaseEvidence,
  ): Promise<void> {
    await evidence
      .step("node", `restart ${label}`, { containers: containers.join(",") }, async () => {
        await setRunning([...containers].reverse(), "start");
        for (const container of containers) this.#stopped.delete(container);
      })
      .catch((error: unknown) => {
        console.log(`[kms-context-qa][warn] could not restart ${containers.join(",")}: ${String(error)}`);
      });
  }

  /**
   * Final safety net: restarts anything still recorded as stopped.
   *
   * Called from the profile runner's outermost `finally`, so a case that failed between a stop and
   * its scope exit cannot leave the stack degraded for the next run. Never throws.
   */
  async restoreAll(): Promise<void> {
    const remaining = [...this.#stopped];
    if (!remaining.length) return;
    console.log(
      `[kms-context-qa] restoring ${remaining.length} container(s) left stopped: ${remaining.join(", ")}`,
    );
    try {
      await setRunning(remaining.reverse(), "start");
      this.#stopped.clear();
    } catch (error) {
      console.log(`[kms-context-qa][warn] restoreAll could not restart every container: ${String(error)}`);
    }
  }
}
