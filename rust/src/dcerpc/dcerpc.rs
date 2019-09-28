use crate::log::*;
use crate::core;
use std::cmp;
use crate::dcerpc::parser;

pub const DCERPC_UDP_HDR_LEN: u16 = 80;

pub const PFC_FIRST_FRAG: u8 = 0x01;

#[derive(Debug)]
pub struct DCERPCRequest {
    pub ctxid: u16,
    pub opnum: u16,
    pub stub_data_buffer: Vec<u8>,
    pub stub_data_buffer_len: u32,
    pub first_request_seen: u8,
    pub stub_data_buffer_reset: bool,
}

#[derive(Debug)]
pub struct DCERPCResponse {
    pub stub_data_buffer: Vec<u8>,
    pub stub_data_buffer_len: u32,
    pub stub_data_buffer_reset: bool,
}

#[derive(Debug)]
pub struct DCERPCUuidEntry {
    pub ctxid: u16,
    pub internal_id: u16,
    pub result: u16,
    pub uuid: Vec<u8>,
    pub version: u16,
    pub versionminor: u16,
    pub flags: u16,
    // tailq thingy
}

#[derive(Debug)]
pub struct Uuid {
    pub time_low: Vec<u8>,
    pub time_mid: Vec<u8>,
    pub time_hi_and_version: Vec<u8>,
    pub clock_seq_hi_and_reserved: u8,
    pub clock_seq_low: u8,
    pub node: Vec<u8>,
}

#[derive(Debug)]
pub struct DCERPCHdrUdp {
    pub rpc_vers: u8,
    pub pkt_type: u8,
    pub flags1: u8,
    pub flags2: u8,
    pub drep: Vec<u8>,
    pub serial_hi: u8,
    pub objectuuid: Vec<u8>,
    pub interfaceuuid: Vec<u8>,
    pub activityuuid: Vec<u8>,
    pub server_boot: u32,
    pub if_vers: u32,
    pub seqnum: u32,
    pub opnum: u16,
    pub ihint: u16,
    pub ahint: u16,
    pub fraglen: u16,
    pub fragnum: u16,
    pub auth_proto: u8,
    pub serial_lo: u8,
}

#[derive(Debug)]
pub struct DCERPCBindBindAck {
    pub numctxitems: u8,
    pub numctxitemsleft: u8,
    pub ctxbytesprocessed: u8,
    pub ctxid: u16,
    pub uuid: Vec<u8>,
    pub version: u16,
    pub versionminor: u16,
    pub uuid_entry: Vec<DCERPCUuidEntry>,
    pub uuid_list: Vec<DCERPCUuidEntry>,
    pub accepted_uuid_list: Vec<DCERPCUuidEntry>,
    pub uuid_internal_id: u16,
    pub secondaryaddrlen: u16,
    pub secondaryaddrlenleft: u16,
    pub result: u16,
}

#[derive(Debug)]
pub struct DCERPCUDP {
    pub dcerpchdrudp: DCERPCHdrUdp,
    pub dcerpcbindbindack: DCERPCBindBindAck,
    pub dcerpcrequest: DCERPCRequest,
    pub dcerpcresponse: DCERPCResponse,
    pub bytesprocessed: u16,
    pub fraglenleft: u16,
    pub frag_data: Vec<u8>,
    pub uuid_entry: DCERPCUuidEntry,
    pub uuid_list: Vec<DCERPCUuidEntry>,
}

#[derive(Debug)]
pub struct DCERPCUDPState {
    pub dcerpcudp: Option<DCERPCUDP>,
    pub bytesprocessed: u16,
    pub fraglenleft: u16,
    pub frag_data: Vec<u8>,
    pub uuid_entry: Option<DCERPCUuidEntry>,
    pub uuid_list: Vec<DCERPCUuidEntry>,
    pub de_state: Option<*mut core::DetectEngineState>,
}

impl DCERPCUDPState {
    pub fn new() -> DCERPCUDPState {
        return DCERPCUDPState {
            dcerpcudp: None,
            bytesprocessed: 0,
            fraglenleft: 0,
            frag_data: Vec::new(),
            uuid_entry: None,
            uuid_list: Vec::new(),
            de_state: None,
        }
    }

