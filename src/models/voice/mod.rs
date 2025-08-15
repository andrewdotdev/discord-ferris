use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const VOICE_GATEWAY_VERSION: &str = "8";

/**
 * @see {@link https://discord.com/developers/docs/topics/opcodes-and-status-codes#voice-voice-opcodes}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum VoiceOpcodes {
    /**
     * Begin a voice websocket connection
     */
    Identify = 0,
    /**
     * Select the voice protocol
     */
    SelectProtocol = 1,
    /**
     * Complete the websocket handshake
     */
    Ready = 2,
    /**
     * Keep the websocket connection alive
     */
    Heartbeat = 3,
    /**
     * Describe the session
     */
    SessionDescription = 4,
    /**
     * Indicate which users are speaking
     */
    Speaking = 5,
    /**
     * Sent to acknowledge a received client heartbeat
     */
    HeartbeatAck = 6,
    /**
     * Resume a connection
     */
    Resume = 7,
    /**
     * Time to wait between sending heartbeats in milliseconds
     */
    Hello = 8,
    /**
     * Acknowledge a successful session resume
     */
    Resumed = 9,
    /**
     * One or more clients have connected to the voice channel
     */
    ClientsConnect = 11,
    /**
     * A client has disconnected from the voice channel
     */
    ClientDisconnect = 13,
    /**
     * A downgrade from the DAVE protocol is upcoming
     */
    DavePrepareTransition = 21,
    /**
     * Execute a previously announced protocol transition
     */
    DaveExecuteTransition = 22,
    /**
     * Acknowledge readiness previously announced transition
     */
    DaveTransitionReady = 23,
    /**
     * A DAVE protocol version or group change is upcoming
     */
    DavePrepareEpoch = 24,
    /**
     * Credential and public key for MLS external sender
     */
    DaveMlsExternalSender = 25,
    /**
     * MLS Key Package for pending group member
     */
    DaveMlsKeyPackage = 26,
    /**
     * MLS Proposals to be appended or revoked
     */
    DaveMlsProposals = 27,
    /**
     * MLS Commit with optional MLS Welcome messages
     */
    DaveMlsCommitWelcome = 28,
    /**
     * MLS Commit to be processed for upcoming transition
     */
    DaveMlsAnnounceCommitTransition = 29,
    /**
     * MLS Welcome to group for upcoming transition
     */
    DaveMlsWelcome = 30,
    /**
     * Flag invalid commit or welcome, request re-add
     */
    DaveMlsInvalidCommitWelcome = 31,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/opcodes-and-status-codes#voice-voice-close-event-codes}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u16)]
