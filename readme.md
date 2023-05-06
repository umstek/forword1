# forward1

[![build](https://github.com/umstek/forward1/actions/workflows/build.yml/badge.svg)](https://github.com/umstek/forward1/actions/workflows/build.yml)

A dead simple TCP MITM proxy for testing.  
Exchange data, and print a comprehensive log of everything exchanged.

## Usage

Configure environment variables or put them in a `.env` file. See `.env.example`.  
Then start your TCP server at `SEND_ADDR`, and run `forward1`.  
Then connect your client(s) to `LISTEN_ADDR`.

When your client connects, `forward1` will connect to `SEND_ADDR` and exchange data. It will disconnect when the client does.

You can kill `forward1` to simulate a disconnection.

## Note

Some code written with the help of Bing AI/ChatGPT etc.
