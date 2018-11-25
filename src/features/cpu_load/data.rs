use feature;
use settings;

#[derive(Debug)]
pub struct CpuLoadData {
    pub one: f32,
    pub five: f32,
    pub fifteen: f32,
}

impl feature::Renderable for CpuLoadData {
    fn render(&self, settings: &settings::Settings) -> String {
        settings
            .cpu_load
            .template
            .replace("{CL1}", &format!("{:.2}", self.one))
            .replace("{CL5}", &format!("{:.2}", self.five))
            .replace("{CL15}", &format!("{:.2}", self.fifteen))
    }
}

/* temporarily disabled because missing mock possibilty in tests
#[cfg(test)]
mod tests {
    use super::*;
    use feature::Renderable;

    #[test]
    fn test_display() {
        let data = CpuLoadData {
            one: 0.5,
            five: 1.52,
            fifteen: 2.1234,
            template: String::from("{CL5} {CL1} {CL15} {CL2} {CL1}"),
        };

        assert_eq!(data.render(), "1.52 0.50 2.12 {CL2} 0.50");
    }
}
*/
