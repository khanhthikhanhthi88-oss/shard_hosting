# shard_hosting

## Project Title
shard_hosting

## Project Description
shard_hosting is a community-funded game-server hosting dApp on Stellar Soroban. Today, indie game communities struggle to keep public servers online: a single host shoulders the full monthly bill or shuts the shard down. shard_hosting flips the model — any game-server operator can register a shard, declare its monthly cost, and let players and community members sponsor the shard with micro-contributions. Hosts report uptime each billing period and claim a payout proportional to how well they actually kept the shard online, so the crowd only pays for real availability. Trust comes from on-chain, transparent accounting rather than from a centralized billing platform.

## Project Vision
Our vision is to make persistent game worlds a public good. By turning server hosting into a transparent, crowdfunded, and performance-incentivized activity, shard_hosting aims to remove the single point of failure (a bored or burnt-out host) that kills most indie multiplayer games. Long term, we want shard_hosting to become the default funding and reputation layer for community-run game servers, where uptime, sponsorship history, and host reliability are all verifiable on-chain.

## Key Features
- **Shard registration**: A host authenticates and opens a shard with a unique id, a game tag, and a declared monthly cost.
- **Open sponsorship**: Any address can sponsor a registered shard by contributing funds; sponsor count and shard balance are tracked on-chain.
- **Uptime-based payouts**: Hosts report an uptime percentage per period; payouts are 100% of monthly cost at >=90% uptime, proportional at 50-89%, and 0% below 50%.
- **Host claims**: Hosts pull their accrued payout out of the shard balance; unclaimed funds remain available for future periods.
- **Transparent read API**: Anyone can query the number of sponsors and the remaining shard balance at any time.
- **Auth-gated actions**: Every state-changing action requires `require_auth` from the relevant party (host or sponsor), preventing unauthorized mutations.

## Contract

- **Network:** Stellar Testnet (Public)
- **Scope:** gaming dApp — see `contracts/shard_hosting/src/lib.rs` for the full shard_hosting business logic.
- **Functions exposed:** see `Key Features` above and the `pub fn` list in `lib.rs`.
- **Contract ID:** `CBTIZIUILJ4LOSZBGDS2LJF474DDNM57AC6HATPQ4Y4JFZVBACSYGIY2`
- **Explorer template:** `https://stellar.expert/explorer/testnet/tx/795a3d5a7d2f84a2073c9d0473a96d4f62a204a8c7d9a81d46c0ceb618102bdd`

## Future Scope
- **Multi-asset sponsorship**: Accept USDC, custom game tokens, and other Stellar assets in addition to native XLM.
- **Refund flow**: Allow sponsors to withdraw their share from a shard whose host has gone silent for N periods.
- **Oracle-driven uptime**: Replace self-reported uptime with attestations from independent uptime oracles or community vote contracts.
- **Host reputation**: Track per-host lifetime uptime, total payouts, and slash score, exposed as a queryable reputation metric.
- **Shard metadata**: Store region, max players, and game version in a richer `ShardInfo` struct, surfaced through additional view functions.
- **Pagination and indexing**: Add iteration helpers over registered shards and sponsor lists for off-chain indexers and dashboards.
- **Testnet deployment + CI**: Wire up a CI pipeline that builds the WASM, deploys to Stellar Testnet, and publishes the contract id and tx hash back into this README.

## Profile

- **Name:** <!-- Fill github name -->
- **Project:** `shard_hosting` (gaming)
- **Built with:** Soroban SDK 25, Rust, Stellar Testnet
