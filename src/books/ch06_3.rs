

#[derive(Debug)]
pub enum Side {
    Left,
    Right,
}

impl Side {
    fn add(&self, other: &str) -> String {
        let mut ch = match self {
            Side::Left => String::from("L"),
            Side::Right => String::from("R"),
        };
        ch.push_str(other);

        ch
    }
}

pub enum FW {
    CentreForward,
    SecondStriker,
    Winger(Side),
}

pub enum MF {
    AttackingMidfielder,
    CentreMidfielder,
    SideMidfielder(Side),
    DefensiveMidfielder,
}

pub enum DF {
    CentreBack,
    FullBack(Side),
    WingBack(Side),
}

// #[derive(Debug)]
pub enum Position {
    Forward(FW),
    Midfielder(MF),
    Defender(DF),
    Goalkeeper,
}

pub fn make_all_positions() -> [Position; 15] {
    let cf  = Position::Forward(FW::CentreForward);
    let ss  = Position::Forward(FW::SecondStriker);
    let lwf = Position::Forward(FW::Winger(Side::Left));
    let rwf = Position::Forward(FW::Winger(Side::Right));
    let am  = Position::Midfielder(MF::AttackingMidfielder);
    let cm  = Position::Midfielder(MF::CentreMidfielder);
    let lm  = Position::Midfielder(MF::SideMidfielder(Side::Left));
    let rm  = Position::Midfielder(MF::SideMidfielder(Side::Right));
    let dm  = Position::Midfielder(MF::DefensiveMidfielder);
    let cb  = Position::Defender(DF::CentreBack);
    let lb  = Position::Defender(DF::FullBack(Side::Left));
    let rb  = Position::Defender(DF::FullBack(Side::Right));
    let lwb = Position::Defender(DF::WingBack(Side::Left));
    let rwb = Position::Defender(DF::WingBack(Side::Right));
    let gk  = Position::Goalkeeper;

    let positions = [
        cf, ss, lwf, rwf,
        am, cm, lm, rm, dm,
        cb, lb, rb, lwb, rwb,
        gk,
    ];

    positions
}

impl Position {
    fn value_in_position(&self) -> &str {
        match self {
            Position::Forward(fw) => {
                match fw {
                    FW::CentreForward => "CF",
                    FW::SecondStriker => "SS",
                    FW::Winger(side) => {
                        match side {
                            Side::Left => "LWF",
                            Side::Right => "RWF",
                        }
                    },
                }
            },
            Position::Midfielder(mf) => {
                match mf {
                    MF::AttackingMidfielder => "AM",
                    MF::CentreMidfielder => "CM",
                    MF::SideMidfielder(side) => {
                        match side {
                            Side::Left => "LM",
                            Side::Right => "RM",
                        }
                    },
                    MF::DefensiveMidfielder => "DM",
                }
            },
            Position::Defender(df) => {
                match df {
                    DF::CentreBack => "CB",
                    DF::FullBack(side) => {
                        match side {
                            Side::Left => "LB",
                            Side::Right => "RB",
                        }
                    },
                    DF::WingBack(side) => {
                        match side {
                            Side::Left => "LWB",
                            Side::Right => "RWB",
                        }
                    },
                }
            },
            Position::Goalkeeper => { "GK" },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let lwf = Position::Forward(FW::Winger(Side::Left));
        let am = Position::Midfielder(MF::AttackingMidfielder);
        let rb = Position::Defender(DF::FullBack(Side::Right));
        let gk = Position::Goalkeeper;

        assert_eq!(lwf.value_in_position(), "LWF");
        assert_eq!(am.value_in_position(), "AM");
        assert_eq!(rb.value_in_position(), "RB");
        assert_eq!(gk.value_in_position(), "GK");
    }

    // #[test]
    // fn works_1() {
    //     let left = Side::Left;
    //     let right = Side::Right;

    //     let letter = format!("{}WF", left);
    //     println!("letter: {}", letter);

    //     println!("left: {}", left);
    //     println!("right: {}", right);
    // }

}
