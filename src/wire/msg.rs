/// Implement general message functionalities and concepts
/// 
use wire::msgver;
use wire::msgblock;
use wire::msgreject;

use consensus::encode::{self, Encodable, Decodable, VarInt, Error};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Message {
    VersionMessage(msgver::VersionMessage),

    GetHeadersMessage(msgblock::GetHeadersMessage),
    GetBlocksMessage(msgblock::GetBlocksMessage),

    RejectMessage(msgreject::RejectMessage),

    /// Mesages types to be implemented
    VersionAcknoledgeMessage,
    NotFoundMessage,

    PingMessage,
    PongMessage,
    GetCFiltersMessage,
    SendCFilterMessage,
    
    // ...
}