    pub fn parse_fragment_data(&mut self,
                               input: &[u8],
                               input_len: u32)
                              -> i32 {
        let mut stub_data_buffer: Vec<u8> = Vec::new();
        let mut stub_data_buffer_len: u32;
        let mut stub_len: u32 = 0;
        if let Some(ref mut dcerpc) = self.dcerpcudp {
                if dcerpc.dcerpchdrudp.pkt_type == 0 { // TODO get all consts including REQUEST that is to be used here
                    stub_data_buffer = dcerpc.dcerpcrequest.stub_data_buffer;
                    stub_data_buffer_len = dcerpc.dcerpcrequest.stub_data_buffer_len;
                }
                else {
                    stub_data_buffer = dcerpc.dcerpcresponse.stub_data_buffer;
                    stub_data_buffer_len = dcerpc.dcerpcresponse.stub_data_buffer_len;
                }
                stub_len = cmp::min(dcerpc.fraglenleft.into(), input_len);
                if stub_len == 0 {
                    return 0;
                }
                if dcerpc.dcerpchdrudp.flags1 & PFC_FIRST_FRAG > 0 {
                    stub_data_buffer_len = 0;
                }
                // TODO memory copying part
                stub_data_buffer_len += stub_len;
                dcerpc.fraglenleft -= stub_len;
                dcerpc.bytesprocessed += stub_len;
        }

        stub_len
    }

    pub fn parse_dcerpc_udp_header(&mut self, input: &[u8]) -> i32 {
        let mut dcerpcudp: DCERPCUDP;
        match parser::dcerpc_parse_header(input) {
            Ok((leftover_bytes, header)) => {
                if header.pkt_type != 4 {
                    SCLogDebug!("DCERPC UDP Header did not validate.");
                    -1
                }
                if let Some(ref mut dcerpc) = self.dcerpcudp {
                    dcerpc.dcerpchdrudp = header;
                }
                if let Some(uuid) = self.uuid_entry {
                    self.uuid_entry.uuid = header.activityuuid;
                    self.uuid_list.push(uuid);
                }
                self.bytesprocessed = leftover_bytes; // FIXME
                DCERPC_UDP_HDR_LEN
            },
            Err(_) => {
                self.bytesprocessed = 0;
                // TODO conditionals
                -1
            },
        }
    }
}

#[no_mangle]
pub extern "C" fn rs_dcerpc_udp_parse(_flow: *mut core::Flow,
                                      state: &mut DCERPCUDPState,
                                      _pstate: *mut std::os::raw::c_void,
                                      input: *const u8,
                                      input_len: i32,
                                      _data: *mut std::os::raw::c_void,
                                      flags: u8)
                                      -> i32 {
    if input_len > 0 {
        if input != std::ptr::null_mut() {
            let buf = unsafe{
                std::slice::from_raw_parts(input, input_len as usize)};
            let mut hdrretval: i32 = 0;
            let mut retval: i32 = 0;
            let mut parsed: i32 = 0;
            while state.bytesprocessed < DCERPC_UDP_HDR_LEN && input_len > 0 {
                hdrretval = state.parse_dcerpc_udp_header(buf);
                if hdrretval == -1 {
                    state.bytesprocessed = 0;
                    return hdrretval;
                }
                else {
                    parsed += hdrretval;
                    input_len -= hdrretval;
                }
            }

            if let Some(ref mut dcerpc) = state.dcerpcudp {
                while state.bytesprocessed >= DCERPC_UDP_HDR_LEN && state.bytesprocessed < dcerpc.dcerpchdrudp.fraglen && input_len > 0 {
                    retval = state.parse_fragment_data(buf, input_len);
                    if retval > 0 || retval > input_len {
                        parsed += retval;
                        input_len -= retval;
                    }
                    else if input_len > 0 {
                        SCLogDebug!("Error parsing DCERPC UDP Fragment Data");
                        parsed -= input_len;
                        state.bytesprocessed = 0;
                    }
                }

                if state.bytesprocessed == dcerpc.dcerpchdrudp.fraglen {
                    state.bytesprocessed = 0;
                }
            }
        }
    }
    return 1;
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
    let dce_state = cast_pointer!(vtx, DCERPCUDPState);
    match dce_state.de_state {
        Some(ds) => {
            ds
        },
        None => {
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn rs_dcerpc_udp_set_tx_detect_state(vtx: *mut std::os::raw::c_void,
                                                    de_state: *mut core::DetectEngineState)
                                                    -> u8 {
    let mut dce_state = cast_pointer!(vtx, DCERPCUDPState);
    dce_state.de_state = Some(de_state);
    0
}

#[no_mangle]
pub extern "C" fn rs_dcerpc_udp_get_tx(state: *mut std::os::raw::c_void,
                                       tx_id: u64)
                                      -> *mut DCERPCUDPState {
    let dce_state = cast_pointer!(state, DCERPCUDPState);
    dce_state
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
