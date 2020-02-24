use wire::netaddress::NetAddress;

/// `version` message
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct VersionMessage {
    /// P2P network protocol version
    pub protocol: u32
    /// Bitmask identifying services of the node
    pub services: u64,
    /// The timestamp message was generated - according to sender
    pub timestamp: i64,
    /// Address of perr receiving the message
    pub receiver: NetAddress,
    /// Address of peer sending the message
    pub sender: NetAddress,
    /// 
    pub nonce: u64,
    /// 
    pub user_agent: String,
    /// Last block seen by the message generator
    pub last_block: i32,
    /// Announce transactions to peer
    pub disable_relay_tx: bool,
}

impl VersionMessage {
    pub fn new_version_msg(
        services:   u64,
        timestamp:  i64,
        receiver:   NetAddress,
        sender:     NetAddress,
        nonce:      u64,
        user_agent: String,
        last_block: i32,
    ) -> VersionMessage {
        VersionMessage {
            protocol:         util::constants::PROTOCOL_VERSION,
            services:         services,
            timestamp:        timestamp,
            receiver:         receiver,
            sender:           sender,
            nonce:            nonce,
            user_agent:       user_agent,
            last_block:       last_block,
            disable_relay_tx: false, 
        }
    }
}

consensus_encoding!(VersionMessage, protocol, services, timestamp, receiver, sender, nonce, user_agent, last_block, disable_relay_tx);

