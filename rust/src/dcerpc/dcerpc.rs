use core::{Flow}

pub enum dcerpc {
    DCERPC_FIELD_NONE = 0,
    DCERPC_PARSE_DCERPC_HEADER,
    DCERPC_PARSE_DCERPC_BIND,
    DCERPC_PARSE_DCERPC_BIND_ACK,
    DCERPC_PARSE_DCERPC_REQUEST,
    /* must be last */
    DCERPC_FIELD_MAX,
}
