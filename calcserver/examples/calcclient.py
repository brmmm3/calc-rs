import sys
import asyncio

import aiohttp

URL = "http://127.0.0.1:4711"


async def add(value: float):
    headers = {"Content-Type": "application/json"}
    async with aiohttp.ClientSession() as session:
        data = {"value": value}
        async with session.post(f"{URL}/add", headers=headers, json=data) as response:
            response = await response.read()
            print(f"Status: {response}")


async def add(value: float):
    headers = {"Content-Type": "application/json"}
    async with aiohttp.ClientSession() as session:
        data = {"value": value}
        async with session.post(f"{URL}/add", headers=headers, json=data) as response:
            response = await response.read()
            print(f"Status: {response}")


async def sub(value: float):
    headers = {"Content-Type": "application/json"}
    async with aiohttp.ClientSession() as session:
        data = {"value": value}
        async with session.post(f"{URL}/sub", headers=headers, json=data) as response:
            response = await response.read()
            print(f"Status: {response}")


async def result():
    headers = {"Content-Type": "application/json"}
    async with aiohttp.ClientSession() as session:
        async with session.get(f"{URL}/result", headers=headers) as response:
            response = await response.read()
            print(f"Status: {response}")


if __name__ == "__main__":
    loop = asyncio.new_event_loop()
    for cmd, func in (("add", add), ("sub", sub), ("result", result)):
        if cmd not in sys.argv:
            continue
        index = sys.argv.index(cmd)
        if cmd == "add" or cmd == "sub":
            loop.run_until_complete(func(float(sys.argv[index + 1])))
        else:
            loop.run_until_complete(func())
        break
    else:
        print(f"Invalid syntax. Usage: {__file__} add|sub|result [value]")
