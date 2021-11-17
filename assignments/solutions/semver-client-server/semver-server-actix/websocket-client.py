#!/usr/bin/env python3
"""websocket cmd client for wssrv.py example."""

# installation:
# $ python3 -m venv ve  # run this once
# $ source ve/bin/activate  # run this for every new terminal session
# $ pip3 install aiohttp  # run this once

import argparse
import asyncio
import signal
import sys

import aiohttp


ARGS = argparse.ArgumentParser(
    description="websocket console client for wssrv.py example."
)
ARGS.add_argument(
    "--host", action="store", dest="host", default="127.0.0.1", help="Host name"
)
ARGS.add_argument(
    "--port", action="store", dest="port", default=8080, type=int, help="Port number"
)


async def input_task(loop, reader, ws):
    while True:
        line = await reader.read(100)
        line = line.decode("utf-8").rstrip()
        if not line:
            loop.stop()
        else:
            # await ws.send_str(name + ": " + line)
            await ws.send_str(line)
            # 👆 change to this


async def ws_loop(ws):
    async for msg in ws:
        if msg.type == aiohttp.WSMsgType.TEXT:
            print(f"< {msg.data}")
            if msg.data == "close cmd":
                await ws.close()
                break
        elif msg.type == aiohttp.WSMsgType.PING:
            await ws.pong()
        elif msg.type == aiohttp.WSMsgType.PONG:
            print("Pong received")
        elif msg.type == aiohttp.WSMsgType.CLOSE:
            await ws.close()
        elif msg.type == aiohttp.WSMsgType.ERROR:
            break


async def main(url):
    loop = asyncio.get_running_loop()

    loop.add_signal_handler(signal.SIGINT, loop.stop)
    reader = asyncio.StreamReader()
    protocol = asyncio.StreamReaderProtocol(reader)
    await loop.connect_read_pipe(lambda: protocol, sys.stdin)

    async with aiohttp.ClientSession() as session:
        async with session.ws_connect(url, autoclose=False, autoping=False) as ws:
            await asyncio.gather(input_task(loop, reader, ws), ws_loop(ws))

    print("oh no, here")
    try:
        loop.run_forever()
    finally:
        loop.close()


if __name__ == "__main__":
    args = ARGS.parse_args()
    if ":" in args.host:
        args.host, port = args.host.split(":", 1)
        args.port = int(port)

    url = "http://{}:{}/ws/".format(args.host, args.port)

    # name = input("Please enter your name: ")
    name = "ohai"
    asyncio.run(main(url))
