mod pizza_order {
    struct Pizza {
        dough: String,
        cheese: String,
        topping: String,
    }

    impl Pizza {
        fn lunch(topping: &str) -> Pizza {
            Pizza {
                dough: String::from("Regular dough"),
                cheese: String::from("Mozarella"),
                topping: String::from(topping),
            }
        }
    }

    pub mod help_customer {
        fn seat_at_table() {
            println!("Customer seated at table");
        }

        pub fn take_order() {
            seat_at_table();

            // "super" - access Pizza in parent scope
            let customer_pizza: super::Pizza = super::Pizza::lunch("veggies");

            serve_customer(customer_pizza);
        }

        fn serve_customer(cust_pizza: super::Pizza) {
            println!("The customer is served a regular pizza with {}", cust_pizza.topping);
        }
    }
}

pub fn order_food() {
    //     directory     module        module       function
    crate::restaurant::pizza_order::help_customer::take_order();
}
