#![feature(type_alias_impl_trait)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
use ringbuf::{Consumer, Producer, RingBuffer};
use std::any::Any;
use std::cell::Cell;
use std::path::Path;

//continue synctest.cpp
//sync.cpp#init

#[test]
fn main_test() {
    println!("PATATAS {}", ggpo_succeeded(GGPO_OK));
}

/////////////////////////
/// Udp Message
/////////////////////////

struct ConnectStatus {
    disconnected: u32,
    last_frame: i32,
}

impl Default for ConnectStatus {
    fn default() -> Self {
        ConnectStatus {
            disconnected: 1,
            last_frame: 31,
        }
    }
}

/////////////////////////
/// InputQueue
/////////////////////////

struct InputQueue;

impl InputQueue {
    fn discard_confirmed_frames(&self, frame: i32) {}

    fn set_frame_delay(&self, delay: i32) {}

    fn add_input(&self, input: GameInput) {}
}

/////////////////////////
/// Sync
/////////////////////////

const MAX_PREDICTION_FRAMES: usize = 8;

struct GGPOSync {
    savedstate: Cell<Option<GGPOSyncSavedState>>,
    config: Cell<Option<GGPOSyncConfig>>,
    framecount: Cell<i32>,
    rollingback: Cell<bool>,
    last_confirmed_frame: Cell<i32>,
    max_prediction_frames: Cell<i32>,
    input_queues: Cell<Option<Vec<InputQueue>>>,
    event_queue: Cell<(Producer<GGPOSyncEvent>, Consumer<GGPOSyncEvent>)>,
    local_connect_status: ConnectStatus,
}

struct GGPOSyncConfig {
    cb: Box<dyn GGPOSessionCallbacks>,
    num_prediction_frames: i32,
    num_players: i32,
    input_size: i32,
}

enum GGPOSyncEvent {
    GameInput,
}

struct GGPOSyncSavedFrame {
    buf: Vec<u8>,
    cbuf: usize,
    frame: i32,
    checksum: i32,
}

impl Default for GGPOSyncSavedFrame {
    fn default() -> Self {
        GGPOSyncSavedFrame {
            buf: Vec::new(),
            cbuf: 0,
            frame: -1,
            checksum: 0,
        }
    }
}

struct GGPOSyncSavedState {
    frames: [GGPOSyncSavedFrame; MAX_PREDICTION_FRAMES + 2],
    head: i32,
}

impl Drop for GGPOSync {
    fn drop(&mut self) {
        let saved_state = self.savedstate.get_mut();
        // TODO Change to if let. See https://stackoverflow.com/a/27363626/11821486
        self.config.get_mut().into_iter().for_each(|conf| {
            saved_state.into_iter().for_each(|state| {
                state.frames.iter().for_each(|frame| {
                    conf.cb.free_buffer(&frame.buf);
                })
            })
        });
        self.input_queues.set(None);
    }
}

impl GGPOSync {
    fn new(connect_status: ConnectStatus) -> GGPOSync {
        GGPOSync {
            config: Cell::new(None),
            rollingback: Cell::new(false),
            savedstate: Cell::new(None),
            framecount: Cell::new(0),
            last_confirmed_frame: Cell::new(-1),
            max_prediction_frames: Cell::new(0),
            local_connect_status: connect_status,
            input_queues: Cell::new(None),
            event_queue: Cell::new(RingBuffer::new(32).split()),
        }
    }

    fn init(&mut self, config: GGPOSyncConfig) {
        let num_players = config.num_players;
        let input_size = config.input_size;
        self.max_prediction_frames.set(config.num_prediction_frames);
        self.config = Cell::new(Some(config));
        self.framecount.set(0);
        self.rollingback.set(true);
        self.create_queues(num_players, input_size);
    }

    fn set_last_confirmed_frame(&mut self, frame: i32) {
        self.last_confirmed_frame.set(frame);
        let conf = self.config.get_mut().as_ref();
        let num_players = conf.expect("Config is set").num_players as usize;
        if self.last_confirmed_frame.get() > 0 {
            if let Some(ref mut input_queues) = self.input_queues.get_mut() {
                for i in 0..num_players {
                    if let Some(q) = input_queues.get_mut(i) {
                        q.discard_confirmed_frames(frame - 1);
                    }
                }
            }
        }
    }

    fn set_frame_delay(&mut self, queue: i32, delay: i32) {
        if let Some(ref mut input_queues) = self.input_queues.get_mut() {
            if let Some(q) = input_queues.get_mut(queue as usize) {
                q.set_frame_delay(delay);
            }
        }
    }

