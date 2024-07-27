#[cfg(feature = "alloc")]
extern crate alloc;

mod hex;

#[cfg(feature = "pkcs8")]
pub mod pkcs8;

#[cfg(feature = "serde")]
mod serde;

pub use signature::{self, Error, SignatureEncoding};

#[cfg(feature = "pkcs8")]
pub use crate::pkcs8::{KeypairBytes, PublicKeyBytes};

use core::fmt;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Size of a single component of an Ed25519 signature.
const COMPONENT_SIZE: usize = 32;

/// Size of an `R` or `s` component of an Ed25519 signature when serialized
/// as bytes.
pub type ComponentBytes = [u8; COMPONENT_SIZE];

/// Ed25519 signature serialized as a byte array.
pub type SignatureBytes = [u8; Signature::BYTE_SIZE];

/// Ed25519 signature.
///
/// This type represents a container for the byte serialization of an Ed25519
/// signature, and does not necessarily represent well-formed field or curve
/// elements.
///
/// Signature verification libraries are expected to reject invalid field
/// elements at the time a signature is verified.
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct Signature {
    R: ComponentBytes,
    s: ComponentBytes,
}

impl Signature {
    /// Size of an encoded Ed25519 signature in bytes.
    pub const BYTE_SIZE: usize = COMPONENT_SIZE * 2;

    /// Parse an Ed25519 signature from a byte slice.
    pub fn from_bytes(bytes: &SignatureBytes) -> Self {
        let mut R = ComponentBytes::default();
        let mut s = ComponentBytes::default();

        let components = bytes.split_at(COMPONENT_SIZE);
        R.copy_from_slice(components.0);
        s.copy_from_slice(components.1);

        Self { R, s }
    }

    /// Parse an Ed25519 signature from its `R` and `s` components.
    pub fn from_components(R: ComponentBytes, s: ComponentBytes) -> Self {
        Self { R, s }
    }

    /// Parse an Ed25519 signature from a byte slice.
    ///
    /// # Returns
    /// - `Ok` on success
    /// - `Err` if the input byte slice is not 64-bytes
    pub fn from_slice(bytes: &[u8]) -> signature::Result<Self> {
        SignatureBytes::try_from(bytes)
            .map(Into::into)
            .map_err(|_| Error::new())
    }

    /// Bytes for the `R` component of a signature.
    pub fn r_bytes(&self) -> &ComponentBytes {
        &self.R
    }

    /// Bytes for the `s` component of a signature.
    pub fn s_bytes(&self) -> &ComponentBytes {
        &self.s
    }

    /// Return the inner byte array.
    pub fn to_bytes(&self) -> SignatureBytes {
        let mut ret = [0u8; Self::BYTE_SIZE];
        let (R, s) = ret.split_at_mut(COMPONENT_SIZE);
        R.copy_from_slice(&self.R);
        s.copy_from_slice(&self.s);
        ret
    }

    /// Convert this signature into a byte vector.
    #[cfg(feature = "alloc")]
    pub fn to_vec(&self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}

impl SignatureEncoding for Signature {
    type Repr = SignatureBytes;

    fn to_bytes(&self) -> SignatureBytes {
        self.to_bytes()
    }
}

impl From<Signature> for SignatureBytes {
    fn from(sig: Signature) -> SignatureBytes {
        sig.to_bytes()
    }
}

impl From<&Signature> for SignatureBytes {
    fn from(sig: &Signature) -> SignatureBytes {
        sig.to_bytes()
    }
}

impl From<SignatureBytes> for Signature {
    fn from(bytes: SignatureBytes) -> Self {
        Signature::from_bytes(&bytes)
    }
}

impl From<&SignatureBytes> for Signature {
    fn from(bytes: &SignatureBytes) -> Self {
        Signature::from_bytes(bytes)
    }
}

impl TryFrom<&[u8]> for Signature {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> signature::Result<Self> {
        Self::from_slice(bytes)
    }
}

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ed25519::Signature")
            .field("R", &hex::ComponentFormatter(self.r_bytes()))
            .field("s", &hex::ComponentFormatter(self.s_bytes()))
            .finish()
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", self)
    }
}