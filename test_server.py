from socket import create_server


if __name__ == "__main__":
    data = """{
  "summary": [
    {
      "STATUS": [
        {
          "STATUS": "S",
          "When": 81262,
          "Code": 11,
          "Msg": "Summary",
          "Description": "cgminer 4.11.1"
        }
      ],
      "SUMMARY": [
        {
          "Elapsed": 81249,
          "MHS av": 81332660.35,
          "MHS 30s": 82238725.37,
          "MHS 1m": 82448320.78,
          "MHS 5m": 82385261.49,
          "MHS 15m": 82251462.79,
          "Found Blocks": 0,
          "Getworks": 2033,
          "Accepted": 3104,
          "Rejected": 1,
          "Hardware Errors": 12313,
          "Utility": 2.29,
          "Discarded": 180052280,
          "Stale": 0,
          "Get Failures": 0,
          "Local Work": 3029997,
          "Remote Failures": 0,
          "Network Blocks": 137,
          "Total MH": 6606032867049.0,
          "Work Utility": 1136216.48,
          "Difficulty Accepted": 1565720576.0,
          "Difficulty Rejected": 524288.0,
          "Difficulty Stale": 0.0,
          "Best Share": 32094191483,
          "Device Hardware%": 0.0008,
          "Device Rejected%": 0.0341,
          "Pool Rejected%": 0.0335,
          "Pool Stale%": 0.0,
          "Last getwork": 0
        }
      ],
      "id": 1
    }
  ],
  "pools": [
    {
      "STATUS": [
        {
          "STATUS": "S",
          "When": 81262,
          "Code": 7,
          "Msg": "3 Pool(s)",
          "Description": "cgminer 4.11.1"
        }
      ],
      "POOLS": [
        {
          "POOL": 0,
          "URL": "stratum+tcp://btc-cudc-ru.f2pool.com:3333",
          "Status": "Alive",
          "Priority": 0,
          "Quota": 1,
          "Long Poll": "N",
          "Getworks": 2031,
          "Accepted": 3104,
          "Rejected": 1,
          "Works": 1514866,
          "Discarded": 0,
          "Stale": 0,
          "Get Failures": 0,
          "Remote Failures": 0,
          "User": "temporal15.k2lx112x5x9",
          "Last Share Time": 81253,
          "Diff1 Shares": 1538614272,
          "Proxy Type": "",
          "Proxy": "",
          "Difficulty Accepted": 1565720576.0,
          "Difficulty Rejected": 524288.0,
          "Difficulty Stale": 0.0,
          "Last Share Difficulty": 524288.0,
          "Work Difficulty": 524288.0,
          "Has Stratum": true,
          "Stratum Active": true,
          "Stratum URL": "btc-cudc-ru.f2pool.com",
          "Stratum Difficulty": 524288.0,
          "Has Vmask": true,
          "Has GBT": false,
          "Best Share": 32094191483,
          "Pool Rejected%": 0.0335,
          "Pool Stale%": 0.0,
          "Bad Work": 137,
          "Current Block Height": 844418,
          "Current Block Version": 536870912
        },
        {
          "POOL": 1,
          "URL": "stratum+tcp://btc.f2pool .com:1314",
          "Status": "Alive",
          "Priority": 1,
          "Quota": 1,
          "Long Poll": "N",
          "Getworks": 2,
          "Accepted": 0,
          "Rejected": 0,
          "Works": 0,
          "Discarded": 0,
          "Stale": 0,
          "Get Failures": 0,
          "Remote Failures": 0,
          "User": "temporal15.k2lx112x5x9",
          "Last Share Time": 0,
          "Diff1 Shares": 0,
          "Proxy Type": "",
          "Proxy": "",
          "Difficulty Accepted": 0.0,
          "Difficulty Rejected": 0.0,
          "Difficulty Stale": 0.0,
          "Last Share Difficulty": 0.0,
          "Work Difficulty": 65536.0,
          "Has Stratum": true,
          "Stratum Active": false,
          "Stratum URL": "",
          "Stratum Difficulty": 0.0,
          "Has Vmask": true,
          "Has GBT": false,
          "Best Share": 0,
          "Pool Rejected%": 0.0,
          "Pool Stale%": 0.0,
          "Bad Work": 0,
          "Current Block Height": 844282,
          "Current Block Version": 536870912
        },
        {
          "POOL": 2,
          "URL": "stra tum+tcp://stratum.slushpool.com:3333",
          "Status": "Alive",
          "Priority": 2,
          "Quota": 1,
          "Long Poll": "N",
          "Getworks": 0,
          "Accepted": 0,
          "Rejected": 0,
          "Works": 0,
          "Discarded": 0,
          "Stale": 0,
          "Get Failures": 0,
          "Remote Failures": 0,
          "User": "Serhio_Ramos.temporal15xk2lx 112x5x9",
          "Last Share Time": 0,
          "Diff1 Shares": 0,
          "Proxy Type": "",
          "Proxy": "",
          "Difficulty Accepted": 0.0,
          "Difficulty Rejected": 0.0,
          "Difficulty Stale": 0.0,
          "Last Share Difficulty": 0.0,
          "Work Difficulty": 0.0,
          "Has Stratum": true,
          "Stratum Active": false,
          "Stratum URL": "",
          "Stratum Difficulty": 0.0,
          "Has Vmask": true,
          "Has GBT": false,
          "Best Share": 0,
          "Pool Rejected%": 0.0,
          "Pool Stale%": 0.0,
          "Bad Work": 0,
          "Current Block Height": 0,
          "Current Block Version": 0
        }
      ],
      "id": 1
    }
  ],
  "version": [
    {
      "STATUS": [
        {
          "STATUS": "S",
          "When": 81262,
          "Code": 22,
          "Msg": "CG Miner versions",
          "Description": "cgminer 4.11.1"
        }
      ],
      "VERSION": [
        {
          "CGMiner": "4.11.1",
          "API": "3.7",
          "STM8": "20.08.01",
          "PROD": "AvalonMiner 1246-83",
          "MODEL": "1246-83",
          "HWTYPE": "MM3v2_X3",
          "SWTYPE": "MM314",
          "VERSION": "21042001_4ec6bb0_61407fa",
          "LOADER": "d0d779de.00",
          "DNA": "020100003fc244a1",
          "MAC": "b4a2eb3526c3",
          "UPAPI": "2"
        }
      ],
      "id": 1
    }
  ]
}
""".encode()
    with create_server(('localhost', 8008)) as server:
        while True:
            opened_sock, addr = server.accept()
            try:
                opened_sock.recv(1)
                opened_sock.sendall(data)
            except KeyboardInterrupt:
                opened_sock.close()
                raise
            finally:
                opened_sock.close()
