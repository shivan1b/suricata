//use core::{Flow};

//pub enum dcerpc {
//    DCERPC_FIELD_NONE = 0,
//    DCERPC_PARSE_DCERPC_HEADER,
//    DCERPC_PARSE_DCERPC_BIND,
//    DCERPC_PARSE_DCERPC_BIND_ACK,
//    DCERPC_PARSE_DCERPC_REQUEST,
//    /* must be last */
//    DCERPC_FIELD_MAX,
//}
//

#[derive(Debug)]
struct DCERPCRequest {
    ctxid: u16,
    opnum: u16,
    stub_data_buffer: Vec<u8>,
    stub_data_buffer_len: u32,
    first_request_seen: u8,
    stub_data_buffer_reset: bool,
}

#[derive(Debug)]
struct DCERPCResponse {
    stub_data_buffer: Vec<u8>,
    stub_data_buffer_len: u32,
    stub_data_buffer_reset: bool,
}

#[derive(Debug)]
struct DCERPCUuidEntry {
    ctxid: u16,
    internal_id: u16,
    result: u16,
    uuid: Vec<u8>,
    version: u16,
    versionminor: u16,
    flags: u16,
    // tailq thingy
}


#[derive(Debug)]
struct DCERPCHdrUdp {
    rpc_vers: u8,
    pkt_type: u8,
    flags1: u8,
    flags2: u8,
    drep: Vec<u8>,
    serial_hi: u8,
    objectuuid: Vec<u8>,
    interfaceuuid: Vec<u8>,
    activityuuid: Vec<u8>,
    server_boot: u32,
    if_vers: u32,
    seqnum: u32,
    opnum: u16,
    ihint: u16,
    ahint: u16,
    fraglen: u16,
    fragnum: u16,
    auth_proto: u8,
    serial_lo: u8,
}

#[derive(Debug)]
struct DCERPCBindBindAck {
    numctxitems: u8,
    numctxitemsleft: u8,
    ctxbytesprocessed: u8,
    ctxid: u16,
    uuid: Vec<u8>,
    version: u16,
    versionminor: u16,
    uuid_entry: Vec<DCERPCUuidEntry>,
    uuid_list: Vec<DCERPCUuidEntry>,
    accepted_uuid_list: Vec<DCERPCUuidEntry>,
    uuid_internal_id: u16,
    secondaryaddrlen: u16,
    secondaryaddrlenleft: u16,
    result: u16,
}

#[derive(Debug)]
struct DCERPCUDP {
    dcerpchdrudp: DCERPCHdrUdp,
    dcerpcbindbindack: DCERPCBindBindAck,
    dcerpcrequest: DCERPCRequest,
    dcerpcresponse: DCERPCResponse,
    bytesprocessed: u16,
    fraglenleft: u16,
    frag_data: Vec<u8>,
    uuid_entry: Vec<DCERPCUuidEntry>,
    uuid_list: Vec<uuid_entry>,
}

#[derive(Debug)]
struct DCERPCUDPState {
    dcerpcudp: DCERPCUDP,
    bytesprocessed: u16,
    fraglenleft: u16,
    frag_data: Vec<u8>,
    uuid_entry: Vec<DCERPCUuidEntry>,
    uuid_list: Vec<uuid_entry>,
}
