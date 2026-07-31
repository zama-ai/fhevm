//! Canonical text rendering.
//!
//! A total, deterministic function of validated fields: it takes `PermitFields`,
//! which cannot exist unvalidated, and returns the text — no `Result`, because
//! there is nothing left to reject. The text is printable ASCII plus line feeds,
//! its line order is fixed, its final line carries no line feed, integers are
//! decimal without leading zeros, and the timestamp is rendered to the second.
//!
//! An empty ACL-domain list renders as one explicit line naming the permissive
//! breadth, rather than as an empty enumeration block, so a human signer sees how
//! wide the grant is.
//!
//! The text is assembled as a list of lines joined by a single line feed. That is not
//! a stylistic choice: it is what makes "the final line carries no line feed" a
//! property of the construction rather than a rule someone has to remember when
//! appending a line.

use crate::{
    fingerprint::transport_key_fingerprint,
    types::{Identity, KmsRouting, PermitFields},
};

/// First line: names the protocol and the version of this text form. A reader who
/// cannot parse the rest still learns what they were shown.
const HEADER: &str = "Zama fhevm Solana user-decrypt permit v1";
/// The one line an empty domain list renders as.
const PERMISSIVE_DOMAINS_LINE: &str = "ACL domains: ALL (permissive)";

/// Renders the canonical text a wallet signs.
pub fn render_canonical_text(fields: &PermitFields) -> String {
    let mut lines = vec![
        HEADER.to_string(),
        format!("User: {}", base58(fields.user_pubkey().as_bytes())),
        format!(
            "Verifying program: {}",
            base58(fields.verifying_program_id().as_bytes())
        ),
        format!("Chain id: {}", fields.chain_id()),
        // The key itself does not fit a wallet screen, so the text commits to a digest of
        // it — recomputed here from the full key, never taken as an input.
        format!(
            "Transport key (SHAKE-256): {}",
            base58(&transport_key_fingerprint(fields.transport_key()))
        ),
    ];

    // The routing lines belong to the routing version, which is why they are produced by
    // the match rather than by the template around it. A future version adds an arm that
    // owns its own lines, and has to make them distinguishable from these; it cannot
    // inherit this arm's text by omission.
    match fields.extra_data() {
        KmsRouting::ContextAndEpoch {
            kms_context_id,
            kms_epoch_id,
        } => {
            lines.push(format!(
                "KMS context: {}",
                base58(kms_context_id.as_bytes())
            ));
            lines.push(format!("KMS epoch: {}", base58(kms_epoch_id.as_bytes())));
        }
    }

    lines.push(format!(
        "Valid from: {} for {} seconds",
        render_timestamp(fields.start_timestamp()),
        fields.duration_seconds()
    ));

    let domains = fields.allowed_acl_domain_keys();
    if domains.is_permissive() {
        lines.push(PERMISSIVE_DOMAINS_LINE.to_string());
    } else {
        lines.push(format!("ACL domains ({}):", domains.as_slice().len()));
        lines.extend(
            domains
                .as_slice()
                .iter()
                .map(|key: &Identity| format!("- {}", base58(key.as_bytes()))),
        );
    }

    lines.join("\n")
}

/// Base58, Bitcoin alphabet: the encoding every Solana identity is displayed in, so the
/// text shows a signer the same string their explorer and their wallet do.
fn base58(bytes: &[u8]) -> String {
    bs58::encode(bytes).into_string()
}

