use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Turtle Beach VelocityOne Racing Wheel & Pedal System".to_string(),
            price: 99.99,
            description: "Race wheel & pedal system is a high-performance racing peripheral designed for use with Xbox and PC gaming platforms. With race-inspired controls and ultra-realistic K: Drive force feedback motor, it delivers an immersive and realistic racing experience.".to_string(),
            image: "/steeringwheel.png".to_string()
        },
        Product {
            id: 2,
            name: "Logitech G305 12000 DPI Wireless Optical Gaming Mouse - White".to_string(),
            price: 39.99,
            description: "Experience next-level gaming with the Logitech G305 wireless optical mouse. It features a HERO sensor with up to 12,000dpi for ultra-precise tracking and LIGHTSPEED wireless technology that delivers a 1ms response time.".to_string(),
            image: "/whitemouse.png".to_string()
        },
        Product {
            id: 3,
            name: "Logitech MX Master 2S Bluetooth Laser Mouse - Graphite".to_string(),
            price: 99.99,
            description: "Enjoy ultimate speed and precision with this Logitech MX Master 2S Bluetooth laser mouse. It features Darkfield tracking technology that offers high-precision cursor control on almost any surface.".to_string(),
            image: "/blackmouse.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Ocean Explorer's Puzzle Ball".to_string(),
            price: 11.99,
            description: "Challenge your pet's problem-solving skills with the Ocean Explorer's Puzzle Ball. This interactive toy features hidden compartments and treats, providing mental stimulation and entertainment.".to_string(),
            image: "/ocean.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Pirate Parrot Teaser Wand".to_string(),
            price: 8.99,
            description: "Engage your cat in a playful pursuit with the Pirate Parrot Teaser Wand. The colorful feathers and jingling bells mimic the mischievous charm of a pirate's parrot.".to_string(),
            image: "/pirate.jpg".to_string()
        },
        Product {
            id: 6,
            name: "Seafarer's Tug Rope".to_string(),
            price: 14.99,
            description: "Tug-of-war meets nautical adventure with the Seafarer's Tug Rope. Made from marine-grade rope, it's perfect for interactive play and promoting dental health in dogs.".to_string(),
            image: "/tug.jpg".to_string()
        },
        Product {
            id: 7,
            name: "Seashell Snuggle Bed".to_string(),
            price: 19.99,
            description: "Give your furry friend a cozy spot to curl up with the Seashell Snuggle Bed. Shaped like a seashell, this plush bed provides comfort and relaxation for cats and small dogs.".to_string(),
            image: "/bed.jpg".to_string()
        },
        Product {
            id: 8,
            name: "Nautical Knot Ball".to_string(),
            price: 7.99,
            description: "Unleash your dog's inner sailor with the Nautical Knot Ball. Made from sturdy ropes, it's perfect for fetching, tugging, and satisfying their chewing needs.".to_string(),
            image: "/knot.jpg".to_string()
        },
        Product {
            id: 9,
            name: "Contoso Claw's Crabby Cat Toy".to_string(),
            price: 3.99,
            description: "Watch your cat go crazy for Contoso Claw's Crabby Cat Toy. This crinkly and catnip-filled toy will awaken their hunting instincts and provide endless entertainment.".to_string(),
            image: "/crabby.jpg".to_string()
        },
        Product {
            id: 10,
            name: "Ahoy Doggy Life Jacket".to_string(),
            price: 5.99,
            description: "Ensure your furry friend stays safe during water adventures with the Ahoy Doggy Life Jacket. Designed for dogs, this flotation device offers buoyancy and visibility in style.".to_string(),
            image: "/lifejacket.jpg".to_string()
        }
    ]
}