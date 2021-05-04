use std::hash::Hasher;

#[derive(Clone, Debug, Default)]
pub struct IdentityHasher {
    hash: u64,
}

impl Hasher for IdentityHasher {
    fn write(&mut self, bytes: &[u8]) {
        let mut u64_bytes = [0; 8];
        for (i, byte) in bytes.iter().enumerate().take(8) {
            u64_bytes[i] = *byte;
        }
        self.hash = u64::from_ne_bytes(u64_bytes);
    }

    fn finish(&self) -> u64 {
        self.hash
    }
}

#[cfg(test)]
mod tests {
    use crate::IdentityHasher;
    use std::hash::Hasher;

    #[test]
    fn write() {
        let mut hasher = IdentityHasher::default();

        hasher.write(&42u64.to_ne_bytes());

        assert_eq!(hasher.finish(), 42);
    }

    #[test]
    fn write_less_than_8_bytes() {
        let mut hasher = IdentityHasher::default();

        hasher.write(&42u8.to_ne_bytes());

        assert_eq!(hasher.finish(), 42);
    }

    #[test]
    fn write_more_than_8_bytes() {
        let mut hasher = IdentityHasher::default();

        hasher.write(&42u128.to_ne_bytes());

        assert_eq!(hasher.finish(), 43);
    }
}
