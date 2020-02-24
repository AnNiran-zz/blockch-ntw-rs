use std::string;
use std::string::String;

/// Identifies services supported by the peer
type ServiceFlag = u64;
type IPAddress = [u16, 8];

const SERVICE_FLAGS: [String; 3] = ["SFNodeNetwork", "SFNodeBloom", "SFNodeCF"];

implf ServiceFalg {
    /// Returns Service flag in human readable form
    pub fn string(&self) -> str {
        /// No flags are set
        if self == 0 {
            let ndsrv = "0x0"
        }

        /// Node Service is a result of concatinating strings
        /// We use heap memory for the concatenation results
        let mut ndsrv = String::from("");
        // to be implemented
    }
}