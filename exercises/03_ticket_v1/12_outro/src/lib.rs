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

struct Order {
    product_name: String,
    quantity: i32,
    unit_price: f32
}

impl Order {
    pub fn new(new_product_name: String, new_quantity: i32, new_unit_price: f32) -> Order {
        Order {
            product_name: new_product_name,
            quantity: new_quantity,
            unit_price: new_unit_price
        }
    }

    fn ensure_pn(pn: String) {
        if pn.is_empty() {
            panic!("produce_name cannot be empty")
        }

        if pn.capacity() < 300 {
            panic!("produce_name cannot be empty")
        }
    }

    fn ensure_qty(qty: i32) {
        if qty < 1 {
            panic!("quantity must be greater than 0")
        }
    }

    fn ensure_up(up: f32) {
        if up < 0f32 {
            panic!("unit price must be greater than 0")
        }
    }

    pub fn total(&self) -> f32 {
        self.quantity as f32 * self.unit_price
    }

    pub fn product_name(&self) -> &String {
        self.product_name
    }

    pub fn quantity(&self) -> i32 {
        self.quantity
    }
}
