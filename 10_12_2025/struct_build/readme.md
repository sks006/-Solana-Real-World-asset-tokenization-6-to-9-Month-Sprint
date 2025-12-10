## self vs Self vs &mut self

###   1. Self (capital S)
#### Type alias for the implementing struct/enum
#### Used in return types or generic contexts

```
impl Loan {
    fn new(amount: u32, share: u32) -> Self {  // Returns Loan type
        Self { amount, share }  // Same as Loan { amount, share }
    }
}

```
###   2. self (lowercase)
####  Takes ownership of the instance
####  Consumes the instance (moves it)

    ```
    impl Loan {
    fn consume(self) {
        // Can't use 'l' after this call
    }
}
    ```

### 3. &self
#### Borrows the instance immutably (read-only)
```
impl Loan {
    fn get_amount(&self) -> u32 {
        self.amount  // Can read but not modify
    }
}

```
###   4. &mut self
####  Borrows the instance mutably (read-write)
```
impl Loan {
    fn inc_amount(&mut self, intre: u32) {
        self.amount *= intre;  // Can modify
    }
}

```
### Real code 

```
struct Loan {
    amount: u32,
    share: u32,
}

impl Loan {
    // Using Self in return type
    fn new(amount: u32, share: u32) -> Self {
        Self { amount, share }  // Self refers to Loan
    }
    
    // Using &mut self to modify
    fn inc_amount(&mut self, intre: u32) {
        self.amount *= intre;
    }
    
    // Using &self to read
    fn get_amount(&self) -> u32 {
        self.amount
    }
    
    // Using self to consume/own
    fn into_amount(self) -> u32 {
        self.amount  // Takes ownership, can't use instance after
    }
}

fn main() {
    let mut l = Loan::new(2000, 20);
    
    // Modify with &mut self
    l.inc_amount(6);
    
    // Read with &self
    println!("Amount: {}", l.get_amount());
    
    // Consume with self
    let amount = l.into_amount();
    println!("Consumed amount: {}", amount);
    
    // l is now moved, can't use it anymore!
}

```
### When to use which:

```
Parameter           	Use when    	              Example
Self	        Referring to the type itself	      Constructor returns Self

self	        You want to consume/take ownership	  fn into_parts(self)

&self           You need read-only access	          fn get_value(&self)

&mut self	    You need to modify the instance	      fn update(&mut self, value)

```