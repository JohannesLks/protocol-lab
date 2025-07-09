# protocol-lab

**protocol-lab** is a low-level learning and research project focused on implementing common network protocols from scratch across the OSI model. Each protocol is developed manually, from byte-level parsing to packet generation, with the goal of gaining a deeper understanding of networking, protocol behavior, and security implications.

The project emphasizes transparency, reproducibility, and educational clarity. It is intended as both a practical tool and a learning framework for cybersecurity engineers, system programmers, and protocol enthusiasts.

## Objectives

- Implement key network protocols manually, without relying on high-level protocol libraries
- Understand protocol design by reconstructing them from raw specifications (e.g. RFCs)
- Analyze and validate packet behavior using real-world data (e.g. pcap traces)
- Document structural patterns, edge cases, and potential vulnerabilities
- Build a testable, extensible protocol lab for research, fuzzing, and visualization

## Project Structure

```

protocol-lab/
├── layer2/         # Ethernet, ARP
├── layer3/         # IPv4, IPv6, ICMP
├── layer4/         # UDP, TCP
├── layer7/         # DNS, HTTP, SMTP, TLS (partial), etc.
├── shared/         # Checksum logic, binary encoders, utilities
├── tests/          # Packet validation and test vectors
└── docs/           # RFC references, diagrams, notes

```

Each protocol implementation includes:

- Byte-level serializers and parsers
- Header validation logic
- RFC-based test vectors
- Annotated code linked to relevant RFC sections

## Implementation Principles

- **Raw-first**: Protocol logic is implemented by hand, based on raw byte streams
- **RFC-driven**: Each feature is documented and validated against its specification
- **Testable**: All modules include tests based on real packet captures and edge cases
- **Modular**: Code is structured by layer and reusable across protocols
- **Minimal dependencies**: No external protocol libraries (e.g. Scapy) are used in core logic

## Protocol Roadmap

| OSI Layer | Protocols (planned / in progress) |
|-----------|------------------------------------|
| Layer 2   | Ethernet (802.3), ARP              |
| Layer 3   | IPv4, IPv6, ICMP                   |
| Layer 4   | UDP, TCP (3-Way-Handshake, Flags, Windowing) |
| Layer 7   | DNS, HTTP 1.1, SMTP, FTP, TLS (partial), SSH (simplified) |
| Control   | DHCP, BGP (experimental), QUIC (exploratory) |

## Tooling

The following tools are used for testing, validation, and inspection:

- **Wireshark** / **tcpdump**: Packet inspection and validation
- **pytest** or **cargo test**: Automated tests per protocol
- **pcap files**: Ground truth data for parser validation
- **Scapy** (optional): Used for generating test cases (not in production code)

## Contribution & Licensing

This project is open for collaboration, feedback, and educational reuse. Contributions are welcome, especially in the form of additional protocols, test vectors, or fuzzing tools.

All code is licensed under the MIT License.

## Acknowledgements

Special thanks to [**Leon Morten Richter**](https://leonmortenrichter.de/) for the original idea and inspiration behind this project. His interest in building network protocols from first principles helped shape the vision and direction of protocol-lab.



