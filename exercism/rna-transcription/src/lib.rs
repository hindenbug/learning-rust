#[derive(PartialEq)]
#[derive(Debug)]

pub struct RibonucleicAcid(String);
pub struct DeoxyribonucleicAcid(String);

impl RibonucleicAcid{
    pub fn new(strand: &str) -> RibonucleicAcid{
        RibonucleicAcid(String::from(strand))
    }

}

impl DeoxyribonucleicAcid{
    pub fn new(strand: &str) -> DeoxyribonucleicAcid{
        DeoxyribonucleicAcid(String::from(strand))
    }

    pub fn to_rna(&self) -> RibonucleicAcid {
        let ref strand = *self.0;
        RibonucleicAcid(strand.chars().map(|c| map_dna_nucleotide_to_rna(c)) .collect())
    }
}

fn map_dna_nucleotide_to_rna(c: char) -> char {
    match c {
        'G' => 'C',
        'C' => 'G',
        'T' => 'A',
        'A' => 'U',
        _ => c
    }
}
