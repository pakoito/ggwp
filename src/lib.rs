#![feature(type_alias_impl_trait)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
use std::any::Any;

mod opaque {
    use super::*;
    pub trait Opaque {
        fn do_poll(timeout: i32) -> GGPOErrorCode {
            GGPO_OK
        }
        fn add_player(player: &GGPOPlayer) -> GGPOResult<GGPOPlayerHandle>;
        fn add_local_input(player: GGPOPlayerHandle, values: &dyn Any, size: i32) -> GGPOErrorCode;
        fn sync_input(values: &dyn Any, size: i32, disconnect_flags: &i32) -> GGPOErrorCode;
        fn increment_frame() -> GGPOErrorCode {
            GGPO_OK
        }
        fn chat(text: &str) -> GGPOErrorCode {
            GGPO_OK
        }
        fn disconnect_player(handle: GGPOPlayerHandle) -> GGPOErrorCode {
            GGPO_OK
        }
        fn get_network_stats(stats: &GGPONetworkStats, handle: GGPOPlayerHandle) -> GGPOErrorCode {
            GGPO_OK
        }
        fn logv(msg: &str) -> GGPOErrorCode {
            println!("{}", msg);
            GGPO_OK
        }
        fn set_frame_delay(player: GGPOPlayerHandle, delay: i32) -> GGPOErrorCode {
            GGPOErrorCode::GGPO_ERRORCODE_UNSUPPORTED
        }
        fn set_disconnect_timeout(timeout: i32) -> GGPOErrorCode {
            GGPOErrorCode::GGPO_ERRORCODE_UNSUPPORTED
        }
        fn set_disconnect_notify_start(timeout: i32) -> GGPOErrorCode {
            GGPOErrorCode::GGPO_ERRORCODE_UNSUPPORTED
        }
    }
}

pub trait IGGPOSession: opaque::Opaque {}

struct P2PSession;

impl opaque::Opaque for P2PSession {
    fn add_player(player: &GGPOPlayer) -> GGPOResult<GGPOPlayerHandle> {
        Ok(0)
    }
    fn add_local_input(player: GGPOPlayerHandle, values: &dyn Any, size: i32) -> GGPOErrorCode {
        GGPO_OK
    }
    fn sync_input(values: &dyn Any, size: i32, disconnect_flags: &i32) -> GGPOErrorCode {
        GGPO_OK
    }
}

impl IGGPOSession for P2PSession {}

pub type GGPOSession = impl IGGPOSession;

pub type GGPOPlayerHandle = i32;

pub enum GGPOPlayerType {
    GGPO_PLAYERTYPE_LOCAL,
    GGPO_PLAYERTYPE_REMOTE,
    GGPO_PLAYERTYPE_SPECTATOR,
}

pub struct GGPOPlayerLocationRemote {
    ip_address: [u8; 32],
    port: u8,
}

pub enum GGPOPlayerLocation {
    Local,
    Remote(GGPOPlayerLocationRemote),
}

pub struct GGPOPlayer {
    size: i32,
    ttype: GGPOPlayerType,
    player_num: i32,
    u: GGPOPlayerLocation,
}

pub struct GGPOLocalEndpoint {
    player_num: i32,
}

pub type GGPOResult<T> = Result<T, GGPOErrorCode>;

pub const GGPO_OK: GGPOErrorCode = GGPOErrorCode::GGPO_ERRORCODE_SUCCESS;

pub enum GGPOErrorCode {
    GGPO_ERRORCODE_SUCCESS = 0,
    GGPO_ERRORCODE_GENERAL_FAILURE = -1,
    GGPO_ERRORCODE_INVALID_SESSION = 1,
    GGPO_ERRORCODE_INVALID_PLAYER_HANDLE = 2,
    GGPO_ERRORCODE_PLAYER_OUT_OF_RANGE = 3,
    GGPO_ERRORCODE_PREDICTION_THRESHOLD = 4,
    GGPO_ERRORCODE_UNSUPPORTED = 5,
    GGPO_ERRORCODE_NOT_SYNCHRONIZED = 6,
    GGPO_ERRORCODE_IN_ROLLBACK = 7,
    GGPO_ERRORCODE_INPUT_DROPPED = 8,
    GGPO_ERRORCODE_PLAYER_DISCONNECTED = 9,
    GGPO_ERRORCODE_TOO_MANY_SPECTATORS = 10,
    GGPO_ERRORCODE_INVALID_REQUEST = 11,
}

