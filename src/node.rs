use crate::{
    context::Context,
    error::RCLResult,
    qos,
    rcl::{self, rosidl_message_type_support_t},
    service::{client::Client, server::Server},
    topic::publisher::Publisher,
    topic::subscriber::Subscriber,
};
use std::{ffi::CString, sync::Arc};

pub struct Node {
    node: rcl::rcl_node_t,
    pub(crate) context: Arc<Context>,
}

impl Node {
    pub(crate) fn new(
        context: Arc<Context>,
        name: &str,
        namespace: Option<&str>,
        options: NodeOptions,
    ) -> RCLResult<Arc<Self>> {
        let mut node = rcl::MTSafeFn::rcl_get_zero_initialized_node();

        let name = CString::new(name).unwrap();
        let namespace = CString::new(namespace.unwrap_or_default()).unwrap();

        {
            let guard = rcl::MT_UNSAFE_FN.lock().unwrap();
            guard.rcl_node_init(
                &mut node,
                name.as_ptr(),
                namespace.as_ptr(),
                unsafe { context.as_ptr_mut() },
                options.as_ptr(),
            )?;
        }

        Ok(Arc::new(Node { node, context }))
    }

    pub(crate) fn as_ptr(&self) -> *const rcl::rcl_node_t {
        &self.node
    }

    pub(crate) unsafe fn as_ptr_mut(&self) -> *mut rcl::rcl_node_t {
        &self.node as *const _ as *mut _
    }

    pub fn create_publisher<T>(
        self: &Arc<Self>,
        topic_name: &str,
        type_support: *const rosidl_message_type_support_t,
        qos: Option<qos::Profile>,
    ) -> RCLResult<Publisher<T>> {
        Publisher::new(self.clone(), topic_name, type_support, qos)
    }

    pub fn create_subscriber<T>(
        self: &Arc<Self>,
        topic_name: &str,
        type_support: *const rosidl_message_type_support_t,
        qos: Option<qos::Profile>,
    ) -> RCLResult<Subscriber<T>> {
        Subscriber::new(self.clone(), topic_name, type_support, qos)
    }

    pub fn create_server<T1, T2>(
        self: &Arc<Self>,
        service_name: &str,
        type_support: *const (),
        qos: Option<qos::Profile>,
    ) -> RCLResult<Server<T1, T2>> {
        Server::new(self.clone(), service_name, type_support, qos)
    }

    pub fn create_client<T1, T2>(
        self: &Arc<Self>,
        service_name: &str,
        type_support: *const (),
        qos: Option<qos::Profile>,
    ) -> RCLResult<Client<T1, T2>> {
        Client::new(self.clone(), service_name, type_support, qos)
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        let guard = rcl::MT_UNSAFE_FN.lock().unwrap();
        guard.rcl_node_fini(&mut self.node).unwrap();
    }
}

/// Options for nodes.
pub struct NodeOptions {
    options: rcl::rcl_node_options_t,
}

impl Default for NodeOptions {
    fn default() -> Self {
        let options = rcl::MTSafeFn::rcl_node_get_default_options();
        NodeOptions { options }
    }
}

impl NodeOptions {
    /// Create options to create a node
    pub fn new() -> Self {
        // TODO: allow setting options
        Default::default()
    }

    pub(crate) fn as_ptr(&self) -> *const rcl::rcl_node_options_t {
        &self.options
    }
}

impl Drop for NodeOptions {
    fn drop(&mut self) {
        let guard = rcl::MT_UNSAFE_FN.lock().unwrap();
        guard.rcl_node_options_fini(&mut self.options).unwrap();
    }
}

unsafe impl Sync for Node {}
unsafe impl Send for Node {}
