use std::fmt;

pub struct SafetyFactor {
    pub safety_factor: f64,
    pub blocks: Vec<crate::block::Block>,
}

pub trait SafetyFactorTrait {
    fn estate(&self) -> SafetyFactor;
}

impl fmt::Display for SafetyFactor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.safety_factor)
    }
}

impl SafetyFactorTrait for SafetyFactor {
    // 土質力学 p.281
    fn estate(&self) -> SafetyFactor {
        
        let mut upper_value = 0.;
        let mut lower_value = 0.;
        
        for block in &self.blocks {
            let m_alpha = (1. + block.alpha.tan() * block.phi.tan() / self.safety_factor) * block.alpha.cos().powi(2);


            upper_value += (block.w * block.phi.tan() + block.c * block.b) / m_alpha;
            lower_value += block.w * block.alpha.tan();
        }

        return SafetyFactor {
            safety_factor: upper_value / lower_value,
            blocks: self.blocks.clone(),
        };
    }
}
