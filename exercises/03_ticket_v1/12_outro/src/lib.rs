// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: i32,
    unit_price: i32
}

impl Order {
    pub fn new(new_product_name: String, new_quantity: i32, new_unit_price: i32) -> Order {
        Order::ensure_pn(&new_product_name);
        Order::ensure_qty(new_quantity);
        Order::ensure_up(new_unit_price);

        Order {
            product_name: new_product_name,
            quantity: new_quantity,
            unit_price: new_unit_price
        }
    }

    fn ensure_pn(pn: &String) {
        if pn.is_empty() {
            panic!("produce_name cannot be empty")
        }

        if pn.capacity() > 300 {
            panic!("produce_name cannot be longer than 300 characters")
        }
    }

    fn ensure_qty(qty: i32) {
        if qty < 1 {
            panic!("quantity must be greater than 0")
        }
    }

    fn ensure_up(up: i32) {
        if up < 1 {
            panic!("unit price must be greater than 0")
        }
    }

    pub fn total(&self) -> i32 {
        self.quantity * self.unit_price
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn set_product_name(&mut self, new_product_name: String) {
        Order::ensure_pn(&new_product_name);
        self.product_name = new_product_name;
    }

    pub fn quantity(&self) -> &i32 {
        &self.quantity
    }

    pub fn set_quantity(&mut self, new_quantity: i32) {
        Order::ensure_qty(new_quantity);
        self.quantity = new_quantity;
    }

    pub fn unit_price(&self) -> &i32 {
        &self.unit_price
    }

    pub fn set_unit_price(&mut self, new_unit_price: i32) {
            Order::ensure_up(new_unit_price);
            self.unit_price = new_unit_price;
        }
}
