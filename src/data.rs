use crate::model::Product;
use crate::configuration::Settings;
 
pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Sony Noise Cancelling Headphones".to_string(),
            price: 299.99,
            description: "Experience unparalleled sound quality with Sony's premium noise-canceling headphones. Designed with advanced audio technology and all-day comfort, these headphones are the ultimate companion for travel, work, and leisure..".to_string(),
            image: "/sony.png".to_string()
        },
        Product {
            id: 2,
            name: "Samsung Galaxy Tab S9".to_string(),
            price: 399.99,
            description: "Discover unmatched versatility with the Samsung Galaxy Tab S9. Featuring a stunning AMOLED display and a lightning-fast processor, it's your ideal companion for productivity, entertainment, and more.".to_string(),
            image: "/galaxy.png".to_string()
        },
        Product {
            id: 3,
            name: "Dyson V15 Detect Vacuum Cleaner".to_string(),
            price: 458.99,
            description: "Keep your home spotless with the Dyson V15 Detect. Featuring laser dust detection and powerful suction, this vacuum takes the hassle out of cleaning, leaving your space sparkling.".to_string(),
            image: "/vaccum_cleaner.png".to_string()
        },
        Product {
            id: 4,
            name: "Apple MacBook Pro 14".to_string(),
            price: 3099.99,
            description: "Unleash your creativity with the Apple MacBook Pro 14\". Packed with the M2 Pro chip and a stunning Liquid Retina XDR display, this laptop is designed for professionals and enthusiasts alike.".to_string(),
            image: "/macbook_14.png".to_string()
        }
    ]
}
