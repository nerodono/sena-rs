/// # Bidirectional communication with channels, in request-response format
pub mod comm;

/// # Message sent through the [`comm`] primitives
pub mod message;

/// # Shutdown token
pub mod shutdown;

#[cfg(feature = "tokio")]
/// # implementation for the tokio channels
pub mod tokio;
