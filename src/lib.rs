#![feature(type_alias_impl_trait)]

pub trait GGPOSession {}

pub type IGGPOSession = dyn GGPOSession;

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

fn ggpo_succeeded(code: GGPOErrorCode) -> bool {
    match code {
        GGPO_ERRORCODE_SUCCESS => true,
        _ => false,
    }
}

const GGPO_INVALID_HANDLE: i32 = -1;

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

trait GGPOSessionCallbacks {
    fn begin_game(game: &str);
    fn save_game_state(buffer: &Vec<u8>, len: &i32, checksum: &i32, frame: i32);
    fn load_game_state(buffer: Vec<u8>, len: i32);
    fn log_game_state(filename: &str, buffer: Vec<u8>, len: i32);
    fn free_buffer(buffer: &dyn std::any::Any);
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
pub fn ggpo_start_session(...) -> GGPOErrorCode {
    GGPOErrorCode::GGPO_ERRORCODE_SUCCESS
}
