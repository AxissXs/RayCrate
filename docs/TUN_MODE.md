# RayCrate TUN Mode Deep Dive

TUN mode allows RayCrate to proxy 100% of system traffic (TCP and UDP) without requiring individual applications to be configured with SOCKS5 or HTTP proxy settings.

---

## 1. How TUN Mode Works in RayCrate

1. **Virtual Interface Creation**:
   - RayCrate requests elevated privileges (root on Linux/macOS, Administrator on Windows).
   - Creates a virtual TUN network interface (`raycrate-tun0` or system default).
2. **IP Packet Interception**:
   - The OS routing table is modified so that default gateway traffic (or targeted CIDRs) is directed into the virtual TUN interface.
3. **User-Space Packet Processing**:
   - RayCrate's Rust core reads IP packets from the TUN interface.
   - Using crates like `tun2` and `smoltcp`, TCP/UDP packets are parsed and forwarded to the local proxy inbound (SOCKS5/TUN inbound) provided by Xray-core.
4. **DNS Handling**:
   - DNS requests are intercepted or forwarded securely over the proxy (DNS over HTTPS / remote DNS resolution via Xray) to prevent DNS leaks.

---

## 2. Platform-Specific Implementation Details

### Linux
- **Interface**: `tun` device created via kernel TUN/TAP driver.
- **Routing**: Configures default route via `ip route` or `nftables`/`iptables` mark-based routing.
- **Privileges**: Requires `CAP_NET_ADMIN` capability or `sudo`.

### macOS
- **Interface**: `utun` device.
- **Routing**: Uses `route add` commands to redirect traffic through the virtual interface.
- **Privileges**: Requires administrator authorization via `AuthorizationExecuteWithPrivileges` or `sudo`.

### Windows
- **Interface**: Uses the official **Wintun** driver (high performance layer 3 TUN driver).
- **Routing**: Modifies Windows IPv4/IPv6 routing tables (`netsh interface ip set address` / PowerShell routing commands).
- **Privileges**: Requires administrator elevation (UAC prompt).
