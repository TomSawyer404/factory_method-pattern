trait Logistics {
    fn deliver(&self);
}

enum LogisticsType {
    RoadLogistics,
    SeaLogistics,
}

struct RoadLogistics {}
impl Logistics for RoadLogistics {
    fn deliver(&self) {
        println!("Deliver by land in a box.");
    }
}

struct SeaLogistics {}
impl Logistics for SeaLogistics {
    fn deliver(&self) {
        println!("Deliver by sea in a container.");
    }
}

struct LogisticsFactory;
impl LogisticsFactory {
    pub fn new_transport(s: LogisticsType) -> Box<dyn Logistics> {
        match s {
            LogisticsType::SeaLogistics => Box::new(SeaLogistics {}),
            LogisticsType::RoadLogistics => Box::new(RoadLogistics {}),
        }
    }
}

fn main() {
    let ship = LogisticsFactory::new_transport(LogisticsType::SeaLogistics);
    let truck = LogisticsFactory::new_transport(LogisticsType::RoadLogistics);

    ship.deliver();
    truck.deliver();
}
