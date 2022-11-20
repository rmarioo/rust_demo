

fn main() {

}


#[cfg(test)]
mod test {

// Example 1
/*    #[test]
    fn  ownership_test() {

        let  x = "BARCELLONA".to_string();
        let  y = x;

        let res = format!("word {} has lenght {}", x, x.len());

        assert_eq!(res,"word BARCELLONA has lenght 10")
    }
*/


// Example 2
/*
    #[test]
    fn count_words_in_string() {
        let input = String::from("first second");
        let num: usize = words_in(input);

        println!("number of work in {} is {}", input, num);
        assert_eq!(num, 2);
    }

    fn words_in(input: String) -> usize {
        let split = input.split(" ");
        split.count()
    }

    fn words_in_ref(input: &str) -> usize {
        let split = input.split(" ");
        split.count()
    }
*/


    use crate::test::ProductType::{DRINK, FRUIT};

    struct Product {
        name: String,
        productType: ProductType
    }

    #[derive(PartialEq,Clone)]
    enum ProductType {
        FRUIT,
        DRINK
    }

// Example 3
/*
    #[test]
    fn  count_product_by_type() {
        let products = vec![
            Product { name: "apple".to_string(), productType: FRUIT },
            Product { name: "pear".to_string() , productType: FRUIT },
            Product { name: "mojito".to_string(), productType: DRINK }
        ];

        let  fruits = count_fruits(products);
        let  drinks = count_drinks(products);

        assert_eq!(fruits,2);
        assert_eq!(drinks,1);

    }

    fn count_fruits(products: Vec<Product>) -> usize {
        products.iter().filter(|p| p.productType == FRUIT).count()
    }

    fn count_drinks(products: Vec<Product>) -> usize {
        products.iter().filter(|p| p.productType == DRINK).count()
    }
*/

// Example 3. Solution with tuples
/*
      #[test]
      fn  count_product_by_type_tuple() {
          let products = vec![
              Product { name: "apple".to_string(), productType: FRUIT },
              Product { name: "pear".to_string() , productType: FRUIT },
              Product { name: "mojito".to_string(), productType: DRINK }
          ];

          let (fruits ,products) = count_fruits_tuple(products);
          let (drinks,products) = count_drinks_tuple(products);

          assert_eq!(fruits,2);
          assert_eq!(drinks,1);

      }

      fn count_drinks_tuple(products: Vec<Product>) -> (usize, Vec<Product>) {
          (products.iter().filter(|p| p.productType == DRINK).count(),products )
      }
      fn count_fruits_tuple(products: Vec<Product>) -> (usize, Vec<Product>) {
          (products.iter().filter(|p| p.productType == FRUIT).count(),products )
      }
*/

// Example 3. Solution with refercences
/*    #[test]
    fn  count_product_by_type_using_ref() {
        let products = vec![
            Product { name: "apple".to_string(), productType: FRUIT },
            Product { name: "pear".to_string() , productType: FRUIT },
            Product { name: "mojito".to_string(), productType: DRINK }
        ];

        let slice = &products[..];
        let  fruits = count_fruits_ref(slice);
        let  drinks = count_drinks_ref(slice);

        assert_eq!(fruits,2);
        assert_eq!(drinks,1);

    }

    fn count_fruits_ref(products: &[Product]) -> usize {
        products.into_iter().filter(|p| p.productType == FRUIT).count()
    }
    fn count_drinks_ref(products: &[Product]) -> usize {
        products.into_iter().filter(|p| p.productType == DRINK).count()
    }*/
}

