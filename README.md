# Overview

![Badges](assets/flat.svg)


## A simple redis message bus subscriber

A simple subscriber service used to transform and publish json data to some persistent store
used in a unikernel framework message bus.

## Usage

A make file is included to simplify tasks

### Testing

Execute the following command

```
make test

# to test a specific test use this command

TEST=<name-of-test> make test-by-name
```

### Building

```
# debug target
make build 

# release
make build-release
```

### Generate cover artifacts

```
make test
make cover

# view html coverage in ./targets/coverage/html
```

### Build the 'message-subscriber' unikernel

A file is included to config envars (config.json)

Ensure you have the ops installed (https://docs.ops.city/ops/getting_started)

```
ops build -c config.json ./target/release/rust-redis-subscriber -i message-subscriber 
ops image list
```

### Execute the unikernel

```
# ensure the redis server (message bus) is running
ops instance list
# if not start it (refer to ops docs to downlad redis package)
ops instance create -i redis -p 6379 redis-server
# start the subscriber
ops instance create -i subscriber message-subscriber
```
