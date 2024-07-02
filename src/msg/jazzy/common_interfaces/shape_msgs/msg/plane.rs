// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn shape_msgs__msg__Plane__init(msg: *mut Plane) -> bool;
    fn shape_msgs__msg__Plane__fini(msg: *mut Plane);
    fn shape_msgs__msg__Plane__are_equal(lhs: *const Plane, rhs: *const Plane) -> bool;
    fn shape_msgs__msg__Plane__Sequence__init(msg: *mut PlaneSeqRaw, size: usize) -> bool;
    fn shape_msgs__msg__Plane__Sequence__fini(msg: *mut PlaneSeqRaw);
    fn shape_msgs__msg__Plane__Sequence__are_equal(lhs: *const PlaneSeqRaw, rhs: *const PlaneSeqRaw) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__shape_msgs__msg__Plane() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct Plane {
    pub coef: [f64; 4],
}

impl Plane {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { shape_msgs__msg__Plane__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Plane {
    fn drop(&mut self) {
        unsafe { shape_msgs__msg__Plane__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct PlaneSeqRaw {
    data: *mut Plane,
    size: size_t,
    capacity: size_t,
}

/// Sequence of Plane.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct PlaneSeq<const N: usize> {
    data: *mut Plane,
    size: size_t,
    capacity: size_t,
}

impl<const N: usize> PlaneSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size > N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: PlaneSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { shape_msgs__msg__Plane__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: PlaneSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[Plane] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size as _) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [Plane] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size as _) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Plane> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Plane> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for PlaneSeq<N> {
    fn drop(&mut self) {
        let mut msg = PlaneSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { shape_msgs__msg__Plane__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for PlaneSeq<N> {}
unsafe impl<const N: usize> Sync for PlaneSeq<N> {}


impl TypeSupport for Plane {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__shape_msgs__msg__Plane()
        }
    }
}

impl PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            shape_msgs__msg__Plane__are_equal(self, other)
        }
    }
}

impl<const N: usize> PartialEq for PlaneSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = PlaneSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
            let msg2 = PlaneSeqRaw{data: other.data, size: other.size, capacity: other.capacity};
            shape_msgs__msg__Plane__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
