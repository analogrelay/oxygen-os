use thiserror::Error;
use crate::memory::page::Frame;

#[derive(Debug, Error)]
pub enum Error {
    #[error("page is already mapped to frame {0}")]
    PageAlreadyMapped(Frame),

    #[error("frame allocation failed")]
    FrameAllocationFailed,

    #[error("{0}")]
    Other(&'static str),
}

#[derive(Error, Debug, Eq, PartialEq)]
#[error("address is not properly aligned")]
pub struct NotAlignedError;
