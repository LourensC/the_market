pub mod market {
    #[derive(PartialEq)] 
    #[derive(Debug)] 
    pub struct MarketError {
        error:String,
    }

    //    impl MarketError {
    //        fn new(error: String) -> Self { Self { error } }
    //    }

    #[derive(PartialEq)] 
    #[derive(Clone)] 
    #[derive(Debug)] 
    pub struct SellOrder {
        pub account_id:String,
        pub ask:i32,
        pub amount:i32,
    }

    impl SellOrder {
        pub fn new(account_id: String, ask: i32, amount: i32) -> Self { 
            Self { account_id, ask, amount} 
        }
    }

    #[derive(PartialEq)] 
    #[derive(Clone)] 
    #[derive(Debug)] 
    pub struct BuyOrder {
        pub account_id:String,
        pub bid:i32,
        pub amount:i32,
    }

    impl BuyOrder {
        pub fn new( account_id: String, bid: i32, amount: i32) -> Self { 
            Self { account_id, bid , amount} 
        }
    }

    pub struct Market {
        buy_orders: Vec<BuyOrder>,
        sell_orders: Vec<SellOrder>,
    }

    impl Market {
        pub fn new()->Market{
            Market {
                buy_orders: Vec::new(),
                sell_orders: Vec::new(),
            }
        }
        pub fn place_sell_order(&mut self,sell_order:SellOrder) {
            self.sell_orders.push(sell_order);
        }
        pub fn place_buy_order(&mut self,buy_order: BuyOrder) {
            self.buy_orders.push(buy_order);
        }

        fn sort_orders (&mut self) {
            let _ = &self.buy_orders.sort_by(|a,b|b.bid.cmp(&a.bid));
            let _ = &self.sell_orders.sort_by(|a,b|a.ask.cmp(&b.ask));
        } 

        pub fn get_order_book(&mut self)->
            Result<(&Vec<BuyOrder>,&Vec<SellOrder>), MarketError> {
                self.sort_orders();
                Ok( (&self.buy_orders, &self.sell_orders) )
            }


        pub fn resolve_orders(&mut self) -> Result<Vec<Trade>,MarketError>{
            self.sort_orders();

            let mut trades:Vec<Trade> = Vec::new();

            println!("{:?}",self.buy_orders[0].bid);
            println!("{:?}",self.sell_orders[0].ask );

            if self.sell_orders[0].ask <= self.buy_orders[0].bid {
                trades.push(
                    Trade{
                        buyer: Transaction {
                            debit: Debit {
                                account_id: self.buy_orders[0].account_id.clone(),
                                amount: self.buy_orders[0].amount, },
                                credit: Credit {
                                    account_id: self.buy_orders[0].account_id.clone(),
                                    amount: self.buy_orders[0].bid, } 
                        },
                        seller: Transaction {
                            debit: Debit {
                                account_id: self.sell_orders[0].account_id.clone(),
                                amount: self.sell_orders[0].ask, },
                                credit: Credit {
                                    account_id: self.sell_orders[0].account_id.clone(),
                                    amount: self.sell_orders[0].amount, } 
                        },
                    });
                
                if self.buy_orders[0].amount > self.sell_orders[0].amount {
                    self.buy_orders[0].amount=
                        self.buy_orders[0].amount
                        - self.sell_orders[0].amount;
                    self.sell_orders[0].amount=0;
                } else if self.buy_orders[0].amount < self.sell_orders[0].amount {
                    self.sell_orders[0].amount= self.sell_orders[0].amount
                        - self.buy_orders[0].amount;
                    self.buy_orders[0].amount=0;
                } else {
                    self.buy_orders[0].amount=0;
                    self.sell_orders[0].amount=0;
                }

                self.buy_orders= 
                    self.buy_orders.iter()
                    .filter(|bo| bo.amount>0)
                    .map(|bo| bo.clone())
                    .collect();

                self.sell_orders= 
                    self.sell_orders.iter()
                    .filter(|so| so.amount>0)
                    .map(|so| so.clone())
                    .collect();
            }
            Ok(trades)
        }
    }

    pub struct Debit {
        pub account_id: String,
        pub amount: i32,
    }

    pub struct Credit {
        pub account_id: String,
        pub amount: i32,
    }

    pub struct Transaction {
        pub debit: Debit,
        pub credit: Credit,
    }

    pub struct Trade{
        pub buyer: Transaction,
        pub seller: Transaction,
    }
}

#[cfg(test)]
mod market_behaviour{
    use crate::market;
    #[test]
    fn update_partially_fulfilled_orders_when_sell_amount_more_than_buy_amount(){
        let mut testing_market = market::Market::new();
        testing_market.place_buy_order(
            market::BuyOrder{ account_id: "buyer".to_string(), bid: 100, amount: 2});
        testing_market.place_sell_order(
            market::SellOrder{ account_id: "seller".to_string(), ask: 100, amount: 1});

        let trades:Vec<market::Trade> = 
            testing_market
            .resolve_orders()
            .expect("Where are the trades");

        assert_eq!(trades.len(),1);

        let (buy_orders ,sell_orders)  = 
            testing_market.get_order_book().expect("WTF? Where is my order book?");

        assert_eq!(buy_orders.len(), 1);
        assert_eq!(sell_orders.len() ,0);

        assert_eq!(buy_orders[0].amount,1); 
     }

