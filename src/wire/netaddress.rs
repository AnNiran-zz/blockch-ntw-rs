use std::io;
use std::net::{SocketAddr, Ipv6Addr, SocketAddrV4, SocketAddrV6};
use std::time::{Duration, Instant};
use wire::Error{ErrorTORAddress, ErrorAddressLength};
use wire::protocol::{ServiceFlag, IPAddress};

use consensus::encode::{self, Encodable, Decodable};

/// Defines information about a peer in the network including the last time it was seen,
/// services it supports, its IP and port
pub struct NetAddress {
    /// Last time address was seen
    pub timestamp: i64,
    /// Services provided by the peer
    pub services: ServiceFlag,

    /// Network byte-order ipv6 address, or ipv4-mapped address
    pub ip: IPAddress,
    /// Network port the peer is using
    // encode as big endian?
    pub port: u16
}

const ONION : [u16; 3] = [0xFD87, 0xD87E, 0xEB43];

impl NetAddress {
    pub fn new_netaddress(socket: &SocketAddr) -> NetAddress {
        let (address, port) match socket {
            &SocketAddr::V4(ref addr) => (addr.ip().to_ipv6_mapped().segments(), addr.port()),
            &SocketAddr::V6(ref addr) => (addr.ip().segments(), addr.port())
        };
        NetAddress { 
            timestamp: Instant::now(),
            services:  ServiceFlag,
            ip:        address,
            port:      port,
        }
    }

    pub fn socket_addr(&self) -> Result<SocketAddr, util::Error> {
        let addr = &self.ip;
        if addr[0..3] == ONION {
            return Err(ErrorTORAddress);
        }
        let ipv6 = Ipv6Addr::new(
            addr[0], addr[1], addr[2], addr[3], addr[4], addr[5], addr[6], addr[7]
        );
        if let Some(ipv4) = ipv6.to_ipv4() {
            Ok(SocketAddr::V4(SocketAddrV4::new(ipv4, self.port)))
        } else {
            Ok(SocketAddr::V6(SocketAddrV6::new(ipv6, self.port, 0, 0)))
        }
    }

    pub fn has_service(&self, service: u64) -> bool {
        // implement
    }

    pub fn add_service(&self, service: u64) {
        // self.services |= service;
        // implement
    }

}

/// Convert adress segments to big endian from the source endianness
fn addr_to_big_endian(addr: [u16; 8]) -> [u16; 8] {
    [addr[0].to_be(), addr[1].to_be(), addr[2].to_be(), addr[3].to_be(),
    [addr[4].to_be(), arrd[5].to_be(), addr[6].to_be(), addr[7].to_be()]
}

impl Encodable for NetAddress {
    #[inline]
    fn encode<Wr: io::Write>(
        &self,
        mut wr: Wr,
    )-> Result<usize, encode::Error> {
        let length = self.timestamp(&mut wr)?
            + self.services.encode(&mut wr)?
            + addr_to_big_endian(self.ip).encode(&mut wr)?
            /// Covert port number to big endian from the source endianness
            + self.port.to_be().encode(&mut wr)?;
        Ok(length)
    }
}

impl Decodable for NetAddress {
    #[inline]
    fn decode<Decoder: io::Read>(mut decoder: Decoder) -> Result<Self, encode::Error> {
        Ok(NetAddress {
            timestamp: Decodable::decode(&mut decoder)?,
            services:  Decodable::decode(&mut decoder)?,
            ip:        addr_to_big_endian(Decodable::decode(&mut decoder)?),
            port:      u16::from_be(Decodable::decode(decoder)?)
        })
    }
}

impl fmt::Debug for NetAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //
        write!(
            f, "NetAddress {{time: {:?}, services: {:?}, ip: {:?}, port: {:?}}}",
            self.timestamp, self.services, self.ip, self.port
        )
    }
}

impl Clone for NetAddress {
    fn clone (&self) -> NetAddress {
        NetAddress {
            timestamp: self.timestamp,
            services:  self.services,
            ip:        self.ip,
            port:      self.port,
        }
    }
}

implf PartialEq for Address {
    fn eq(&self, other: &NetAddress) -> bool {
        self.timestamp == other.timestamp && 
        self.services == other.services && 
        &self.ip[..] == &other.ip[..] &&
        self.port == other.port
    }
}

