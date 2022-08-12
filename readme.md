# forward1

A dead simple TCP MITM proxy for testing.
Doesn't log, doesn't print. Just sit in the middle and exchange data.

## Usage

Configure environment variables or put them in a `.env` file. See `.env.example`.  
Then start your TCP server at `SEND_ADDR`, and run `forward1`.  
Then connect your client to `LISTEN_ADDR`. Only 1 client can be connected.

You can kill `forward1` to simulate a disconnection.
