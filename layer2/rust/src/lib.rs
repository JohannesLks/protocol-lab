pub struct EthernetFrame {
    pub dst_mac: [u8; 6],
    pub src_mac: [u8; 6],
    pub ethertype: u16,
    pub payload: Vec<u8>,
}

impl EthernetFrame {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < 14 {
            return Err("Frame too short");
        }
        let dst_mac = bytes[0..6].try_into().unwrap();
        let src_mac = bytes[6..12].try_into().unwrap();
        let ethertype = u16::from_be_bytes([bytes[12], bytes[13]]);
        let payload = bytes[14..].to_vec();

        Ok(EthernetFrame {
            dst_mac,
            src_mac,
            ethertype,
            payload,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(14 + self.payload.len());
        buf.extend_from_slice(&self.dst_mac);
        buf.extend_from_slice(&self.src_mac);
        buf.extend_from_slice(&self.ethertype.to_be_bytes());
        buf.extend_from_slice(&self.payload);
        buf
    }

    pub fn dst_mac_str(&self) -> String {
        self.dst_mac.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(":")
    }

}