pub enum VoiceCloseCodes {
    /**
     * You sent an invalid opcode
     */
    UnknownOpcode = 4_001,
    /**
     * You sent a invalid payload in your identifying to the Gateway
     */
    FailedToDecode = 4_002,
    /**
     * You sent a payload before identifying with the Gateway
     */
    NotAuthenticated = 4_003,
    /**
     * The token you sent in your identify payload is incorrect
     */
    AuthenticationFailed = 4_004,
    /**
     * You sent more than one identify payload. Stahp
     */
    AlreadyAuthenticated = 4_005,
    /**
     * Your session is no longer valid
     */
    SessionNoLongerValid = 4_006,
    /**
     * Your session has timed out
     */
    SessionTimeout = 4_009,
    /**
     * We can't find the server you're trying to connect to
     */
    ServerNotFound = 4_011,
    /**
     * We didn't recognize the protocol you sent
     */
    UnknownProtocol = 4_012,
    /**
     * Either the channel was deleted, you were kicked, or the main gateway session was dropped. Should not reconnect
     */
    Disconnected = 4_014,
    /**
     * The server crashed. Our bad! Try resuming
     */
    VoiceServerCrashed = 4_015,
    /**
     * We didn't recognize your encryption
     */
    UnknownEncryptionMode = 4_016,
    /**
     * You sent a malformed request
     */
    BadRequest = 4_020,
    /**
     * Disconnect due to rate limit exceeded. Should not reconnect
     */
    RateLimited = 4_021,
    /**
     * Disconnect all clients due to call terminated (channel deleted, voice server changed, etc.). Should not reconnect
     */
    CallTerminated = 4_022,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/voice-connections#transport-encryption-modes}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VoiceEncryptionMode {
    /**
     * AEAD AES256-GCM (RTP Size)
     */
    #[serde(rename = "aead_aes256_gcm_rtpsize")]
    AeadAes256GcmRtpSize,
    /**
     * AEAD XChaCha20 Poly1305 (RTP Size)
     */
    #[serde(rename = "aead_xchacha20_poly1305_rtpsize")]
    AeadXChaCha20Poly1305RtpSize,
    /**
     * XSalsa20 Poly1305 Lite (RTP Size)
     *
     * @deprecated This encryption mode has been discontinued.
     */
    #[serde(rename = "xsalsa20_poly1305_lite_rtpsize")]
    XSalsa20Poly1305LiteRtpSize,
    /**
     * AEAD AES256-GCM
     *
     * @deprecated This encryption mode has been discontinued.
     */
    #[serde(rename = "aead_aes256_gcm")]
    AeadAes256Gcm,
    /**
     * XSalsa20 Poly1305
     *
     * @deprecated This encryption mode has been discontinued.
     */
    #[serde(rename = "xsalsa20_poly1305")]
    XSalsa20Poly1305,
    /**
     * XSalsa20 Poly1305 Suffix
     *
     * @deprecated This encryption mode has been discontinued.
     */
    #[serde(rename = "xsalsa20_poly1305_suffix")]
    XSalsa20Poly1305Suffix,
    /**
     * XSalsa20 Poly1305 Lite
     *
     * @deprecated This encryption mode has been discontinued.
     */
    #[serde(rename = "xsalsa20_poly1305_lite")]
    XSalsa20Poly1305Lite,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/voice-connections#speaking}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum VoiceSpeakingFlags {
    /**
     * Normal transmission of voice audio
     */
    Microphone = 1 << 0,
    /**
     * 	Transmission of context audio for video, no speaking indicator
     */
    Soundshare = 1 << 1,
    /**
     * Priority speaker, lowering audio of other speakers
     */
    Priority = 1 << 2,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VoiceSendPayload {
    VoiceDaveMlsInvalidCommitWelcome(VoiceDaveMlsInvalidCommitWelcome),
    VoiceDaveTransitionReady(VoiceDaveTransitionReady),
    VoiceHeartbeat(VoiceHeartbeat),
    VoiceIdentify(VoiceIdentify),
    VoiceResume(VoiceResume),
    VoiceSelectProtocol(VoiceSelectProtocol),
    VoiceSpeakingSend(VoiceSpeakingSend),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VoiceReceivePayload {
    VoiceClientDisconnect(VoiceClientDisconnect),
    VoiceClientsConnect(VoiceClientsConnect),
    VoiceDaveExecuteTransition(VoiceDaveExecuteTransition),
    VoiceDavePrepareEpochRecv(VoiceDavePrepareEpoch),
    VoiceDavePrepareTransition(VoiceDavePrepareTransition),
    VoiceHeartbeatAck(VoiceHeartbeatAck),
    VoiceHello(VoiceHello),
    VoiceReady(VoiceReady),
    VoiceResumed(VoiceResumed),
    VoiceSessionDescription(VoiceSessionDescription),
    VoiceSpeaking(VoiceSpeaking),
}

// #region Server Payloads

/**
 * @see {@link https://discord.com/developers/docs/topics/voice-connections#heartbeating}
 */
pub type VoiceHello = _DataPayload<VoiceOpcodes, VoiceHelloData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/voice-connections#heartbeating}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceHelloData {
    /**
     * Voice gateway version
     *
     * @see {@link https://discord.com/developers/docs/topics/voice-connections#voice-gateway-versioning-gateway-versions}
     */
    pub v: i64,
    /**
     * The interval (in milliseconds) the client should heartbeat with
     */
    pub heartbeat_interval: i64,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/voice-connections#establishing-a-voice-websocket-connection}
 */
pub type VoiceReady = _DataPayload<VoiceOpcodes, VoiceReadyData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/voice-connections#establishing-a-voice-websocket-connection}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceReadyData {
    /**
     * SSRC identifier
     */
    pub ssrc: i64,
    /**
     * UDP IP
     */
    pub ip: String,
    /**
     * UDP port
     */
    pub port: i64,
    /**
     * Supported encryption modes
     *
     * @see {@link https://discord.com/developers/docs/topics/voice-connections#transport-encryption-modes}
     */
    pub modes: Vec<VoiceEncryptionMode>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/voice-connections#heartbeating}
 */
pub type VoiceHeartbeatAck = _DataPayload<VoiceOpcodes, VoiceHeartbeatAckData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/voice-connections#heartbeating}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceHeartbeatAckData {
    /**
     * The integer nonce
     */
    pub t: i64,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/voice-connections#transport-encryption-and-sending-voice}
 */
pub type VoiceSessionDescription = _DataPayload<VoiceOpcodes, VoiceSessionDescriptionData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/voice-connections#transport-encryption-and-sending-voice}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceSessionDescriptionData {
    /**
     * The selected mode
     */
    pub mode: VoiceEncryptionMode,
    /**
     * The secret key
     */
    pub secret_key: Vec<i64>,
    /**
     * The selected DAVE protocol version
     *
     * @see {@link https://daveprotocol.com/#select_protocol_ack-4}
     */
    pub dave_protocol_version: i64,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/voice-connections#resuming-voice-connection}
 */
pub type VoiceResumed = _DataPayload<VoiceOpcodes, ()>;

/**
 * @see {@link https://discord.com/developers/docs/topics/voice-connections#speaking}
 */
pub type VoiceSpeaking = _DataPayload<VoiceOpcodes, VoiceSpeakingData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/voice-connections#speaking}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceSpeakingData {
    /**
     * The speaking mode flags
     */
    pub speaking: VoiceSpeakingFlags,
    /**
     * SSRC identifier
     */
    pub ssrc: i64,
    /**
     * User id
     */
    pub user_id: String,
}

/**
 * @see {@link https://daveprotocol.com/#clients_connect-11}
 */
pub type VoiceClientsConnect = _DataPayload<VoiceOpcodes, VoiceClientsConnectData>;

/**
 * @see {@link https://daveprotocol.com/#clients_connect-11}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceClientsConnectData {
    /**
     * The connected user ids
     */
    pub user_ids: Vec<String>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/voice-connections}
 */
