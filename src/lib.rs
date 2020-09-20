#![feature(type_alias_impl_trait)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
use std::any::Any;

TODO synctest.cpp

mod opaque {
    use super::*;
    pub trait Opaque: Drop {
        fn do_poll(&self, timeout: i32) -> GGPOErrorCode {
            GGPO_OK
        }
        fn add_player(
            &self,
            player: &GGPOPlayer,
            handle: &GGPOPlayerHandle,
        ) -> GGPOResult<GGPOPlayerHandle>;
        fn add_local_input(
            &self,
            player: GGPOPlayerHandle,
            values: &dyn Any,
            size: i32,
        ) -> GGPOErrorCode;
        fn sync_input(&self, values: &dyn Any, size: i32, disconnect_flags: &i32) -> GGPOErrorCode;
        fn increment_frame(&self) -> GGPOErrorCode {
            GGPO_OK
        }
        fn chat(&self, text: &str) -> GGPOErrorCode {
            GGPO_OK
        }
        fn disconnect_player(&self, handle: GGPOPlayerHandle) -> GGPOErrorCode {
            GGPO_OK
        }
        fn get_network_stats(&self, handle: GGPOPlayerHandle) -> GGPOResult<GGPONetworkStats> {
            Err(GGPO_OK) // TODO
        }
        fn logv(&self, msg: &str) {
            println!("{}", msg)
        }
        fn set_frame_delay(&self, player: GGPOPlayerHandle, delay: i32) -> GGPOErrorCode {
            GGPOErrorCode::GGPO_ERRORCODE_UNSUPPORTED
        }
        fn set_disconnect_timeout(&self, timeout: i32) -> GGPOErrorCode {
            GGPOErrorCode::GGPO_ERRORCODE_UNSUPPORTED
        }
        fn set_disconnect_notify_start(&self, timeout: i32) -> GGPOErrorCode {
            GGPOErrorCode::GGPO_ERRORCODE_UNSUPPORTED
        }
    }
}

pub trait GGPOSession: opaque::Opaque {}

struct P2PSession {
    cb: Box<dyn GGPOSessionCallbacks>,
    num_players: i32,
    input_size: i32,
    game_name: String,
    local_port: u8,
}

impl Drop for P2PSession {
    fn drop(&mut self) {}
}

impl opaque::Opaque for P2PSession {
    fn add_player(
        &self,
        player: &GGPOPlayer,
        handle: &GGPOPlayerHandle,
    ) -> GGPOResult<GGPOPlayerHandle> {
        Ok(0)
    }
    fn add_local_input(
        &self,
        player: GGPOPlayerHandle,
        values: &dyn Any,
        size: i32,
    ) -> GGPOErrorCode {
        GGPO_OK
    }
    fn sync_input(&self, values: &dyn Any, size: i32, disconnect_flags: &i32) -> GGPOErrorCode {
        GGPO_OK
    }
}

impl GGPOSession for P2PSession {}

struct SyncTestSession {
    cb: Box<dyn GGPOSessionCallbacks>,
    num_players: i32,
    game_name: String,
    frames: i32,
}

impl Drop for SyncTestSession {
    fn drop(&mut self) {}
}

impl opaque::Opaque for SyncTestSession {
    fn add_player(
        &self,
        player: &GGPOPlayer,
        handle: &GGPOPlayerHandle,
    ) -> GGPOResult<GGPOPlayerHandle> {
        Ok(0)
    }
    fn add_local_input(
        &self,
        player: GGPOPlayerHandle,
        values: &dyn Any,
        size: i32,
    ) -> GGPOErrorCode {
        GGPO_OK
    }
    fn sync_input(&self, values: &dyn Any, size: i32, disconnect_flags: &i32) -> GGPOErrorCode {
        GGPO_OK
    }
}

impl GGPOSession for SyncTestSession {}

struct SpectatorSession {
    cb: Box<dyn GGPOSessionCallbacks>,
    num_players: i32,
    game_name: String,
    local_port: u8,
    input_size: i32,
    host_ip: String,
    host_port: u8,
}

impl Drop for SpectatorSession {
    fn drop(&mut self) {}
}

impl opaque::Opaque for SpectatorSession {
    fn add_player(
        &self,
        player: &GGPOPlayer,
        handle: &GGPOPlayerHandle,
    ) -> GGPOResult<GGPOPlayerHandle> {
        Ok(0)
    }
    fn add_local_input(
        &self,
        player: GGPOPlayerHandle,
        values: &dyn Any,
        size: i32,
    ) -> GGPOErrorCode {
        GGPO_OK
    }
    fn sync_input(&self, values: &dyn Any, size: i32, disconnect_flags: &i32) -> GGPOErrorCode {
        GGPO_OK
    }
}

