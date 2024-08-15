use market::Economy;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Goods {
    Food,
    // Electronics,
}

fn main() {
    let mut economy = Economy::new();
    economy.add_good(Goods::Food, 20, 250);
    // economy.add_good(Goods::Electronics, 50, 40);

    for day in 1..=75 {
        economy.do_trades();
        let stats = economy.get_latest_stats(&Goods::Food).unwrap();
        let volume = stats.volume();
        let price = stats.price();

        println!(
            "Day {day:2}: {volume} @ {price}/ea; Market value: {}",
            volume * price
        );

        if day % 15 == 0 {
            let qty = 200;
            let price = 35;
            println!("==> Player attempting to buy {qty} Food @ {price}/ea");
            let volume = economy.buy_good(&Goods::Food, price, qty).unwrap();
            println!(
                "==> Player successfully bought {volume} units for {} total",
                volume * price
            );
        } else if day % 20 == 0 {
            let qty = 200;
            let price = 25;
            println!("==> Player attempting to sell {qty} Food @ {price}/ea");
            let volume = economy.sell_good(&Goods::Food, price, qty).unwrap();
            println!(
                "==> Player successfully sold {volume} units for {} total",
                volume * price
            );
        }
    }

    let stats = economy.get_total_stats(&Goods::Food).unwrap();
    let volume = stats.volume();
    let price = stats.price();

    println!("\nTotal Volume: {volume}\nAverage Price: {price}");
}