pub type VoiceClientDisconnect = _DataPayload<VoiceOpcodes, VoiceClientDisconnectData>;

/**
 * @see {@link https://discord.com/developers/docs/topics/voice-connections}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceClientDisconnectData {
    /**
     * The disconnected user id
     */
    pub user_id: String,
}

/**
 * @see {@link https://daveprotocol.com/#dave_protocol_prepare_transition-21}
 */
pub type VoiceDavePrepareTransition = _DataPayload<VoiceOpcodes, VoiceDavePrepareTransitionData>;

/**
 * @see {@link https://daveprotocol.com/#dave_protocol_prepare_transition-21}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceDavePrepareTransitionData {
    /**
     * The protocol version
     */
    pub protocol_version: i64,
    /**
     * The transition id
     */
    pub transition_id: i64,
}

/**
 * @see {@link https://daveprotocol.com/#dave_protocol_execute_transition-22}
 */
pub type VoiceDaveExecuteTransition = _DataPayload<VoiceOpcodes, VoiceDaveExecuteTransitionData>;

/**
 * @see {@link https://daveprotocol.com/#dave_protocol_execute_transition-22}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceDaveExecuteTransitionData {
    /**
     * The transition id
     */
    pub transition_id: i64,
}

/**
 * @see {@link https://daveprotocol.com/#dave_mls_external_sender_package-25}
 */
pub type VoiceDaveMlsExternalSender = VoiceBinaryPayload;

/**
 * @see {@link https://daveprotocol.com/#dave_mls_proposals-27}
 */
pub type VoiceDaveMlsProposals = VoiceBinaryPayload;

/**
 * @see {@link https://daveprotocol.com/#dave_mls_announce_commit_transition-29}
 */
pub type VoiceDaveMlsAnnounceCommitTransition = VoiceBinaryPayload;

/**
 * @see {@link https://daveprotocol.com/#dave_mls_welcome-30}
 */
pub type VoiceDaveMlsWelcome = VoiceBinaryPayload;

// #endregion Server Payloads

// #region Sendable Payloads

