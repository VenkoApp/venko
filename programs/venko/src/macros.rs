//! Macros

/// Generates the signer seeds for a [crate::Stream].
#[macro_export]
macro_rules! stream_seeds {
    ($stream: expr) => {
        &[&[
            b"Stream" as &[u8],
            &$stream.mint.to_bytes(),
            &[$stream.bump],
        ]]
    };
}