/// Renders unix seconds as `YYYY-MM-DDTHH:MM:SSZ`.
///
/// Fixed width to the second, zero-padded, no fractional part and no offset other than
/// `Z`. Total for every `u64`: past the typed form's bound the year field simply grows
/// wider, so there is no input for which this fails — the bound is what keeps the width
/// fixed, not what keeps the function defined.
fn render_timestamp(unix_seconds: u64) -> String {
    const SECONDS_PER_MINUTE: u64 = 60;
    const SECONDS_PER_HOUR: u64 = 60 * SECONDS_PER_MINUTE;
    const SECONDS_PER_DAY: u64 = 24 * SECONDS_PER_HOUR;

    let (year, month, day) = civil_from_days(unix_seconds / SECONDS_PER_DAY);
    let seconds_of_day = unix_seconds % SECONDS_PER_DAY;
    let hour = seconds_of_day / SECONDS_PER_HOUR;
    let minute = (seconds_of_day % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE;
    let second = seconds_of_day % SECONDS_PER_MINUTE;

    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

/// Converts days since 1970-01-01 into a proleptic Gregorian calendar date.
///
/// Howard Hinnant's `civil_from_days`, restricted to non-negative days — which is all the
/// typed form admits, since the validity window starts at or after the epoch. Written out
/// rather than taken from a date library because five implementations have to agree on
/// this arithmetic, and a shared 40-line algorithm is easier to agree on than five
/// libraries' opinions about calendars, locales and leap seconds. Unix time has no leap
/// seconds, so every day here is exactly 86400 seconds long.
///
/// The constants are the algorithm's: 146097 days per 400-year era, 719468 days from
/// 0000-03-01 to 1970-01-01, and the 153/5 pair that walks a March-first month table.
fn civil_from_days(days: u64) -> (u64, u64, u64) {
    // Shift the epoch to 0000-03-01, so that a leap day lands at the end of a year and
    // the month table needs no special case for February.
    let shifted = days + 719_468;
    let era = shifted / 146_097;
    let day_of_era = shifted - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_position = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_position + 2) / 5 + 1;

    // Back from the March-first year to the calendar year.
    let month = if month_position < 10 {
        month_position + 3
    } else {
        month_position - 9
    };
    let year = year_of_era + era * 400 + u64::from(month <= 2);

    (year, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The inverse conversion, written independently of the one under test: it walks the
    /// calendar forward from a date instead of decomposing a day count. Only used to
    /// check that the forward direction lands where it came from.
    fn days_from_civil(year: u64, month: u64, day: u64) -> u64 {
        let year = year - u64::from(month <= 2);
        let era = year / 400;
        let year_of_era = year - era * 400;
        let month_position = if month > 2 { month - 3 } else { month + 9 };
        let day_of_year = (153 * month_position + 2) / 5 + day - 1;
        let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
        era * 146_097 + day_of_era - 719_468
    }

    /// Every day the typed form admits converts to a date that converts back to the same
    /// day. This is the property a table of nine expected strings cannot give: it visits
    /// every month length, every leap year, every century boundary and the year-9999 edge.
    ///
    /// Dense over the first century and a half — where every permit anyone will actually
    /// sign lands — then strided by a prime, so the sample is not aligned to any period
    /// of the calendar.
    #[test]
    fn every_admitted_day_round_trips_through_the_calendar() {
        const LAST_DAY: u64 = crate::types::MAX_START_TIMESTAMP / 86_400;

        let dense = 0..=64_000u64; // 1970-01-01 through 2145
        let strided = (64_001..=LAST_DAY).step_by(1_009);
        let last_years = (LAST_DAY - 4_000)..=LAST_DAY;

        let mut checked = 0u64;
        for days in dense.chain(strided).chain(last_years) {
            let (year, month, day) = civil_from_days(days);
            assert!(
                (1..=12).contains(&month),
                "day {days} produced month {month}"
            );
            assert!((1..=31).contains(&day), "day {days} produced day {day}");
            assert_eq!(
                days_from_civil(year, month, day),
                days,
                "day {days} rendered as {year:04}-{month:02}-{day:02}, which is a different day"
            );
            checked += 1;
        }
        // Pinned exactly rather than as a lower bound: a sample that quietly shrinks
        // reports a number here instead of still passing.
        assert_eq!(checked, 70_846, "the sample size changed");
    }

    /// Dates advance by exactly one day at a time: no repeated dates, no gaps, no
    /// backwards steps anywhere in the admitted range.
    #[test]
    fn consecutive_days_are_consecutive_dates() {
        let mut previous = civil_from_days(0);
        for days in 1..=64_000u64 {
            let current = civil_from_days(days);
            assert!(
                current > previous,
                "day {days} is not after day {}: {previous:?} then {current:?}",
                days - 1
            );
            let (year, month, day) = current;
            let (previous_year, previous_month, previous_day) = previous;
            let continues_the_month =
                year == previous_year && month == previous_month && day == previous_day + 1;
            let starts_the_next_month =
                day == 1 && year == previous_year && month == previous_month + 1;
            let starts_the_next_year = day == 1 && month == 1 && year == previous_year + 1;
            assert!(
                continues_the_month || starts_the_next_month || starts_the_next_year,
                "day {days} is neither the next day of its month, nor the first of the \
                 next month, nor the first of the next year: {previous:?} then {current:?}"
            );
            previous = current;
        }
    }

    /// The last second of the admitted range is the last second of year 9999 — the
    /// boundary the typed form's bound is chosen to sit on.
    #[test]
    fn the_timestamp_bound_is_the_end_of_year_9999() {
        assert_eq!(
            render_timestamp(crate::types::MAX_START_TIMESTAMP),
            "9999-12-31T23:59:59Z"
        );
        assert_eq!(render_timestamp(0), "1970-01-01T00:00:00Z");
    }

    /// No `u64` makes the timestamp rendering fail, including values the typed form
    /// refuses and the very top of the range.
    ///
    /// Totality is what lets the renderer return a text rather than a result, and this is
    /// the only place the claim can be checked: strict decoding never lets a timestamp
    /// past its bound reach the renderer through the public API. It also guards the
    /// calendar arithmetic against a debug-mode overflow panic, which would turn the
    /// "cannot fail" function into one that aborts.
    #[test]
    fn the_timestamp_rendering_is_total_over_every_u64() {
        for unix_seconds in [
            crate::types::MAX_START_TIMESTAMP + 1,
            1_000_000_000_000,
            u64::MAX / 2,
            u64::MAX,
        ] {
            let rendered = render_timestamp(unix_seconds);
            assert!(
                rendered.ends_with('Z') && rendered.len() >= 20,
                "{unix_seconds} rendered as {rendered:?}"
            );
        }
    }
}