/**
 * @see {@link https://discord.com/developers/docs/topics/voice-connections#establishing-a-voice-websocket-connection}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceIdentify {
    pub op: VoiceOpcodes,
    pub d: VoiceIdentifyData,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/voice-connections#establishing-a-voice-websocket-connection}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceIdentifyData {
    /**
     * The id of the server to connect to
     */
    pub server_id: String,
    /**
     * The id of the user to connect as
     */
    pub user_id: String,
    /**
     * Voice state session id
     */
    pub session_id: String,
    /**
     * Voice connection token
     */
    pub token: String,
    /**
     * The maximum DAVE protocol version supported
     */
    pub max_dave_protocol_version: Option<i64>,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/voice-connections#establishing-a-voice-websocket-connection}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceHeartbeat {
    pub op: VoiceOpcodes,
    pub d: VoiceHeartbeatData,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/voice-connections#establishing-a-voice-websocket-connection}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceHeartbeatData {
    /**
     * The integer nonce
     */
    pub t: i64,
    /**
     * The last sequence number recieved
     */
    pub seq_ack: i64,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/voice-connections#establishing-a-voice-udp-connection}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceSelectProtocol {
    pub op: VoiceOpcodes,
    pub d: VoiceSelectProtocolData,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/voice-connections#establishing-a-voice-udp-connection}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceSelectProtocolData {
    /**
     * Voice protocol
     */
    pub protocol: String,
    /**
     * Data associated with the protocol
     */
    pub data: VoiceUDPProtocolData,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/voice-connections#establishing-a-voice-udp-connection}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceUDPProtocolData {
    /**
     * External address
     */
    pub address: String,
    /**
     * External UDP port
     */
    pub port: i64,
    /**
     * Selected mode
     *
     * @see {@link https://discord.com/developers/docs/topics/voice-connections#transport-encryption-modes}
     */
    pub mode: VoiceEncryptionMode,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/voice-connections#resuming-voice-connection}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceResume {
    pub op: VoiceOpcodes,
    pub d: VoiceResumeData,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/voice-connections#resuming-voice-connection}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceResumeData {
    /**
     * The id of the server to connect to
     */
    pub server_id: String,
    /**
     * Voice state session id
     */
    pub session_id: String,
    /**
     * Voice connection token
     */
    pub token: String,
    /**
     * Last recieved sequence number
     */
    pub seq_ack: i64,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/voice-connections#speaking}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceSpeakingSend {
    pub op: VoiceOpcodes,
    pub d: VoiceSpeakingSendData,
}

/**
 * @see {@link https://discord.com/developers/docs/topics/voice-connections#speaking}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceSpeakingSendData {
    /**
     * The speaking mode flags
     */
    pub speaking: VoiceSpeakingFlags,
    /**
     * Voice delay
     */
    pub delay: i64,
    /**
     * SSRC identifier
     */
    pub ssrc: i64,
}

/**
 * @see {@link https://daveprotocol.com/#dave_protocol_ready_for_transition-23}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceDaveTransitionReady {
    pub op: VoiceOpcodes,
    pub d: VoiceDaveTransitionReadyData,
}

/**
 * @see {@link https://daveprotocol.com/#dave_protocol_ready_for_transition-23}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceDaveTransitionReadyData {
    /**
     * The transition id
     */
    pub transition_id: i64,
}

/**
 * @see {@link https://daveprotocol.com/#dave_protocol_prepare_epoch-24}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceDavePrepareEpoch {
    pub op: VoiceOpcodes,
    pub d: VoiceDavePrepareEpochData,
}

/**
 * @see {@link https://daveprotocol.com/#dave_protocol_prepare_epoch-24}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceDavePrepareEpochData {
    /**
     * The protocol version
     */
    pub protocol_version: i64,
    /**
     * The epoch id
     */
    pub epoch: i64,
}

/**
 * @see {@link https://daveprotocol.com/#dave_mls_invalid_commit_welcome-31}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceDaveMlsInvalidCommitWelcome {
    pub op: VoiceOpcodes,
    pub d: VoiceDaveMlsInvalidCommitWelcomeData,
}

/**
 * @see {@link https://daveprotocol.com/#dave_mls_invalid_commit_welcome-31}
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceDaveMlsInvalidCommitWelcomeData {
    /**
     * The transition id
     */
    pub transition_id: i64,
}

/**
 * @see {@link https://daveprotocol.com/#dave_mls_key_package-26}
 */
pub type VoiceDaveMlsKeyPackage = VoiceBinaryPayload;

/**
 * @see {@link https://daveprotocol.com/#dave_mls_commit_welcome-28}
 */
pub type VoiceDaveMlsCommitWelcome = VoiceBinaryPayload;
// #endregion

// #region Shared
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct _BasePayload {
    /**
     * Opcode for the payload
     */
    pub op: VoiceOpcodes,
    /**
     * Event data
     */
    pub d: Option<Value>,
    /**
     * Sequence number, used for resuming sessions and heartbeats
     */
    pub seq: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct _DataPayload<Op, D> {
    pub op: Op,
    pub d: D,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq: Option<i64>,
}

pub type VoiceBinaryPayload = Vec<u8>;
// #endregion Shared
