use std::collections::HashMap;
use std::fs;

fn pay_debts<'a>(
    from: &'a str,
    balances: &mut HashMap<&'a str, i64>,
    debts: &mut Vec<(&'a str, &'a str, i64)>,
) {
    for i in 0..debts.len() {
        let d = debts[i];
        if d.0 == from && d.2 > 0 {
            let from_balance = balances[d.0];
            if from_balance > 0 {
                let to_balance = balances[d.1];
                let to_transfer = from_balance.min(d.2);
                balances.insert(d.0, from_balance - to_transfer);
                balances.insert(d.1, to_balance + to_transfer);
                debts[i].2 -= to_transfer;
                pay_debts(d.1, balances, debts);
                if from_balance == to_transfer {
                    // no more money left to transfer
                    break;
                }
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let (initial_balances, transactions) = input.split_once("\n\n").unwrap();

    for part in [1, 2, 3] {
        let mut balances = HashMap::new();
        for h in initial_balances.lines() {
            let (name, balance) = h.split_once(" HAS ").unwrap();
            balances.insert(name, balance.parse::<i64>().unwrap());
        }

        let mut debts = Vec::new();
        for t in transactions.lines() {
            let parts = t.split_whitespace().collect::<Vec<_>>();
            let from = parts[1];
            let to = parts[3];
            let amount = parts[5].parse::<i64>().unwrap();
            let from_balance = balances[from];
            let to_balance = balances[to];
            if part == 1 || from_balance >= amount {
                balances.insert(from, from_balance - amount);
                balances.insert(to, to_balance + amount);
            } else {
                if part == 3 {
                    let new_debt = amount - from_balance;
                    debts.push((from, to, new_debt));
                }
                balances.insert(from, 0);
                balances.insert(to, to_balance + from_balance);
            }

            pay_debts(to, &mut balances, &mut debts);
            debts.retain(|d| d.2 > 0);
        }

        let mut balances_list = balances.values().copied().collect::<Vec<_>>();
        balances_list.sort_unstable();
        let total = balances_list[balances.len() - 3..].iter().sum::<i64>();
        println!("{}", total);
    }
}