    #[test]
    fn update_partially_fulfilled_orders_when_sell_amount_less_than_buy_amount(){
        let mut testing_market = market::Market::new();
        testing_market.place_buy_order(
            market::BuyOrder{ account_id: "buyer".to_string(), bid: 100, amount: 1});
        testing_market.place_sell_order(
            market::SellOrder{ account_id: "seller".to_string(), ask: 100, amount: 3});

        let trades:Vec<market::Trade> = 
            testing_market
            .resolve_orders()
            .expect("Where are the trades");

        assert_eq!(trades.len(),1);

        let (buy_orders ,sell_orders)  = 
            testing_market.get_order_book().expect("WTF? Where is my order book?");

        assert_eq!(buy_orders.len(), 0);
        assert_eq!(sell_orders.len() ,1);

        assert_eq!(sell_orders[0].amount,2); 
     }

    #[test]
    fn resolve_orders_should_return_trades_and_update_the_order_book() {
        let mut testing_market = market::Market::new();
        testing_market.place_buy_order(
            market::BuyOrder{ account_id: "buyerC".to_string(), bid: 98, amount: 1});
        testing_market.place_buy_order(
            market::BuyOrder{ account_id: "buyerB".to_string(), bid: 95, amount: 1});
        testing_market.place_buy_order(
            market::BuyOrder{ account_id: "buyerA".to_string(), bid: 99, amount: 1});
        testing_market.place_buy_order(
            market::BuyOrder{ account_id: "buyer".to_string(), bid: 100, amount: 1});
        testing_market.place_sell_order(
            market::SellOrder{ account_id: "sellerB".to_string(), ask: 110, amount: 1});
        testing_market.place_sell_order(
            market::SellOrder{ account_id: "sellerA".to_string(), ask: 101, amount: 1});
        testing_market.place_sell_order(
            market::SellOrder{ account_id: "seller".to_string(), ask: 100, amount: 1});
        testing_market.place_sell_order(
            market::SellOrder{ account_id: "sellerC".to_string(), ask: 109, amount: 1});

        let trades:Vec<market::Trade> = 
            testing_market
            .resolve_orders()
            .expect("Where are the trades");

        assert_eq!(trades.len(),1);

        assert_eq!(trades[0].buyer.debit.account_id,"buyer".to_string());
        assert_eq!(trades[0].buyer.debit.amount,1);

        assert_eq!(trades[0].buyer.credit.account_id,"buyer".to_string());
        assert_eq!(trades[0].buyer.credit.amount,100);


        assert_eq!(trades[0].seller.debit.account_id,"seller".to_string());
        assert_eq!(trades[0].seller.debit.amount,100);

        assert_eq!(trades[0].seller.credit.account_id,"seller".to_string());
        assert_eq!(trades[0].seller.credit.amount,1);

        let (buy_orders ,sell_orders)  = 
            testing_market.get_order_book().expect("WTF? Where is my order book?");

        assert_eq!(buy_orders.len(), 3);
        assert_eq!(sell_orders.len() ,3);

        assert_eq!(buy_orders[0].bid,99); 
        assert_eq!(sell_orders[0].ask,101); 
    }

    #[test]
    fn orders_are_sorted_with_the_closest_matches_at_the_top (){
        let mut testing_market = market::Market::new();
        testing_market.place_buy_order(
            market::BuyOrder{ account_id: "1".to_string(), bid: 98, amount: 1});
        testing_market.place_buy_order(
            market::BuyOrder{ account_id: "1".to_string(), bid: 95, amount: 1});
        testing_market.place_buy_order(
            market::BuyOrder{ account_id: "1".to_string(), bid: 97, amount: 1});
        testing_market.place_buy_order(
            market::BuyOrder{ account_id: "1".to_string(), bid: 99, amount: 1});
        testing_market.place_sell_order(
            market::SellOrder{ account_id: "2".to_string(), ask: 104, amount: 1});
        testing_market.place_sell_order(
            market::SellOrder{ account_id: "2".to_string(), ask: 101, amount: 1});
        testing_market.place_sell_order(
            market::SellOrder{ account_id: "2".to_string(), ask: 100, amount: 1});
        testing_market.place_sell_order(
            market::SellOrder{ account_id: "2".to_string(), ask: 109, amount: 1});

        let (buy_orders ,sell_orders)  = 
            testing_market.get_order_book().expect("WTF? Where is my order book?");

        let buy_pairs_itr = buy_orders.windows(2);
        assert!(buy_pairs_itr.len() > 2 );
        for pair in buy_pairs_itr {
            assert!(pair[0].bid > pair[1].bid)
        }

        let sell_pairs_itr = sell_orders.windows(2);
        assert!(sell_pairs_itr.len() > 2 );
        for pair in sell_pairs_itr {
            assert!(pair[0].ask < pair[1].ask)
        }
    }

    #[test]
    fn added_orders_should_show_in_the_order_book (){
        let mut testing_market = market::Market::new();

        testing_market.place_buy_order(
            market::BuyOrder{
                account_id: "1".to_string(), 
                bid: 100 ,
                amount: 1 ,
            });

        testing_market.place_sell_order(
            market::SellOrder{
                account_id: "2".to_string(), 
                ask: 1000,
                amount: 1 ,
            });

        let (buy_orders ,sell_orders)  = 
            testing_market.get_order_book().expect("WTF? Where is my order book?");

        println!("{:?}",buy_orders);
        println!("{:?}",sell_orders);

        assert_eq!(buy_orders.len(), 1);
        assert_eq!(sell_orders.len() ,1);
    }

    #[test] 
    fn a_new_market_should_return_an_empty_order_book() {
        let mut testing_market = market::Market::new();

        let (buy_orders ,sell_orders)  = 
            testing_market.get_order_book().expect("WTF? Where is my order book?");

        assert_eq!(buy_orders.len(), 0);
        assert_eq!(sell_orders.len() ,0);
    }
}
