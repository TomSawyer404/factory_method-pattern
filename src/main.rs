trait Logistics {
    fn deliver(&self);
}

enum LogisticsType {
    RoadLogistics,
    SeaLogistics,
}

struct Transport {
    payload: u32,
    logistics_type: LogisticsType,
}

impl Transport {
    fn new(payload: u32, logistics_type: LogisticsType) -> Transport {
        Transport {
            payload,
            logistics_type,
        }
    }

    fn get_payload(&self) -> u32 {
        self.payload
    }
}

impl Logistics for Transport {
    fn deliver(&self) {
        match self.logistics_type {
            LogisticsType::SeaLogistics => {
                println!("Deliver by sea in a container.");
            }
            LogisticsType::RoadLogistics => {
                println!("Deliver by land in a box.");
            }
        }
    }
}

trait Factory {
    type Object;
    fn create(&mut self, payload: u32, logistics_type: LogisticsType) -> Box<Self::Object> {
        let product = self.create_product(payload, logistics_type);
        self.register_product(&product);
        product
    }

    fn create_product(&self, owner: u32, logistics_type: LogisticsType) -> Box<Self::Object>;
    fn register_product(&mut self, product: &Box<Self::Object>);
}

struct LogisticsFactory {
    payloads: Vec<u32>,
}

impl LogisticsFactory {
    fn new() -> LogisticsFactory {
        LogisticsFactory {
            payloads: Vec::new(),
        }
    }
}

impl Factory for LogisticsFactory {
    type Object = Transport;

    fn create_product(&self, payload: u32, logistics_type: LogisticsType) -> Box<Self::Object> {
        Box::new(Transport::new(payload, logistics_type))
    }

    fn register_product(&mut self, product: &Box<Self::Object>) {
        self.payloads.push(product.get_payload());
    }
}
fn main() {
    let mut factory = LogisticsFactory::new();
    let transport1 = factory.create(32, LogisticsType::SeaLogistics);
    let transport2 = factory.create(13, LogisticsType::RoadLogistics);
    let transport3 = factory.create(22, LogisticsType::RoadLogistics);

    transport1.deliver();
    transport2.deliver();
    transport3.deliver();
}
