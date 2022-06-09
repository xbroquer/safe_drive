// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn std_msgs__msg__UInt16__init(msg: *mut UInt16) -> bool;
    fn std_msgs__msg__UInt16__fini(msg: *mut UInt16);
    fn std_msgs__msg__UInt16__Sequence__init(msg: *mut UInt16Sequence, size: usize) -> bool;
    fn std_msgs__msg__UInt16__Sequence__fini(msg: *mut UInt16Sequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__UInt16() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct UInt16 {
    pub data: u16,
}

impl UInt16 {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__UInt16__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for UInt16 {
    fn drop(&mut self) {
        unsafe { std_msgs__msg__UInt16__fini(self) };
    }
}

impl TopicMsg for UInt16 {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__UInt16()
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct UInt16Sequence {
    data: *mut UInt16,
    size: usize,
    capacity: usize,
}

impl UInt16Sequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__UInt16__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[UInt16]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [UInt16]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for UInt16Sequence {
    fn drop(&mut self) {
        unsafe { std_msgs__msg__UInt16__Sequence__fini(self) };
    }
}
