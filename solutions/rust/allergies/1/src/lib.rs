pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Allergen {
    Eggs,       
    Peanuts,     
    Shellfish,   
    Strawberries,
    Tomatoes,    
    Chocolate,  
    Pollen,      
    Cats,     
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    //Check if a specific allergen is present in the score
    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let value = match allergen {
            Allergen::Eggs => 1,
            Allergen::Peanuts => 2,
            Allergen::Shellfish => 4,
            Allergen::Strawberries => 8,
            Allergen::Tomatoes => 16,
            Allergen::Chocolate => 32,
            Allergen::Pollen => 64,
            Allergen::Cats => 128,
        };
        self.score & value != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let all = vec![
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];

        let mut result = Vec::new();
        for allergen in all {
            if self.is_allergic_to(&allergen) {
                result.push(allergen);
            }
        }
        result
    }
}
