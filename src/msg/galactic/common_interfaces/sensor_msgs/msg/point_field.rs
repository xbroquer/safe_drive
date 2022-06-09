// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn sensor_msgs__msg__PointField__init(msg: *mut PointField) -> bool;
    fn sensor_msgs__msg__PointField__fini(msg: *mut PointField);
    fn sensor_msgs__msg__PointField__Sequence__init(msg: *mut PointFieldSequence, size: usize) -> bool;
    fn sensor_msgs__msg__PointField__Sequence__fini(msg: *mut PointFieldSequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__PointField() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct PointField {
    pub INT8: u8,
    pub UINT8: u8,
    pub INT16: u8,
    pub UINT16: u8,
    pub INT32: u8,
    pub UINT32: u8,
    pub FLOAT32: u8,
    pub FLOAT64: u8,
    pub name: crate::msg::RosString<0>,
    pub offset: u32,
    pub datatype: u8,
    pub count: u32,
}

impl PointField {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__PointField__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for PointField {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__PointField__fini(self) };
    }
}

impl TopicMsg for PointField {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__PointField()
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct PointFieldSequence {
    data: *mut PointField,
    size: usize,
    capacity: usize,
}

impl PointFieldSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__PointField__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[PointField]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [PointField]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for PointFieldSequence {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__PointField__Sequence__fini(self) };
    }
}
