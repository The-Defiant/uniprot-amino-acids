use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use clap::Parser;
fn main(){
    println!("Running AA changes.");
    let amino_acid_static_dataset = "aa.tsv";
    let file = File::open(amino_acid_static_dataset).unwrap();
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines();
    let alphabet: &mut AminoAcidAlphabet = &mut AminoAcidAlphabet::new();
    while let Some(Ok(line)) = lines.next() {
        let split: Vec<String> = line.split_whitespace().map(String::from).collect();
        let amino_acid = AminoAcid::from_row(split.as_slice());
        alphabet.add(amino_acid);
    }

    let a1 = 'G';
    let a2 = 'V';
    println!("{:?}", alphabet.find_combination(a1, a2))
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct AminoAcid {
    letter: char,
    codon: [char; 3],
    full_name: String,
    short_name: String,
}

impl AminoAcid  {
    fn from_row(row: &[String]) -> Self{
        AminoAcid {
            codon: row[0].chars().collect::<Vec<_>>().try_into().expect("Wrong size"),
            short_name: row[1].to_owned(),
            letter: row[2].chars().next().expect("No char"),
            full_name: row[3].to_owned(),
        }
    }
    
}

#[derive(Debug)]
struct AminoAcidAlphabet {
    amino_acids: HashMap<char, Vec<[char; 3]>>,
}


impl AminoAcidAlphabet {
    fn new() -> Self {
        AminoAcidAlphabet {
            amino_acids: HashMap::new()
        }
    }
    fn add(&mut self, amino_acid: AminoAcid) {
        let check = self.amino_acids.get_mut(&amino_acid.letter);
        match check {
            Some(codons) => {codons.push(amino_acid.codon); },
            None =>{ self.amino_acids.insert(amino_acid.letter,  vec![amino_acid.codon]); },
        }
    }

    fn find_combination(&self, a1: char, a2: char) -> (&Vec<[char;3]>, &Vec<[char;3]>){
        let aa1 = self.amino_acids.get(&a1).unwrap();
        let aa2 = self.amino_acids.get(&a2).unwrap();
        (aa1, aa2)
    } 
}