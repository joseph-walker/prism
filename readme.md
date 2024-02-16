# Turbo Prism

## Setup

```sh
git clone https://github.com/joseph-walker/prism
cd prism
make
```

## Development

Depending on if you want to work on the client or the server, you can run a dev build two different ways:

### I want to work on the server

```sh
# Kill and restart when you change code
node simulate-logs | cargo run
```

### I want to work on the client

```sh
# First
make build-debug

# In one terminal
node simulate-logs.js | target/debug/prism

# In another terminal
cd client
npm run dev -- --open
```