impl GGPOSession for SpectatorSession {}

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
    fn begin_game(&self, game: &str);
    fn save_game_state(&self, buffer: &Vec<u8>, len: &i32, checksum: &i32, frame: i32);
    fn load_game_state(&self, buffer: Vec<u8>, len: i32);
    fn log_game_state(&self, filename: &str, buffer: Vec<u8>, len: i32);
    fn free_buffer(&self, buffer: &dyn Any);
    fn advance_frame(&self, flags: i32);
    fn on_event(&self, info: &GGPOEvent);
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

// C-style API
pub fn ggpo_start_session(
    cb: impl GGPOSessionCallbacks + 'static,
    game: &str,
    num_players: i32,
    input_size: i32,
    local_port: u8,
) -> GGPOResult<Box<impl GGPOSession>> {
    Ok(Box::new(P2PSession {
        cb: Box::new(cb),
        game_name: game.to_owned(),
        input_size,
        local_port,
        num_players,
    }))
}

pub fn ggpo_add_player(
    session: &mut impl GGPOSession,
    player: &GGPOPlayer,
    handle: &GGPOPlayerHandle,
) -> GGPOResult<GGPOPlayerHandle> {
    session.add_player(player, handle)
}

pub fn ggpo_start_synctest(
    cb: impl GGPOSessionCallbacks + 'static,
    game: &str,
    num_players: i32,
    input_size: i32,
    frames: i32,
) -> GGPOResult<impl GGPOSession> {
    Ok(SyncTestSession {
        cb: Box::new(cb),
        num_players,
        game_name: game.to_owned(),
        frames,
    })
}

pub fn ggpo_start_spectating(
    cb: impl GGPOSessionCallbacks + 'static,
    game: &str,
    num_players: i32,
    input_size: i32,
    local_port: u8,
    host_ip: &str,
    host_port: u8,
) -> GGPOResult<impl GGPOSession> {
    Ok(SpectatorSession {
        cb: Box::new(cb),
        num_players,
        game_name: game.to_owned(),
        input_size,
        local_port,
        host_ip: host_ip.to_owned(),
        host_port,
    })
}

pub fn ggpo_close_session(session: &mut impl GGPOSession) -> GGPOErrorCode {
    drop(session);
    GGPO_OK
}

pub fn ggpo_set_frame_delay(
    session: &mut impl GGPOSession,
    player: GGPOPlayerHandle,
    frame_delay: i32,
) -> GGPOErrorCode {
    session.set_frame_delay(player, frame_delay)
}

pub fn ggpo_idle(session: &mut impl GGPOSession, timeout: i32) -> GGPOErrorCode {
    session.do_poll(timeout)
}

pub fn ggpo_add_local_input(
    session: &mut impl GGPOSession,
    player: GGPOPlayerHandle,
    values: &dyn Any,
    size: i32,
) -> GGPOErrorCode {
    session.add_local_input(player, values, size)
}
pub fn ggpo_synchronize_input(
    session: &mut impl GGPOSession,
    values: &dyn Any,
    size: i32,
    disconnect_flags: &i32,
) -> GGPOErrorCode {
    session.sync_input(values, size, disconnect_flags)
}

pub fn ggpo_disconnect_player(
    session: &mut impl GGPOSession,
    player: GGPOPlayerHandle,
) -> GGPOErrorCode {
    session.disconnect_player(player)
}

pub fn ggpo_advance_frame(session: &mut impl GGPOSession) -> GGPOErrorCode {
    session.increment_frame()
}

pub fn ggpo_get_network_stats(
    session: &mut impl GGPOSession,
    handle: GGPOPlayerHandle,
) -> GGPOResult<GGPONetworkStats> {
    session.get_network_stats(handle)
}

pub fn ggpo_set_disconnect_timeout(session: &mut impl GGPOSession, timeout: i32) -> GGPOErrorCode {
    session.set_disconnect_timeout(timeout)
}

pub fn ggpo_set_disconnect_notify_start(
    session: &mut impl GGPOSession,
    timeout: i32,
) -> GGPOErrorCode {
    session.set_disconnect_notify_start(timeout)
}

pub fn ggpo_log(session: &mut impl GGPOSession, message: &str) {
    session.logv(message)
}

pub fn ggpo_logv(session: &mut impl GGPOSession, message: &str) {
    session.logv(message)
}
