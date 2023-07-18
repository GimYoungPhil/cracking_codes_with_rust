#[derive(Debug)]
enum WhiskyKind {
    Blended,
    SingleMalt,
    Bourbone,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let chivas_regal = WhiskyKind::Blended;
        let glenfiddich = WhiskyKind::SingleMalt;
        let wild_turkey = WhiskyKind::Bourbone;

        println!("{:?}", chivas_regal);
        println!("{:?}", glenfiddich);
        println!("{:?}", wild_turkey);
    }

    #[test]
    fn works_0() {
        let wild = WhiskyKind::Bourbone;

        let kink = match wild {
            WhiskyKind::Blended => { "블랜디드" },
            WhiskyKind::SingleMalt => { "싱글몰트" },
            WhiskyKind::Bourbone => { "버번" },
        };

        assert_eq!(kink, "버번");
    }

    // #[test]
    // fn works_1() {
    //     let wild = WhiskyKind::Bourbone;

    //     if wild == WhiskyKind::Bourbone {
    //         println!("{:?}", wild);
    //     }
    // }

    #[test]
    fn works_2() {
        let wild = WhiskyKind::Bourbone;

        if let wild = WhiskyKind::Bourbone {
            println!("아메리카?");
        }

    }
}
