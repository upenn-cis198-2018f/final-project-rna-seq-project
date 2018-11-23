use primes::PrimeSet;
use std::cmp;

pub struct DNAHashTable {
	hash_table : Vec<Vec<Kmer>>,
	pub size : usize,
	k : usize,
	j : usize,
}

impl DNAHashTable {
	
	//k is the size of the k-mers to be hashed, and j is the maximum index of the k-mer that can be used for hashing before overflow
	pub fn new(segments : &Vec<String>, k : usize) -> DNAHashTable {
		let j : usize = DNAHashTable::get_max_j(k);
		let size : usize = DNAHashTable::get_table_size(segments, k);
		let mut hash_table : Vec<Vec<Kmer>> = vec![Vec::<Kmer>::new(); size];

		for segment_index in 0..segments.len() {
			let segment : &String = &segments[segment_index];
			for i in 0..(segment.len() - k + 1) {
				let hash_value : usize = DNAHashTable::hash_function(&segment[i..(i+k)], j, size);
				hash_table[hash_value].push(Kmer {
					segment_index : segment_index,
					position : i,
				});
			}
		}

		DNAHashTable {
			hash_table : hash_table,
			size : size,
			k : k,
			j : j
		}
	}

	pub fn get_kmer(&self, kmer_string : &String) -> Option<Vec<Kmer>> {
		if kmer_string.len() != self.k {
			None
		} else {
			let hash_value : usize = DNAHashTable::hash_function(kmer_string, self.j, self.size);
			let hash_element : &Vec<Kmer> = &self.hash_table[hash_value];
			if hash_element.len() == 0 {
				None
			} else {
				Some(&hash_element[])
			}
		}
	}

	//Get the table size
	fn get_table_size(segments : &Vec<String>, k : usize) -> usize {
		let mut size : usize = 0;
		let mut pset = PrimeSet::new();
		
		for segment in segments {
			size += segment.len() - k + 1;
		}
		let new_size = (size * 13) / 10; //Recommended to make size 1.3 times number of keys
		let (_index, prime_size) = pset.find(new_size as u64);
		prime_size as usize
	}

	fn get_max_j(k : usize) -> usize {
		let max_value : usize = usize::max_value();
		let j : usize = DNAHashTable::integer_log_base_4(max_value);
		cmp::min(j, k)

	}

	fn integer_log_base_4(mut value : usize) -> usize {
		let mut i = 0;
		while value > 0 {
			value /= 4;
			i += 1;
		}
		i - 1
	}

	fn dna_to_int(dna_letter : char) -> usize {
		match dna_letter {
			'A' => 0,
			'C' => 1,
			'G' => 2,
			'T' => 3,
			_ => 0,
		}
	}

	fn hash_function(kmer : &str, j : usize, size : usize) -> usize {
		let mut hash_value : usize = 0;
		let mut i = 0;
		for dna_letter in kmer[0..j].chars() {
			hash_value += DNAHashTable::dna_to_int(dna_letter) * 4_usize.pow(i);
			i += 1;
		}
		hash_value % size
	}
}

//A locus is a location in the genome, which we represent by the segment that the k-mer mapped to and the location on the segment
#[derive(Clone)]
pub struct Kmer {
	pub segment_index : usize,
	pub position : usize,
}