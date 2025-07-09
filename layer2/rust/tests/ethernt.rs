#[test]
fn test_parse_valid_frame() {
    let raw = [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff,             // dst MAC
        0x00, 0x0c, 0x29, 0xab, 0xcd, 0xef,             // src MAC
        0x08, 0x00,                                     // EtherType = IPv4
        0xde, 0xad, 0xbe, 0xef                          // payload (4 bytes)
    ];
    let frame = EthernetFrame::from_bytes(&raw).unwrap();
    assert_eq!(frame.dst_mac, [0xff; 6]);
    assert_eq!(frame.src_mac, [0x00, 0x0c, 0x29, 0xab, 0xcd, 0xef]);
    assert_eq!(frame.ethertype, 0x0800);
    assert_eq!(frame.payload, vec![0xde, 0xad, 0xbe, 0xef]);
}
