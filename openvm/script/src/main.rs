use clap::Parser;
use openvm_benchmarks_prove::util::BenchmarkCli;
use openvm_circuit::arch::instructions::exe::VmExe;
use openvm_sdk::config::{AppConfig, SdkVmConfig};
use openvm_sdk::{StdIn, F};
use openvm_stark_sdk::bench::run_with_metric_collection;
use ecies::SecretKey;

pub const ELF: &[u8] = include_bytes!("../../target/openvm/release/ecies-program.vmexe");

fn main() {
    let args: BenchmarkCli = BenchmarkCli::parse();

    let app_config: AppConfig<SdkVmConfig> = toml::from_str(include_str!("../../program/openvm.toml")).unwrap();

    let app_exe: VmExe<F> = bitcode::deserialize(ELF).unwrap();

    let secret_key = SecretKey::random(&mut rand::rng());
    let public_key = secret_key.public_key();
    let address: [u8; 20] = rand::random();
    let ciphertext = public_key.encrypt(&mut rand::rng(), &address);
    let decrypted = secret_key.try_decrypt(&ciphertext).unwrap();
    assert_eq!(decrypted, address);
    println!("[+] address: {:?}", address);
    println!("[+] encrypted address length: {}", ciphertext.len());
    println!("[+] Running benchmark:");

    let repetitions = 1000;

    run_with_metric_collection("OUT_PATH", || {
        let mut stdin = StdIn::default();
        stdin.write(&repetitions);
        stdin.write(&true);
        stdin.write_bytes(&secret_key.to_bytes());
        stdin.write_bytes(&ciphertext);

        args.bench_from_exe("baseline", app_config.app_vm_config.clone(), app_exe.clone(), stdin).unwrap();

        let mut stdin = StdIn::default();
        stdin.write(&repetitions);
        stdin.write(&false);
        stdin.write_bytes(&secret_key.to_bytes());
        stdin.write_bytes(&ciphertext);
        args.bench_from_exe("all", app_config.app_vm_config.clone(), app_exe.clone(), stdin).unwrap();
    });
}
