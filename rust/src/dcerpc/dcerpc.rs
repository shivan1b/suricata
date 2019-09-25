use crate::dcerpc::parser;

DCERPC_UDP_HDR_LEN = 80;

#[derive(Debug)]
pub struct DCERPCRequest {
    ctxid: u16,
    opnum: u16,
    stub_data_buffer: Vec<u8>,
    stub_data_buffer_len: u32,
    first_request_seen: u8,
    stub_data_buffer_reset: bool,
}

#[derive(Debug)]
pub struct DCERPCResponse {
    stub_data_buffer: Vec<u8>,
    stub_data_buffer_len: u32,
    stub_data_buffer_reset: bool,
}

#[derive(Debug)]
pub struct DCERPCUuidEntry {
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
pub struct Uuid {
    time_low: Vec<u8>,
    time_mid: Vec<u8>,
    time_hi_and_version: Vec<u8>,
    clock_seq_hi_and_reserved: u8,
    clock_seq_low: u8,
    node: Vec<u8>,
}

#[derive(Debug)]
pub struct DCERPCHdrUdp {
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
pub struct DCERPCBindBindAck {
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
pub struct DCERPCUDP {
    dcerpchdrudp: DCERPCHdrUdp,
    dcerpcbindbindack: DCERPCBindBindAck,
    dcerpcrequest: DCERPCRequest,
    dcerpcresponse: DCERPCResponse,
    bytesprocessed: u16,
    fraglenleft: u16,
    frag_data: Vec<u8>,
    uuid_entry: DCERPCUuidEntry,
    uuid_list: Vec<uuid_entry>,
}

#[derive(Debug)]
pub struct DCERPCUDPState {
    dcerpcudp: Option<DCERPCUDP>,
    bytesprocessed: u16,
    fraglenleft: u16,
    frag_data: Vec<u8>,
    uuid_entry: Vec<DCERPCUuidEntry>,
    uuid_list: Vec<uuid_entry>,
    de_state: Option<*mut core::DetectEngineState>,
}

impl DCERPCUDPState {
    pub fn new() -> DCERPCUDPState {
        return DCERPCUDPState {
            dcerpcudp: None,
            bytesprocessed: 0,
            fraglenleft: 0,
            frag_data: Vec::new(),
            uuid_entry: Vec::new(),
            uuild_list: Vec::new(),
            de_state: None,
        }
    }

    pub fn parse_dcerpc_udp_header(&mut self, input: &[u8]) -> bool {
        let mut dcerpcudp: DCERPCUDP;
        match parser::dcerpc_parse_header(input) {
            Ok((leftover_bytes, header)) => {
                if (header.pkt_type != 4) {
                    SCLogDebug!("DCERPC UDP Header did not validate.");
                    false
                }
                self.dcerpcudp.dcerpchdrudp = header;
                self.uuid_entry = header.activityuuid;
                self.bytesprocessed = header.len() - input.len(); // FIXME
            },
            Err(_) => {
                self.bytesprocessed = 0;
                // TODO conditionals
            },
        }
    }

}

#[no_mangle]
pub extern "C" fn rs_dcerpc_udp_parse(_flow: *mut core::Flow,
                                      state: &mut DCERPCUDPState,
                                      _pstate: *mut std::os::raw::c_void,
                                      input: *const u8,
                                      input_len: u32,
                                      _data: *mut std::os::raw::c_void,
                                      flags: const u8)
                                      -> i8 {
    // TODO
}

#[no_mangle]
pub extern "C" fn rs_dcerpc_udp_state_free(state: *mut std::os::raw::c_void) {
    // TODO
}

#[no_mangle]
pub extern "C" fn rs_dcerpc_udp_state_transaction_free(state: *mut std::os::raw::c_void,
                                                       tx_id: u64) {
    // do nothing
}

#[no_mangle]
pub extern "C" fn rs_dcerpc_udp_get_tx_detect_state(vtx: *mut std::os::raw::c_void)
                                                    -> *mut core::DetectEngineState {
    // TODO
}

#[no_mangle]
pub extern "C" fn rs_dcerpc_udp_set_tx_detect_state(vtx: *mut std::os::raw::c_void,
                                                    de_state: *mut core::DetectEngineState)
                                                    -> u8 {
    // TODO
}

#[no_mangle]
pub extern "C" fn rs_dcerpc_udp_get_tx(state: *mut std::os::raw::c_void,
                                       tx_id: u64)
                                      -> *mut DCERPCUDPState {
    // TODO
}

#[no_mangle]
pub extern "C" fn rs_dcerpc_udp_get_tx_cnt(state: *mut std::os::raw::c_void)
                                          -> u8 {
    1
}

#[no_mangle]
pub extern "C" fn rs_dcerpc_udp_get_alstate_progress(tx: *mut std::os::raw::c_void,
                                                     direction: u8)
                                                    -> u8 {
    0
}

#[no_mangle]
pub extern "C" fn rs_dcerpc_udp_get_alstate_progress_completion_status(direction: u8)
                                                                      -> u8 {
    1
}
