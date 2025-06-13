use crate::models::BudgetCategory;

pub fn get_default_categories() -> Vec<BudgetCategory> {
    vec![
        BudgetCategory {
            name: "INCOME".to_string(),
            subcategories: vec![
                "Wages".to_string(),
                "Interest/dividends".to_string(),
                "Miscellaneous".to_string(),
            ],
            is_income: true,
        },
        BudgetCategory {
            name: "HOME".to_string(),
            subcategories: vec![
                "Mortgage".to_string(),
                "Insurance".to_string(),
                "Repairs".to_string(),
                "Services".to_string(),
                "Enbridge gas".to_string(),
                "Hydro electricity".to_string(),
            ],
            is_income: false,
        },
        BudgetCategory {
            name: "DAILY LIVING".to_string(),
            subcategories: vec![
                "Groceries".to_string(),
                "Child care".to_string(),
                "Dry cleaning".to_string(),
                "Dining out".to_string(),
                "Housecleaning service".to_string(),
                "Dog walker".to_string(),
            ],
            is_income: false,
        },
        BudgetCategory {
            name: "TRANSPORTATION".to_string(),
            subcategories: vec![
                "Gas/fuel".to_string(),
                "Insurance".to_string(),
                "Car wash/detailing services".to_string(),
                "Parking".to_string(),
                "Public transportation".to_string(),
            ],
            is_income: false,
        },
        BudgetCategory {
            name: "ENTERTAINMENT".to_string(),
            subcategories: vec![
                "Cable TV".to_string(),
                "Video/DVD rentals".to_string(),
                "Movies/plays".to_string(),
                "Concerts/clubs".to_string(),
            ],
            is_income: false,
        },
        BudgetCategory {
            name: "HEALTH".to_string(),
            subcategories: vec![
                "Health club dues".to_string(),
                "Insurance".to_string(),
                "Prescriptions".to_string(),
                "Over-the-counter drugs".to_string(),
                "Co-payments/out-of-pocket".to_string(),
                "Veterinarians/pet medicines".to_string(),
                "Life insurance".to_string(),
            ],
            is_income: false,
        },
        BudgetCategory {
            name: "MISC PAYMENTS".to_string(),
            subcategories: vec![
                "Uoft loans".to_string(),
                "IT bootcamp loans".to_string(),
                "Netflix".to_string(),
                "Chat GPT".to_string(),
                "Apple cloud".to_string(),
            ],
            is_income: false,
        },
    ]
}
