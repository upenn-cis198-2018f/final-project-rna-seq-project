extern crate primes;
extern crate rayon;
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate structopt;
#[macro_use]

mod dna_hash_table;
mod dna_read_graph;
mod read_inputs;
mod args;

use env_logger::Env;
use rayon::prelude::*;
use std::collections::HashMap;
use dna_hash_table::DNAHashTable;
use structopt::StructOpt;
use log::*;

fn main() {

    let opt = args::Opt::from_args();

    if opt.verbose {
        
        let env = Env::default()
           .filter_or("MY_LOG_LEVEL", "trace")
           .write_style_or("MY_LOG_STYLE", "always");

        env_logger::init_from_env(env);
        
    } else {
        let env = Env::default()
           .filter_or("MY_LOG_LEVEL", "info")
           .write_style_or("MY_LOG_STYLE", "always");

        env_logger::init_from_env(env);
    }

    let fa_col_db = read_inputs::read_fa_file_to_cols(opt.seq_input_filename.as_str());

    for s in &fa_col_db.seg_ids {
        debug!("Read in {}", s);
    }
    let kmer_hash_table = DNAHashTable::new(&fa_col_db.seg_strings, opt.k);
    let mut reads2 : Vec<String> = read_inputs::read_fq_fasta_file(opt.read_input_filename.as_str());

    let mut global_map = perform_map_reduce(opt.n_partition, &mut reads2, &kmer_hash_table);

    read_inputs::write_output4(opt.seqcount_output_filename.as_str(), &mut global_map, fa_col_db.seg_ids);
}

fn perform_map_reduce(n_partition : usize, 
        reads : &mut Vec<String>, 
        kmer_hash_table : &DNAHashTable) -> HashMap<i32, i32> {

    let ratio : usize = (reads.len() / n_partition) as usize;

    let mut partitions :Vec<Vec<String>> = Vec::new();
    let mut i = 0;
    for chunk in reads.chunks_mut(ratio) {
        for c in chunk.iter() {
            debug!("Partition created {} {}", i, c);
        }
        i = i + 1;

        partitions.push(chunk.to_vec());
    } 

    i = 0;
    let partitions_map = partitions.par_iter()
        .map(|ref chunk| dna_hash_table::get_segments(&kmer_hash_table, &chunk)); 
    let comp_result : Vec<HashMap<i32, i32>> = partitions_map.collect();

    let mut global_map : HashMap<i32, i32> = HashMap::new();

    debug!("Length {}", comp_result.len());
    for p in comp_result.iter() {
        for (k, v) in p.iter() {
            debug!("Partition {} has {} {}", i, k, v);

            let cnt = global_map.entry(*k).or_insert(0);
            *cnt += *v;
        }
        i = i + 1;
    }

    return global_map
} 




