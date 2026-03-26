use alloy::contract::Error as ContractError;

/// Stringify an alloy contract error, stripping RPC endpoint URLs.
///
/// `reqwest::Error::Display` appends ` for url (https://...)` which can leak
/// RPC endpoints (possibly with embedded API keys) into logs and HTTP
/// responses. This walks the source chain, finds the `reqwest::Error`, and
/// strips the URL suffix using `reqwest::Error::url()`.
pub fn redact_alloy_error(err: &ContractError) -> String {
    match err {
        ContractError::TransportError(rpc_err) => {
            let mut source: Option<&(dyn std::error::Error + 'static)> = Some(rpc_err);
            while let Some(err) = source {
                if let Some(reqwest_err) = err.downcast_ref::<reqwest::Error>() {
                    return strip_url_suffix(reqwest_err);
                }
                source = err.source();
            }
            // Non-HTTP transport (BackendGone, PubsubUnavailable, etc.) — no URLs.
            rpc_err.to_string()
        }
        other => other.to_string(),
    }
}

/// Strip the ` for url (…)` suffix that `reqwest::Error::Display` appends.
fn strip_url_suffix(err: &reqwest::Error) -> String {
    let msg = err.to_string();
    match err.url() {
        Some(url) => {
            let suffix = format!(" for url ({url})");
            msg.strip_suffix(&suffix).unwrap_or(&msg).to_owned()
        }
        None => msg,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_transport_error_passes_through() {
        let err = ContractError::AbiError(alloy::dyn_abi::Error::SolTypes(
            alloy::sol_types::Error::Other("test error".into()),
        ));
        assert_eq!(redact_alloy_error(&err), err.to_string());
    }
}
