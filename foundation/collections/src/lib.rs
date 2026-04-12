#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use alloc::vec::Vec;
use alloc::vec;

#[derive(Debug, Clone)]
pub enum BufRef<const N: usize> {
    Stack { elements: [u8; N], length: usize },
    Heap(Vec<u8>)
}

impl<const N: usize> Default for BufRef<N> {
    fn default() -> Self {
        Self::Stack { elements: [0u8; N], length: 0 }
    }
}

impl<const N: usize> BufRef<N> {
    pub fn with_capacity(capacity: usize) -> Self {
        if capacity <= N {
            Self::Stack {
                elements: [0u8; N],
                length: 0,
            }
        } else {
            Self::Heap(Vec::with_capacity(capacity))
        }
    }

    pub fn push(&mut self, value: u8) {
        match self {
            Self::Stack { elements, length } => {
                if *length < N {
                    elements[*length] = value;
                    *length += 1;
                } else {
                    // Migrate to heap
                    let mut vec = elements[..*length].to_vec();
                    vec.push(value);
                    *self = Self::Heap(vec);
                }
            }
            Self::Heap(vec) => vec.push(value),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Stack { length, .. } => *length,
            Self::Heap(vec) => vec.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> BufRef<N> {
    pub fn from_slice(slice: &[u8]) -> Self {
        if slice.len() <= N {
            let mut elements = [0u8; N];
            elements[..slice.len()].copy_from_slice(slice);
            Self::Stack { elements, length: slice.len() }
        } else {
            Self::Heap(slice.to_vec())
        }
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[u8] {
        match self {
            Self::Stack { elements, length } => &elements[..*length],
            Self::Heap(vec) => vec.as_slice(),
        }
    }

    #[inline(always)]
    pub fn to_vec(&self) -> Vec<u8> {
        self.as_slice().to_vec()
    }
}

impl<const N: usize> core::ops::Index<usize> for BufRef<N> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl<const N: usize> BufRef<N> {
    pub fn with_len(len: usize) -> Self {
        if len <= N {
            Self::Stack {
                elements: [0u8; N],
                length: len,
            }
        } else {
            Self::Heap(vec![0u8; len])
        }
    }

    pub fn get(&self, index: usize) -> Option<u8> {
        match self {
            Self::Stack { elements, length } => {
                if index < *length {
                    Some(elements[index])
                } else {
                    None
                }
            }
            Self::Heap(elements) => elements.get(index).copied(),
        }
    }

    pub fn set(&mut self, index: usize, value: u8) {
        match self {
            Self::Stack { elements, length } => {
                if index < *length {
                    elements[index] = value;
                }
            }
            Self::Heap(elements) => {
                if index < elements.len() {
                    elements[index] = value;
                }
            }
        }
    }
}