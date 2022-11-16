
struct Country { name: String }

struct City { name: String }

struct Hotel { name: String }

struct Price { value: f32 }

#[derive(Debug)]
enum MyError {
    MissingCity,
    MissingCountry,
    MissingHotel,
    MissingPrice,
}

fn  find_country(country: &str) -> Result<Country, MyError> {
    return if country == "italy" { Ok(Country{ name: "italy".to_string() }) } else { Err(MyError::MissingCountry)  }
}

fn  find_city(city: &str,country: &Country) -> Result<City, MyError> {
    return if city == "milan" && country.name =="italy"  { Ok(City{ name: "milan".to_string() }) } else { Err(MyError::MissingCity)  }
}

fn  find_hotel(hotel: &str,city: &City) -> Result<Hotel, MyError> {
    return if hotel == "gallia" && city.name == "milan" { Ok(Hotel{ name: "gallia".to_string() }) } else { Err(MyError::MissingHotel)  }
}

fn  find_expenses(hotel: Hotel, city: &City,country: &Country) -> Result<Price, MyError> {
    return if hotel.name == "gallia" && city.name == "milan" && country.name =="italy" { Ok(Price{ value: 20.0 }) }  else { Err(MyError::MissingPrice) }
}

fn find_price_for(hotel_name: &str, city_name: &str, country_name: &str) -> Result<Price, MyError> {

    let country= find_country(country_name)?;
    let city   = find_city(city_name, &country)?;
    let hotel = find_hotel(hotel_name, &city)?;
    let price = find_expenses(hotel, &city, &country)?;

    return Ok(price);
}

/*
   NOTE: this will not compile at last line country is not visible:

   find_country(country_name)
    .and_then(|country| find_city(city_name, &country))
    .and_then(|city|    find_hotel(hotel_name, &city))
    .and_then(|hotel|   find_expenses(hotel, &city, &country))
 */

/*
  NOTE: something like this can work but is difficult to read

 country = find_country("italy")
 if (country != null ) {
         city = find_city("milan",country)
         if (city != null ) {
              hotel = find_hotel("gallia",city)
                  if (hotel != null ) {
                       price =  find_expenses(hotel,city,country) {
                            if (price != null ) {
                                 return price
                            }
                       }
                  }
         }
        else {
           "error retrieving city"
        }
 }
 else {
   "error retrieving country"
 }

*/


#[cfg(test)]
mod test {
    use super::*;
   // use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn find_price_for_existing_hotel_city_country() {
        let result = find_price_for("gallia", "milan", "italy");

        let price = result.expect("error");
        assert_eq!(price.value, 20.0);
    }

    #[test]
    fn find_price_for_not_existing_hotel() {
        let result = find_price_for("not_existing_hotel", "milan", "italy");

        let  outcome= match result {
            Err(myerr) => match myerr {
                MyError::MissingCity    => { "MissingCity"}
                MyError::MissingCountry => {"MissingCountry"}
                MyError::MissingHotel   => {"MissingHotel"}
                MyError::MissingPrice   => {"MissingPrice"}
            },
            Ok(_p) => "ok"
        };
        assert_eq!(outcome, "MissingHotel");
    }
}