pub fn ggpo_succeeded(code: GGPOErrorCode) -> bool {
    match code {
        GGPOErrorCode::GGPO_ERRORCODE_SUCCESS => true,
        _ => false,
    }
}

pub const GGPO_INVALID_HANDLE: i32 = -1;

pub enum GGPOEventCode {
    GGPO_EVENTCODE_CONNECTED_TO_PEER = 1000,
    GGPO_EVENTCODE_SYNCHRONIZING_WITH_PEER = 1001,
    GGPO_EVENTCODE_SYNCHRONIZED_WITH_PEER = 1002,
    GGPO_EVENTCODE_RUNNING = 1003,
    GGPO_EVENTCODE_DISCONNECTED_FROM_PEER = 1004,
    GGPO_EVENTCODE_TIMESYNC = 1005,
    GGPO_EVENTCODE_CONNECTION_INTERRUPTED = 1006,
    GGPO_EVENTCODE_CONNECTION_RESUMED = 1007,
}

pub struct GGPOEventcodeConnectedToPeer {
    player: GGPOPlayerHandle,
}

pub struct GGPOEventcodeSynchronizingWithPeer {
    player: GGPOPlayerHandle,
    count: i32,
    total: i32,
}

pub struct GGPOEventcodeSynchronizedWithPeer {
    player: GGPOPlayerHandle,
}

pub struct GGPOEventcodeDisconnectedFromPeer {
    player: GGPOPlayerHandle,
}

pub struct GGPOEventcodeTimesync {
    frames_ahead: i32,
}

pub struct GGPOEventcodeConnectionInterrupted {
    player: GGPOPlayerHandle,
}

pub struct GGPOEventcodeConnectionResumed {
    player: GGPOPlayerHandle,
}

pub enum GGPOEventValue {
    GGPO_EVENTCODE_CONNECTED_TO_PEER(GGPOEventcodeConnectedToPeer),
    GGPO_EVENTCODE_SYNCHRONIZING_WITH_PEER(GGPOEventcodeSynchronizingWithPeer),
    GGPO_EVENTCODE_SYNCHRONIZED_WITH_PEER(GGPOEventcodeSynchronizedWithPeer),
    GGPO_EVENTCODE_RUNNING,
    GGPO_EVENTCODE_DISCONNECTED_FROM_PEER(GGPOEventcodeDisconnectedFromPeer),
    GGPO_EVENTCODE_TIMESYNC(GGPOEventcodeTimesync),
    GGPO_EVENTCODE_CONNECTION_INTERRUPTED(GGPOEventcodeConnectionInterrupted),
    GGPO_EVENTCODE_CONNECTION_RESUMED(GGPOEventcodeConnectionResumed),
}

pub struct GGPOEvent {
    code: GGPOEventCode,
    u: GGPOEventValue,
}

pub trait GGPOSessionCallbacks {
    fn begin_game(game: &str);
    fn save_game_state(buffer: &Vec<u8>, len: &i32, checksum: &i32, frame: i32);
    fn load_game_state(buffer: Vec<u8>, len: i32);
    fn log_game_state(filename: &str, buffer: Vec<u8>, len: i32);
    fn free_buffer(buffer: &dyn Any);
    fn advance_frame(flags: i32);
    fn on_event(info: &GGPOEvent);
}

pub struct GGPONetworkStatsNetwork {
    send_queue_len: i32,
    recv_queue_len: i32,
    ping: i32,
    kbps_sent: i32,
}

