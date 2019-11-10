use neli::{
    consts::{
        nl::Rtm,
        rtnl::{Arphrd, Iff, Ifla, RtAddrFamily},
        NlFamily, NlmF,
    },
    nl::Nlmsghdr,
    rtnl::{Ifinfomsg, Rtattr},
    socket::NlSocket,
};

pub fn list() -> anyhow::Result<()> {
    let mut socket = NlSocket::connect(NlFamily::Route, None, None, true)?;

    let flags: Vec<Iff> = vec![];
    let attrs: Vec<Rtattr<Ifla, Vec<u8>>> = vec![Rtattr {
        rta_len: 0,
        rta_type: Ifla::Address,
        rta_payload: vec![],
    }];
    let ifinfomsg = Ifinfomsg::new(RtAddrFamily::Inet, Arphrd::Ether, 0, flags, attrs);
    let nlhdr = {
        let len = None;
        let nl_type = Rtm::Getlink;
        let flags = vec![NlmF::Request, NlmF::Dump];
        let seq = None;
        let pid = None;
        let payload = ifinfomsg;
        Nlmsghdr::new(len, nl_type, flags, seq, pid, payload)
    };

    socket.send_nl(nlhdr)?;
    let mut iter = socket.iter::<Rtm, Ifinfomsg<Ifla>>();

    while let Some(Ok(response)) = iter.next() {
        let payload = response.nl_payload;
        print!(
            "{:?} {:?} {:?} {:?} ",
            payload.ifi_family, payload.ifi_type, payload.ifi_index, payload.ifi_flags
        );
        for rtattr in payload.rtattrs {
            print!(" {:?}", rtattr.rta_type);
        }
        println!();
    }

    Ok(())
}