    fn add_local_input(&mut self, queue: i32, input: GameInput) -> bool {
        let frames_behind = self.framecount.get() - self.last_confirmed_frame.get();
        let max_prediction_frames = self.max_prediction_frames.get();
        let framecount = self.framecount.get();
        if framecount >= max_prediction_frames && frames_behind >= max_prediction_frames {
            return false;
        }
        if framecount == 0 {
            self.save_current_frame();
        }

        input.frame.set(Some(framecount));
        if let Some(ref mut input_queues) = self.input_queues.get_mut() {
            if let Some(q) = input_queues.get_mut(queue as usize) {
                q.add_input(input);
                return true;
            }
        }
        false
    }

    fn add_remote_input(&mut self, queue: i32, input: GameInput) {
        if let Some(ref mut input_queues) = self.input_queues.get_mut() {
            if let Some(q) = input_queues.get_mut(queue as usize) {
                q.add_input(input)
            }
        }
    }

    fn get_confirmed_inputs(&mut self, values: &dyn Any, size: usize, frame: i32) -> i32 {
        todo!();
    }

    fn synchronize_inputs(&mut self, values: &dyn Any, size: usize) -> i32 {
        todo!();
    }

    fn check_simulation(&mut self, timeout: i32) {
        todo!();
    }

    fn adjust_simulation(&mut self, seek_to: i32) {
        todo!();
    }

    fn increment_frame(&self) {
        todo!();
    }

    fn get_frame_count(&self) -> i32 {
        self.framecount.get()
    }

    fn in_rollback(&self) -> bool {
        self.rollingback.get()
    }

    fn get_event() -> GGPOSyncEvent {
        todo!();
    }

    // Protected
    fn load_frame(&self, frame: i32) {}

    fn save_current_frame(&self) {}

    fn find_saved_frame_index(frame: i32) {}

    fn get_last_saved_frame(&self) -> &SavedInfo {
        todo!();
    }

    fn create_queues(&self, num_players: i32, input_size: i32) -> bool {
        todo!();
    }

    fn check_simulation_consistency(seek_to: &mut i32) -> bool {
        todo!();
    }
    fn reset_prediction(frame_number: i32) {
        todo!();
    }
}

/////////////////////////
/// SyncTest
/////////////////////////

const GAMEINPUT_MAX_BYTES: usize = 9;
const GAMEINPUT_MAX_PLAYERS: usize = 2;

#[derive(Default)]
struct SavedInfo {
    frame: i32,
    checksum: i32,
    buf: Vec<u8>,
    cbuf: usize,
    input: GameInput,
}

#[derive(Default, Clone)]
struct GameInput {
    frame: Cell<Option<i32>>,
    size: usize,
    bits: Cell<[u8; GAMEINPUT_MAX_BYTES * GAMEINPUT_MAX_PLAYERS]>,
}

impl GameInput {
    fn erase(&mut self) {
        self.bits
            .set([0; GAMEINPUT_MAX_BYTES * GAMEINPUT_MAX_PLAYERS])
    }
}

struct SyncTestSession {
    cb: Box<dyn GGPOSessionCallbacks>,
    sync: GGPOSync,
    num_players: i32,
    rollingback: Cell<bool>,
    running: Cell<bool>,
    logfp: Option<Box<Path>>,
    game: String,
    check_distance: i32,
    last_verified: Cell<i32>,
    current_input: Cell<GameInput>,
    last_input: Cell<GameInput>,
    saved_frames: Cell<(Producer<SavedInfo>, Consumer<SavedInfo>)>,
}

impl Drop for SyncTestSession {
    fn drop(&mut self) {}
}

impl opaque::Opaque for SyncTestSession {
    fn do_poll(&self, timeout: i32) -> GGPOErrorCode {
        if !self.running.get() {
            let info = GGPOEvent {
                code: GGPOEventCode::GGPO_EVENTCODE_RUNNING,
                u: GGPOEventValue::GGPO_EVENTCODE_RUNNING,
            };
            self.cb.on_event(&info);
            self.running.set(true);
        }
        GGPO_OK
    }

    fn add_player(
        &self,
        player: &GGPOPlayer,
        handle: &GGPOPlayerHandle,
    ) -> GGPOResult<GGPOPlayerHandle> {
        if player.player_num < 1 || player.player_num > self.num_players {
            return Err(GGPOErrorCode::GGPO_ERRORCODE_PLAYER_OUT_OF_RANGE);
        }
        Ok(player.player_num - 1)
    }

    fn add_local_input(
        &self,
        player: GGPOPlayerHandle,
        values: &dyn Any,
        size: usize,
    ) -> GGPOErrorCode {
        if !self.running.get() {
            return GGPOErrorCode::GGPO_ERRORCODE_NOT_SYNCHRONIZED;
        }
        let idx = player as usize;
        for i in 0..size {
            //let mut input = self.current_input.get_mut();
            // TODO make this work
            //input.bits[idx * size + i] |= (values as &[u8])[i];
        }
        GGPO_OK
    }

