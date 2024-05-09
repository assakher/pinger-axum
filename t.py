from socket import AF_INET, SOCK_STREAM, create_server, socket


if __name__ == "__main__":
    data = """{
  "STATUS": [
    {
      "STATUS": "S",
      "When": 161232,
      "Code": 11,
      "Msg": "Summary",
      "Description": "cgminer 4.11.1"
    }
  ],
  "SUMMARY": [
    {
      "Elapsed": 161218,
      "MHS av": 88027946.77,
      "MHS 30s": 87240933.02,
      "MHS 1m": 87801784.43,
      "MHS 5m": 87831734.35,
      "MHS 15m": 87908560.29,
      "Found Blocks": 0,
      "Getworks": 5745,
      "Accepted": 11330,
      "Rejected": 5,
      "Hardware Errors": 0,
      "Utility": 4.22,
      "Discarded": 851964100,
      "Stale": 4,
      "Get Failures": 0,
      "Local Work": 3265864,
      "Remote Failures": 0,
      "Network Blocks": 268,
      "Total MH": 14185230482682,
      "Work Utility": 1229760.34,
      "Difficulty Accepted": 3317465088,
      "Difficulty Rejected": 1572864,
      "Difficulty Stale": 0,
      "Best Share": 12764451807,
      "Device Hardware%": 0,
      "Device Rejected%": 0.0476,
      "Pool Rejected%": 0.0474,
      "Pool Stale%": 0,
      "Last getwork": 0
    }
  ],
  "id": 1
}
""".encode()
    with create_server(('localhost', 8008)) as server:
        while True:
            opened_sock, addr = server.accept()
            try:
                opened_sock.recv(1)
                opened_sock.sendall(data)
            finally:
                opened_sock.close()
