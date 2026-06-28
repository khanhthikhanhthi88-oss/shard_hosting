#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Map, Symbol};

/// ShardInfo describes a registered game-server shard and the host
/// that operates it. The struct is stored per-shard in contract storage.
#[derive(Clone)]
pub struct ShardInfo {
    pub host: Address,
    pub game: Symbol,
    pub monthly_cost: u64,
    pub reported_periods: u32,
    pub pending_payout: u64,
    pub paid_out: u64,
}

/// ShardHosting is a community-funded game-server hosting contract.
///
/// Hosts register a server shard, players sponsor it with funding,
/// and hosts report uptime each billing period. The host can claim
/// a payout proportional to reported uptime, while unclaimed funds
/// remain in the shard's balance for future periods or sponsor
/// refunds.
#[contract]
pub struct ShardHosting;

#[contractimpl]
impl ShardHosting {
    /// Register a new game-server shard. The host authenticates and
    /// opens a shard identified by `shard_id`, declaring the game
    /// tag and the monthly operating cost (in the smallest unit).
    /// Panics if the shard is already registered or the cost is zero.
    pub fn register_shard(
        env: Env,
        host: Address,
        shard_id: Symbol,
        game: Symbol,
        monthly_cost: u64,
    ) {
        host.require_auth();

        if monthly_cost == 0 {
            panic!("monthly_cost must be greater than zero");
        }

        let mut shards: Map<Symbol, ShardInfo> = env
            .storage()
            .instance()
            .get(&"shards")
            .unwrap_or(Map::new(&env));

        if shards.contains_key(shard_id.clone()) {
            panic!("shard already registered");
        }

        let info = ShardInfo {
            host,
            game,
            monthly_cost,
            reported_periods: 0,
            pending_payout: 0,
            paid_out: 0,
        };

        shards.set(shard_id, info);
        env.storage().instance().set(&"shards", &shards);
    }

    /// Sponsor a shard by contributing `amount` to its funding pool.
    /// The sponsor's authentication is required. The shard's balance
    /// and sponsor count are updated. Panics if the amount is zero
    /// or the shard is not registered.
    pub fn sponsor(env: Env, sponsor: Address, shard_id: Symbol, amount: u64) {
        sponsor.require_auth();

        if amount == 0 {
            panic!("sponsor amount must be greater than zero");
        }

        let shards: Map<Symbol, ShardInfo> = env
            .storage()
            .instance()
            .get(&"shards")
            .unwrap_or(Map::new(&env));
        if !shards.contains_key(shard_id.clone()) {
            panic!("shard not found");
        }

        let mut balances: Map<Symbol, u64> = env
            .storage()
            .instance()
            .get(&"balances")
            .unwrap_or(Map::new(&env));
        let current = balances.get(shard_id.clone()).unwrap_or(0u64);
        balances.set(shard_id.clone(), current + amount);
        env.storage().instance().set(&"balances", &balances);

        let mut counts: Map<Symbol, u32> = env
            .storage()
            .instance()
            .get(&"sponsor_counts")
            .unwrap_or(Map::new(&env));
        let c = counts.get(shard_id.clone()).unwrap_or(0u32);
        counts.set(shard_id.clone(), c + 1);
        env.storage().instance().set(&"sponsor_counts", &counts);
    }

    /// Host reports the uptime percentage (0..=100) for a billing
    /// `period` of a shard. The accrued payout is proportional to
    /// uptime: >=90% grants 100% of `monthly_cost`, 50..=89% grants
    /// a proportional share, and <50% grants nothing. Only the
    /// registered host may report. Accrued payout is added to the
    /// host's pending payout for the shard.
    pub fn report_uptime(
        env: Env,
        host: Address,
        shard_id: Symbol,
        period: u64,
        uptime_pct: u32,
    ) {
        host.require_auth();

        if uptime_pct > 100 {
            panic!("uptime_pct must be <= 100");
        }

        let mut shards: Map<Symbol, ShardInfo> = env
            .storage()
            .instance()
            .get(&"shards")
            .unwrap_or(Map::new(&env));

        let mut info = shards.get(shard_id.clone()).expect("shard not found");

        if info.host != host {
            panic!("only the shard host can report uptime");
        }

        let _ = period;

        let payout: u64 = if uptime_pct >= 90 {
            info.monthly_cost
        } else if uptime_pct >= 50 {
            (info.monthly_cost / 100) * (uptime_pct as u64)
        } else {
            0
        };

        info.pending_payout = info.pending_payout + payout;
        info.reported_periods = info.reported_periods + 1;

        shards.set(shard_id, info);
        env.storage().instance().set(&"shards", &shards);
    }

    /// Host claims all accrued pending payout for a shard. Returns
    /// the amount claimed. Only the registered host of the shard
    /// may claim. The shard's balance is reduced by the claimed
    /// amount (floored at zero), and the host's `paid_out` total
    /// is increased.
    pub fn claim_host_payout(env: Env, host: Address, shard_id: Symbol) -> u64 {
        host.require_auth();

        let mut shards: Map<Symbol, ShardInfo> = env
            .storage()
            .instance()
            .get(&"shards")
            .unwrap_or(Map::new(&env));

        let mut info = shards.get(shard_id.clone()).expect("shard not found");

        if info.host != host {
            panic!("only the shard host can claim");
        }

        let amount = info.pending_payout;
        if amount == 0 {
            return 0;
        }

        let mut balances: Map<Symbol, u64> = env
            .storage()
            .instance()
            .get(&"balances")
            .unwrap_or(Map::new(&env));
        let current = balances.get(shard_id.clone()).unwrap_or(0u64);
        let new_balance = if current >= amount {
            current - amount
        } else {
            0u64
        };
        balances.set(shard_id.clone(), new_balance);
        env.storage().instance().set(&"balances", &balances);

        info.pending_payout = 0;
        info.paid_out = info.paid_out + amount;
        shards.set(shard_id, info);
        env.storage().instance().set(&"shards", &shards);

        amount
    }

    /// Returns the number of sponsors that have funded the shard.
    /// Returns 0 if the shard is unknown.
    pub fn shard_sponsors(env: Env, shard_id: Symbol) -> u32 {
        let counts: Map<Symbol, u32> = env
            .storage()
            .instance()
            .get(&"sponsor_counts")
            .unwrap_or(Map::new(&env));
        counts.get(shard_id).unwrap_or(0u32)
    }

    /// Returns the current remaining balance of the shard's funding
    /// pool. Returns 0 if the shard is unknown or has no funding.
    pub fn shard_balance(env: Env, shard_id: Symbol) -> u64 {
        let balances: Map<Symbol, u64> = env
            .storage()
            .instance()
            .get(&"balances")
            .unwrap_or(Map::new(&env));
        balances.get(shard_id).unwrap_or(0u64)
    }
}
