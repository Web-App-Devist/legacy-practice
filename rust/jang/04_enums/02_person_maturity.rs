enum PersonMaturityStage {
    CHILD,
    ADULT,
}

fn check_person_maturity_state() {
    let age = 21;
    let maturity_stage = match age {
        0..=17 => PersonMaturityStage::CHILD,
        _ => PersonMaturityStage::ADULT,
    };

    let stage_str = match maturity_stage {
        PersonMaturityStage::CHILD => "child",
        PersonMaturityStage::ADULT => "adult",
    };

    println!("{}", stage_str);
}

fn main() {
    check_person_maturity_state();
}