pub struct GGPONetworkStatsTimesync {
    local_frames_behind: i32,
    remote_frames_behind: i32,
}

pub enum GGPONetworkStats {
    Network(GGPONetworkStatsNetwork),
    Timesync(GGPONetworkStatsTimesync),
}

// API
pub fn ggpo_start_session(
    cb: &impl GGPOSessionCallbacks,
    game: &str,
    num_players: i32,
    input_size: i32,
    localport: u8,
) -> GGPOResult<Box<GGPOSession>> {
    Ok(Box::new(P2PSession))
}

pub fn ggpo_add_player(
    session: &mut GGPOSession,
    stats: &GGPONetworkStats,
) -> GGPOResult<GGPOPlayerHandle> {
    Ok(0)
}

pub fn ggpo_start_synctest(
    session: &mut GGPOSession,
    cb: &impl GGPOSessionCallbacks,
    game: &str,
    num_players: i32,
    input_size: i32,
    frames: i32,
) -> GGPOErrorCode {
    GGPOErrorCode::GGPO_ERRORCODE_SUCCESS
}

pub fn ggpo_start_spectating(
    session: &mut GGPOSession,
    cb: &impl GGPOSessionCallbacks,
    game: &str,
    num_players: i32,
    input_size: i32,
    local_port: u8,
    host_ip: &str,
    host_port: u8,
) -> GGPOErrorCode {
    GGPOErrorCode::GGPO_ERRORCODE_SUCCESS
}

pub fn ggpo_close_session(session: &mut GGPOSession) -> GGPOErrorCode {
    GGPOErrorCode::GGPO_ERRORCODE_SUCCESS
}

pub fn ggpo_set_frame_delay(
    session: &mut GGPOSession,
    player: GGPOPlayerHandle,
    frame_delay: i32,
) -> GGPOErrorCode {
    GGPOErrorCode::GGPO_ERRORCODE_SUCCESS
}

pub fn ggpo_idle(session: &mut GGPOSession, timeout: i32) -> GGPOErrorCode {
    GGPOErrorCode::GGPO_ERRORCODE_SUCCESS
}

pub fn ggpo_add_local_input(
    session: &mut GGPOSession,
    player: GGPOPlayerHandle,
    values: &dyn Any,
    size: i32,
) -> GGPOErrorCode {
    GGPOErrorCode::GGPO_ERRORCODE_SUCCESS
}
pub fn ggpo_synchronize_input(
    session: &mut GGPOSession,
    values: &dyn Any,
    size: i32,
    disconnect_flags: &i32,
) -> GGPOErrorCode {
    GGPOErrorCode::GGPO_ERRORCODE_SUCCESS
}

pub fn ggpo_disconnect_player(
    session: &mut GGPOSession,
    player: GGPOPlayerHandle,
) -> GGPOErrorCode {
    GGPOErrorCode::GGPO_ERRORCODE_SUCCESS
}

pub fn ggpo_advance_frame(session: &mut GGPOSession) -> GGPOErrorCode {
    GGPOErrorCode::GGPO_ERRORCODE_SUCCESS
}

pub fn ggpo_get_network_stats(
    session: &mut GGPOSession,
    player: GGPOPlayerHandle,
    stats: &mut GGPONetworkStats, // out parameter
) -> GGPOErrorCode {
    GGPOErrorCode::GGPO_ERRORCODE_SUCCESS
}

pub fn ggpo_set_disconnect_timeout(session: &mut GGPOSession, timeout: i32) -> GGPOErrorCode {
    GGPOErrorCode::GGPO_ERRORCODE_SUCCESS
}

pub fn ggpo_set_disconnect_notify_start(session: &mut GGPOSession, timeout: i32) -> GGPOErrorCode {
    GGPOErrorCode::GGPO_ERRORCODE_SUCCESS
}

pub fn ggpo_log(_session: &mut GGPOSession, message: &str) {
    println!("{}", message)
}

pub fn ggpo_logv(_session: &mut GGPOSession, message: &str) {
    println!("{}", message)
}
