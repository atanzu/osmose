import os
import time


def test_simple_client_server(systest):
    osmose_server = None
    test_server = None

    try:
        osmose_server = systest.run_binary(
            "osmose-server",
            "--rules",
            os.path.normpath(os.path.abspath(__file__) + "/../rules.json"),
        )

        time.sleep(1)

        test_server_addr = "127.0.0.1:7005"

        test_server = systest.run_binary(
            "scs_server",
            test_server_addr,
        )

        time.sleep(1)

        test_client = systest.run_binary(
            "scs_client",
            test_server_addr,
            "127.0.0.1:7010",
        )
        assert test_client.wait(10) == 0

        test_client = systest.run_binary(
            "scs_client",
            test_server_addr,
            "127.0.0.1:7011",
        )
        assert test_client.wait(10) == 1

    finally:
        if test_server:
            test_server.terminate()
        if osmose_server:
            osmose_server.terminate()
