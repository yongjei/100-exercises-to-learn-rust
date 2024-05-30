// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 characters.
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
    quantity: u32,
    unit_price: f64,
}

impl Order {
    pub fn new(new_product_name: &String, new_quantity: u32, new_unit_price: f64) -> Order {
        let mut temp_order: Order = Order {
            product_name: new_product_name.into(),
            quantity: new_quantity,
            unit_price: new_unit_price,
        };

        temp_order.product_name(new_product_name);
        temp_order.quantity(new_quantity);
        temp_order.unit_price(new_unit_price);
        temp_order
    }

    pub fn product_name(&mut self, new_product_name: String) {
        if new_product_name.is_empty() || new_product_name.len() > 300 {
            panic!("The product_name can not be empty and it can not be longer than 300 characters.");
        }

        self.product_name = new_product_name;
    }

    pub fn get_product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&mut self, new_quantity: u32) {
        if new_quantity <= 0 {
            panic!("The quantity must be strictly greater than zero.");
        }

        self.quantity = new_quantity;
    }

    pub fn get_quantity(&self) -> u32 {
        self.quantity
    }

    pub fn unit_price(&mut self, new_unit_price: f64) {
        if new_unit_price <= 0.0 {
            panic!("The unit_price is in cents and must be strictly gerater than zero.");
        }

        self.unit_price = new_unit_price;
    }

    pub fn get_unit_price(&self) -> f64 {
        self.unit_price
    }

    pub fn total(&self) -> f64 {
        self.unit_price * self.quantity as f64
    }
}
