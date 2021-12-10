use super::node::Node;
use smallnum::SmallUnsignedLabel;

// Size-optimized Node Trait -------------------------------------------------------------------------------------------

/// Interface encapsulates `U`.
pub trait SmallNode<K, V: Default> {
    /// Get key.
    fn key(&self) -> &K;

    /// Set key.
    fn set_key(&mut self, key: K);

    // Take key, replacing current with `K::Default()`.
    fn take_key(&mut self) -> K;

    /// Get value.
    fn val(&self) -> &V;

    /// Get key and mutable value.
    fn get_mut(&mut self) -> (&K, &mut V);

    /// Set value.
    fn set_val(&mut self, val: V);

    // Take value, replacing current with `V::Default()`.
    fn take_val(&mut self) -> V;

    /// Get left index as `usize`.
    fn left_idx(&self) -> Option<usize>;

    /// Set left index.
    fn set_left_idx(&mut self, opt_idx: Option<usize>);

    /// Get right index as `usize`.
    fn right_idx(&self) -> Option<usize>;

    /// Set right index.
    fn set_right_idx(&mut self, opt_idx: Option<usize>);

    /// Get subtree size.
    #[cfg(feature = "fast_rebalance")]
    fn subtree_size(&self) -> usize;

    /// Set subtree size.
    #[cfg(feature = "fast_rebalance")]
    fn set_subtree_size(&mut self, size: usize);
}

// Enum Dispatch -------------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub enum SmallNodeDispatch<K: Default, V: Default> {
    NodeUSIZE(Node<K, V, usize>),
    NodeU8(Node<K, V, u8>),

    #[cfg(any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64",
        target_pointer_width = "128",
    ))]
    NodeU16(Node<K, V, u16>),

    #[cfg(any(
        target_pointer_width = "32",
        target_pointer_width = "64",
        target_pointer_width = "128",
    ))]
    NodeU32(Node<K, V, u32>),

    #[cfg(any(target_pointer_width = "64", target_pointer_width = "128",))]
    NodeU64(Node<K, V, u64>),

    #[cfg(target_pointer_width = "128")]
    NodeU128(Node<K, V, u128>),
}

impl<K: Default, V: Default> SmallNodeDispatch<K, V> {
    pub fn new(key: K, val: V, uint: SmallUnsignedLabel) -> Self {
        match uint {
            SmallUnsignedLabel::USIZE => SmallNodeDispatch::NodeUSIZE(Node::<K, V, usize>::new(key, val)),
            SmallUnsignedLabel::U8 => SmallNodeDispatch::NodeU8(Node::<K, V, u8>::new(key, val)),

            #[cfg(any(
                target_pointer_width = "16",
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            SmallUnsignedLabel::U16 => SmallNodeDispatch::NodeU16(Node::<K, V, u16>::new(key, val)),

            #[cfg(any(
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            SmallUnsignedLabel::U32 => SmallNodeDispatch::NodeU32(Node::<K, V, u32>::new(key, val)),

            #[cfg(any(target_pointer_width = "64", target_pointer_width = "128",))]
            SmallUnsignedLabel::U64 => SmallNodeDispatch::NodeU64(Node::<K, V, u64>::new(key, val)),

            #[cfg(target_pointer_width = "128")]
            SmallUnsignedLabel::U128 => SmallNodeDispatch::NodeU128(Node::<K, V, u128>::new(key, val)),

            _ => unreachable!()
        }
    }
}

macro_rules! dispatch {
    ( $self:ident, $func:ident $(, $args:expr)* $(,)? ) => {
        match $self {
            SmallNodeDispatch::NodeUSIZE(node) => node.$func($($args,)*),
            SmallNodeDispatch::NodeU8(node) => node.$func($($args,)*),

            #[cfg(any(
                target_pointer_width = "16",
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            SmallNodeDispatch::NodeU16(node) => node.$func($($args,)*),

            #[cfg(any(
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            SmallNodeDispatch::NodeU32(node) => node.$func($($args,)*),

            #[cfg(any(target_pointer_width = "64", target_pointer_width = "128",))]
            SmallNodeDispatch::NodeU64(node) => node.$func($($args,)*),

            #[cfg(target_pointer_width = "128")]
            SmallNodeDispatch::NodeU128(node) => node.$func($($args,)*),
        }
    };
}

impl<K: Default, V: Default> SmallNode<K, V> for SmallNodeDispatch<K, V> {
    fn key(&self) -> &K {
        dispatch!(self, key)
    }

    fn set_key(&mut self, key: K) {
        dispatch!(self, set_key, key);
    }

    fn take_key(&mut self) -> K {
        dispatch!(self, take_key)
    }

    fn val(&self) -> &V {
        dispatch!(self, val)
    }

    fn get_mut(&mut self) -> (&K, &mut V) {
        dispatch!(self, get_mut)
    }

    fn set_val(&mut self, val: V) {
        dispatch!(self, set_val, val);
    }

    fn take_val(&mut self) -> V {
        dispatch!(self, take_val)
    }

    fn left_idx(&self) -> Option<usize> {
        dispatch!(self, left_idx)
    }

    fn set_left_idx(&mut self, opt_idx: Option<usize>) {
        dispatch!(self, set_left_idx, opt_idx);
    }

    fn right_idx(&self) -> Option<usize> {
        dispatch!(self, right_idx)
    }

    fn set_right_idx(&mut self, opt_idx: Option<usize>) {
        dispatch!(self, set_right_idx, opt_idx);
    }

    #[cfg(feature = "fast_rebalance")]
    fn subtree_size(&self) -> usize {
        dispatch!(self, subtree_size)
    }

    #[cfg(feature = "fast_rebalance")]
    fn set_subtree_size(&mut self, size: usize) {
        dispatch!(self, set_subtree_size, size);
    }
}

#[cfg(test)]
mod tests {
    use super::SmallNodeDispatch;
    use smallnum::{small_unsigned_label, SmallUnsignedLabel};
    use core::mem::size_of_val;

    // TODO: make enum store node refs? And and have arena store the actual nodes of a given size, since it has access to U?
    // Then can remove this test but get same effect for arena?
    #[test]
    fn test_node_dispatch_packing() {
        let label_100 = small_unsigned_label!(100);
        let label_1_000 = small_unsigned_label!(1_000);

        assert_eq!(label_100, SmallUnsignedLabel::U8);
        assert_eq!(label_1_000, SmallUnsignedLabel::U16);

        let small_node = SmallNodeDispatch::new(0, 0, label_100);
        let big_node = SmallNodeDispatch::new(0, 0, label_1_000);

        let small_node_size = size_of_val(&small_node);
        let big_node_size = size_of_val(&big_node);

        println!("\nSmallNodeDispatch sizes:\n");
        println!("Small: {} bytes", small_node_size);
        println!("Big: {} bytes", big_node_size);

        assert!(small_node_size < big_node_size);
    }
}