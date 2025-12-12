use aoc_utils::itertools::Itertools;

#[derive(Clone, Copy)]
struct Item {
    cost: i32,
    dmg: i32,
    arm: i32,
}

#[derive(Clone, Copy)]
struct Fighter {
    hp: i32,
    dmg: i32,
    arm: i32,
}

fn wins(mut player: Fighter, mut boss: Fighter) -> bool {
    loop {
        let pd = (player.dmg - boss.arm).max(1);
        boss.hp -= pd;
        if boss.hp <= 0 {
            return true;
        }

        let bd = (boss.dmg - player.arm).max(1);
        player.hp -= bd;
        if player.hp <= 0 {
            return false;
        }
    }
}

pub fn solve(input: &str) -> (String, String) {
    let mut lines = input.lines();
    let boss = Fighter {
        hp: lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap(),
        dmg: lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap(),
        arm: lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap(),
    };

    let weapons = [
        Item {
            cost: 8,
            dmg: 4,
            arm: 0,
        },
        Item {
            cost: 10,
            dmg: 5,
            arm: 0,
        },
        Item {
            cost: 25,
            dmg: 6,
            arm: 0,
        },
        Item {
            cost: 40,
            dmg: 7,
            arm: 0,
        },
        Item {
            cost: 74,
            dmg: 8,
            arm: 0,
        },
    ];

    let armor = [
        Item {
            cost: 0,
            dmg: 0,
            arm: 0,
        },
        Item {
            cost: 13,
            dmg: 0,
            arm: 1,
        },
        Item {
            cost: 31,
            dmg: 0,
            arm: 2,
        },
        Item {
            cost: 53,
            dmg: 0,
            arm: 3,
        },
        Item {
            cost: 75,
            dmg: 0,
            arm: 4,
        },
        Item {
            cost: 102,
            dmg: 0,
            arm: 5,
        },
    ];

    let rings = [
        Item {
            cost: 0,
            dmg: 0,
            arm: 0,
        },
        Item {
            cost: 0,
            dmg: 0,
            arm: 0,
        },
        Item {
            cost: 25,
            dmg: 1,
            arm: 0,
        },
        Item {
            cost: 50,
            dmg: 2,
            arm: 0,
        },
        Item {
            cost: 100,
            dmg: 3,
            arm: 0,
        },
        Item {
            cost: 20,
            dmg: 0,
            arm: 1,
        },
        Item {
            cost: 40,
            dmg: 0,
            arm: 2,
        },
        Item {
            cost: 80,
            dmg: 0,
            arm: 3,
        },
    ];

    let mut best_win = i32::MAX;
    let mut worst_loss = 0;

    for w in &weapons {
        for a in &armor {
            for rs in rings.iter().combinations(2) {
                let cost = w.cost + a.cost + rs[0].cost + rs[1].cost;
                let dmg = w.dmg + a.dmg + rs[0].dmg + rs[1].dmg;
                let arm = w.arm + a.arm + rs[0].arm + rs[1].arm;

                let player = Fighter { hp: 100, dmg, arm };

                if wins(player, boss) {
                    best_win = best_win.min(cost);
                } else {
                    worst_loss = worst_loss.max(cost);
                }
            }
        }
    }

    (best_win.to_string(), worst_loss.to_string())
}
