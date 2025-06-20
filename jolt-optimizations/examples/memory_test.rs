//! Memory test example for GLV-4 precomputation methods
//! 
//! This example measures the memory usage of different precomputation strategies
//! for GLV-4 scalar multiplication on BN254 G2 points.

use ark_ff::UniformRand;
use ark_std::test_rng;
use jolt_optimizations::{
    glv_four_precompute, glv_four_precompute_windowed2_signed,
    G2Projective, PrecomputedShamir4Data, Windowed2Signed4Data,
};
use std::env;
use std::mem;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let num_points = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(100)
    } else {
        100
    };

    println!("Memory Test for GLV-4 Precomputation Methods");
    println!("Testing with {} G2 points", num_points);
    println!("{}", "=".repeat(60));

    // Generate random G2 points
    let mut rng = test_rng();
    let points: Vec<G2Projective> = (0..( 1 << num_points))
        .map(|_| G2Projective::rand(&mut rng))
        .collect();

    // Measure size of a single G2Projective point
    let g2_size = mem::size_of::<G2Projective>();1
    println!("\nBase sizes:");
    println!("  Size of G2Projective: {} bytes", g2_size);
    println!("  Size of Vec<G2Projective> overhead: {} bytes", 
             mem::size_of::<Vec<G2Projective>>());

    // Method 1: Online (no precomputation)
    println!("\n1. Online Method (no precomputation):");
    println!("   Memory per point: 0 bytes");
    println!("   Total memory: 0 bytes");

    // Method 2: Full Shamir table (256 entries per point)
    println!("\n2. Full Shamir Table Precomputation:");
    let shamir_data = glv_four_precompute(&points);
    measure_shamir_memory(&shamir_data, num_points, g2_size);

    // Method 3: 2-bit windowed signed (24 entries per point)
    println!("\n3. 2-bit Windowed Signed Precomputation:");
    let windowed_data = glv_four_precompute_windowed2_signed(&points);
    measure_windowed_memory(&windowed_data, num_points, g2_size);

    // Summary
    println!("\n{}", "=".repeat(60));
    println!("Summary for {} points:", num_points);
    println!("  Online: 0 KB");
    println!("  Full Shamir: ~{:.2} KB", 
             (num_points as f64 * 256.0 * g2_size as f64) / 1024.0);
    println!("  2-bit Windowed: ~{:.2} KB", 
             (num_points as f64 * 24.0 * g2_size as f64) / 1024.0);
}

fn measure_shamir_memory(data: &PrecomputedShamir4Data, num_points: usize, g2_size: usize) {
    // Calculate memory usage
    let table_size = if !data.shamir_tables.is_empty() {
        // Each table contains 256 G2 points
        256 * g2_size
    } else {
        0
    };
    
    let total_tables_size = table_size * num_points;
    let overhead = mem::size_of_val(data) + 
                   mem::size_of_val(&data.shamir_tables[..]);
    
    println!("   Entries per point: 256");
    println!("   Memory per point: {} bytes ({:.2} KB)", 
             table_size, table_size as f64 / 1024.0);
    println!("   Total table memory: {} bytes ({:.2} MB)", 
             total_tables_size, total_tables_size as f64 / (1024.0 * 1024.0));
    println!("   Overhead: {} bytes", overhead);
    println!("   Total memory: {} bytes ({:.2} MB)", 
             total_tables_size + overhead, 
             (total_tables_size + overhead) as f64 / (1024.0 * 1024.0));
}

fn measure_windowed_memory(data: &Windowed2Signed4Data, num_points: usize, g2_size: usize) {
    // Calculate memory usage
    let table_size = if !data.windowed2_tables.is_empty() {
        // Each table contains 24 G2 points
        24 * g2_size
    } else {
        0
    };
    
    let total_tables_size = table_size * num_points;
    let overhead = mem::size_of_val(data) + 
                   mem::size_of_val(&data.windowed2_tables[..]);
    
    println!("   Entries per point: 24");
    println!("   Memory per point: {} bytes ({:.2} KB)", 
             table_size, table_size as f64 / 1024.0);
    println!("   Total table memory: {} bytes ({:.2} MB)", 
             total_tables_size, total_tables_size as f64 / (1024.0 * 1024.0));
    println!("   Overhead: {} bytes", overhead);
    println!("   Total memory: {} bytes ({:.2} MB)", 
             total_tables_size + overhead, 
             (total_tables_size + overhead) as f64 / (1024.0 * 1024.0));
    
    // Memory savings compared to full Shamir
    let shamir_memory = 256 * g2_size * num_points;
    let savings_percent = (1.0 - (total_tables_size as f64 / shamir_memory as f64)) * 100.0;
    println!("   Memory savings vs Full Shamir: {:.1}%", savings_percent);
}