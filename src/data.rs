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
            image: "/whitemouse.jpg".to_string()
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
            name: "Rapoo MT760 Multi-Device Wireless Mouse".to_string(),
            price: 14.99,
            description: "The Rapoo MT760 Multi-Device Wireless Mouse features Bluetooth 5.0/3.0 and 2.4GHz tri-mode connectivity, enabling seamless switching between multiple devices.".to_string(),
            image: "/ergonomicblackmouse.jpg".to_string()
        },
        Product {
            id: 7,
            name: "Mesh Loop for Apple Watch Band".to_string(),
            price: 9.99,
            description: "Premium stainless steel mesh loop band compatible with Apple Watch Ultra3, SE3, Series 11–1 in sizes 38mm–49mm. Features one-piece design, breathable sweat-proof mesh, and a powerful magnetic clasp for secure fit.".to_string(),
            image: "/meshwatchband.jpeg".to_string()
        },
        Product {
            id: 8,
            name: "Leather Magnetic Link Replacement Band Strap for Apple Watch".to_string(),
            price: 12.97,
            description: "Leather Magnetic Link Replacement Band Strap for Apple Watch iWatch Series 1 to 10 SE Ultra, 42mm(Series 123) 44mm 45mm 46mm 49mm, Purple".to_string(),
            image: "/leatherwatchband.jpeg".to_string()
        },
        Product {
            id: 9,
            name: "Instant Pot Duo V5 7-in-1 Pressure Cooker - 6QT".to_string(),
            price: 89.99,
            description: "Cook delicious dishes for the whole family with the Instant Pot Duo V5 pressure cooker. This seven-in-one cooker is quick and easy to use, with access to functions like rice cooking, slow cooking, yogurt making, and steaming, plus 13 customizable options.".to_string(),
            image: "/pot.jpeg".to_string()
        },
        Product {
            id: 10,
            name: "Philips 2000 Series Air Fryer - 4.2L (4.4QT) - Black".to_string(),
            price: 69.99,
            description: "Effortlessly cook delicious and healthy recipes with the Philips 2000 series air fryer. This 4.2L appliance features 13 cooking functions like air fry, bake, grill, roast, defrost, reheat, and more to let you prepare a variety of dishes.".to_string(),
            image: "/fryer.jpg".to_string()
        }
    ]
}