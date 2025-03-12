use datafusion::arrow::array::{Array, AsArray, ListArray, StringArray};
use datafusion::functions_aggregate::array_agg::array_agg;
use datafusion::functions_aggregate::string_agg::{StringAgg};
use datafusion::sql::sqlparser::ast::Array;
use datafusion::{prelude::*, sql::parser::DFParser};
use datafusion::arrow::util::pretty::print_batches; // For nice formatting

use tokio;

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    println!("Running AA changes.");
    let amino_acid_static_dataset = "aa.tsv";
    let ctx = SessionContext::new();
    let mut options = CsvReadOptions::new();
    options = options.delimiter(b'\t');
    options = options.file_extension("tsv");
    let result = ctx.read_csv(amino_acid_static_dataset, options);

    let df = match result.await {
        Ok(df) => df,
        Err(msg) => panic!("{msg}")
    };

    let aggregations = df.aggregate(
        vec![col("aminoacid"), col("letter"), col("fullname")], 
        vec![(array_agg(col("codon"))).alias("codons")]
    )?.collect().await?;

    print_batches(&aggregations);
    for batch in aggregations {
        let short_names = batch.column_by_name("aminoacid").unwrap().as_any().downcast_ref::<StringArray>().expect("Failed to cast to StringArray");
        let full_names = batch.column_by_name("fullname").unwrap().as_any().downcast_ref::<StringArray>().expect("Failed to cast to StringArray");
        let letters = batch.column_by_name("letter").unwrap().as_any().downcast_ref::<StringArray>().expect("Failed to cast to StringArray");
        let codons = batch.column_by_name("codons").unwrap().as_any().downcast_ref::<ListArray>().expect("Failed");
     

        for i in 0..batch.num_rows() {
            let short_name = short_names.value(i);
            let full_name = full_names.value(i);
            let letter = letters.value(i);
            let codon_set = codons.value(i).as_any().downcast_ref::<StringArray>().expect("Failed").iter().map();
            // let codon_set_len = codon_set.len().to_owned();
            // for j in 0..codon_set.len() {
            //     let codon = codons.value(j);
            // }
        }

    }    

    Ok(())
}



struct AminoAcid {
    letter: char,
    codon: [char; 3],
    full_name: String,
    short_name: String,
}


struct AminoAcidAlphabet {
    amino_acids: Vec<AminoAcid>,
}