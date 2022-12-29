#[derive(Clone, Copy, PartialEq, Debug)]
enum SRD5Piece {
    Copper,
    Silver,
    Electrum,
    Gold,
    Platinum,
}

trait Piece {}
impl Piece for SRD5Piece {}

#[derive(PartialEq, Debug)]
struct BottomRatio<T: Piece + PartialEq> {
    piece: T,
    ratio: f64,
}

impl<T> BottomRatio<T>
where
    T: Piece + PartialEq,
{
    const fn new(piece: T, ratio: f64) -> Self {
        Self { piece, ratio }
    }
    fn get_piece(&self) -> &T {
        &self.piece
    }
    fn get_ratio(&self) -> f64 {
        self.ratio
    }
}

#[derive(PartialEq, Debug)]
struct Coin<T: Piece + PartialEq> {
    piece: T,
    value: u64,
}

impl<T> Coin<T>
where
    T: Piece + PartialEq,
{
    fn new(piece: T, value: u64) -> Self {
        Self { piece, value }
    }
    fn get_piece(&self) -> &T {
        &self.piece
    }
    fn get_value(&self) -> u64 {
        self.value
    }
}

struct SRD5Converter;

impl SRD5Converter {
    const RATIO: &'static [BottomRatio<SRD5Piece>] = &[
        BottomRatio::new(SRD5Piece::Copper, 1.0),
        BottomRatio::new(SRD5Piece::Silver, 0.1),
        BottomRatio::new(SRD5Piece::Electrum, 0.05),
        BottomRatio::new(SRD5Piece::Gold, 0.01),
        BottomRatio::new(SRD5Piece::Platinum, 0.001),
    ];
    fn convert(coin: Coin<SRD5Piece>) -> Vec<Coin<SRD5Piece>> {
        let copper_value = coin.get_value() as f64
            / SRD5Converter::RATIO
                .iter()
                .find(|&ratio| ratio.get_piece() == coin.get_piece())
                .expect("Coin should use a valid piece for this converter")
                .get_ratio();
        SRD5Converter::RATIO
            .iter()
            .map(|ratio| {
                Coin::new(
                    *ratio.get_piece(),
                    (copper_value as f64 * ratio.get_ratio()) as u64,
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platinum_conversion() {
        let out = vec![
            Coin::new(SRD5Piece::Copper, 1000),
            Coin::new(SRD5Piece::Silver, 100),
            Coin::new(SRD5Piece::Electrum, 50),
            Coin::new(SRD5Piece::Gold, 10),
            Coin::new(SRD5Piece::Platinum, 1),
        ];
        assert!(out == SRD5Converter::convert(Coin::new(SRD5Piece::Platinum, 1)));
    }
}
