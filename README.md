# Classic Stats Tracker

Tracker for All Stats Entry runs in Spelunky Classic

## Usage

This tracker is meant to be used as a browser source in OBS to avoid the need to worry about chroma keys and ugly masking. To setup this up select a `Browser` under sources and fill in `http://127.0.0.1:4224` for the URL. Setting a height of `1600` allowed the full window to be displayed for me.

We also support multiple rulesets in the tracker.

Ruleset 1 - Requires getting all enemy deaths and kills
Ruleset 2 - Requires getting a death OR kill for each enemy

The default ruleset currently is 1 however you can choose an explicit ruleset by adding a `ruleset` query parameter. For example to use ruleset 2 use the URL `http://127.0.0.1:4224?ruleset=2`.

![unknown](https://user-images.githubusercontent.com/231118/166628232-703a7bf7-170c-4863-9bbb-2212dd746442.png)![image](https://user-images.githubusercontent.com/231118/166628340-3edaac5a-901e-4a89-8f61-206d0f08bbd7.png)

## Development

### Frontend

#### Installation

```console
pnpm install
```

#### Development

Running the dev build/server

```console
pnpm dev
```

Building a release

```console
pnpm build
```

### Backend

Running the dev build

```console
cargo run
```

Building a release

```console
cargo build --release
```