    fn sync_input(
        &mut self,
        values: &dyn Any,
        size: usize,
        disconnect_flags: &mut i32,
    ) -> GGPOErrorCode {
        let old_input = if self.rollingback.get() {
            let (_, cons) = self.saved_frames.get_mut();
            // This is SHIIIT. SHEEEEEEEEIT
            let mut first: Option<GameInput> = None;
            cons.for_each(|e| match first {
                None => first = Some(e.input.clone()),
                Some(_) => {}
            });
            first.expect("Buffer should have values")
        } else {
            if self.sync.get_frame_count() == 0 {
                self.sync.save_current_frame();
            }
            self.current_input.get_mut().clone()
        };
        let last_input = GameInput {
            // TODO fix this
            //bits: values as [u8; 18],
            ..old_input
        };
        self.last_input.set(last_input);
        if *disconnect_flags != 0 {
            *disconnect_flags = 0i32;
        }
        GGPO_OK
    }

    fn increment_frame(&mut self) -> GGPOErrorCode {
        self.sync.increment_frame();
        self.current_input.get_mut().erase();

        if self.rollingback.get() {
            return GGPO_OK;
        }
        let frame = self.sync.get_frame_count();
        let last_saved_frame = self.sync.get_last_saved_frame();
        let saved_info = SavedInfo {
            frame,
            input: self.last_input.get_mut().clone(),
            buf: last_saved_frame.buf.clone(),
            ..*last_saved_frame
        };
        let (producer, consumer) = self.saved_frames.get_mut();
        if frame - self.last_verified.get() == self.check_distance {
            self.sync.load_frame(self.last_verified.get());
            self.rollingback.set(true);
            while !producer.is_empty() {
                self.cb.advance_frame(0);

                let maybe_info = consumer.pop();
                let mut info = match maybe_info {
                    None => {
                        continue;
                    }
                    Some(i) => i,
                };
                if info.frame != self.sync.get_frame_count() {
                    panic!(
                        "Frame number {} does not match saved frame number {}",
                        info.frame, frame
                    );
                }

                let checksum = self.sync.get_last_saved_frame().checksum;
                if info.checksum != checksum {
                    panic!(
                        "Checksum for frame {} does not match saved ({} != {})",
                        frame, checksum, info.checksum
                    );
                }
                info.buf.clear();
            }
            self.last_verified.set(frame);
            self.rollingback.set(false);
        }
        GGPO_OK
    }
}

impl GGPOSession for SyncTestSession {}

/////////////////////////
/// P2P
/////////////////////////

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
        size: usize,
    ) -> GGPOErrorCode {
        GGPO_OK
    }
    fn sync_input(
        &mut self,
        values: &dyn Any,
        size: usize,
        disconnect_flags: &mut i32,
    ) -> GGPOErrorCode {
        GGPO_OK
    }
}

impl GGPOSession for P2PSession {}

/////////////////////////
/// Spectator
/////////////////////////

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
        size: usize,
    ) -> GGPOErrorCode {
        GGPO_OK
    }
    fn sync_input(
        &mut self,
        values: &dyn Any,
        size: usize,
        disconnect_flags: &mut i32,
    ) -> GGPOErrorCode {
        GGPO_OK
    }
}

impl GGPOSession for SpectatorSession {}

/////////////////////////
/// Modules
/////////////////////////

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
            size: usize,
        ) -> GGPOErrorCode;
        fn sync_input(
            &mut self,
            values: &dyn Any,
            size: usize,
            disconnect_flags: &mut i32,
        ) -> GGPOErrorCode;
        fn increment_frame(&mut self) -> GGPOErrorCode {
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

/////////////////////////
/// Types
/////////////////////////

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

/////////////////////////
/// C-style API
/////////////////////////

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
    let sess = SyncTestSession {
        cb: Box::new(cb),
        num_players,
        game: game.to_owned(),
        check_distance: frames,
        last_verified: Cell::new(0),
        rollingback: Cell::new(false),
        running: Cell::new(false),
        logfp: None,
        sync: GGPOSync::new(ConnectStatus::default()),
        current_input: Cell::new(GameInput::default()),
        last_input: Cell::new(GameInput::default()),
        saved_frames: Cell::new(RingBuffer::new(32).split()),
    };
    sess.cb.begin_game(game);
    Ok(sess)
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
    size: usize,
) -> GGPOErrorCode {
    session.add_local_input(player, values, size)
}
pub fn ggpo_synchronize_input(
    session: &mut impl GGPOSession,
    values: &dyn Any,
    size: usize,
    disconnect_flags: &mut i32,
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
