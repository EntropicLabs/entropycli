# EntropyCLI - Entropy Beacon Tooling
EntropyCLI is a command line tool for interacting with the Entropy Beacon. Developers can use EntropyCLI to facilitate local development of applications that integrate with the Entropy Beacon.

## Installation
Developers can get started with EntropyCLI by installing it with `cargo`:

```bash
cargo install entropycli
```

## Usage (Local Development)
To get started, EntropyCLI can be used to create a new Entropy Beacon project, which creates a `entropy.json` configuration file in the current directory. To create a new project, run the following command, and follow the prompts:

```bash
entropy beacon init
```

The initialization process will also prompt the developer to deploy a local instance of the Entropy Beacon to the network they have selected. To manually deploy a local instance of the Entropy Beacon, run the following command:

```bash
entropy beacon deploy
```

Once the Entropy Beacon has been deployed, EntropyCLI can be used to interact with the Entropy Beacon and to respond to incoming requests. To start this process, run the following command:

```bash
entropy beacon dev
```

This "dev" mode has three main functions:
1. "Auto-submit Entropy" -- EntropyCLI will automatically submit **random** entropy to the Entropy Beacon as requests come in.
2. "Manual-submit Entropy" -- EntropyCLI will prompt the developer to submit entropy to the Entropy Beacon as requests come in. This is useful for testing specific entropy values, and is not random. In production, this is obviously impossible.
3. "Fetch Active Requests" -- EntropyCLI will output request information to the console as requests come in. This is useful for debugging request specifics.


## Usage (Worker Deployments)
EntropyCLI can also be used to manage Entropy Worker deployments, although this is a **beta** feature. The documentation for this feature has not yet been written.