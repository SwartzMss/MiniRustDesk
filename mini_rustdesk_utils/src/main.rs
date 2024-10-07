use dns_lookup::{lookup_addr, lookup_host};
use sodiumoxide::crypto::sign;
use std::{
    env,
    net::{IpAddr, TcpStream},
    process, str,
};


fn print_help() {
    println!(
        "Usage:
    mini_rustdesk_utils [command]\n
Available Commands:
    genkeypair                                   Generate a new keypair
    validatekeypair [public key] [secret key]    Validate an existing keypair
    doctor [rustdesk-server]                     Check for server connection problems"
    );
    process::exit(0x0001);
}

fn gen_keypair() {
    let (pk, sk) = sign::gen_keypair();
    let public_key = base64::encode(pk);
    let secret_key = base64::encode(sk);
    println!("Public Key:  {public_key}");
    println!("Secret Key:  {secret_key}");
}


fn validate_keypair(pk: &str, sk: &str) -> Result<(), &'static str> {
    let sk1 = base64::decode(sk).map_err(|_|"Invalid secret key")?;
    let secret_key = sign::SecretKey::from_slice(sk1.as_slice()).ok_or("Invalid Secret key")?;

    let pk1 = base64::decode(pk).map_err(|_| "Invalid public key")?;
    let public_key = sign::PublicKey::from_slice(pk1.as_slice()).ok_or("Invalid Public key")?;

    let random_data_to_test = b"This is meh.";
    let signed_data = sign::sign(random_data_to_test, &secret_key);
    let verified_data = sign::verify(&signed_data, &public_key).map_err(|_| "Key pair is INVALID")?;

    if random_data_to_test != &verified_data[..] {
        return Err("Key pair is INVALID");
    }

    Ok(())
}

fn error_then_help(msg: &str) {
    println!("ERROR: {msg}\n");
    print_help();
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        print_help();
    }

    let command = args[1].to_lowercase();
    match command.as_str() {
        "genkeypair" => gen_keypair(),
        "validatekeypair" => {
            if args.len() <= 3 {
                error_then_help("You must supply both the public and the secret key");
            }
            let res = validate_keypair(args[2].as_str(), args[3].as_str());
            if let Err(e) = res {
                println!("{e}");
                process::exit(0x0001);
            }
            println!("Key pair is VALID");
        }
        "doctor" => {
            if args.len() <= 2 {
                error_then_help("You must supply the mini_rustdesk-server address");
            }
            // doctor(args[2].as_str());
        }
        _ => print_help(),
    }
}
