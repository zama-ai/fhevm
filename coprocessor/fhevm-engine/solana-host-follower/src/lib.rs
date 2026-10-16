//! Follows the zama-host program on a Solana cluster and decodes what it executed. The
//! coprocessor's Solana host listener and the Merkle indexer each run one, with their own sink.

use std::time::Duration;

use solana_client::client_error::{ClientError, ClientErrorKind};

mod args;
mod follower;
pub mod host;
mod source;

pub use args::{Follower, FollowerArgs};
pub use follower::{
    block_checkpoint, run, track_finalized_slot, BlockCheckpoint, BlockSink,
    FollowerConfig, IngestFailure, PreparedBlock, PreparedTransaction,
    StartPosition,
};
pub use source::SealedBlock;

/// Timeout of one JSON-RPC request to the live or the archive endpoint.
pub const SOLANA_RPC_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// `url` as its origin: scheme, host and port. A provider's URL can carry an API key in its path,
/// query or userinfo, so a log shows only this.
pub fn url_origin(url: &str) -> String {
    url::Url::parse(url).map_or_else(
        |_| "<unparsable URL>".to_owned(),
        |url| url.origin().ascii_serialization(),
    )
}

/// `error` without its request URL. A hosted Solana RPC URL usually carries its API key, and
/// reqwest errors print the URL.
pub fn without_url(error: ClientError) -> ClientError {
    let ClientError { request, kind } = error;
    let kind = match *kind {
        ClientErrorKind::Reqwest(error) => {
            ClientErrorKind::Reqwest(error.without_url())
        }
        kind => kind,
    };
    ClientError {
        request,
        kind: Box::new(kind),
    }
}

#[cfg(test)]
mod tests {
    use super::url_origin;

    #[test]
    fn url_origin_keeps_scheme_host_and_port_only() {
        assert_eq!(
            url_origin("https://USER:PASSWORD@provider.example:8443/v2/PATH-KEY?api-key=QUERY-KEY"),
            "https://provider.example:8443"
        );
        assert_eq!(
            url_origin("http://127.0.0.1:10000"),
            "http://127.0.0.1:10000"
        );
        assert_eq!(url_origin("not a url"), "<unparsable URL>");
    }
}
