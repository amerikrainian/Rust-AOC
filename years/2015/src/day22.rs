use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct State {
    player_hp: i32,
    player_mana: i32,
    boss_hp: i32,
    shield: i32,
    poison: i32,
    recharge: i32,
    player_turn: bool,
}

#[derive(Clone, Eq, PartialEq)]
struct Node {
    cost: i32,
    state: State,
}

// Min-heap by mana spent
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    const ALL: [Spell; 5] = [
        Spell::MagicMissile,
        Spell::Drain,
        Spell::Shield,
        Spell::Poison,
        Spell::Recharge,
    ];

    fn cost(self) -> i32 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }

    fn can_cast(self, s: &State) -> bool {
        match self {
            Spell::Shield => s.shield == 0,
            Spell::Poison => s.poison == 0,
            Spell::Recharge => s.recharge == 0,
            _ => true,
        }
    }

    fn cast(self, mut s: State) -> State {
        s.player_mana -= self.cost();
        s.player_turn = false;

        match self {
            Spell::MagicMissile => {
                s.boss_hp -= 4;
            }
            Spell::Drain => {
                s.boss_hp -= 2;
                s.player_hp += 2;
            }
            Spell::Shield => {
                s.shield = 6;
            }
            Spell::Poison => {
                s.poison = 6;
            }
            Spell::Recharge => {
                s.recharge = 5;
            }
        }

        s
    }
}

fn apply_effects(mut s: State) -> State {
    if s.poison > 0 {
        s.boss_hp -= 3;
        s.poison -= 1;
    }
    if s.recharge > 0 {
        s.player_mana += 101;
        s.recharge -= 1;
    }
    if s.shield > 0 {
        s.shield -= 1;
    }
    s
}

fn solve_mode(boss_hp: i32, boss_dmg: i32, hard: bool) -> i32 {
    let start = State {
        player_hp: 50,
        player_mana: 500,
        boss_hp,
        shield: 0,
        poison: 0,
        recharge: 0,
        player_turn: true,
    };

    let mut pq = BinaryHeap::new();
    pq.push(Node {
        cost: 0,
        state: start,
    });

    let mut best: HashMap<State, i32> = HashMap::new();

    while let Some(Node { cost, mut state }) = pq.pop() {
        if best.get(&state).map_or(false, |&c| cost > c) {
            continue;
        }

        if hard && state.player_turn {
            state.player_hp -= 1;
            if state.player_hp <= 0 {
                continue;
            }
        }

        state = apply_effects(state);

        // Check win after effects
        if state.boss_hp <= 0 {
            return cost;
        }

        if state.player_turn {
            let mut cast_any = false;

            for spell in Spell::ALL {
                if state.player_mana < spell.cost() {
                    continue;
                }
                if !spell.can_cast(&state) {
                    continue;
                }

                cast_any = true;
                let next = spell.cast(state);
                let new_cost = cost + spell.cost();

                if best.get(&next).map_or(true, |&c| new_cost < c) {
                    best.insert(next, new_cost);
                    pq.push(Node {
                        cost: new_cost,
                        state: next,
                    });
                }
            }

            // No legal spell, lose
            if !cast_any {
                continue;
            }
        } else {
            // Boss turn
            let armor = if state.shield > 0 { 7 } else { 0 };
            let dmg = (boss_dmg - armor).max(1);

            state.player_hp -= dmg;
            if state.player_hp > 0 {
                state.player_turn = true;
                if best.get(&state).map_or(true, |&c| cost < c) {
                    best.insert(state, cost);
                    pq.push(Node { cost, state });
                }
            }
        }
    }

    unreachable!()
}

pub fn solve(input: &str) -> (String, String) {
    let mut lines = input.lines();
    let boss_hp: i32 = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let boss_dmg: i32 = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();

    let part1 = solve_mode(boss_hp, boss_dmg, false);
    let part2 = solve_mode(boss_hp, boss_dmg, true);

    (part1.to_string(), part2.to_string())
}
