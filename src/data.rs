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
            name: "Logitech G203 LIGHTSYNC 8000 DPI Optical Gaming Mouse - Black".to_string(),
            price: 27.99,
            description: "Level up your game with the Logitech G203 LIGHTSYNC optical gaming mouse. Featuring an 8,000dpi gaming-grade sensor and five adjustable sensitivity levels, it delivers fast, precise tracking and versatile performance.".to_string(),
            image: "/blackmousewired.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Logitech M220 Silent 1000 DPI Wireless Optical Mouse - Rose".to_string(),
            price: 24.99,
            description: "Smooth and noiseless, the Logitech M220 Silent 1000 DPI mouse is an excellent pick for your laptop. This wireless mouse comes with 18 months of battery life and 10m wireless range having compatibility with Windows, Mac, and other devices.".to_string(),
            image: "/pinkwiredmouse.jpg".to_string()
        },
        Product {
            id: 6,
            name: "Seafarer's Tug Rope".to_string(),
            price: 14.99,
            description: "".to_string(),
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