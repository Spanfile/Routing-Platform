use anyhow::anyhow;
use byteorder::{ByteOrder, LittleEndian};
use eui48::MacAddress;
use neli::{consts::rtnl::Ifla, rtnl::Ifinfomsg};
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Link {
    name: String,
    index: i32,
    address: MacAddress,
    mtu: u32,
}

impl TryFrom<Ifinfomsg<Ifla>> for Link {
    type Error = anyhow::Error;

    fn try_from(value: Ifinfomsg<Ifla>) -> Result<Self, Self::Error> {
        let index = value.ifi_index;
        let mut name = None;
        let mut address = None;
        let mut mtu = None;

        for rtattr in value.rtattrs {
            match rtattr.rta_type {
                Ifla::Ifname => {
                    // strip the null-byte from the payload
                    let len = rtattr.rta_payload.len();
                    name = Some(String::from_utf8(
                        rtattr.rta_payload.into_iter().take(len - 1).collect(),
                    )?);
                }
                Ifla::Address => {
                    address = Some(MacAddress::from_bytes(&rtattr.rta_payload)?);
                }
                Ifla::Mtu => {
                    mtu = Some(LittleEndian::read_u32(&rtattr.rta_payload));
                }
                _ => (),
            }
        }

        Ok(Link {
            index,
            name: name.ok_or_else(|| anyhow!("missing name in ifinfomsg"))?,
            address: address.ok_or_else(|| anyhow!("missing address in ifinfomsg"))?,
            mtu: mtu.ok_or_else(|| anyhow!("missing mtu in ifinfomsg"))?,
        })
    }
}
