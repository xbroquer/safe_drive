// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn sensor_msgs__msg__MagneticField__init(msg: *mut MagneticField) -> bool;
    fn sensor_msgs__msg__MagneticField__fini(msg: *mut MagneticField);
    fn sensor_msgs__msg__MagneticField__Sequence__init(msg: *mut MagneticFieldSequence, size: usize) -> bool;
    fn sensor_msgs__msg__MagneticField__Sequence__fini(msg: *mut MagneticFieldSequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__MagneticField() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct MagneticField {
    pub header: std_msgs::msg::Header,
    pub magnetic_field: geometry_msgs::msg::Vector3,
    pub magnetic_field_covariance: [f64; 9],
}

impl MagneticField {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__MagneticField__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for MagneticField {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__MagneticField__fini(self) };
    }
}

impl TopicMsg for MagneticField {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__MagneticField()
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct MagneticFieldSequence {
    data: *mut MagneticField,
    size: usize,
    capacity: usize,
}

impl MagneticFieldSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__MagneticField__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[MagneticField]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [MagneticField]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for MagneticFieldSequence {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__MagneticField__Sequence__fini(self) };
    }
}
